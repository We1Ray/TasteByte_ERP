-- 037_q3q4_features.sql
-- Q3: Reports, Analytics, Dashboard Templates, Multi-currency

CREATE TABLE IF NOT EXISTS report_definitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    report_code VARCHAR(100) UNIQUE NOT NULL,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    operation_id UUID REFERENCES lc_operations(id) ON DELETE SET NULL,
    data_source_sql TEXT NOT NULL,
    columns JSONB NOT NULL DEFAULT '[]',
    filters JSONB NOT NULL DEFAULT '[]',
    grouping JSONB DEFAULT '[]',
    chart_config JSONB,
    default_sort VARCHAR(200),
    default_sort_dir VARCHAR(4) DEFAULT 'ASC',
    page_size INT DEFAULT 50,
    is_public BOOLEAN DEFAULT false,
    created_by UUID,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS usage_analytics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID,
    operation_id UUID,
    event_type VARCHAR(50) NOT NULL,
    event_data JSONB DEFAULT '{}',
    page_url VARCHAR(500),
    duration_ms INT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS dashboard_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    template_code VARCHAR(100) UNIQUE NOT NULL,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    category VARCHAR(50) DEFAULT 'GENERAL',
    definition JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

INSERT INTO dashboard_templates (template_code, name, description, category, definition) VALUES
('REVENUE', 'Revenue Overview', 'Revenue KPIs and trends', 'FINANCE', '{"widgets":[]}'),
('INVENTORY', 'Inventory Status', 'Stock levels and alerts', 'WAREHOUSE', '{"widgets":[]}'),
('HR_OVERVIEW', 'HR Dashboard', 'Employee and attendance stats', 'HR', '{"widgets":[]}'),
('PRODUCTION', 'Production KPIs', 'Production order tracking', 'PRODUCTION', '{"widgets":[]}')
ON CONFLICT (template_code) DO NOTHING;

CREATE TABLE IF NOT EXISTS exchange_rates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_currency VARCHAR(3) NOT NULL,
    to_currency VARCHAR(3) NOT NULL,
    rate NUMERIC(18,6) NOT NULL,
    valid_from DATE NOT NULL,
    valid_until DATE,
    source VARCHAR(50) DEFAULT 'MANUAL',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(from_currency, to_currency, valid_from)
);

INSERT INTO exchange_rates (from_currency, to_currency, rate, valid_from) VALUES
('USD', 'TWD', 31.5, '2024-01-01'),
('EUR', 'TWD', 34.2, '2024-01-01'),
('JPY', 'TWD', 0.21, '2024-01-01'),
('GBP', 'TWD', 39.8, '2024-01-01'),
('CNY', 'TWD', 4.35, '2024-01-01')
ON CONFLICT DO NOTHING;

CREATE INDEX IF NOT EXISTS idx_usage_analytics_event ON usage_analytics(event_type, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_usage_analytics_user ON usage_analytics(user_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_exchange_rates_pair ON exchange_rates(from_currency, to_currency, valid_from DESC);
