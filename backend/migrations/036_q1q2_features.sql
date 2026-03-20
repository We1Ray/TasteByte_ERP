-- 036_q1q2_features.sql
-- Q1+Q2: Approval Matrix, BPM Workflow, Output Determination, Form Variants, Number Range UI, Auth Trace

-- ── Approval Matrix ──────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS approval_matrices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    operation_id UUID REFERENCES lc_operations(id) ON DELETE CASCADE,
    description TEXT,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS approval_levels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    matrix_id UUID NOT NULL REFERENCES approval_matrices(id) ON DELETE CASCADE,
    level_order INT NOT NULL,
    name VARCHAR(200) NOT NULL,
    condition_field VARCHAR(200),
    condition_operator VARCHAR(20) DEFAULT 'gte',
    condition_value NUMERIC,
    approver_type VARCHAR(20) NOT NULL DEFAULT 'ROLE',
    approver_role VARCHAR(100),
    approver_user_id UUID,
    is_parallel BOOLEAN DEFAULT false,
    sla_hours INT DEFAULT 24,
    auto_escalate BOOLEAN DEFAULT false,
    escalate_to_role VARCHAR(100),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS approval_instances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    matrix_id UUID NOT NULL REFERENCES approval_matrices(id),
    operation_id UUID NOT NULL,
    record_id UUID NOT NULL,
    status VARCHAR(20) DEFAULT 'PENDING',
    current_level INT DEFAULT 1,
    submitted_by UUID,
    submitted_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS approval_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    instance_id UUID NOT NULL REFERENCES approval_instances(id) ON DELETE CASCADE,
    level_id UUID NOT NULL REFERENCES approval_levels(id),
    action VARCHAR(20) NOT NULL,
    acted_by UUID NOT NULL,
    comment TEXT,
    acted_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS approval_delegates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    delegate_to UUID NOT NULL,
    valid_from TIMESTAMPTZ NOT NULL,
    valid_until TIMESTAMPTZ NOT NULL,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ── BPM Workflow Engine ──────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS workflow_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    operation_id UUID REFERENCES lc_operations(id) ON DELETE CASCADE,
    trigger_event VARCHAR(50) NOT NULL DEFAULT 'ON_CREATE',
    definition JSONB NOT NULL DEFAULT '{}'::jsonb,
    is_active BOOLEAN DEFAULT true,
    version INT DEFAULT 1,
    created_by UUID,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS workflow_instances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    definition_id UUID NOT NULL REFERENCES workflow_definitions(id),
    record_id UUID NOT NULL,
    operation_id UUID NOT NULL,
    current_node VARCHAR(200),
    status VARCHAR(20) DEFAULT 'RUNNING',
    context JSONB DEFAULT '{}'::jsonb,
    started_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    error_message TEXT
);

CREATE TABLE IF NOT EXISTS workflow_execution_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    instance_id UUID NOT NULL REFERENCES workflow_instances(id) ON DELETE CASCADE,
    node_id VARCHAR(200) NOT NULL,
    node_type VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL,
    input_data JSONB,
    output_data JSONB,
    error_message TEXT,
    executed_at TIMESTAMPTZ DEFAULT NOW()
);

-- ── Output Determination ─────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS output_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    operation_id UUID REFERENCES lc_operations(id) ON DELETE CASCADE,
    trigger_event VARCHAR(50) NOT NULL DEFAULT 'ON_CREATE',
    condition_field VARCHAR(200),
    condition_operator VARCHAR(20),
    condition_value TEXT,
    output_type VARCHAR(20) NOT NULL DEFAULT 'EMAIL',
    email_template_code VARCHAR(100),
    print_layout_code VARCHAR(100),
    recipient_type VARCHAR(20) DEFAULT 'FIELD',
    recipient_field VARCHAR(200),
    recipient_static TEXT,
    is_active BOOLEAN DEFAULT true,
    sort_order INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS output_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rule_id UUID REFERENCES output_rules(id) ON DELETE SET NULL,
    operation_id UUID,
    record_id UUID,
    output_type VARCHAR(20),
    recipient TEXT,
    status VARCHAR(20) DEFAULT 'PENDING',
    error_message TEXT,
    executed_at TIMESTAMPTZ DEFAULT NOW()
);

-- ── Form Variants ────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS form_variants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_id UUID NOT NULL REFERENCES lc_operations(id) ON DELETE CASCADE,
    variant_name VARCHAR(200) NOT NULL,
    condition_field VARCHAR(200),
    condition_value TEXT,
    hidden_fields TEXT[] DEFAULT '{}',
    readonly_fields TEXT[] DEFAULT '{}',
    required_fields TEXT[] DEFAULT '{}',
    default_values JSONB DEFAULT '{}'::jsonb,
    layout_overrides JSONB DEFAULT '{}'::jsonb,
    is_default BOOLEAN DEFAULT false,
    sort_order INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(operation_id, variant_name)
);

-- ── Cross-field Validation Rules ─────────────────────────────────────
CREATE TABLE IF NOT EXISTS cross_field_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_id UUID NOT NULL REFERENCES lc_operations(id) ON DELETE CASCADE,
    rule_name VARCHAR(200) NOT NULL,
    description TEXT,
    rule_type VARCHAR(20) NOT NULL DEFAULT 'VALIDATION',
    source_field VARCHAR(200) NOT NULL,
    operator VARCHAR(20) NOT NULL,
    target_field VARCHAR(200),
    target_value TEXT,
    error_message TEXT NOT NULL,
    is_active BOOLEAN DEFAULT true,
    sort_order INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ── Calculation Formulas ─────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS calculation_formulas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_id UUID NOT NULL REFERENCES lc_operations(id) ON DELETE CASCADE,
    target_field VARCHAR(200) NOT NULL,
    formula TEXT NOT NULL,
    trigger_fields TEXT[] NOT NULL DEFAULT '{}',
    description TEXT,
    is_active BOOLEAN DEFAULT true,
    sort_order INT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ── Number Range Configuration ───────────────────────────────────────
CREATE TABLE IF NOT EXISTS number_range_config (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    range_prefix VARCHAR(20) NOT NULL UNIQUE,
    description VARCHAR(200),
    current_value BIGINT DEFAULT 0,
    start_value BIGINT DEFAULT 1,
    end_value BIGINT DEFAULT 99999999,
    padding INT DEFAULT 3,
    separator VARCHAR(5) DEFAULT '-',
    fiscal_year_dependent BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Insert default number ranges
INSERT INTO number_range_config (range_prefix, description, current_value, padding) VALUES
('LCP', 'Lowcode Projects', 0, 3),
('LCO', 'Lowcode Operations', 0, 3),
('LCR', 'Lowcode Releases', 0, 3),
('TRN', 'Transport Orders', 0, 3),
('SO', 'Sales Orders', 0, 5),
('PO', 'Purchase Orders', 0, 5),
('INV', 'Invoices', 0, 5),
('PRD', 'Production Orders', 0, 5),
('MAT', 'Materials', 0, 5),
('JE', 'Journal Entries', 0, 5),
('FB', 'Feedback Tickets', 0, 4)
ON CONFLICT (range_prefix) DO NOTHING;

-- ── Authorization Trace ──────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS auth_trace_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    resource_type VARCHAR(50) NOT NULL,
    resource_id UUID,
    action VARCHAR(20) NOT NULL,
    result VARCHAR(10) NOT NULL,
    reason TEXT,
    checked_roles TEXT[],
    checked_permissions JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_approval_instances_record ON approval_instances(operation_id, record_id);
CREATE INDEX IF NOT EXISTS idx_approval_instances_status ON approval_instances(status);
CREATE INDEX IF NOT EXISTS idx_workflow_instances_record ON workflow_instances(operation_id, record_id);
CREATE INDEX IF NOT EXISTS idx_workflow_instances_status ON workflow_instances(status);
CREATE INDEX IF NOT EXISTS idx_output_rules_operation ON output_rules(operation_id, trigger_event);
CREATE INDEX IF NOT EXISTS idx_output_log_record ON output_log(operation_id, record_id);
CREATE INDEX IF NOT EXISTS idx_form_variants_operation ON form_variants(operation_id);
CREATE INDEX IF NOT EXISTS idx_cross_field_rules_op ON cross_field_rules(operation_id);
CREATE INDEX IF NOT EXISTS idx_calc_formulas_op ON calculation_formulas(operation_id);
CREATE INDEX IF NOT EXISTS idx_auth_trace_user ON auth_trace_log(user_id, created_at DESC);
