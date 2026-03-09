-- Low-Code Workflow: dev journal, releases, feedback, notifications, navigation

-- Dev journal: git-like change log
CREATE TABLE IF NOT EXISTS lc_dev_journal (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    operation_id UUID NOT NULL REFERENCES lc_operations(id),
    changed_by UUID NOT NULL REFERENCES users(id),
    change_type VARCHAR(30) NOT NULL
        CHECK (change_type IN (
            'FORM_CREATED', 'FORM_UPDATED',
            'FIELD_ADDED', 'FIELD_UPDATED', 'FIELD_DELETED',
            'SECTION_ADDED', 'SECTION_UPDATED', 'SECTION_DELETED',
            'PERMISSION_CHANGED', 'PUBLISHED', 'UNPUBLISHED', 'ROLLBACK'
        )),
    entity_type VARCHAR(30),
    entity_id UUID,
    old_values JSONB,
    new_values JSONB,
    diff_summary TEXT,
    form_snapshot JSONB,
    version INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Feedback: bug reports and feature requests (created before releases so FK works)
CREATE TABLE IF NOT EXISTS lc_feedback (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_number VARCHAR(20) NOT NULL UNIQUE,
    operation_id UUID NOT NULL REFERENCES lc_operations(id),
    feedback_type VARCHAR(20) NOT NULL
        CHECK (feedback_type IN ('BUG', 'FEATURE_REQUEST', 'IMPROVEMENT')),
    title VARCHAR(300) NOT NULL,
    description TEXT NOT NULL,
    priority VARCHAR(20) NOT NULL DEFAULT 'MEDIUM'
        CHECK (priority IN ('LOW', 'MEDIUM', 'HIGH', 'CRITICAL')),
    status VARCHAR(20) NOT NULL DEFAULT 'OPEN'
        CHECK (status IN ('OPEN', 'IN_PROGRESS', 'RESOLVED', 'CLOSED', 'WONT_FIX')),
    assigned_to UUID REFERENCES users(id),
    submitted_by UUID NOT NULL REFERENCES users(id),
    resolved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Releases: version releases
CREATE TABLE IF NOT EXISTS lc_releases (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    release_number VARCHAR(20) NOT NULL UNIQUE,
    operation_id UUID NOT NULL REFERENCES lc_operations(id),
    version INTEGER NOT NULL,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'DRAFT'
        CHECK (status IN ('DRAFT', 'SUBMITTED', 'TESTING', 'APPROVED', 'RELEASED', 'REJECTED')),
    submitted_by UUID REFERENCES users(id),
    reviewed_by UUID REFERENCES users(id),
    review_notes TEXT,
    form_snapshot JSONB NOT NULL,
    submitted_at TIMESTAMPTZ,
    reviewed_at TIMESTAMPTZ,
    released_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Release feedback links: links releases to resolved feedback tickets
CREATE TABLE IF NOT EXISTS lc_release_feedback_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    release_id UUID NOT NULL REFERENCES lc_releases(id) ON DELETE CASCADE,
    feedback_id UUID NOT NULL REFERENCES lc_feedback(id) ON DELETE CASCADE,
    UNIQUE(release_id, feedback_id)
);

-- Feedback comments: discussion thread per ticket
CREATE TABLE IF NOT EXISTS lc_feedback_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    feedback_id UUID NOT NULL REFERENCES lc_feedback(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Notifications: in-app notification queue
CREATE TABLE IF NOT EXISTS lc_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(300) NOT NULL,
    message TEXT,
    notification_type VARCHAR(30) NOT NULL,
    reference_type VARCHAR(30),
    reference_id UUID,
    is_read BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Navigation items: dynamic sidebar configuration
CREATE TABLE IF NOT EXISTS lc_navigation_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parent_id UUID REFERENCES lc_navigation_items(id),
    title VARCHAR(100) NOT NULL,
    icon VARCHAR(50),
    route VARCHAR(200),
    operation_id UUID REFERENCES lc_operations(id),
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_visible BOOLEAN NOT NULL DEFAULT true,
    required_role VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes: dev journal
CREATE INDEX idx_lc_dev_journal_operation_id ON lc_dev_journal(operation_id);
CREATE INDEX idx_lc_dev_journal_changed_by ON lc_dev_journal(changed_by);
CREATE INDEX idx_lc_dev_journal_change_type ON lc_dev_journal(change_type);
CREATE INDEX idx_lc_dev_journal_created_at ON lc_dev_journal(created_at);

-- Indexes: feedback
CREATE INDEX idx_lc_feedback_operation_id ON lc_feedback(operation_id);
CREATE INDEX idx_lc_feedback_status ON lc_feedback(status);
CREATE INDEX idx_lc_feedback_priority ON lc_feedback(priority);
CREATE INDEX idx_lc_feedback_assigned_to ON lc_feedback(assigned_to);
CREATE INDEX idx_lc_feedback_submitted_by ON lc_feedback(submitted_by);
CREATE INDEX idx_lc_feedback_feedback_type ON lc_feedback(feedback_type);

-- Indexes: releases
CREATE INDEX idx_lc_releases_operation_id ON lc_releases(operation_id);
CREATE INDEX idx_lc_releases_status ON lc_releases(status);
CREATE INDEX idx_lc_releases_submitted_by ON lc_releases(submitted_by);
CREATE INDEX idx_lc_releases_reviewed_by ON lc_releases(reviewed_by);

-- Indexes: release feedback links
CREATE INDEX idx_lc_release_feedback_links_release_id ON lc_release_feedback_links(release_id);
CREATE INDEX idx_lc_release_feedback_links_feedback_id ON lc_release_feedback_links(feedback_id);

-- Indexes: feedback comments
CREATE INDEX idx_lc_feedback_comments_feedback_id ON lc_feedback_comments(feedback_id);
CREATE INDEX idx_lc_feedback_comments_user_id ON lc_feedback_comments(user_id);

-- Indexes: notifications
CREATE INDEX idx_lc_notifications_user_id ON lc_notifications(user_id);
CREATE INDEX idx_lc_notifications_is_read ON lc_notifications(user_id, is_read) WHERE is_read = false;
CREATE INDEX idx_lc_notifications_reference ON lc_notifications(reference_type, reference_id);

-- Indexes: navigation items
CREATE INDEX idx_lc_navigation_items_parent_id ON lc_navigation_items(parent_id);
CREATE INDEX idx_lc_navigation_items_operation_id ON lc_navigation_items(operation_id);
CREATE INDEX idx_lc_navigation_items_sort_order ON lc_navigation_items(sort_order);
