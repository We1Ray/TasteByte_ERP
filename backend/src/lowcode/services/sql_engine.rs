use sqlparser::ast::{Expr, Statement, VisitMut, VisitorMut};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use sqlx::postgres::PgRow;
use sqlx::{Column, PgPool, Row};

use crate::shared::AppError;

/// Blocked functions that could be dangerous
const BLOCKED_FUNCTIONS: &[&str] = &[
    "pg_sleep",
    "dblink",
    "dblink_connect",
    "dblink_exec",
    "lo_import",
    "lo_export",
    "pg_read_file",
    "pg_read_binary_file",
    "pg_write_file",
    "pg_ls_dir",
    "pg_stat_file",
    "pg_terminate_backend",
    "pg_cancel_backend",
    "pg_reload_conf",
    "set_config",
    "current_setting",
    "pg_advisory_lock",
    "pg_advisory_unlock",
];

const MAX_ROWS: usize = 1000;
const STATEMENT_TIMEOUT_MS: u32 = 5000;

/// Layer 1: Validate SQL at save time. Only SELECT is allowed.
pub fn validate_sql(sql: &str) -> Result<(), AppError> {
    let dialect = PostgreSqlDialect {};
    let ast = Parser::parse_sql(&dialect, sql)
        .map_err(|e| AppError::Validation(format!("SQL parse error: {e}")))?;

    if ast.is_empty() {
        return Err(AppError::Validation("Empty SQL statement".to_string()));
    }

    for stmt in &ast {
        match stmt {
            Statement::Query(_) => {}
            _ => {
                return Err(AppError::Validation(
                    "Only SELECT statements are allowed".to_string(),
                ));
            }
        }
    }

    // Check for blocked functions by walking the AST
    let mut checker = BlockedFunctionChecker { found: None };
    let mut ast_mut = ast;
    for stmt in &mut ast_mut {
        let _ = stmt.visit(&mut checker);
        if let Some(ref name) = checker.found {
            return Err(AppError::Validation(format!("Blocked function: {name}")));
        }
    }

    Ok(())
}

struct BlockedFunctionChecker {
    found: Option<String>,
}

impl VisitorMut for BlockedFunctionChecker {
    type Break = ();

    fn pre_visit_expr(&mut self, expr: &mut Expr) -> std::ops::ControlFlow<Self::Break> {
        if let Expr::Function(func) = expr {
            let name = func.name.to_string().to_lowercase();
            for blocked in BLOCKED_FUNCTIONS {
                if name.contains(blocked) {
                    self.found = Some(name);
                    return std::ops::ControlFlow::Break(());
                }
            }
        }
        std::ops::ControlFlow::Continue(())
    }
}

/// Safe functions allowed in filter expressions.
const SAFE_FILTER_FUNCTIONS: &[&str] = &[
    "lower",
    "upper",
    "trim",
    "ltrim",
    "rtrim",
    "length",
    "coalesce",
    "nullif",
    "now",
    "current_date",
    "current_timestamp",
    "date_trunc",
    "extract",
    "cast",
    "abs",
    "ceil",
    "floor",
    "round",
];

/// Validate a filter_sql expression (used in record policies).
/// The expression must be a valid boolean expression that can appear in a WHERE clause.
/// Only allows comparisons, AND, OR, IN, BETWEEN, IS NULL, LIKE, and safe functions.
/// Blocks subqueries, DML, DDL, and dangerous function calls.
pub fn validate_filter_expression(sql: &str) -> Result<(), String> {
    let trimmed = sql.trim();
    if trimmed.is_empty() {
        return Err("Empty filter expression".to_string());
    }

    // Wrap the expression in a SELECT WHERE to parse it as valid SQL
    let wrapper = format!("SELECT 1 WHERE {trimmed}");
    let dialect = PostgreSqlDialect {};
    let ast = Parser::parse_sql(&dialect, &wrapper)
        .map_err(|e| format!("Filter SQL parse error: {e}"))?;

    if ast.len() != 1 {
        return Err("Filter must be a single expression".to_string());
    }

    // Validate the AST: walk all expressions and block unsafe constructs
    let mut checker = FilterExpressionChecker { error: None };
    let mut ast_mut = ast;
    for stmt in &mut ast_mut {
        let _ = stmt.visit(&mut checker);
        if let Some(ref err) = checker.error {
            return Err(err.clone());
        }
    }

    Ok(())
}

struct FilterExpressionChecker {
    error: Option<String>,
}

impl VisitorMut for FilterExpressionChecker {
    type Break = ();

    fn pre_visit_expr(&mut self, expr: &mut Expr) -> std::ops::ControlFlow<Self::Break> {
        match expr {
            // Block subqueries
            Expr::Subquery(_) | Expr::Exists { .. } | Expr::InSubquery { .. } => {
                self.error = Some("Subqueries are not allowed in filter expressions".to_string());
                return std::ops::ControlFlow::Break(());
            }
            // Check function calls against allowlist
            Expr::Function(func) => {
                let name = func.name.to_string().to_lowercase();
                // Check blocked functions first
                for blocked in BLOCKED_FUNCTIONS {
                    if name.contains(blocked) {
                        self.error = Some(format!("Blocked function in filter: {name}"));
                        return std::ops::ControlFlow::Break(());
                    }
                }
                // Then check allowlist
                if !SAFE_FILTER_FUNCTIONS.iter().any(|&safe| name == safe) {
                    self.error = Some(format!(
                        "Function '{}' is not allowed in filter expressions",
                        name
                    ));
                    return std::ops::ControlFlow::Break(());
                }
            }
            _ => {}
        }
        std::ops::ControlFlow::Continue(())
    }
}

/// Layer 2+3+4: Execute a validated SQL with params, read-only, with timeout and row limit.
/// Named params like $search in the input are mapped to positional $1, $2, etc.
pub async fn execute_safe_query(
    pool: &PgPool,
    sql: &str,
    params: &Option<serde_json::Value>,
) -> Result<(Vec<String>, Vec<serde_json::Value>), AppError> {
    // Validate first
    validate_sql(sql)?;

    // Build the final SQL with LIMIT and timeout
    let limited_sql = if sql.to_lowercase().contains("limit") {
        sql.to_string()
    } else {
        format!("{sql} LIMIT {MAX_ROWS}")
    };

    // Replace named params ($search, $param1, etc.) with positional params
    let (final_sql, param_values) = resolve_params(&limited_sql, params)?;

    // Execute in a read-only transaction with timeout
    let mut tx = pool.begin().await?;
    sqlx::query("SET TRANSACTION READ ONLY")
        .execute(&mut *tx)
        .await?;
    sqlx::query(&format!(
        "SET LOCAL statement_timeout = '{STATEMENT_TIMEOUT_MS}ms'"
    ))
    .execute(&mut *tx)
    .await?;

    let mut query = sqlx::query(&final_sql);
    for val in &param_values {
        query = query.bind(val);
    }

    let rows: Vec<PgRow> = query
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| AppError::Validation(format!("Query execution error: {e}")))?;

    tx.rollback().await.ok();

    // Extract column names and row data
    let columns: Vec<String> = if let Some(first) = rows.first() {
        first
            .columns()
            .iter()
            .map(|c| c.name().to_string())
            .collect()
    } else {
        vec![]
    };

    let mut result_rows = Vec::with_capacity(rows.len());
    for row in &rows {
        let mut map = serde_json::Map::new();
        for col in row.columns() {
            let name = col.name();
            let val: serde_json::Value = row
                .try_get::<serde_json::Value, _>(name)
                .or_else(|_| {
                    row.try_get::<String, _>(name)
                        .map(serde_json::Value::String)
                })
                .or_else(|_| {
                    row.try_get::<i64, _>(name)
                        .map(|v| serde_json::Value::Number(v.into()))
                })
                .or_else(|_| row.try_get::<bool, _>(name).map(serde_json::Value::Bool))
                .unwrap_or(serde_json::Value::Null);
            map.insert(name.to_string(), val);
        }
        result_rows.push(serde_json::Value::Object(map));
    }

    Ok((columns, result_rows))
}

/// Replace named params ($search, $param1) with positional ($1, $2, ...)
fn resolve_params(
    sql: &str,
    params: &Option<serde_json::Value>,
) -> Result<(String, Vec<String>), AppError> {
    let params_map = match params {
        Some(serde_json::Value::Object(map)) => map.clone(),
        Some(_) => {
            return Err(AppError::Validation(
                "params must be a JSON object".to_string(),
            ))
        }
        None => return Ok((sql.to_string(), vec![])),
    };

    let mut final_sql = sql.to_string();
    let mut values = Vec::new();
    let mut idx = 1;

    // Sort keys for deterministic replacement
    let mut keys: Vec<String> = params_map.keys().cloned().collect();
    keys.sort_by_key(|b| std::cmp::Reverse(b.len())); // longest first to avoid partial replacements

    for key in &keys {
        let placeholder = format!("${key}");
        if final_sql.contains(&placeholder) {
            let positional = format!("${idx}");
            final_sql = final_sql.replace(&placeholder, &positional);
            let val = params_map
                .get(key)
                .map(|v| match v {
                    serde_json::Value::String(s) => s.clone(),
                    other => other.to_string(),
                })
                .unwrap_or_default();
            values.push(val);
            idx += 1;
        }
    }

    Ok((final_sql, values))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // --- validate_sql tests ---

    #[test]
    fn select_is_allowed() {
        assert!(validate_sql("SELECT * FROM users").is_ok());
    }

    #[test]
    fn insert_is_blocked() {
        assert!(validate_sql("INSERT INTO users (name) VALUES ('x')").is_err());
    }

    #[test]
    fn update_is_blocked() {
        assert!(validate_sql("UPDATE users SET name = 'x'").is_err());
    }

    #[test]
    fn delete_is_blocked() {
        assert!(validate_sql("DELETE FROM users").is_err());
    }

    #[test]
    fn empty_sql_is_rejected() {
        assert!(validate_sql("").is_err());
    }

    #[test]
    fn blocked_function_pg_sleep() {
        let result = validate_sql("SELECT pg_sleep(10)");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Blocked function"));
    }

    #[test]
    fn blocked_function_pg_read_file() {
        let result = validate_sql("SELECT pg_read_file('/etc/passwd')");
        assert!(result.is_err());
    }

    #[test]
    fn select_with_where_clause() {
        assert!(validate_sql("SELECT id, name FROM products WHERE price > 100").is_ok());
    }

    // --- resolve_params tests ---

    #[test]
    fn resolve_named_params() {
        let params = Some(json!({"search": "hello"}));
        let (sql, values) =
            resolve_params("SELECT * FROM t WHERE name = $search", &params).unwrap();
        assert_eq!(sql, "SELECT * FROM t WHERE name = $1");
        assert_eq!(values, vec!["hello"]);
    }

    #[test]
    fn resolve_params_none() {
        let (sql, values) = resolve_params("SELECT 1", &None).unwrap();
        assert_eq!(sql, "SELECT 1");
        assert!(values.is_empty());
    }

    // --- validate_filter_expression tests ---

    #[test]
    fn filter_simple_comparison() {
        assert!(validate_filter_expression("status = 'ACTIVE'").is_ok());
    }

    #[test]
    fn filter_and_or() {
        assert!(validate_filter_expression("status = 'ACTIVE' AND price > 100").is_ok());
    }

    #[test]
    fn filter_in_clause() {
        assert!(validate_filter_expression("status IN ('ACTIVE', 'PENDING')").is_ok());
    }

    #[test]
    fn filter_between() {
        assert!(validate_filter_expression("price BETWEEN 10 AND 100").is_ok());
    }

    #[test]
    fn filter_is_null() {
        assert!(validate_filter_expression("deleted_at IS NULL").is_ok());
    }

    #[test]
    fn filter_like() {
        assert!(validate_filter_expression("name LIKE '%test%'").is_ok());
    }

    #[test]
    fn filter_safe_function_lower() {
        assert!(validate_filter_expression("lower(name) = 'test'").is_ok());
    }

    #[test]
    fn filter_blocks_subquery() {
        let result = validate_filter_expression("id IN (SELECT id FROM users)");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Subqueries"));
    }

    #[test]
    fn filter_blocks_pg_sleep() {
        let result = validate_filter_expression("pg_sleep(10) IS NOT NULL");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Blocked function"));
    }

    #[test]
    fn filter_blocks_unsafe_function() {
        let result = validate_filter_expression("version() = '1'");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not allowed"));
    }

    #[test]
    fn filter_empty_is_rejected() {
        assert!(validate_filter_expression("").is_err());
    }

    #[test]
    fn filter_invalid_sql_rejected() {
        assert!(validate_filter_expression("DROP TABLE users").is_err());
    }
}
