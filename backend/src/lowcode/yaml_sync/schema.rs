use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OperationDef {
    pub operation_code: String,
    pub name: String,
    pub description: Option<String>,
    pub module: String,
    pub operation_type: Option<String>,
    pub project_code: String,
    pub is_published: Option<bool>,
    pub version: Option<i32>,
    pub sidebar: Option<SidebarDef>,
    pub form: Option<FormDef>,
    pub cross_field_rules: Option<Vec<CrossFieldRuleDef>>,
    pub calculation_formulas: Option<Vec<CalculationFormulaDef>>,
    pub approval: Option<ApprovalDef>,
    pub output_rules: Option<Vec<OutputRuleDef>>,
    pub buttons: Option<Vec<ButtonDef>>,
    pub form_variants: Option<Vec<FormVariantDef>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SidebarDef {
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormDef {
    pub layout: Option<serde_json::Value>,
    pub settings: Option<serde_json::Value>,
    pub sections: Vec<SectionDef>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SectionDef {
    pub title: String,
    pub description: Option<String>,
    pub columns: Option<i32>,
    pub is_collapsible: Option<bool>,
    pub is_default_collapsed: Option<bool>,
    pub visibility_rule: Option<serde_json::Value>,
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FieldDef {
    pub field_name: String,
    pub label: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub required: Option<bool>,
    pub unique: Option<bool>,
    pub searchable: Option<bool>,
    pub column_span: Option<i32>,
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
    pub default_value: Option<String>,
    pub default_value_sql: Option<String>,
    pub validation: Option<ValidationDef>,
    pub db_mapping: Option<DbMappingDef>,
    pub data_source: Option<DataSourceDef>,
    pub depends_on: Option<String>,
    pub visibility_rule: Option<serde_json::Value>,
    pub config: Option<serde_json::Value>,
    pub options: Option<Vec<FieldOptionDef>>,
    pub sub_table_columns: Option<serde_json::Value>,
    pub lookup_fill_fields: Option<serde_json::Value>,
    pub required_rule: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidationDef {
    pub regex: Option<String>,
    pub regex_message: Option<String>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbMappingDef {
    pub table: Option<String>,
    pub column: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataSourceDef {
    pub sql: Option<String>,
    pub display_column: Option<String>,
    pub value_column: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FieldOptionDef {
    pub label: String,
    pub value: String,
    pub is_default: Option<bool>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CrossFieldRuleDef {
    pub name: String,
    pub description: Option<String>,
    pub rule_type: Option<String>,
    pub source_field: String,
    pub operator: String,
    pub target_field: Option<String>,
    pub target_value: Option<String>,
    pub error_message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CalculationFormulaDef {
    pub target_field: String,
    pub formula: String,
    pub trigger_fields: Vec<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApprovalDef {
    pub name: String,
    pub description: Option<String>,
    pub levels: Vec<ApprovalLevelDef>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApprovalLevelDef {
    pub name: String,
    pub level_order: i32,
    pub condition_field: Option<String>,
    pub condition_operator: Option<String>,
    pub condition_value: Option<f64>,
    pub approver_type: String,
    pub approver_role: Option<String>,
    pub approver_user_id: Option<String>,
    pub sla_hours: Option<i32>,
    pub auto_escalate: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutputRuleDef {
    pub name: String,
    pub trigger_event: Option<String>,
    pub condition_field: Option<String>,
    pub condition_operator: Option<String>,
    pub condition_value: Option<String>,
    pub output_type: Option<String>,
    pub recipient_type: Option<String>,
    pub recipient_value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ButtonDef {
    pub button_key: String,
    pub label: String,
    pub icon: Option<String>,
    pub variant: Option<String>,
    pub action_type: Option<String>,
    pub action_config: Option<serde_json::Value>,
    pub confirm_message: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormVariantDef {
    pub variant_name: String,
    pub condition_field: Option<String>,
    pub condition_value: Option<String>,
    pub hidden_fields: Option<Vec<String>>,
    pub readonly_fields: Option<Vec<String>>,
    pub required_fields: Option<Vec<String>>,
    pub default_values: Option<serde_json::Value>,
    pub is_default: Option<bool>,
}
