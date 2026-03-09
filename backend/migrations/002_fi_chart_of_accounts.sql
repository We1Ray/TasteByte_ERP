-- FI Module: Chart of Accounts

CREATE TABLE IF NOT EXISTS fi_company_codes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(10) NOT NULL UNIQUE,
    name VARCHAR(200) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'TWD',
    country VARCHAR(2) NOT NULL DEFAULT 'TW'
);

CREATE TABLE IF NOT EXISTS fi_fiscal_years (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_code_id UUID NOT NULL REFERENCES fi_company_codes(id),
    year INT NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    is_closed BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS fi_fiscal_periods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    fiscal_year_id UUID NOT NULL REFERENCES fi_fiscal_years(id),
    period INT NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    is_closed BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS fi_account_groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(20) NOT NULL,
    name VARCHAR(200) NOT NULL,
    account_type VARCHAR(20) NOT NULL CHECK (account_type IN ('ASSET', 'LIABILITY', 'EQUITY', 'REVENUE', 'EXPENSE'))
);

CREATE TABLE IF NOT EXISTS fi_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_number VARCHAR(20) NOT NULL UNIQUE,
    name VARCHAR(200) NOT NULL,
    account_group_id UUID REFERENCES fi_account_groups(id),
    account_type VARCHAR(20) NOT NULL CHECK (account_type IN ('ASSET', 'LIABILITY', 'EQUITY', 'REVENUE', 'EXPENSE')),
    is_reconciliation BOOLEAN NOT NULL DEFAULT false,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
