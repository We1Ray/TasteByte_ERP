use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::shared::AppError;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CrossFieldRule {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub rule_name: String,
    pub description: Option<String>,
    pub rule_type: String,
    pub source_field: String,
    pub operator: String,
    pub target_field: Option<String>,
    pub target_value: Option<String>,
    pub error_message: String,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CalculationFormula {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub target_field: String,
    pub formula: String,
    pub trigger_fields: Vec<String>,
    pub description: Option<String>,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCrossFieldRule {
    pub rule_name: String,
    pub description: Option<String>,
    pub rule_type: Option<String>,
    pub source_field: String,
    pub operator: String,
    pub target_field: Option<String>,
    pub target_value: Option<String>,
    pub error_message: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCalculationFormula {
    pub target_field: String,
    pub formula: String,
    pub trigger_fields: Vec<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CrossFieldError {
    pub rule_name: String,
    pub message: String,
}

pub async fn list_rules(
    pool: &PgPool,
    operation_id: Uuid,
) -> Result<Vec<CrossFieldRule>, AppError> {
    Ok(sqlx::query_as::<_, CrossFieldRule>(
        "SELECT * FROM cross_field_rules WHERE operation_id = $1 AND is_active = true ORDER BY sort_order",
    )
    .bind(operation_id)
    .fetch_all(pool)
    .await?)
}

pub async fn create_rule(
    pool: &PgPool,
    operation_id: Uuid,
    input: CreateCrossFieldRule,
) -> Result<CrossFieldRule, AppError> {
    let rule = sqlx::query_as::<_, CrossFieldRule>(
        "INSERT INTO cross_field_rules (operation_id, rule_name, description, rule_type, source_field, operator, target_field, target_value, error_message) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING *",
    )
    .bind(operation_id)
    .bind(&input.rule_name)
    .bind(&input.description)
    .bind(input.rule_type.as_deref().unwrap_or("VALIDATION"))
    .bind(&input.source_field)
    .bind(&input.operator)
    .bind(&input.target_field)
    .bind(&input.target_value)
    .bind(&input.error_message)
    .fetch_one(pool)
    .await?;
    Ok(rule)
}

pub async fn delete_rule(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM cross_field_rules WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_formulas(
    pool: &PgPool,
    operation_id: Uuid,
) -> Result<Vec<CalculationFormula>, AppError> {
    Ok(sqlx::query_as::<_, CalculationFormula>(
        "SELECT * FROM calculation_formulas WHERE operation_id = $1 AND is_active = true ORDER BY sort_order",
    )
    .bind(operation_id)
    .fetch_all(pool)
    .await?)
}

pub async fn create_formula(
    pool: &PgPool,
    operation_id: Uuid,
    input: CreateCalculationFormula,
) -> Result<CalculationFormula, AppError> {
    let formula = sqlx::query_as::<_, CalculationFormula>(
        "INSERT INTO calculation_formulas (operation_id, target_field, formula, trigger_fields, description) VALUES ($1,$2,$3,$4,$5) RETURNING *",
    )
    .bind(operation_id)
    .bind(&input.target_field)
    .bind(&input.formula)
    .bind(&input.trigger_fields)
    .bind(&input.description)
    .fetch_one(pool)
    .await?;
    Ok(formula)
}

pub async fn delete_formula(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM calculation_formulas WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Validate cross-field rules against data
pub fn validate_cross_field_rules(
    rules: &[CrossFieldRule],
    data: &serde_json::Value,
) -> Vec<CrossFieldError> {
    let mut errors = Vec::new();
    let obj = match data.as_object() {
        Some(o) => o,
        None => return errors,
    };

    for rule in rules {
        let source_val = obj.get(&rule.source_field);
        let passed = match rule.operator.as_str() {
            "gt" | "greater_than" => {
                let s = source_val.and_then(|v| v.as_f64()).unwrap_or(0.0);
                if let Some(ref tf) = rule.target_field {
                    let t = obj.get(tf).and_then(|v| v.as_f64()).unwrap_or(0.0);
                    s > t
                } else {
                    let t = rule
                        .target_value
                        .as_ref()
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.0);
                    s > t
                }
            }
            "lt" | "less_than" => {
                let s = source_val.and_then(|v| v.as_f64()).unwrap_or(0.0);
                if let Some(ref tf) = rule.target_field {
                    let t = obj.get(tf).and_then(|v| v.as_f64()).unwrap_or(0.0);
                    s < t
                } else {
                    let t = rule
                        .target_value
                        .as_ref()
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.0);
                    s < t
                }
            }
            "gte" => {
                let s = source_val.and_then(|v| v.as_f64()).unwrap_or(0.0);
                if let Some(ref tf) = rule.target_field {
                    let t = obj.get(tf).and_then(|v| v.as_f64()).unwrap_or(0.0);
                    s >= t
                } else {
                    let t = rule
                        .target_value
                        .as_ref()
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.0);
                    s >= t
                }
            }
            "lte" => {
                let s = source_val.and_then(|v| v.as_f64()).unwrap_or(0.0);
                if let Some(ref tf) = rule.target_field {
                    let t = obj.get(tf).and_then(|v| v.as_f64()).unwrap_or(0.0);
                    s <= t
                } else {
                    let t = rule
                        .target_value
                        .as_ref()
                        .and_then(|v| v.parse::<f64>().ok())
                        .unwrap_or(0.0);
                    s <= t
                }
            }
            "equals" | "eq" => {
                let s = source_val.map(|v| v.to_string()).unwrap_or_default();
                if let Some(ref tf) = rule.target_field {
                    let t = obj.get(tf).map(|v| v.to_string()).unwrap_or_default();
                    s == t
                } else {
                    s == rule.target_value.as_deref().unwrap_or("")
                }
            }
            "not_equals" | "ne" => {
                let s = source_val.map(|v| v.to_string()).unwrap_or_default();
                if let Some(ref tf) = rule.target_field {
                    let t = obj.get(tf).map(|v| v.to_string()).unwrap_or_default();
                    s != t
                } else {
                    s != rule.target_value.as_deref().unwrap_or("")
                }
            }
            "not_empty" => source_val
                .map(|v| !v.is_null() && v.as_str().map(|s| !s.is_empty()).unwrap_or(true))
                .unwrap_or(false),
            "before" => {
                let s = source_val.and_then(|v| v.as_str()).unwrap_or("");
                if let Some(ref tf) = rule.target_field {
                    let t = obj.get(tf).and_then(|v| v.as_str()).unwrap_or("");
                    s < t
                } else {
                    s < rule.target_value.as_deref().unwrap_or("")
                }
            }
            "after" => {
                let s = source_val.and_then(|v| v.as_str()).unwrap_or("");
                if let Some(ref tf) = rule.target_field {
                    let t = obj.get(tf).and_then(|v| v.as_str()).unwrap_or("");
                    s > t
                } else {
                    s > rule.target_value.as_deref().unwrap_or("")
                }
            }
            _ => true,
        };

        if !passed {
            errors.push(CrossFieldError {
                rule_name: rule.rule_name.clone(),
                message: rule.error_message.clone(),
            });
        }
    }
    errors
}

/// Apply calculation formulas to data.
/// Supports per-row formulas for master_detail fields when target_field contains a dot,
/// e.g. "order_items.row_total" applies the formula to each row in order_items.items[].
pub fn apply_formulas(formulas: &[CalculationFormula], data: &mut serde_json::Value) {
    let obj = match data.as_object_mut() {
        Some(o) => o,
        None => return,
    };

    for formula in formulas {
        if let Some(dot_pos) = formula.target_field.find('.') {
            // Per-row formula: "master_detail_field.column_name"
            let md_field = &formula.target_field[..dot_pos];
            let row_col = &formula.target_field[dot_pos + 1..];

            if let Some(md_val) = obj.get_mut(md_field) {
                if let Some(items) = md_val.get_mut("items").and_then(|v| v.as_array_mut()) {
                    for row in items.iter_mut() {
                        // Evaluate formula in the context of each row
                        let result = evaluate_formula(&formula.formula, row);
                        if let Some(val) = result {
                            if let Some(row_obj) = row.as_object_mut() {
                                row_obj.insert(row_col.to_string(), val);
                            }
                        }
                    }
                }
            }
        } else {
            // Standard top-level formula
            let result =
                evaluate_formula(&formula.formula, &serde_json::Value::Object(obj.clone()));
            if let Some(val) = result {
                obj.insert(formula.target_field.clone(), val);
            }
        }
    }
}

fn evaluate_formula(formula: &str, data: &serde_json::Value) -> Option<serde_json::Value> {
    let result = eval_expression(formula.trim(), data)?;
    Some(serde_json::json!(result))
}

// Two-pass evaluation: first handle +/-, then handle */÷ within each term
fn eval_expression(expr: &str, data: &serde_json::Value) -> Option<f64> {
    // Split by + and - (keeping operators), but not inside parentheses
    let mut terms: Vec<f64> = Vec::new();
    let mut ops: Vec<char> = Vec::new();

    let mut current_term = String::new();
    for ch in expr.chars() {
        if (ch == '+' || ch == '-') && !current_term.is_empty() {
            terms.push(eval_term(current_term.trim(), data)?);
            ops.push(ch);
            current_term = String::new();
        } else {
            current_term.push(ch);
        }
    }
    if !current_term.is_empty() {
        terms.push(eval_term(current_term.trim(), data)?);
    }

    if terms.is_empty() {
        return None;
    }

    let mut result = terms[0];
    for (i, op) in ops.iter().enumerate() {
        match op {
            '+' => result += terms[i + 1],
            '-' => result -= terms[i + 1],
            _ => {}
        }
    }
    Some(result)
}

// Evaluate a term (handles * and /)
fn eval_term(term: &str, data: &serde_json::Value) -> Option<f64> {
    let mut factors: Vec<f64> = Vec::new();
    let mut ops: Vec<char> = Vec::new();

    let mut current = String::new();
    for ch in term.chars() {
        if (ch == '*' || ch == '/') && !current.is_empty() {
            factors.push(resolve_value(current.trim(), data)?);
            ops.push(ch);
            current = String::new();
        } else {
            current.push(ch);
        }
    }
    if !current.is_empty() {
        factors.push(resolve_value(current.trim(), data)?);
    }

    if factors.is_empty() {
        return None;
    }

    let mut result = factors[0];
    for (i, op) in ops.iter().enumerate() {
        match op {
            '*' => result *= factors[i + 1],
            '/' => {
                if factors[i + 1] == 0.0 {
                    return None;
                }
                result /= factors[i + 1];
            }
            _ => {}
        }
    }
    Some(result)
}

fn resolve_value(token: &str, data: &serde_json::Value) -> Option<f64> {
    // Try as number literal first
    if let Ok(n) = token.parse::<f64>() {
        return Some(n);
    }
    // Try as field reference
    data.get(token).and_then(|v| v.as_f64())
}
