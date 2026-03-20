-- 044_form_improvements.sql
-- Fix field type constraint, add sub-table support, cross-op actions, state machine, conditional required

-- 1. Fix field_type CHECK constraint to allow all new types
ALTER TABLE lc_field_definitions DROP CONSTRAINT IF EXISTS lc_field_definitions_field_type_check;
ALTER TABLE lc_field_definitions ADD CONSTRAINT lc_field_definitions_field_type_check
    CHECK (field_type IN (
        'TEXT','NUMBER','DECIMAL','DROPDOWN','MULTI_SELECT','TEXTAREA','CHECKBOX',
        'FILE_UPLOAD','LOOKUP_WINDOW','COMPOSITE','DATE','DATETIME','HIDDEN','READONLY_COMPUTED',
        'TREE_TABLE','DOCUMENT_FLOW','TOGGLE','COLOR','CURRENCY','RADIO_GROUP',
        'TIME_PICKER','RICH_TEXT','APPROVAL_BUTTONS','MASTER_DETAIL',
        'SUB_TABLE'
    ));

-- 2. Add sub_table_config to field definitions (for SUB_TABLE type fields)
-- The config defines columns of the sub-table as JSON array
-- Data stored as JSON array in the field value: [{"col1":"val1","col2":"val2"}, ...]
ALTER TABLE lc_field_definitions ADD COLUMN IF NOT EXISTS sub_table_columns JSONB;
-- Format: [{"key":"material","label":"物料","type":"TEXT","required":true}, ...]

-- 3. Add lookup_fill_fields to field definitions
-- When a lookup selects a value, auto-fill other fields
ALTER TABLE lc_field_definitions ADD COLUMN IF NOT EXISTS lookup_fill_fields JSONB;
-- Format: {"source_column": "target_field_name", ...}
-- e.g., {"vendor_name": "vendor", "payment_terms": "terms"}

-- 4. Cross-operation write actions
CREATE TABLE IF NOT EXISTS cross_operation_actions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_operation_id UUID NOT NULL REFERENCES lc_operations(id) ON DELETE CASCADE,
    trigger_event VARCHAR(30) NOT NULL DEFAULT 'ON_CREATE',
    condition_field VARCHAR(200),
    condition_operator VARCHAR(20),
    condition_value TEXT,
    target_operation_code VARCHAR(20) NOT NULL,
    field_mapping JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN DEFAULT true,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
-- field_mapping format: {"target_field": "source_field_or_literal", ...}
-- e.g., {"movement_type": "'GI_SO'", "material_name": "material", "quantity": "shipped_qty"}
-- Values starting with ' are literals, others are source field references

CREATE INDEX IF NOT EXISTS idx_cross_op_actions_source ON cross_operation_actions(source_operation_id, trigger_event);

-- 5. State machine transitions
CREATE TABLE IF NOT EXISTS status_transitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_id UUID NOT NULL REFERENCES lc_operations(id) ON DELETE CASCADE,
    status_field VARCHAR(200) NOT NULL DEFAULT 'status',
    from_status VARCHAR(50) NOT NULL,
    to_status VARCHAR(50) NOT NULL,
    allowed_roles TEXT[] DEFAULT '{}',
    requires_approval BOOLEAN DEFAULT false,
    approval_matrix_id UUID,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(operation_id, status_field, from_status, to_status)
);

CREATE INDEX IF NOT EXISTS idx_status_transitions_op ON status_transitions(operation_id);

-- 6. Conditional required rules
ALTER TABLE lc_field_definitions ADD COLUMN IF NOT EXISTS required_rule JSONB;
-- Same format as visibility_rule: {"dependent_field":"reason","operator":"equals","value":"DEFECTIVE","action":"require"}

-- ═══════════════════════════════════════════════════════════════════
-- Seed: Cross-operation actions for key workflows
-- ═══════════════════════════════════════════════════════════════════

-- SD-DELIVERY shipped -> auto-create WM-MOVE (出庫)
INSERT INTO cross_operation_actions (source_operation_id, trigger_event, condition_field, condition_operator, condition_value, target_operation_code, field_mapping, description)
SELECT o.id, 'ON_CREATE', 'status', 'equals', 'SHIPPED', 'WM-MOVE',
    '{"movement_type":"''GI_SO''","material_name":"material_name","quantity":"shipped_qty","unit":"''PCS''","from_warehouse":"''WH-001''","reference_doc":"delivery_number","movement_date":"delivery_date","reason":"''銷售出貨 - '' || customer_name"}'::jsonb,
    '出貨確認後自動建立出庫異動'
FROM lc_operations o WHERE o.operation_code = 'SD-DELIVERY'
ON CONFLICT DO NOTHING;

-- MM-GRN confirmed -> auto-create WM-MOVE (入庫)
INSERT INTO cross_operation_actions (source_operation_id, trigger_event, condition_field, condition_operator, condition_value, target_operation_code, field_mapping, description)
SELECT o.id, 'ON_CREATE', 'status', 'equals', 'CONFIRMED', 'WM-MOVE',
    '{"movement_type":"''GR_PO''","material_name":"material_name","quantity":"received_qty","unit":"''KG''","to_warehouse":"warehouse","reference_doc":"grn_number","movement_date":"receipt_date","reason":"''採購收貨 - '' || vendor_name"}'::jsonb,
    '收貨確認後自動建立入庫異動'
FROM lc_operations o WHERE o.operation_code = 'MM-GRN'
ON CONFLICT DO NOTHING;

-- PP-CONSUME -> auto-create WM-MOVE (生產領料)
INSERT INTO cross_operation_actions (source_operation_id, trigger_event, condition_field, condition_operator, condition_value, target_operation_code, field_mapping, description)
SELECT o.id, 'ON_CREATE', null, null, null, 'WM-MOVE',
    '{"movement_type":"''GI_PROD''","material_name":"material_consumed","quantity":"actual_qty","unit":"''KG''","from_warehouse":"''WH-001''","reference_doc":"production_order","movement_date":"work_date","reason":"''生產領料 - '' || product_name"}'::jsonb,
    '生產用料確認後自動建立領料異動'
FROM lc_operations o WHERE o.operation_code = 'PP-CONSUME'
ON CONFLICT DO NOTHING;

-- ═══════════════════════════════════════════════════════════════════
-- Seed: State machine transitions for key operations
-- ═══════════════════════════════════════════════════════════════════

-- HR-LEAVE status transitions
INSERT INTO status_transitions (operation_id, status_field, from_status, to_status, allowed_roles, requires_approval, description)
SELECT o.id, 'status', f, t, r::text[], a, d FROM lc_operations o,
(VALUES
    ('PENDING','APPROVED','{"ADMIN","HR_MANAGER"}',true,'主管核准'),
    ('PENDING','REJECTED','{"ADMIN","HR_MANAGER"}',false,'主管退回'),
    ('PENDING','CANCELLED','{}',false,'申請人取消'),
    ('APPROVED','CANCELLED','{"ADMIN"}',false,'管理員取消')
) AS v(f,t,r,a,d)
WHERE o.operation_code = 'HR-LEAVE'
ON CONFLICT DO NOTHING;

-- MM-GRN status transitions
INSERT INTO status_transitions (operation_id, status_field, from_status, to_status, allowed_roles, description)
SELECT o.id, 'status', f, t, r::text[], d FROM lc_operations o,
(VALUES
    ('DRAFT','CONFIRMED','{"MM_MANAGER","ADMIN"}','採購確認收貨'),
    ('CONFIRMED','RECEIVED','{"MM_MANAGER","ADMIN"}','入庫完成'),
    ('RECEIVED','CLOSED','{"MM_MANAGER","ADMIN"}','結案')
) AS v(f,t,r,d)
WHERE o.operation_code = 'MM-GRN'
ON CONFLICT DO NOTHING;

-- SD-DELIVERY status transitions
INSERT INTO status_transitions (operation_id, status_field, from_status, to_status, allowed_roles, description)
SELECT o.id, 'status', f, t, r::text[], d FROM lc_operations o,
(VALUES
    ('PREPARING','SHIPPED','{"SD_MANAGER","ADMIN"}','出貨'),
    ('SHIPPED','IN_TRANSIT','{}','運送中'),
    ('IN_TRANSIT','DELIVERED','{}','已送達'),
    ('DELIVERED','SIGNED','{}','已簽收')
) AS v(f,t,r,d)
WHERE o.operation_code = 'SD-DELIVERY'
ON CONFLICT DO NOTHING;

-- ═══════════════════════════════════════════════════════════════════
-- Seed: Conditional required rules
-- ═══════════════════════════════════════════════════════════════════

-- MM-GRN: inspection_result=FAIL -> notes required
UPDATE lc_field_definitions SET required_rule = '{"dependent_field":"inspection_result","operator":"equals","value":"FAIL","action":"require"}'
WHERE field_name = 'notes' AND section_id IN (
    SELECT s.id FROM lc_form_sections s
    JOIN lc_form_definitions f ON f.id = s.form_id
    JOIN lc_operations o ON o.id = f.operation_id
    WHERE o.operation_code = 'MM-GRN'
) AND required_rule IS NULL;
