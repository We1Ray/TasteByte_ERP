// Low-Code Platform TypeScript Types

export interface LowCodeProject {
  id: string;
  code: string;
  name: string;
  description: string;
  status: "active" | "archived";
  owner_id: string;
  owner_name?: string;
  created_at: string;
  updated_at: string;
}

export interface LowCodeOperation {
  id: string;
  project_id: string;
  code: string;
  name: string;
  description: string;
  operation_type: "form" | "list" | "dashboard" | "report" | "workflow";
  table_name: string;
  status: "draft" | "published";
  version: number;
  created_by: string;
  created_at: string;
  updated_at: string;
  module?: 'FI' | 'CO' | 'MM' | 'SD' | 'PP' | 'HR' | 'WM' | 'QM' | null;
  sidebar_icon?: string;
  sidebar_sort_order?: number;
}

export type ErpModule = 'FI' | 'CO' | 'MM' | 'SD' | 'PP' | 'HR' | 'WM' | 'QM';

export interface OperationButton {
  id: string;
  operation_id: string;
  button_key: string;
  label: string;
  icon?: string;
  variant: 'primary' | 'secondary' | 'danger' | 'ghost';
  action_type: 'NAVIGATE' | 'API_CALL' | 'MODAL' | 'CUSTOM_JS';
  action_config: Record<string, unknown>;
  confirm_message?: string;
  required_permission?: string;
  is_visible: boolean;
  sort_order: number;
  created_at: string;
  updated_at: string;
}

export interface ModuleOperationSummary {
  id: string;
  operation_code: string;
  name: string;
  operation_type: string;
  sidebar_icon?: string;
  sidebar_sort_order: number;
}

export type FieldType =
  | "text"
  | "number"
  | "dropdown"
  | "multi_select"
  | "textarea"
  | "checkbox"
  | "file"
  | "lookup"
  | "composite"
  | "date"
  | "datetime"
  | "tree_table"
  | "document_flow"
  | "toggle"
  | "color"
  | "currency"
  | "radio_group"
  | "time_picker"
  | "rich_text"
  | "approval_buttons"
  | "master_detail";

export interface FieldValidation {
  required?: boolean;
  min_length?: number;
  max_length?: number;
  min_value?: number;
  max_value?: number;
  regex_pattern?: string;
  regex_message?: string;
  max_file_size?: number;
  allowed_extensions?: string[];
}

export interface FieldDataSource {
  type: "static" | "sql";
  static_options?: { label: string; value: string }[];
  sql_query?: string;
  value_column?: string;
  label_column?: string;
}

export interface VisibilityRule {
  field_id: string;
  operator: "equals" | "not_equals" | "contains" | "gt" | "lt";
  value: string;
}

export interface LookupConfig {
  operation_code: string;
  display_columns: string[];
  value_column: string;
  label_column: string;
}

export interface CompositeStep {
  title: string;
  fields: string[];
}

export interface FieldDefinition {
  id: string;
  section_id: string;
  field_key: string;
  field_type: FieldType;
  label: string;
  placeholder?: string;
  default_value?: string;
  help_text?: string;
  validation: FieldValidation;
  data_source?: FieldDataSource;
  db_column?: string;
  visibility_rules?: VisibilityRule[];
  lookup_config?: LookupConfig;
  composite_steps?: CompositeStep[];
  sort_order: number;
  is_readonly?: boolean;
  width?: "full" | "half" | "third" | "quarter";
  field_config?: Record<string, unknown>;
}

export interface FormSection {
  id: string;
  title: string;
  description?: string;
  columns: 1 | 2 | 3 | 4;
  collapsible: boolean;
  collapsed_default: boolean;
  sort_order: number;
  fields: FieldDefinition[];
}

export interface FormSettings {
  wizard?: WizardConfig;
}

export interface LayoutConfig {
  tabGroups?: TabGroupConfig[];
}

export interface TabGroupConfig {
  id: string;
  label: string;
  icon?: string;
  sort_order: number;
}

export interface FormDefinition {
  id: string;
  operation_id: string;
  version: number;
  sections: FormSection[];
  form_settings?: FormSettings;
  layout_config?: LayoutConfig;
  created_at: string;
  updated_at: string;
}

export interface FormRecord {
  id: string;
  operation_code: string;
  data: Record<string, unknown>;
  created_by: string;
  created_at: string;
  updated_at: string;
}

export interface OperationPermission {
  id: string;
  operation_id: string;
  role_id?: string | null;
  user_id?: string | null;
  can_create: boolean;
  can_read: boolean;
  can_update: boolean;
  can_delete: boolean;
  custom_permissions?: Record<string, unknown>;
  created_at?: string;
  updated_at?: string;
}

export interface FieldPermission {
  id: string;
  field_id: string;
  role_id?: string | null;
  user_id?: string | null;
  visibility: string;
  is_editable: boolean;
  created_at?: string;
  updated_at?: string;
}

export interface RecordPermission {
  id: string;
  operation_id: string;
  role_id?: string | null;
  user_id?: string | null;
  policy_name: string;
  filter_sql: string;
  is_active: boolean;
  created_at?: string;
  updated_at?: string;
}

export interface Release {
  id: string;
  operation_id: string;
  release_number?: string;
  version: number;
  title: string;
  description: string;
  status: "draft" | "submitted" | "approved" | "rejected" | "released";
  submitted_by?: string;
  submitted_at?: string;
  reviewed_by?: string;
  reviewed_at?: string;
  review_notes?: string;
  released_at?: string;
  form_snapshot?: unknown;
  created_at: string;
  updated_at: string;
}

export interface Feedback {
  id: string;
  project_id: string;
  operation_id?: string;
  feedback_type: "bug" | "feature" | "improvement";
  title: string;
  description: string;
  priority: "low" | "medium" | "high" | "critical";
  status: "open" | "in_progress" | "resolved" | "closed";
  reported_by: string;
  reporter_name?: string;
  assigned_to?: string;
  comments?: FeedbackComment[];
  created_at: string;
  updated_at: string;
}

export interface FeedbackComment {
  id: string;
  feedback_id: string;
  user_id: string;
  user_name: string;
  content: string;
  created_at: string;
}

export interface JournalEntry {
  id: string;
  operation_id: string;
  changed_by: string;
  change_type: string;
  entity_type?: string;
  entity_id?: string;
  old_values?: unknown;
  new_values?: unknown;
  diff_summary?: string;
  form_snapshot?: unknown;
  version?: number;
  created_at: string;
  /** Frontend-enriched field (not from backend) */
  user_name?: string;
}

export interface NavigationItem {
  id: string;
  parent_id: string | null;
  label: string;
  icon?: string;
  href?: string;
  operation_code?: string;
  sort_order: number;
  visible: boolean;
  roles?: string[];
  children?: NavigationItem[];
}

export interface FileAttachment {
  id: string;
  filename: string;
  content_type: string;
  size: number;
  url: string;
  uploaded_by: string;
  created_at: string;
}

export interface Notification {
  id: string;
  user_id: string;
  title: string;
  message: string;
  notification_type: "info" | "warning" | "error" | "success";
  is_read: boolean;
  link?: string;
  created_at: string;
}

// ── User Profile & Role Management ─────────────────────────────────────

export interface UserProfile {
  platform_roles: string[];
  projects: UserProjectRole[];
}

export interface UserProjectRole {
  id: string;
  name: string;
  role: "LEAD" | "DEVELOPER" | "VIEWER";
}

export interface UserWithRoles {
  id: string;
  username: string;
  email: string;
  full_name: string;
  is_active: boolean;
  platform_roles: string[];
}

export interface ProjectDeveloperInfo {
  id: string;
  user_id: string;
  username: string;
  full_name: string;
  email: string;
  role: string;
  created_at: string;
}

// ── Datasource ─────────────────────────────────────────────────────────

export interface DatasourceQueryResult {
  columns: string[];
  rows: Record<string, unknown>[];
  total: number;
}

export interface TableInfo {
  table_name: string;
  columns: ColumnInfo[];
}

export interface ColumnInfo {
  column_name: string;
  data_type: string;
  is_nullable: boolean;
  is_primary_key: boolean;
  column_default?: string;
  ordinal_position: number;
}

export interface SqlValidationResult {
  valid: boolean;
  error?: string;
  columns?: string[];
}

// ── LIST Operation Types ───────────────────────────────────────────────

export interface ListDefinition {
  id: string;
  operation_id: string;
  data_source_sql: string;
  default_page_size: number;
  enable_search: boolean;
  enable_export: boolean;
  enable_import: boolean;
  settings: Record<string, unknown>;
  version: number;
  columns: ListColumn[];
  actions: ListAction[];
  created_at: string;
  updated_at: string;
}

export interface ListColumn {
  id: string;
  list_id: string;
  field_key: string;
  label: string;
  data_type: string;
  width?: number;
  min_width?: number;
  is_sortable: boolean;
  is_filterable: boolean;
  is_visible: boolean;
  format_pattern?: string;
  cell_renderer?: string;
  sort_order: number;
}

export interface ListAction {
  id: string;
  list_id: string;
  action_key: string;
  label: string;
  icon?: string;
  action_type: "navigate" | "modal" | "api_call" | "delete";
  target_url?: string;
  confirm_message?: string;
  sort_order: number;
}

// ── DASHBOARD Operation Types ──────────────────────────────────────────

export interface DashboardDefinition {
  id: string;
  operation_id: string;
  grid_columns: number;
  refresh_interval?: number;
  settings: Record<string, unknown>;
  version: number;
  widgets: DashboardWidget[];
  created_at: string;
  updated_at: string;
}

export interface DashboardWidget {
  id: string;
  dashboard_id: string;
  title: string;
  widget_type: "bar" | "line" | "pie" | "kpi" | "table";
  data_source_sql: string;
  x_axis_key?: string;
  y_axis_key?: string;
  series_config: { dataKey: string; color: string; name?: string }[];
  colors: string[];
  grid_x: number;
  grid_y: number;
  grid_w: number;
  grid_h: number;
  widget_config: Record<string, unknown>;
  sort_order: number;
}

// ── Field Config Types ─────────────────────────────────────────────────

export interface ToggleFieldConfig {
  onLabel?: string;
  offLabel?: string;
  defaultValue?: boolean;
}

export interface ColorFieldConfig {
  presets?: string[];
  allowCustom?: boolean;
}

export interface CurrencyFieldConfig {
  currency?: "TWD" | "USD" | "EUR" | "JPY" | "GBP" | "CNY";
  decimals?: number;
  showSymbol?: boolean;
}

export interface RadioGroupFieldConfig {
  layout?: "horizontal" | "vertical" | "button";
  options?: { value: string; label: string; color?: string }[];
}

export interface TimePickerFieldConfig {
  format?: "12h" | "24h";
  minuteStep?: number;
}

export interface RichTextFieldConfig {
  toolbar?: string[];
  maxLength?: number;
}

export interface ApprovalButtonsFieldConfig {
  actions?: {
    label: string;
    targetStatus: string;
    requireComment?: boolean;
    color?: string;
  }[];
  statusField?: string;
}

export interface MasterDetailFieldConfig {
  detailColumns?: { field_key: string; label: string; field_type: string; width?: number }[];
  foreignKey?: string;
}

export interface TreeTableFieldConfig {
  parent_field?: string;
  id_field?: string;
  expand_level?: number;
  columns?: { field_key: string; label: string; width?: number }[];
  data_source?: FieldDataSource;
}

export interface DocumentFlowConfig {
  document_type_field?: string;
  document_id_field?: string;
}

// ── View Config Types ──────────────────────────────────────────────────

export interface CalendarViewConfig {
  dateField: string;
  titleField: string;
  colorField?: string;
}

export interface KanbanViewConfig {
  statusField: string;
  titleField: string;
  descriptionField?: string;
  columns: { value: string; label: string; color: string }[];
}

export interface WizardConfig {
  steps: { title: string; sectionIds: string[]; description?: string }[];
}

// ── Document Flow ──────────────────────────────────────────────────────

export interface DocumentFlowNode {
  id: string;
  document_type: string;
  document_id: string;
  document_number?: string;
  status?: string;
  created_at?: string;
  children: DocumentFlowNode[];
}
