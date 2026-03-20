use regex::Regex;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::lowcode::models::{FieldError, FieldWithOptions, ValidationResult};
use crate::shared::AppError;

/// Validate and prepare data for create operation.
/// Applies defaults, type checking, required, min/max, regex, unique check.
pub async fn validate_for_create(
    pool: &PgPool,
    operation_id: Uuid,
    fields: &[FieldWithOptions],
    data: &Value,
    _user_id: Uuid,
) -> Result<ValidationResult, AppError> {
    let mut errors = Vec::new();
    let mut prepared = match data {
        Value::Object(map) => map.clone(),
        _ => return Err(AppError::Validation("Data must be a JSON object".into())),
    };

    for fwo in fields {
        let field = &fwo.field;
        let field_name = &field.field_name;
        let field_label = &field.field_label;

        // Evaluate visibility rules - skip hidden fields
        if should_skip_field(&field.visibility_rule, &Value::Object(prepared.clone())) {
            prepared.remove(field_name);
            continue;
        }

        // Apply default value if field is missing or null
        if !prepared.contains_key(field_name) || prepared[field_name].is_null() {
            if let Some(ref dv) = field.default_value {
                if !dv.is_empty() {
                    prepared.insert(field_name.clone(), Value::String(dv.clone()));
                }
            }
            // default_value_sql: execute SQL to get default
            if let Some(ref sql) = field.default_value_sql {
                if !sql.is_empty() {
                    if let Ok(val) = execute_default_sql(pool, sql).await {
                        prepared.insert(field_name.clone(), val);
                    }
                }
            }
        }

        let value = prepared.get(field_name);

        // Required check
        if field.is_required
            && (value.is_none()
                || value == Some(&Value::Null)
                || value == Some(&Value::String(String::new())))
        {
            errors.push(FieldError {
                field_name: field_name.clone(),
                field_label: field_label.clone(),
                message: format!("{} is required", field_label),
            });
            continue;
        }

        let val = match value {
            Some(v) if !v.is_null() => v,
            _ => continue,
        };

        // Type checking
        if let Some(err) = check_type(&field.field_type, val, field_name, field_label) {
            errors.push(err);
            continue;
        }

        // String validations
        if let Value::String(s) = val {
            validate_string(s, field_name, field_label, field, &mut errors);
        }

        // Numeric validations
        validate_numeric(val, field_name, field_label, field, &mut errors);

        // Unique check
        if field.is_unique {
            if let Err(e) = check_unique(pool, operation_id, field_name, val, None).await {
                errors.push(FieldError {
                    field_name: field_name.clone(),
                    field_label: field_label.clone(),
                    message: e,
                });
            }
        }
    }

    Ok(ValidationResult {
        is_valid: errors.is_empty(),
        errors,
        prepared_data: Value::Object(prepared),
    })
}

/// Validate and prepare data for update operation.
/// Same as create but excludes self from unique checks and protects MASKED fields.
pub async fn validate_for_update(
    pool: &PgPool,
    operation_id: Uuid,
    record_id: Uuid,
    fields: &[FieldWithOptions],
    data: &Value,
    _user_id: Uuid,
    masked_field_names: &[String],
) -> Result<ValidationResult, AppError> {
    let mut errors = Vec::new();
    let mut prepared = match data {
        Value::Object(map) => map.clone(),
        _ => return Err(AppError::Validation("Data must be a JSON object".into())),
    };

    // Remove masked fields from update data (user cannot modify them)
    for mf in masked_field_names {
        prepared.remove(mf);
    }

    for fwo in fields {
        let field = &fwo.field;
        let field_name = &field.field_name;
        let field_label = &field.field_label;

        // Skip masked fields in validation
        if masked_field_names.contains(field_name) {
            continue;
        }

        // Evaluate visibility rules
        if should_skip_field(&field.visibility_rule, &Value::Object(prepared.clone())) {
            prepared.remove(field_name);
            continue;
        }

        let value = prepared.get(field_name);

        // Required check
        if field.is_required
            && (value.is_none()
                || value == Some(&Value::Null)
                || value == Some(&Value::String(String::new())))
        {
            errors.push(FieldError {
                field_name: field_name.clone(),
                field_label: field_label.clone(),
                message: format!("{} is required", field_label),
            });
            continue;
        }

        let val = match value {
            Some(v) if !v.is_null() => v,
            _ => continue,
        };

        // Type checking
        if let Some(err) = check_type(&field.field_type, val, field_name, field_label) {
            errors.push(err);
            continue;
        }

        // String validations
        if let Value::String(s) = val {
            validate_string(s, field_name, field_label, field, &mut errors);
        }

        // Numeric validations
        validate_numeric(val, field_name, field_label, field, &mut errors);

        // Unique check (exclude self)
        if field.is_unique {
            if let Err(e) = check_unique(pool, operation_id, field_name, val, Some(record_id)).await
            {
                errors.push(FieldError {
                    field_name: field_name.clone(),
                    field_label: field_label.clone(),
                    message: e,
                });
            }
        }
    }

    Ok(ValidationResult {
        is_valid: errors.is_empty(),
        errors,
        prepared_data: Value::Object(prepared),
    })
}

/// Check if a field value should be skipped based on visibility rules.
fn should_skip_field(visibility_rule: &Option<Value>, data: &Value) -> bool {
    let rule = match visibility_rule {
        Some(r) if r.is_object() => r,
        _ => return false,
    };

    let dependent_field = rule
        .get("dependent_field")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let operator = rule
        .get("operator")
        .and_then(|v| v.as_str())
        .unwrap_or("equals");
    let expected = rule.get("value");

    if dependent_field.is_empty() {
        return false;
    }

    let actual = data.get(dependent_field);

    // If rule says "show when condition is met", then when condition is NOT met, field is hidden
    let action = rule
        .get("action")
        .and_then(|v| v.as_str())
        .unwrap_or("show");
    let condition_met = match operator {
        "equals" => actual == expected,
        "not_equals" => actual != expected,
        "contains" => actual
            .and_then(|a| a.as_str())
            .zip(expected.and_then(|e| e.as_str()))
            .map(|(a, e)| a.contains(e))
            .unwrap_or(false),
        "gt" => actual
            .and_then(|a| a.as_f64())
            .zip(expected.and_then(|e| e.as_f64()))
            .map(|(a, e)| a > e)
            .unwrap_or(false),
        "lt" => actual
            .and_then(|a| a.as_f64())
            .zip(expected.and_then(|e| e.as_f64()))
            .map(|(a, e)| a < e)
            .unwrap_or(false),
        _ => false,
    };

    match action {
        "show" => !condition_met,
        "hide" => condition_met,
        _ => false,
    }
}

fn check_type(
    field_type: &str,
    val: &Value,
    field_name: &str,
    field_label: &str,
) -> Option<FieldError> {
    let ok = match field_type {
        "text" | "textarea" | "rich_text" | "color" | "time_picker" => val.is_string(),
        "number" | "currency" => {
            val.is_number()
                || val
                    .as_str()
                    .map(|s| s.parse::<f64>().is_ok())
                    .unwrap_or(false)
        }
        "toggle" => val.is_boolean(),
        "date" | "date_picker" => val.is_string(), // ISO date strings
        "dropdown" | "radio_group" => val.is_string() || val.is_number(),
        "multi_select" => val.is_array(),
        "file" => true, // File handled separately
        _ => true,      // Unknown types pass through
    };
    if ok {
        None
    } else {
        Some(FieldError {
            field_name: field_name.to_string(),
            field_label: field_label.to_string(),
            message: format!(
                "{} has invalid type for field type {}",
                field_label, field_type
            ),
        })
    }
}

fn validate_string(
    s: &str,
    field_name: &str,
    field_label: &str,
    field: &crate::lowcode::models::FieldDefinition,
    errors: &mut Vec<FieldError>,
) {
    if let Some(min) = field.min_length {
        if (s.len() as i32) < min {
            errors.push(FieldError {
                field_name: field_name.to_string(),
                field_label: field_label.to_string(),
                message: format!("{} must be at least {} characters", field_label, min),
            });
        }
    }
    if let Some(max) = field.max_length {
        if (s.len() as i32) > max {
            errors.push(FieldError {
                field_name: field_name.to_string(),
                field_label: field_label.to_string(),
                message: format!("{} must be at most {} characters", field_label, max),
            });
        }
    }
    if let Some(ref pattern) = field.validation_regex {
        if !pattern.is_empty() {
            match Regex::new(pattern) {
                Ok(re) => {
                    if !re.is_match(s) {
                        let msg = field
                            .validation_message
                            .as_deref()
                            .unwrap_or("Invalid format");
                        errors.push(FieldError {
                            field_name: field_name.to_string(),
                            field_label: field_label.to_string(),
                            message: msg.to_string(),
                        });
                    }
                }
                Err(_) => {
                    tracing::warn!(
                        "Invalid regex pattern for field {}: {}",
                        field_name,
                        pattern
                    );
                }
            }
        }
    }
}

fn validate_numeric(
    val: &Value,
    field_name: &str,
    field_label: &str,
    field: &crate::lowcode::models::FieldDefinition,
    errors: &mut Vec<FieldError>,
) {
    if let Some(num) = val.as_f64() {
        if let Some(ref min) = field.min_value {
            use rust_decimal::prelude::ToPrimitive;
            if let Some(min_f) = min.to_f64() {
                if num < min_f {
                    errors.push(FieldError {
                        field_name: field_name.to_string(),
                        field_label: field_label.to_string(),
                        message: format!("{} must be at least {}", field_label, min),
                    });
                }
            }
        }
        if let Some(ref max) = field.max_value {
            use rust_decimal::prelude::ToPrimitive;
            if let Some(max_f) = max.to_f64() {
                if num > max_f {
                    errors.push(FieldError {
                        field_name: field_name.to_string(),
                        field_label: field_label.to_string(),
                        message: format!("{} must be at most {}", field_label, max),
                    });
                }
            }
        }
    }
}

async fn check_unique(
    pool: &PgPool,
    operation_id: Uuid,
    field_name: &str,
    value: &Value,
    exclude_id: Option<Uuid>,
) -> Result<(), String> {
    let value_str = value.as_str().unwrap_or(&value.to_string()).to_string();

    let count: i64 = match exclude_id {
        Some(eid) => {
            let (count,): (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM lc_operation_data WHERE operation_id = $1 AND data->>$2 = $3 AND id != $4",
            )
            .bind(operation_id)
            .bind(field_name)
            .bind(&value_str)
            .bind(eid)
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Unique check failed: {}", e))?;
            count
        }
        None => {
            let (count,): (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM lc_operation_data WHERE operation_id = $1 AND data->>$2 = $3",
            )
            .bind(operation_id)
            .bind(field_name)
            .bind(&value_str)
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Unique check failed: {}", e))?;
            count
        }
    };

    if count > 0 {
        Err("Value must be unique".to_string())
    } else {
        Ok(())
    }
}

async fn execute_default_sql(pool: &PgPool, sql: &str) -> Result<Value, AppError> {
    let row: (Value,) = sqlx::query_as(&format!("SELECT ({}) AS val", sql))
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::Internal(format!("Default SQL error: {}", e)))?;
    Ok(row.0)
}
