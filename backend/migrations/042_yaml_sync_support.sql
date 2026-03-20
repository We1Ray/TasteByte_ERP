-- 042_yaml_sync_support.sql
-- Add is_yaml_managed flag to lc_operations for YAML-based operation definitions

ALTER TABLE lc_operations ADD COLUMN IF NOT EXISTS is_yaml_managed BOOLEAN NOT NULL DEFAULT false;
UPDATE lc_operations SET is_yaml_managed = true WHERE operation_code IN ('MM-GRN','SD-DELIVERY','WM-MOVE','MM-EVAL','QM-INSP','HR-LEAVE','PP-CONSUME');
CREATE INDEX IF NOT EXISTS idx_lc_operations_yaml ON lc_operations(is_yaml_managed) WHERE is_yaml_managed = true;
