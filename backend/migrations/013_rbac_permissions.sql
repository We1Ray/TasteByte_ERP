-- RBAC Permissions: seed permissions per module, grant to roles

-- Add VND and MVT number ranges (fix for vendor/movement number ranges)
INSERT INTO number_ranges (object_type, prefix, current_number, pad_length) VALUES
    ('VND', 'VND', 0, 8),
    ('MVT', 'MVT', 0, 8)
ON CONFLICT (object_type) DO NOTHING;

-- Add unique constraint on permissions (module, action) for idempotent inserts
ALTER TABLE permissions ADD CONSTRAINT uq_permissions_module_action UNIQUE (module, action);

-- Seed permissions for each module
INSERT INTO permissions (module, action, description) VALUES
    ('fi', 'read', 'View financial data'),
    ('fi', 'write', 'Create/modify financial data'),
    ('co', 'read', 'View controlling data'),
    ('co', 'write', 'Create/modify controlling data'),
    ('mm', 'read', 'View materials management data'),
    ('mm', 'write', 'Create/modify materials management data'),
    ('sd', 'read', 'View sales data'),
    ('sd', 'write', 'Create/modify sales data'),
    ('pp', 'read', 'View production data'),
    ('pp', 'write', 'Create/modify production data'),
    ('hr', 'read', 'View HR data'),
    ('hr', 'write', 'Create/modify HR data'),
    ('wm', 'read', 'View warehouse data'),
    ('wm', 'write', 'Create/modify warehouse data'),
    ('qm', 'read', 'View quality data'),
    ('qm', 'write', 'Create/modify quality data')
ON CONFLICT (module, action) DO NOTHING;

-- Grant all permissions to ADMIN role
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p WHERE r.name = 'ADMIN'
ON CONFLICT DO NOTHING;

-- Grant module-specific permissions to module managers
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p
WHERE (r.name = 'FI_MANAGER' AND p.module = 'fi')
   OR (r.name = 'MM_MANAGER' AND p.module = 'mm')
   OR (r.name = 'SD_MANAGER' AND p.module = 'sd')
   OR (r.name = 'PP_MANAGER' AND p.module = 'pp')
   OR (r.name = 'HR_MANAGER' AND p.module = 'hr')
   OR (r.name = 'WM_MANAGER' AND p.module = 'wm')
   OR (r.name = 'QM_MANAGER' AND p.module = 'qm')
ON CONFLICT DO NOTHING;

-- Grant read permissions to OPERATOR role
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p WHERE r.name = 'OPERATOR' AND p.action = 'read'
ON CONFLICT DO NOTHING;
