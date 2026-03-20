use sqlx::PgPool;
use uuid::Uuid;

use super::schema::*;

const NAMESPACE: Uuid = Uuid::from_bytes([
    0x74, 0x61, 0x73, 0x74, 0x65, 0x62, 0x79, 0x74, 0x65, 0x2e, 0x65, 0x72, 0x70, 0x2e, 0x6c,
    0x63,
]);

fn det_uuid(key: &str) -> Uuid {
    Uuid::new_v5(&NAMESPACE, key.as_bytes())
}

#[derive(Debug, Default)]
pub struct SyncReport {
    pub synced: usize,
    pub failed: usize,
    pub errors: Vec<String>,
}

pub async fn sync_all(pool: &PgPool, operations_dir: &str) -> SyncReport {
    let dir = std::path::Path::new(operations_dir);
    let files = super::loader::discover_yaml_files(dir);
    let mut report = SyncReport::default();

    tracing::info!(
        "YAML sync: found {} operation files in {:?}",
        files.len(),
        dir
    );

    for file in &files {
        match super::loader::parse_operation(file) {
            Ok(op_def) => match sync_operation(pool, &op_def).await {
                Ok(()) => {
                    tracing::info!("YAML sync: {} OK", op_def.operation_code);
                    report.synced += 1;
                }
                Err(e) => {
                    let msg = format!("{}: {}", op_def.operation_code, e);
                    tracing::error!("YAML sync failed: {}", msg);
                    report.errors.push(msg);
                    report.failed += 1;
                }
            },
            Err(e) => {
                let msg = format!("{:?}: {}", file, e);
                tracing::error!("YAML parse failed: {}", msg);
                report.errors.push(msg);
                report.failed += 1;
            }
        }
    }

    tracing::info!(
        "YAML sync complete: {} synced, {} failed",
        report.synced,
        report.failed
    );
    report
}

async fn sync_operation(pool: &PgPool, op: &OperationDef) -> Result<(), String> {
    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    // Resolve project
    let project_id = det_uuid(&format!("proj:{}", op.project_code));
    sqlx::query(
        "INSERT INTO lc_projects (id, project_number, name, is_active) \
         VALUES ($1, $2, $2, true) ON CONFLICT (project_number) DO NOTHING",
    )
    .bind(project_id)
    .bind(&op.project_code)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    // Upsert operation
    let op_id = det_uuid(&format!("op:{}", op.operation_code));
    let op_type = op.operation_type.as_deref().unwrap_or("FORM");
    let icon = op.sidebar.as_ref().and_then(|s| s.icon.as_deref());
    let sort = op
        .sidebar
        .as_ref()
        .and_then(|s| s.sort_order)
        .unwrap_or(100);

    sqlx::query(
        "INSERT INTO lc_operations (id, operation_code, project_id, name, description, \
         operation_type, is_published, version, module, sidebar_icon, sidebar_sort_order, \
         is_yaml_managed) \
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,true) \
         ON CONFLICT (operation_code) DO UPDATE SET \
         name=$4, description=$5, operation_type=$6, is_published=$7, version=$8, \
         module=$9, sidebar_icon=$10, sidebar_sort_order=$11, is_yaml_managed=true, \
         updated_at=NOW()",
    )
    .bind(op_id)
    .bind(&op.operation_code)
    .bind(project_id)
    .bind(&op.name)
    .bind(&op.description)
    .bind(op_type)
    .bind(op.is_published.unwrap_or(false))
    .bind(op.version.unwrap_or(1))
    .bind(&op.module)
    .bind(icon)
    .bind(sort)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    // Fetch the actual operation ID (may differ from det_uuid if the row pre-existed)
    let op_id: Uuid = sqlx::query_scalar("SELECT id FROM lc_operations WHERE operation_code = $1")
        .bind(&op.operation_code)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

    // Sync form
    if let Some(ref form) = op.form {
        sync_form(&mut tx, op_id, &op.operation_code, form).await?;
    }

    // Sync cross-field rules
    if let Some(ref rules) = op.cross_field_rules {
        sync_cross_field_rules(&mut tx, op_id, &op.operation_code, rules).await?;
    }

    // Sync calculation formulas
    if let Some(ref formulas) = op.calculation_formulas {
        sync_calculation_formulas(&mut tx, op_id, &op.operation_code, formulas).await?;
    }

    // Sync output rules
    if let Some(ref outputs) = op.output_rules {
        sync_output_rules(&mut tx, op_id, &op.operation_code, outputs).await?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;
    Ok(())
}

async fn sync_form(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    op_id: Uuid,
    op_code: &str,
    form: &FormDef,
) -> Result<(), String> {
    let form_id = det_uuid(&format!("form:{}", op_code));
    let layout = form
        .layout
        .clone()
        .unwrap_or(serde_json::json!({}));
    let settings = form
        .settings
        .clone()
        .unwrap_or(serde_json::json!({}));

    sqlx::query(
        "INSERT INTO lc_form_definitions (id, operation_id, layout_config, form_settings) \
         VALUES ($1,$2,$3,$4) \
         ON CONFLICT (operation_id) DO UPDATE SET \
         layout_config=$3, form_settings=$4, \
         version=lc_form_definitions.version+1, updated_at=NOW()",
    )
    .bind(form_id)
    .bind(op_id)
    .bind(&layout)
    .bind(&settings)
    .execute(&mut **tx)
    .await
    .map_err(|e| e.to_string())?;

    // Fetch actual form_id (may differ from det_uuid if form pre-existed)
    let form_id: Uuid =
        sqlx::query_scalar("SELECT id FROM lc_form_definitions WHERE operation_id = $1")
            .bind(op_id)
            .fetch_one(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

    // Delete existing sections (CASCADE deletes field_definitions and field_options too)
    // so we can re-create them cleanly with deterministic UUIDs
    sqlx::query("DELETE FROM lc_form_sections WHERE form_id = $1")
        .bind(form_id)
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

    let mut section_ids = Vec::new();
    for (sidx, section) in form.sections.iter().enumerate() {
        let sec_id = det_uuid(&format!("sec:{}/{}", op_code, section.title));
        section_ids.push(sec_id);

        sqlx::query(
            "INSERT INTO lc_form_sections \
             (id, form_id, title, description, columns, sort_order, \
              is_collapsible, is_default_collapsed, visibility_rule) \
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) \
             ON CONFLICT (id) DO UPDATE SET \
             title=$3, description=$4, columns=$5, sort_order=$6, \
             is_collapsible=$7, is_default_collapsed=$8, visibility_rule=$9, \
             updated_at=NOW()",
        )
        .bind(sec_id)
        .bind(form_id)
        .bind(&section.title)
        .bind(&section.description)
        .bind(section.columns.unwrap_or(2))
        .bind(sidx as i32)
        .bind(section.is_collapsible.unwrap_or(false))
        .bind(section.is_default_collapsed.unwrap_or(false))
        .bind(&section.visibility_rule)
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        let mut field_ids = Vec::new();
        for (fidx, field) in section.fields.iter().enumerate() {
            let fld_id = det_uuid(&format!(
                "fld:{}/{}/{}",
                op_code, section.title, field.field_name
            ));
            field_ids.push(fld_id);

            let min_val = field
                .validation
                .as_ref()
                .and_then(|v| v.min_value)
                .and_then(|v| rust_decimal::Decimal::try_from(v).ok());
            let max_val = field
                .validation
                .as_ref()
                .and_then(|v| v.max_value)
                .and_then(|v| rust_decimal::Decimal::try_from(v).ok());
            let min_len = field.validation.as_ref().and_then(|v| v.min_length);
            let max_len = field.validation.as_ref().and_then(|v| v.max_length);
            let regex = field
                .validation
                .as_ref()
                .and_then(|v| v.regex.as_deref());
            let regex_msg = field
                .validation
                .as_ref()
                .and_then(|v| v.regex_message.as_deref());
            let db_table = field
                .db_mapping
                .as_ref()
                .and_then(|d| d.table.as_deref());
            let db_col = field
                .db_mapping
                .as_ref()
                .and_then(|d| d.column.as_deref());
            let ds_sql = field
                .data_source
                .as_ref()
                .and_then(|d| d.sql.as_deref());
            let ds_disp = field
                .data_source
                .as_ref()
                .and_then(|d| d.display_column.as_deref());
            let ds_val = field
                .data_source
                .as_ref()
                .and_then(|d| d.value_column.as_deref());
            let config = field
                .config
                .clone()
                .unwrap_or(serde_json::json!({}));

            sqlx::query(
                "INSERT INTO lc_field_definitions \
                 (id, section_id, field_name, field_label, field_type, \
                  is_required, is_unique, is_searchable, default_value, default_value_sql, \
                  placeholder, help_text, validation_regex, validation_message, \
                  min_value, max_value, min_length, max_length, \
                  db_table, db_column, data_source_sql, display_column, value_column, \
                  visibility_rule, field_config, sort_order, column_span, \
                  sub_table_columns, lookup_fill_fields, required_rule) \
                 VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,\
                         $19,$20,$21,$22,$23,$24,$25,$26,$27,$28,$29,$30) \
                 ON CONFLICT (section_id, field_name) DO UPDATE SET \
                 field_label=$4, field_type=$5, is_required=$6, is_unique=$7, \
                 is_searchable=$8, default_value=$9, default_value_sql=$10, \
                 placeholder=$11, help_text=$12, validation_regex=$13, validation_message=$14, \
                 min_value=$15, max_value=$16, min_length=$17, max_length=$18, \
                 db_table=$19, db_column=$20, data_source_sql=$21, display_column=$22, \
                 value_column=$23, visibility_rule=$24, field_config=$25, sort_order=$26, \
                 column_span=$27, sub_table_columns=$28, lookup_fill_fields=$29, \
                 required_rule=$30, updated_at=NOW()",
            )
            .bind(fld_id)
            .bind(sec_id)
            .bind(&field.field_name)
            .bind(&field.label)
            .bind(&field.field_type)
            .bind(field.required.unwrap_or(false))
            .bind(field.unique.unwrap_or(false))
            .bind(field.searchable.unwrap_or(false))
            .bind(&field.default_value)
            .bind(&field.default_value_sql)
            .bind(&field.placeholder)
            .bind(&field.help_text)
            .bind(regex)
            .bind(regex_msg)
            .bind(min_val)
            .bind(max_val)
            .bind(min_len)
            .bind(max_len)
            .bind(db_table)
            .bind(db_col)
            .bind(ds_sql)
            .bind(ds_disp)
            .bind(ds_val)
            .bind(&field.visibility_rule)
            .bind(&config)
            .bind(fidx as i32)
            .bind(field.column_span.unwrap_or(1))
            .bind(&field.sub_table_columns)
            .bind(&field.lookup_fill_fields)
            .bind(&field.required_rule)
            .execute(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

            // Sync options
            if let Some(ref opts) = field.options {
                let mut opt_ids = Vec::new();
                for (oidx, opt) in opts.iter().enumerate() {
                    let opt_id = det_uuid(&format!(
                        "opt:{}/{}/{}/{}",
                        op_code, section.title, field.field_name, opt.value
                    ));
                    opt_ids.push(opt_id);
                    sqlx::query(
                        "INSERT INTO lc_field_options \
                         (id, field_id, option_label, option_value, sort_order, is_default, is_active) \
                         VALUES ($1,$2,$3,$4,$5,$6,$7) \
                         ON CONFLICT (id) DO UPDATE SET \
                         option_label=$3, option_value=$4, sort_order=$5, is_default=$6, is_active=$7",
                    )
                    .bind(opt_id)
                    .bind(fld_id)
                    .bind(&opt.label)
                    .bind(&opt.value)
                    .bind(oidx as i32)
                    .bind(opt.is_default.unwrap_or(false))
                    .bind(opt.is_active.unwrap_or(true))
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| e.to_string())?;
                }
                // Remove old options
                if !opt_ids.is_empty() {
                    let ids_str: Vec<String> =
                        opt_ids.iter().map(|id| format!("'{}'", id)).collect();
                    let del_sql = format!(
                        "DELETE FROM lc_field_options WHERE field_id = '{}' AND id NOT IN ({})",
                        fld_id,
                        ids_str.join(",")
                    );
                    sqlx::query(&del_sql)
                        .execute(&mut **tx)
                        .await
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        // Remove old fields
        if !field_ids.is_empty() {
            let ids_str: Vec<String> = field_ids.iter().map(|id| format!("'{}'", id)).collect();
            let del_sql = format!(
                "DELETE FROM lc_field_definitions WHERE section_id = '{}' AND id NOT IN ({})",
                sec_id,
                ids_str.join(",")
            );
            sqlx::query(&del_sql)
                .execute(&mut **tx)
                .await
                .map_err(|e| e.to_string())?;
        }
    }
    // Remove old sections
    if !section_ids.is_empty() {
        let ids_str: Vec<String> = section_ids.iter().map(|id| format!("'{}'", id)).collect();
        let del_sql = format!(
            "DELETE FROM lc_form_sections WHERE form_id = '{}' AND id NOT IN ({})",
            form_id,
            ids_str.join(",")
        );
        sqlx::query(&del_sql)
            .execute(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

async fn sync_cross_field_rules(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    op_id: Uuid,
    op_code: &str,
    rules: &[CrossFieldRuleDef],
) -> Result<(), String> {
    for (idx, rule) in rules.iter().enumerate() {
        let rule_id = det_uuid(&format!("rule:{}/{}", op_code, rule.name));
        sqlx::query(
            "INSERT INTO cross_field_rules \
             (id, operation_id, rule_name, description, rule_type, source_field, \
              operator, target_field, target_value, error_message, sort_order) \
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11) \
             ON CONFLICT (id) DO UPDATE SET \
             rule_name=$3, description=$4, rule_type=$5, source_field=$6, \
             operator=$7, target_field=$8, target_value=$9, error_message=$10, sort_order=$11",
        )
        .bind(rule_id)
        .bind(op_id)
        .bind(&rule.name)
        .bind(&rule.description)
        .bind(rule.rule_type.as_deref().unwrap_or("VALIDATION"))
        .bind(&rule.source_field)
        .bind(&rule.operator)
        .bind(&rule.target_field)
        .bind(&rule.target_value)
        .bind(&rule.error_message)
        .bind(idx as i32)
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

async fn sync_calculation_formulas(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    op_id: Uuid,
    op_code: &str,
    formulas: &[CalculationFormulaDef],
) -> Result<(), String> {
    for (idx, f) in formulas.iter().enumerate() {
        let fid = det_uuid(&format!("calc:{}/{}", op_code, f.target_field));
        sqlx::query(
            "INSERT INTO calculation_formulas \
             (id, operation_id, target_field, formula, trigger_fields, description, sort_order) \
             VALUES ($1,$2,$3,$4,$5,$6,$7) \
             ON CONFLICT (id) DO UPDATE SET \
             target_field=$3, formula=$4, trigger_fields=$5, description=$6, sort_order=$7",
        )
        .bind(fid)
        .bind(op_id)
        .bind(&f.target_field)
        .bind(&f.formula)
        .bind(&f.trigger_fields)
        .bind(&f.description)
        .bind(idx as i32)
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

async fn sync_output_rules(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    op_id: Uuid,
    op_code: &str,
    outputs: &[OutputRuleDef],
) -> Result<(), String> {
    for output in outputs {
        let out_id = det_uuid(&format!("out:{}/{}", op_code, output.name));
        sqlx::query(
            "INSERT INTO output_rules \
             (id, name, operation_id, trigger_event, condition_field, condition_operator, \
              condition_value, output_type, recipient_type, recipient_static) \
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10) \
             ON CONFLICT (id) DO UPDATE SET \
             name=$2, trigger_event=$4, condition_field=$5, condition_operator=$6, \
             condition_value=$7, output_type=$8, recipient_type=$9, recipient_static=$10",
        )
        .bind(out_id)
        .bind(&output.name)
        .bind(op_id)
        .bind(output.trigger_event.as_deref().unwrap_or("ON_CREATE"))
        .bind(&output.condition_field)
        .bind(&output.condition_operator)
        .bind(&output.condition_value)
        .bind(output.output_type.as_deref().unwrap_or("NOTIFICATION"))
        .bind(output.recipient_type.as_deref().unwrap_or("STATIC"))
        .bind(&output.recipient_value)
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
