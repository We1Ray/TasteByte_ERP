-- Low-Code Platform: Core tables for projects, operations, form definitions, fields, and data storage

-- Projects: groups operations; developers work within projects
CREATE TABLE IF NOT EXISTS lc_projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_number VARCHAR(20) NOT NULL UNIQUE,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Operations: a screen/form (like SAP t-code)
CREATE TABLE IF NOT EXISTS lc_operations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_code VARCHAR(20) NOT NULL UNIQUE,
    project_id UUID NOT NULL REFERENCES lc_projects(id),
    name VARCHAR(200) NOT NULL,
    description TEXT,
    target_table VARCHAR(100),
    operation_type VARCHAR(20) NOT NULL DEFAULT 'FORM'
        CHECK (operation_type IN ('FORM', 'LIST', 'DASHBOARD', 'REPORT')),
    is_published BOOLEAN NOT NULL DEFAULT false,
    version INTEGER NOT NULL DEFAULT 1,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Form definitions: one per operation, stores layout config
CREATE TABLE IF NOT EXISTS lc_form_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_id UUID NOT NULL REFERENCES lc_operations(id) UNIQUE,
    layout_config JSONB NOT NULL DEFAULT '{}',
    form_settings JSONB NOT NULL DEFAULT '{}',
    snapshot JSONB,
    version INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Form sections: visual groupings (card containers)
CREATE TABLE IF NOT EXISTS lc_form_sections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    form_id UUID NOT NULL REFERENCES lc_form_definitions(id) ON DELETE CASCADE,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    columns INTEGER NOT NULL DEFAULT 2 CHECK (columns BETWEEN 1 AND 4),
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_collapsible BOOLEAN NOT NULL DEFAULT false,
    is_default_collapsed BOOLEAN NOT NULL DEFAULT false,
    visibility_rule JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Field definitions: heart of the low-code system
CREATE TABLE IF NOT EXISTS lc_field_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    section_id UUID NOT NULL REFERENCES lc_form_sections(id) ON DELETE CASCADE,
    field_name VARCHAR(100) NOT NULL,
    field_label VARCHAR(200) NOT NULL,
    field_type VARCHAR(30) NOT NULL
        CHECK (field_type IN (
            'TEXT', 'NUMBER', 'DECIMAL', 'DROPDOWN', 'MULTI_SELECT',
            'TEXTAREA', 'CHECKBOX', 'FILE_UPLOAD', 'LOOKUP_WINDOW',
            'COMPOSITE', 'DATE', 'DATETIME', 'HIDDEN', 'READONLY_COMPUTED'
        )),
    db_table VARCHAR(100),
    db_column VARCHAR(100),
    is_required BOOLEAN NOT NULL DEFAULT false,
    is_unique BOOLEAN NOT NULL DEFAULT false,
    is_searchable BOOLEAN NOT NULL DEFAULT false,
    default_value TEXT,
    default_value_sql TEXT,
    placeholder TEXT,
    help_text TEXT,
    validation_regex VARCHAR(500),
    validation_message VARCHAR(500),
    min_value NUMERIC,
    max_value NUMERIC,
    min_length INTEGER,
    max_length INTEGER,
    depends_on UUID REFERENCES lc_field_definitions(id),
    data_source_sql TEXT,
    display_column VARCHAR(100),
    value_column VARCHAR(100),
    visibility_rule JSONB,
    field_config JSONB NOT NULL DEFAULT '{}',
    sort_order INTEGER NOT NULL DEFAULT 0,
    column_span INTEGER NOT NULL DEFAULT 1 CHECK (column_span BETWEEN 1 AND 4),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(section_id, field_name)
);

-- Field options: static dropdown/multiselect options
CREATE TABLE IF NOT EXISTS lc_field_options (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    field_id UUID NOT NULL REFERENCES lc_field_definitions(id) ON DELETE CASCADE,
    option_label VARCHAR(200) NOT NULL,
    option_value VARCHAR(200) NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_default BOOLEAN NOT NULL DEFAULT false,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Operation data: generic JSONB storage for operations without target_table
CREATE TABLE IF NOT EXISTS lc_operation_data (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_id UUID NOT NULL REFERENCES lc_operations(id),
    data JSONB NOT NULL DEFAULT '{}',
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- File uploads: file metadata
CREATE TABLE IF NOT EXISTS lc_file_uploads (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_id UUID NOT NULL REFERENCES lc_operations(id),
    record_id UUID,
    field_id UUID REFERENCES lc_field_definitions(id),
    file_name VARCHAR(500) NOT NULL,
    file_type VARCHAR(100),
    file_size BIGINT,
    storage_path VARCHAR(1000) NOT NULL,
    uploaded_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes: foreign keys
CREATE INDEX idx_lc_operations_project_id ON lc_operations(project_id);
CREATE INDEX idx_lc_operations_created_by ON lc_operations(created_by);
CREATE INDEX idx_lc_form_sections_form_id ON lc_form_sections(form_id);
CREATE INDEX idx_lc_field_definitions_section_id ON lc_field_definitions(section_id);
CREATE INDEX idx_lc_field_definitions_depends_on ON lc_field_definitions(depends_on);
CREATE INDEX idx_lc_field_options_field_id ON lc_field_options(field_id);
CREATE INDEX idx_lc_operation_data_operation_id ON lc_operation_data(operation_id);
CREATE INDEX idx_lc_file_uploads_operation_id ON lc_file_uploads(operation_id);
CREATE INDEX idx_lc_file_uploads_field_id ON lc_file_uploads(field_id);
CREATE INDEX idx_lc_file_uploads_record_id ON lc_file_uploads(record_id);

-- Indexes: commonly queried columns
CREATE INDEX idx_lc_projects_is_active ON lc_projects(is_active);
CREATE INDEX idx_lc_operations_operation_type ON lc_operations(operation_type);
CREATE INDEX idx_lc_operations_is_published ON lc_operations(is_published);
CREATE INDEX idx_lc_form_sections_sort_order ON lc_form_sections(form_id, sort_order);
CREATE INDEX idx_lc_field_definitions_sort_order ON lc_field_definitions(section_id, sort_order);
CREATE INDEX idx_lc_field_definitions_field_type ON lc_field_definitions(field_type);
CREATE INDEX idx_lc_field_definitions_is_searchable ON lc_field_definitions(is_searchable) WHERE is_searchable = true;
