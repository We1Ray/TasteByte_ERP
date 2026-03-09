-- Low-Code Permissions: platform roles, project developers, operation and field permissions, row-level security

-- Platform roles: three-tier role system
CREATE TABLE IF NOT EXISTS lc_platform_roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_name VARCHAR(50) NOT NULL UNIQUE
        CHECK (role_name IN ('PLATFORM_ADMIN', 'DEVELOPER', 'USER')),
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- User platform role mappings
CREATE TABLE IF NOT EXISTS lc_user_platform_roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES lc_platform_roles(id),
    assigned_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, role_id)
);

-- Project developers: which devs can modify which projects
CREATE TABLE IF NOT EXISTS lc_project_developers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES lc_projects(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(20) NOT NULL DEFAULT 'DEVELOPER'
        CHECK (role IN ('LEAD', 'DEVELOPER', 'VIEWER')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(project_id, user_id)
);

-- Operation permissions: CRUD + custom permissions per role/user per operation
CREATE TABLE IF NOT EXISTS lc_operation_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_id UUID NOT NULL REFERENCES lc_operations(id) ON DELETE CASCADE,
    role_id UUID REFERENCES lc_platform_roles(id),
    user_id UUID REFERENCES users(id),
    can_create BOOLEAN NOT NULL DEFAULT false,
    can_read BOOLEAN NOT NULL DEFAULT true,
    can_update BOOLEAN NOT NULL DEFAULT false,
    can_delete BOOLEAN NOT NULL DEFAULT false,
    custom_permissions JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (role_id IS NOT NULL OR user_id IS NOT NULL)
);

-- Field permissions: field-level visibility and editability
CREATE TABLE IF NOT EXISTS lc_field_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    field_id UUID NOT NULL REFERENCES lc_field_definitions(id) ON DELETE CASCADE,
    role_id UUID REFERENCES lc_platform_roles(id),
    user_id UUID REFERENCES users(id),
    visibility VARCHAR(20) NOT NULL DEFAULT 'VISIBLE'
        CHECK (visibility IN ('VISIBLE', 'HIDDEN', 'MASKED')),
    is_editable BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (role_id IS NOT NULL OR user_id IS NOT NULL)
);

-- Record policies: row-level security via SQL filter clauses
CREATE TABLE IF NOT EXISTS lc_record_policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_id UUID NOT NULL REFERENCES lc_operations(id) ON DELETE CASCADE,
    role_id UUID REFERENCES lc_platform_roles(id),
    user_id UUID REFERENCES users(id),
    policy_name VARCHAR(200) NOT NULL,
    filter_sql TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes: foreign keys
CREATE INDEX idx_lc_user_platform_roles_user_id ON lc_user_platform_roles(user_id);
CREATE INDEX idx_lc_user_platform_roles_role_id ON lc_user_platform_roles(role_id);
CREATE INDEX idx_lc_project_developers_project_id ON lc_project_developers(project_id);
CREATE INDEX idx_lc_project_developers_user_id ON lc_project_developers(user_id);
CREATE INDEX idx_lc_operation_permissions_operation_id ON lc_operation_permissions(operation_id);
CREATE INDEX idx_lc_operation_permissions_role_id ON lc_operation_permissions(role_id);
CREATE INDEX idx_lc_operation_permissions_user_id ON lc_operation_permissions(user_id);
CREATE INDEX idx_lc_field_permissions_field_id ON lc_field_permissions(field_id);
CREATE INDEX idx_lc_field_permissions_role_id ON lc_field_permissions(role_id);
CREATE INDEX idx_lc_field_permissions_user_id ON lc_field_permissions(user_id);
CREATE INDEX idx_lc_record_policies_operation_id ON lc_record_policies(operation_id);
CREATE INDEX idx_lc_record_policies_role_id ON lc_record_policies(role_id);
CREATE INDEX idx_lc_record_policies_user_id ON lc_record_policies(user_id);
CREATE INDEX idx_lc_record_policies_is_active ON lc_record_policies(is_active) WHERE is_active = true;
