-- 035_infrastructure.sql
-- Email templates, scheduled jobs, webhooks, user preferences, saved variants

-- Email Templates
CREATE TABLE IF NOT EXISTS email_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    template_code VARCHAR(100) UNIQUE NOT NULL,
    subject TEXT NOT NULL,
    body_html TEXT NOT NULL,
    body_text TEXT,
    variables JSONB DEFAULT '[]'::jsonb,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Email Log
CREATE TABLE IF NOT EXISTS email_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    template_code VARCHAR(100),
    recipient VARCHAR(255) NOT NULL,
    subject TEXT NOT NULL,
    status VARCHAR(20) DEFAULT 'PENDING',
    error_message TEXT,
    sent_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Scheduled Jobs
CREATE TABLE IF NOT EXISTS scheduled_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_name VARCHAR(200) UNIQUE NOT NULL,
    job_type VARCHAR(50) NOT NULL,
    cron_expression VARCHAR(100) NOT NULL,
    handler VARCHAR(200) NOT NULL,
    config JSONB DEFAULT '{}'::jsonb,
    is_active BOOLEAN DEFAULT true,
    last_run_at TIMESTAMPTZ,
    last_status VARCHAR(20),
    last_error TEXT,
    next_run_at TIMESTAMPTZ,
    created_by UUID,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Job Execution Log
CREATE TABLE IF NOT EXISTS job_execution_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_id UUID REFERENCES scheduled_jobs(id) ON DELETE CASCADE,
    status VARCHAR(20) NOT NULL,
    started_at TIMESTAMPTZ NOT NULL,
    finished_at TIMESTAMPTZ,
    duration_ms BIGINT,
    result JSONB,
    error_message TEXT
);

-- Webhooks
CREATE TABLE IF NOT EXISTS webhooks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    url TEXT NOT NULL,
    secret VARCHAR(255),
    events TEXT[] NOT NULL DEFAULT '{}',
    headers JSONB DEFAULT '{}'::jsonb,
    is_active BOOLEAN DEFAULT true,
    retry_count INT DEFAULT 3,
    created_by UUID,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Webhook Delivery Log
CREATE TABLE IF NOT EXISTS webhook_delivery_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    webhook_id UUID REFERENCES webhooks(id) ON DELETE CASCADE,
    event_type VARCHAR(100) NOT NULL,
    payload JSONB NOT NULL,
    response_status INT,
    response_body TEXT,
    attempt INT DEFAULT 1,
    delivered_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- User Preferences
CREATE TABLE IF NOT EXISTS user_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE,
    language VARCHAR(10) DEFAULT 'zh-TW',
    timezone VARCHAR(50) DEFAULT 'Asia/Taipei',
    date_format VARCHAR(20) DEFAULT 'YYYY-MM-DD',
    theme VARCHAR(20) DEFAULT 'light',
    notifications_enabled BOOLEAN DEFAULT true,
    email_notifications BOOLEAN DEFAULT true,
    page_size INT DEFAULT 20,
    sidebar_collapsed BOOLEAN DEFAULT false,
    custom_settings JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Saved Variants (saved search/filter configurations)
CREATE TABLE IF NOT EXISTS saved_variants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    context VARCHAR(200) NOT NULL,
    variant_name VARCHAR(200) NOT NULL,
    is_default BOOLEAN DEFAULT false,
    filters JSONB NOT NULL DEFAULT '{}'::jsonb,
    columns JSONB,
    sort_config JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, context, variant_name)
);

-- Transport Orders (dev->test->prod promotion)
CREATE TABLE IF NOT EXISTS transport_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transport_number VARCHAR(50) UNIQUE NOT NULL,
    description TEXT,
    source_env VARCHAR(20) DEFAULT 'DEV',
    target_env VARCHAR(20) NOT NULL,
    status VARCHAR(20) DEFAULT 'CREATED',
    payload JSONB NOT NULL,
    object_type VARCHAR(50) NOT NULL,
    object_id UUID,
    created_by UUID,
    approved_by UUID,
    applied_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Data Archive Config
CREATE TABLE IF NOT EXISTS archive_policies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    table_name VARCHAR(200) NOT NULL,
    retention_days INT NOT NULL DEFAULT 365,
    archive_strategy VARCHAR(50) DEFAULT 'DELETE',
    filter_condition TEXT,
    is_active BOOLEAN DEFAULT true,
    last_run_at TIMESTAMPTZ,
    records_archived BIGINT DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Print Layouts
CREATE TABLE IF NOT EXISTS print_layouts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    layout_code VARCHAR(100) UNIQUE NOT NULL,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    operation_id UUID,
    template_html TEXT NOT NULL,
    paper_size VARCHAR(20) DEFAULT 'A4',
    orientation VARCHAR(20) DEFAULT 'portrait',
    margins JSONB DEFAULT '{"top": 20, "right": 15, "bottom": 20, "left": 15}'::jsonb,
    header_html TEXT,
    footer_html TEXT,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_email_log_status ON email_log(status);
CREATE INDEX IF NOT EXISTS idx_job_exec_job_id ON job_execution_log(job_id);
CREATE INDEX IF NOT EXISTS idx_webhook_delivery_webhook ON webhook_delivery_log(webhook_id);
CREATE INDEX IF NOT EXISTS idx_saved_variants_user ON saved_variants(user_id, context);
CREATE INDEX IF NOT EXISTS idx_transport_orders_status ON transport_orders(status);

-- Seed email templates
INSERT INTO email_templates (template_code, subject, body_html, variables) VALUES
('RELEASE_SUBMITTED', 'Release {{release_number}} submitted for review', '<h2>Release Submitted</h2><p>Release <b>{{release_number}}</b> - {{title}} has been submitted for review.</p><p>Version: {{version}}</p>', '["release_number", "title", "version"]'),
('RELEASE_APPROVED', 'Release {{release_number}} approved', '<h2>Release Approved</h2><p>Your release <b>{{release_number}}</b> - {{title}} has been approved.</p>{{#if notes}}<p>Notes: {{notes}}</p>{{/if}}', '["release_number", "title", "notes"]'),
('RELEASE_REJECTED', 'Release {{release_number}} rejected', '<h2>Release Rejected</h2><p>Your release <b>{{release_number}}</b> - {{title}} has been rejected.</p><p>Reason: {{reason}}</p>', '["release_number", "title", "reason"]'),
('PASSWORD_RESET', 'Password Reset Request', '<h2>Password Reset</h2><p>Click <a href="{{reset_link}}">here</a> to reset your password.</p><p>This link expires in {{expiry_hours}} hours.</p>', '["reset_link", "expiry_hours"]'),
('WELCOME', 'Welcome to TasteByte ERP', '<h2>Welcome {{name}}!</h2><p>Your account has been created. Login at <a href="{{login_url}}">{{login_url}}</a></p>', '["name", "login_url"]')
ON CONFLICT (template_code) DO NOTHING;
