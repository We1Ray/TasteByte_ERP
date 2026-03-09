-- Low-Code Seed Data: platform roles, number ranges, admin role assignment, default navigation

-- Platform roles
INSERT INTO lc_platform_roles (role_name, description) VALUES
    ('PLATFORM_ADMIN', 'Full access to all low-code platform features, can manage projects, users, and settings'),
    ('DEVELOPER', 'Can create and modify operations, forms, and fields within assigned projects'),
    ('USER', 'Can use published operations and submit feedback')
ON CONFLICT (role_name) DO NOTHING;

-- Number ranges for low-code entities
INSERT INTO number_ranges (object_type, prefix, current_number, pad_length) VALUES
    ('LCP', 'LCP', 0, 8),
    ('LCO', 'LCO', 0, 8),
    ('LCR', 'LCR', 0, 8),
    ('TKT', 'TKT', 0, 8)
ON CONFLICT (object_type) DO NOTHING;

-- Assign PLATFORM_ADMIN role to admin user
INSERT INTO lc_user_platform_roles (user_id, role_id, assigned_by)
SELECT u.id, r.id, u.id
FROM users u, lc_platform_roles r
WHERE u.username = 'admin' AND r.role_name = 'PLATFORM_ADMIN'
ON CONFLICT (user_id, role_id) DO NOTHING;

-- Default navigation items for the low-code section
INSERT INTO lc_navigation_items (id, parent_id, title, icon, route, sort_order, is_visible, required_role) VALUES
    -- Top-level low-code section
    ('a0000000-0000-0000-0000-000000000001'::uuid, NULL, 'Low-Code Platform', 'Blocks', '/lowcode', 900, true, 'DEVELOPER'),
    -- Sub-items
    ('a0000000-0000-0000-0000-000000000002'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Dashboard', 'LayoutDashboard', '/lowcode/dashboard', 1, true, 'DEVELOPER'),
    ('a0000000-0000-0000-0000-000000000003'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Projects', 'FolderKanban', '/lowcode/projects', 2, true, 'DEVELOPER'),
    ('a0000000-0000-0000-0000-000000000004'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Operations', 'Cog', '/lowcode/operations', 3, true, 'DEVELOPER'),
    ('a0000000-0000-0000-0000-000000000005'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Releases', 'Rocket', '/lowcode/releases', 4, true, 'DEVELOPER'),
    ('a0000000-0000-0000-0000-000000000006'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Feedback', 'MessageSquare', '/lowcode/feedback', 5, true, 'USER'),
    ('a0000000-0000-0000-0000-000000000007'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Notifications', 'Bell', '/lowcode/notifications', 6, true, 'USER'),
    ('a0000000-0000-0000-0000-000000000008'::uuid, 'a0000000-0000-0000-0000-000000000001'::uuid, 'Admin', 'Shield', '/lowcode/admin', 7, true, 'PLATFORM_ADMIN')
ON CONFLICT (id) DO NOTHING;
