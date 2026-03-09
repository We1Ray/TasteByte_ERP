-- Add WORKFLOW to operation_type CHECK constraint
ALTER TABLE lc_operations DROP CONSTRAINT IF EXISTS lc_operations_operation_type_check;
ALTER TABLE lc_operations ADD CONSTRAINT lc_operations_operation_type_check
    CHECK (operation_type IN ('FORM', 'LIST', 'DASHBOARD', 'REPORT', 'WORKFLOW'));
