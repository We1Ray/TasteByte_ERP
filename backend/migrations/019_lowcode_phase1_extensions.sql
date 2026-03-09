-- Migration 019: Low-Code Phase 1 Extensions
-- New field types, LIST/DASHBOARD definitions, document flow tracking

-- 1. Expand field_type CHECK constraint to include new field types
ALTER TABLE lc_field_definitions DROP CONSTRAINT IF EXISTS lc_field_definitions_field_type_check;
ALTER TABLE lc_field_definitions ADD CONSTRAINT lc_field_definitions_field_type_check
    CHECK (field_type IN (
        'TEXT', 'NUMBER', 'DECIMAL', 'DROPDOWN', 'MULTI_SELECT',
        'TEXTAREA', 'CHECKBOX', 'FILE_UPLOAD', 'LOOKUP_WINDOW',
        'COMPOSITE', 'DATE', 'DATETIME', 'HIDDEN', 'READONLY_COMPUTED',
        'TREE_TABLE', 'DOCUMENT_FLOW', 'TOGGLE', 'COLOR', 'CURRENCY',
        'RADIO_GROUP', 'TIME_PICKER', 'RICH_TEXT', 'APPROVAL_BUTTONS', 'MASTER_DETAIL'
    ));

-- 2. List operation definitions
CREATE TABLE IF NOT EXISTS lc_list_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_id UUID NOT NULL REFERENCES lc_operations(id) ON DELETE CASCADE,
    data_source_sql TEXT,
    default_page_size INTEGER NOT NULL DEFAULT 20,
    enable_search BOOLEAN NOT NULL DEFAULT true,
    enable_export BOOLEAN NOT NULL DEFAULT false,
    enable_import BOOLEAN NOT NULL DEFAULT false,
    settings JSONB NOT NULL DEFAULT '{}',
    version INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(operation_id)
);

-- 3. List column definitions
CREATE TABLE IF NOT EXISTS lc_list_columns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    list_id UUID NOT NULL REFERENCES lc_list_definitions(id) ON DELETE CASCADE,
    field_key VARCHAR(100) NOT NULL,
    label VARCHAR(200) NOT NULL,
    data_type VARCHAR(30) NOT NULL DEFAULT 'TEXT',
    width INTEGER,
    min_width INTEGER,
    is_sortable BOOLEAN NOT NULL DEFAULT true,
    is_filterable BOOLEAN NOT NULL DEFAULT false,
    is_visible BOOLEAN NOT NULL DEFAULT true,
    format_pattern VARCHAR(200),
    cell_renderer VARCHAR(50),
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(list_id, field_key)
);

-- 4. List row actions
CREATE TABLE IF NOT EXISTS lc_list_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    list_id UUID NOT NULL REFERENCES lc_list_definitions(id) ON DELETE CASCADE,
    action_key VARCHAR(50) NOT NULL,
    label VARCHAR(100) NOT NULL,
    icon VARCHAR(50),
    action_type VARCHAR(20) NOT NULL DEFAULT 'NAVIGATE'
        CHECK (action_type IN ('NAVIGATE', 'MODAL', 'API_CALL', 'DELETE')),
    target_url VARCHAR(500),
    confirm_message VARCHAR(500),
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 5. Dashboard definitions
CREATE TABLE IF NOT EXISTS lc_dashboard_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_id UUID NOT NULL REFERENCES lc_operations(id) ON DELETE CASCADE,
    grid_columns INTEGER NOT NULL DEFAULT 12,
    refresh_interval INTEGER,
    settings JSONB NOT NULL DEFAULT '{}',
    version INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(operation_id)
);

-- 6. Dashboard widgets
CREATE TABLE IF NOT EXISTS lc_dashboard_widgets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dashboard_id UUID NOT NULL REFERENCES lc_dashboard_definitions(id) ON DELETE CASCADE,
    title VARCHAR(200) NOT NULL,
    widget_type VARCHAR(20) NOT NULL
        CHECK (widget_type IN ('BAR', 'LINE', 'PIE', 'KPI', 'TABLE')),
    data_source_sql TEXT NOT NULL,
    x_axis_key VARCHAR(100),
    y_axis_key VARCHAR(100),
    series_config JSONB NOT NULL DEFAULT '[]',
    colors JSONB NOT NULL DEFAULT '[]',
    grid_x INTEGER NOT NULL DEFAULT 0,
    grid_y INTEGER NOT NULL DEFAULT 0,
    grid_w INTEGER NOT NULL DEFAULT 6,
    grid_h INTEGER NOT NULL DEFAULT 4,
    widget_config JSONB NOT NULL DEFAULT '{}',
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 7. Document flow tracking
CREATE TABLE IF NOT EXISTS lc_document_flows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_type VARCHAR(50) NOT NULL,
    source_id UUID NOT NULL,
    target_type VARCHAR(50) NOT NULL,
    target_id UUID NOT NULL,
    flow_type VARCHAR(50) NOT NULL DEFAULT 'DERIVED',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(source_type, source_id, target_type, target_id)
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_lc_list_columns_list_id ON lc_list_columns(list_id);
CREATE INDEX IF NOT EXISTS idx_lc_list_actions_list_id ON lc_list_actions(list_id);
CREATE INDEX IF NOT EXISTS idx_lc_dashboard_widgets_dashboard_id ON lc_dashboard_widgets(dashboard_id);
CREATE INDEX IF NOT EXISTS idx_lc_document_flows_source ON lc_document_flows(source_type, source_id);
CREATE INDEX IF NOT EXISTS idx_lc_document_flows_target ON lc_document_flows(target_type, target_id);
