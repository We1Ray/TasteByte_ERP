-- 034_lowcode_backend_fixes.sql
-- Fix foreign key constraints and add index

-- Fix lc_operation_data.operation_id: add ON DELETE CASCADE
ALTER TABLE lc_operation_data DROP CONSTRAINT IF EXISTS lc_operation_data_operation_id_fkey;
ALTER TABLE lc_operation_data ADD CONSTRAINT lc_operation_data_operation_id_fkey
    FOREIGN KEY (operation_id) REFERENCES lc_operations(id) ON DELETE CASCADE;

-- Fix lc_file_uploads.operation_id: add ON DELETE CASCADE
ALTER TABLE lc_file_uploads DROP CONSTRAINT IF EXISTS lc_file_uploads_operation_id_fkey;
ALTER TABLE lc_file_uploads ADD CONSTRAINT lc_file_uploads_operation_id_fkey
    FOREIGN KEY (operation_id) REFERENCES lc_operations(id) ON DELETE CASCADE;

-- Fix lc_file_uploads.field_id: change to ON DELETE SET NULL
ALTER TABLE lc_file_uploads DROP CONSTRAINT IF EXISTS lc_file_uploads_field_id_fkey;
ALTER TABLE lc_file_uploads ADD CONSTRAINT lc_file_uploads_field_id_fkey
    FOREIGN KEY (field_id) REFERENCES lc_field_definitions(id) ON DELETE SET NULL;

-- Add composite index on audit_log for table_name + record_id lookup
CREATE INDEX IF NOT EXISTS idx_audit_log_table_record ON audit_log(table_name, record_id);
