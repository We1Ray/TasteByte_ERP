-- 031: Module-Linked Operations & Operation-Level Action Buttons
-- Allows binding low-code operations to ERP modules and defining toolbar buttons.

-- 1. Add module binding fields to lc_operations
ALTER TABLE lc_operations
    ADD COLUMN IF NOT EXISTS module VARCHAR(10)
    CHECK (module IS NULL OR module IN ('FI','CO','MM','SD','PP','HR','WM','QM'));

ALTER TABLE lc_operations
    ADD COLUMN IF NOT EXISTS sidebar_icon VARCHAR(50),
    ADD COLUMN IF NOT EXISTS sidebar_sort_order INTEGER NOT NULL DEFAULT 100;

CREATE INDEX IF NOT EXISTS idx_lc_operations_module
    ON lc_operations(module) WHERE module IS NOT NULL;

-- 2. Operation-level action buttons
CREATE TABLE IF NOT EXISTS lc_operation_buttons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_id UUID NOT NULL REFERENCES lc_operations(id) ON DELETE CASCADE,
    button_key VARCHAR(50) NOT NULL,
    label VARCHAR(100) NOT NULL,
    icon VARCHAR(50),
    variant VARCHAR(20) NOT NULL DEFAULT 'secondary'
        CHECK (variant IN ('primary','secondary','danger','ghost')),
    action_type VARCHAR(20) NOT NULL DEFAULT 'API_CALL'
        CHECK (action_type IN ('NAVIGATE','API_CALL','MODAL','CUSTOM_JS')),
    action_config JSONB NOT NULL DEFAULT '{}',
    confirm_message VARCHAR(500),
    required_permission VARCHAR(50),
    is_visible BOOLEAN NOT NULL DEFAULT true,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(operation_id, button_key)
);
