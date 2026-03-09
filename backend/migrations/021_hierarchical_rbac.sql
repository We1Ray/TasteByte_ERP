-- Hierarchical RBAC: add parent_id and sort_order to roles table for tree structure

-- 1. Add parent_id column for hierarchy (adjacency list model)
ALTER TABLE roles ADD COLUMN IF NOT EXISTS parent_id UUID REFERENCES roles(id) ON DELETE SET NULL;

-- 2. Add sort_order for display ordering
ALTER TABLE roles ADD COLUMN IF NOT EXISTS sort_order INT NOT NULL DEFAULT 0;

-- 3. Index on parent_id for efficient tree queries
CREATE INDEX IF NOT EXISTS idx_roles_parent_id ON roles(parent_id);

-- 4. Set up default hierarchy: ADMIN as root, managers as children, OPERATOR as leaf
-- ADMIN (root, parent_id = NULL)
UPDATE roles SET sort_order = 0 WHERE name = 'ADMIN';

-- Module managers report to ADMIN
UPDATE roles SET parent_id = (SELECT id FROM roles WHERE name = 'ADMIN'), sort_order = 1
  WHERE name = 'FI_MANAGER' AND parent_id IS NULL;
UPDATE roles SET parent_id = (SELECT id FROM roles WHERE name = 'ADMIN'), sort_order = 2
  WHERE name = 'MM_MANAGER' AND parent_id IS NULL;
UPDATE roles SET parent_id = (SELECT id FROM roles WHERE name = 'ADMIN'), sort_order = 3
  WHERE name = 'SD_MANAGER' AND parent_id IS NULL;
UPDATE roles SET parent_id = (SELECT id FROM roles WHERE name = 'ADMIN'), sort_order = 4
  WHERE name = 'PP_MANAGER' AND parent_id IS NULL;
UPDATE roles SET parent_id = (SELECT id FROM roles WHERE name = 'ADMIN'), sort_order = 5
  WHERE name = 'HR_MANAGER' AND parent_id IS NULL;
UPDATE roles SET parent_id = (SELECT id FROM roles WHERE name = 'ADMIN'), sort_order = 6
  WHERE name = 'WM_MANAGER' AND parent_id IS NULL;
UPDATE roles SET parent_id = (SELECT id FROM roles WHERE name = 'ADMIN'), sort_order = 7
  WHERE name = 'QM_MANAGER' AND parent_id IS NULL;

-- OPERATOR is a separate branch under ADMIN (read-only access)
UPDATE roles SET parent_id = (SELECT id FROM roles WHERE name = 'ADMIN'), sort_order = 8
  WHERE name = 'OPERATOR' AND parent_id IS NULL;

-- 5. Fix data mismatch: FI_MANAGER should also have CO module access
--    (matches current hardcoded rbac.rs behavior where CO uses FI_MANAGER)
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.id, p.id FROM roles r, permissions p
WHERE r.name = 'FI_MANAGER' AND p.module = 'co'
ON CONFLICT DO NOTHING;
