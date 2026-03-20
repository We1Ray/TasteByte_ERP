use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// ── Projects ───────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Project {
    pub id: Uuid,
    pub project_number: String,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProject {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProject {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

// ── Operations ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Operation {
    pub id: Uuid,
    pub operation_code: String,
    pub project_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub target_table: Option<String>,
    pub operation_type: String,
    pub is_published: bool,
    pub version: i32,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub module: Option<String>,
    pub sidebar_icon: Option<String>,
    pub sidebar_sort_order: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOperation {
    pub project_id: Uuid,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub description: Option<String>,
    pub target_table: Option<String>,
    pub operation_type: Option<String>,
    pub module: Option<String>,
    pub sidebar_icon: Option<String>,
    pub sidebar_sort_order: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOperation {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub target_table: Option<String>,
    pub operation_type: Option<String>,
    pub module: Option<String>,
    pub sidebar_icon: Option<String>,
    pub sidebar_sort_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct OperationFilter {
    pub project_id: Option<Uuid>,
    pub module: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

// ── Operation Buttons ─────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct OperationButton {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub button_key: String,
    pub label: String,
    pub icon: Option<String>,
    pub variant: String,
    pub action_type: String,
    pub action_config: serde_json::Value,
    pub confirm_message: Option<String>,
    pub required_permission: Option<String>,
    pub is_visible: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SaveOperationButtonsInput {
    pub buttons: Vec<SaveButtonItem>,
}

#[derive(Debug, Deserialize)]
pub struct SaveButtonItem {
    pub button_key: String,
    pub label: String,
    pub icon: Option<String>,
    pub variant: Option<String>,
    pub action_type: Option<String>,
    pub action_config: Option<serde_json::Value>,
    pub confirm_message: Option<String>,
    pub required_permission: Option<String>,
    pub is_visible: Option<bool>,
    pub sort_order: i32,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ModuleOperationSummary {
    pub id: Uuid,
    pub operation_code: String,
    pub name: String,
    pub operation_type: String,
    pub sidebar_icon: Option<String>,
    pub sidebar_sort_order: i32,
}

// ── Form Definitions ───────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct FormDefinition {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub layout_config: serde_json::Value,
    pub form_settings: serde_json::Value,
    pub snapshot: Option<serde_json::Value>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Form Sections ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct FormSection {
    pub id: Uuid,
    pub form_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub columns: i32,
    pub sort_order: i32,
    pub is_collapsible: bool,
    pub is_default_collapsed: bool,
    pub visibility_rule: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Field Definitions ──────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct FieldDefinition {
    pub id: Uuid,
    pub section_id: Uuid,
    pub field_name: String,
    pub field_label: String,
    pub field_type: String,
    pub db_table: Option<String>,
    pub db_column: Option<String>,
    pub is_required: bool,
    pub is_unique: bool,
    pub is_searchable: bool,
    pub default_value: Option<String>,
    pub default_value_sql: Option<String>,
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
    pub validation_regex: Option<String>,
    pub validation_message: Option<String>,
    pub min_value: Option<Decimal>,
    pub max_value: Option<Decimal>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub depends_on: Option<Uuid>,
    pub data_source_sql: Option<String>,
    pub display_column: Option<String>,
    pub value_column: Option<String>,
    pub visibility_rule: Option<serde_json::Value>,
    pub field_config: serde_json::Value,
    pub sort_order: i32,
    pub column_span: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ── Field Options ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct FieldOption {
    pub id: Uuid,
    pub field_id: Uuid,
    pub option_label: String,
    pub option_value: String,
    pub sort_order: i32,
    pub is_default: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

// ── Form Builder DTOs (Bulk Save) ──────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SaveFormRequest {
    pub layout_config: Option<serde_json::Value>,
    pub form_settings: Option<serde_json::Value>,
    pub sections: Vec<SaveSectionInput>,
}

#[derive(Debug, Deserialize)]
pub struct SaveSectionInput {
    pub id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub columns: Option<i32>,
    pub sort_order: i32,
    pub is_collapsible: Option<bool>,
    pub is_default_collapsed: Option<bool>,
    pub visibility_rule: Option<serde_json::Value>,
    pub fields: Vec<SaveFieldInput>,
}

#[derive(Debug, Deserialize)]
pub struct SaveFieldInput {
    pub id: Option<Uuid>,
    pub field_name: String,
    pub field_label: String,
    pub field_type: String,
    pub db_table: Option<String>,
    pub db_column: Option<String>,
    pub is_required: Option<bool>,
    pub is_unique: Option<bool>,
    pub is_searchable: Option<bool>,
    pub default_value: Option<String>,
    pub default_value_sql: Option<String>,
    pub placeholder: Option<String>,
    pub help_text: Option<String>,
    pub validation_regex: Option<String>,
    pub validation_message: Option<String>,
    pub min_value: Option<Decimal>,
    pub max_value: Option<Decimal>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub depends_on: Option<Uuid>,
    pub data_source_sql: Option<String>,
    pub display_column: Option<String>,
    pub value_column: Option<String>,
    pub visibility_rule: Option<serde_json::Value>,
    pub field_config: Option<serde_json::Value>,
    pub sort_order: i32,
    pub column_span: Option<i32>,
    pub options: Option<Vec<SaveFieldOptionInput>>,
}

#[derive(Debug, Deserialize)]
pub struct SaveFieldOptionInput {
    pub id: Option<Uuid>,
    pub option_label: String,
    pub option_value: String,
    pub sort_order: i32,
    pub is_default: Option<bool>,
    pub is_active: Option<bool>,
}

// ── Full Form Response ─────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct FormResponse {
    pub form: FormDefinition,
    pub sections: Vec<SectionWithFields>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SectionWithFields {
    #[serde(flatten)]
    pub section: FormSection,
    pub fields: Vec<FieldWithOptions>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FieldWithOptions {
    #[serde(flatten)]
    pub field: FieldDefinition,
    pub options: Vec<FieldOption>,
}

// ── Operation Data (JSONB storage) ─────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct OperationData {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub data: serde_json::Value,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOperationData {
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOperationData {
    pub data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct DataListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub sort_field: Option<String>,
    pub sort_order: Option<String>,
}

// ── File Uploads ───────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FileUpload {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub record_id: Option<Uuid>,
    pub field_id: Option<Uuid>,
    pub file_name: String,
    pub file_type: Option<String>,
    pub file_size: Option<i64>,
    pub storage_path: String,
    pub uploaded_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

// ── Permissions ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PlatformRole {
    pub id: Uuid,
    pub role_name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct OperationPermission {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub role_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub can_create: bool,
    pub can_read: bool,
    pub can_update: bool,
    pub can_delete: bool,
    pub custom_permissions: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOperationPermission {
    pub role_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub can_create: Option<bool>,
    pub can_read: Option<bool>,
    pub can_update: Option<bool>,
    pub can_delete: Option<bool>,
    pub custom_permissions: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOperationPermission {
    pub can_create: Option<bool>,
    pub can_read: Option<bool>,
    pub can_update: Option<bool>,
    pub can_delete: Option<bool>,
    pub custom_permissions: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FieldPermission {
    pub id: Uuid,
    pub field_id: Uuid,
    pub role_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub visibility: String,
    pub is_editable: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateFieldPermission {
    pub role_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub visibility: Option<String>,
    pub is_editable: Option<bool>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct RecordPolicy {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub role_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub policy_name: String,
    pub filter_sql: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRecordPolicy {
    pub role_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    #[validate(length(min = 1, max = 200))]
    pub policy_name: String,
    pub filter_sql: String,
    pub is_active: Option<bool>,
}

// ── Releases (matches lc_releases table) ───────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Release {
    pub id: Uuid,
    pub release_number: String,
    pub operation_id: Uuid,
    pub version: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub submitted_by: Option<Uuid>,
    pub reviewed_by: Option<Uuid>,
    pub review_notes: Option<String>,
    pub form_snapshot: serde_json::Value,
    pub submitted_at: Option<DateTime<Utc>>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub released_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRelease {
    pub operation_id: Uuid,
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReviewRelease {
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseFilter {
    pub operation_id: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

// ── Feedback (matches lc_feedback table) ───────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Feedback {
    pub id: Uuid,
    pub ticket_number: String,
    pub operation_id: Uuid,
    pub feedback_type: String,
    pub title: String,
    pub description: String,
    pub priority: String,
    pub status: String,
    pub assigned_to: Option<Uuid>,
    pub submitted_by: Uuid,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateFeedback {
    pub operation_id: Uuid,
    pub feedback_type: Option<String>,
    #[validate(length(min = 1, max = 300))]
    pub title: String,
    pub description: String,
    pub priority: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateFeedback {
    #[validate(length(min = 1))]
    pub status: Option<String>,
    #[validate(length(min = 1))]
    pub priority: Option<String>,
    pub assigned_to: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct FeedbackFilter {
    pub operation_id: Option<Uuid>,
    pub status: Option<String>,
    pub feedback_type: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

// ── Feedback Comments (matches lc_feedback_comments table) ─────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FeedbackComment {
    pub id: Uuid,
    pub feedback_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateFeedbackComment {
    #[validate(length(min = 1))]
    pub content: String,
}

// ── Dev Journal (matches lc_dev_journal table) ─────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DevJournalEntry {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub changed_by: Uuid,
    pub change_type: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub diff_summary: Option<String>,
    pub form_snapshot: Option<serde_json::Value>,
    pub version: Option<i32>,
    pub created_at: DateTime<Utc>,
}

// ── Navigation (matches lc_navigation_items table) ─────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct NavigationItem {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub title: String,
    pub icon: Option<String>,
    pub route: Option<String>,
    pub operation_id: Option<Uuid>,
    pub sort_order: i32,
    pub is_visible: bool,
    pub required_role: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateNavigationItem {
    pub parent_id: Option<Uuid>,
    pub operation_id: Option<Uuid>,
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    pub icon: Option<String>,
    pub route: Option<String>,
    pub sort_order: Option<i32>,
    pub is_visible: Option<bool>,
    pub required_role: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateNavigationItem {
    pub parent_id: Option<Uuid>,
    pub operation_id: Option<Uuid>,
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,
    pub icon: Option<String>,
    pub route: Option<String>,
    pub sort_order: Option<i32>,
    pub is_visible: Option<bool>,
    pub required_role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReorderInput {
    pub items: Vec<ReorderItem>,
}

#[derive(Debug, Deserialize)]
pub struct ReorderItem {
    pub id: Uuid,
    pub sort_order: i32,
}

// ── Notifications (matches lc_notifications table) ─────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub message: Option<String>,
    pub notification_type: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

// ── Data Source ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Validate)]
pub struct DataSourceQuery {
    #[validate(length(min = 1))]
    pub sql: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct DataSourceResult {
    pub columns: Vec<String>,
    pub rows: Vec<serde_json::Value>,
    pub row_count: usize,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TableInfo {
    pub table_name: String,
    pub table_schema: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ColumnInfo {
    pub column_name: String,
    pub data_type: String,
    pub is_nullable: String,
    pub column_default: Option<String>,
    pub ordinal_position: i32,
}

#[derive(Debug, Serialize)]
pub struct ColumnInfoResponse {
    pub column_name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_primary_key: bool,
    pub column_default: Option<String>,
    pub ordinal_position: i32,
}

#[derive(Debug, Serialize)]
pub struct SqlValidation {
    pub valid: bool,
    pub error: Option<String>,
}

// ── List Builder ───────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ListDefinitionRow {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub data_source_sql: Option<String>,
    pub default_page_size: i32,
    pub enable_search: bool,
    pub enable_export: bool,
    pub enable_import: bool,
    pub settings: serde_json::Value,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ListColumnRow {
    pub id: Uuid,
    pub list_id: Uuid,
    pub field_key: String,
    pub label: String,
    pub data_type: String,
    pub width: Option<i32>,
    pub min_width: Option<i32>,
    pub is_sortable: bool,
    pub is_filterable: bool,
    pub is_visible: bool,
    pub format_pattern: Option<String>,
    pub cell_renderer: Option<String>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ListActionRow {
    pub id: Uuid,
    pub list_id: Uuid,
    pub action_key: String,
    pub label: String,
    pub icon: Option<String>,
    pub action_type: String,
    pub target_url: Option<String>,
    pub confirm_message: Option<String>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ListResponse {
    pub list: ListDefinitionRow,
    pub columns: Vec<ListColumnRow>,
    pub actions: Vec<ListActionRow>,
}

#[derive(Debug, Deserialize)]
pub struct SaveListRequest {
    pub data_source_sql: Option<String>,
    pub default_page_size: Option<i32>,
    pub enable_search: Option<bool>,
    pub enable_export: Option<bool>,
    pub enable_import: Option<bool>,
    pub settings: Option<serde_json::Value>,
    pub columns: Vec<SaveListColumnInput>,
    pub actions: Vec<SaveListActionInput>,
}

#[derive(Debug, Deserialize)]
pub struct SaveListColumnInput {
    pub field_key: String,
    pub label: String,
    pub data_type: Option<String>,
    pub width: Option<i32>,
    pub min_width: Option<i32>,
    pub is_sortable: Option<bool>,
    pub is_filterable: Option<bool>,
    pub is_visible: Option<bool>,
    pub format_pattern: Option<String>,
    pub cell_renderer: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Deserialize)]
pub struct SaveListActionInput {
    pub action_key: String,
    pub label: String,
    pub icon: Option<String>,
    pub action_type: Option<String>,
    pub target_url: Option<String>,
    pub confirm_message: Option<String>,
    pub sort_order: i32,
}

// ── Dashboard Builder ──────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DashboardDefinitionRow {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub grid_columns: i32,
    pub refresh_interval: Option<i32>,
    pub settings: serde_json::Value,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DashboardWidgetRow {
    pub id: Uuid,
    pub dashboard_id: Uuid,
    pub title: String,
    pub widget_type: String,
    pub data_source_sql: String,
    pub x_axis_key: Option<String>,
    pub y_axis_key: Option<String>,
    pub series_config: serde_json::Value,
    pub colors: serde_json::Value,
    pub grid_x: i32,
    pub grid_y: i32,
    pub grid_w: i32,
    pub grid_h: i32,
    pub widget_config: serde_json::Value,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct DashboardResponse {
    pub dashboard: DashboardDefinitionRow,
    pub widgets: Vec<DashboardWidgetRow>,
}

#[derive(Debug, Deserialize)]
pub struct SaveDashboardRequest {
    pub grid_columns: Option<i32>,
    pub refresh_interval: Option<i32>,
    pub settings: Option<serde_json::Value>,
    pub widgets: Vec<SaveDashboardWidgetInput>,
}

#[derive(Debug, Deserialize)]
pub struct SaveDashboardWidgetInput {
    pub title: String,
    pub widget_type: String,
    pub data_source_sql: String,
    pub x_axis_key: Option<String>,
    pub y_axis_key: Option<String>,
    pub series_config: Option<serde_json::Value>,
    pub colors: Option<serde_json::Value>,
    pub grid_x: i32,
    pub grid_y: i32,
    pub grid_w: i32,
    pub grid_h: i32,
    pub widget_config: Option<serde_json::Value>,
    pub sort_order: i32,
}

// ── Document Flow ──────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DocumentFlowEntry {
    pub id: Uuid,
    pub source_type: String,
    pub source_id: Uuid,
    pub target_type: String,
    pub target_id: Uuid,
    pub flow_type: String,
    pub created_at: DateTime<Utc>,
}

// ── Import / Export ────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct BulkImportRequest {
    pub records: Vec<serde_json::Value>,
    #[serde(default)]
    pub dry_run: bool,
    #[serde(default)]
    pub skip_invalid: bool,
}

#[derive(Debug, Serialize)]
pub struct BulkImportResult {
    pub inserted: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
    pub row_errors: Vec<BulkImportError>,
}

#[derive(Debug, Serialize)]
pub struct BulkImportError {
    pub row_index: usize,
    pub errors: Vec<FieldError>,
}

#[derive(Debug, Deserialize)]
pub struct ExportParams {
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FieldError {
    pub field_name: String,
    pub field_label: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<FieldError>,
    pub prepared_data: serde_json::Value,
}

// ── List Query (user-facing execution) ────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ListQueryParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    // Dynamic filter_* params are extracted manually from the raw query string
}

#[derive(Debug, Serialize)]
pub struct ListQueryResponse {
    pub items: Vec<serde_json::Value>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

// ── Workflow Actions ───────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct TransitionRequest {
    pub target_status: String,
    pub status_field: Option<String>,
    pub comment: Option<String>,
}

// ── AI Chat ───────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AiChatRequest {
    pub message: String,
    pub conversation_history: Option<Vec<AiChatMessage>>,
    pub context_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiChatMessage {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_changes: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct AiChatResponse {
    pub message: String,
    pub proposed_changes: Option<ProposedChanges>,
    pub tool_calls_executed: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProposedChanges {
    pub change_type: String,
    pub current: serde_json::Value,
    pub proposed: serde_json::Value,
    pub diff: serde_json::Value,
    pub summary: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AiStatusResponse {
    pub enabled: bool,
    pub provider: Option<String>,
    pub model: Option<String>,
}
