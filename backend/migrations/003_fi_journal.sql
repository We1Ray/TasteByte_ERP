-- FI Module: Journal Entries, AR/AP Invoices

CREATE TABLE IF NOT EXISTS fi_journal_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_number VARCHAR(30) NOT NULL UNIQUE,
    company_code_id UUID NOT NULL REFERENCES fi_company_codes(id),
    fiscal_year INT NOT NULL,
    fiscal_period INT NOT NULL,
    posting_date DATE NOT NULL,
    document_date DATE NOT NULL,
    reference VARCHAR(100),
    description TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'DRAFT',
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS fi_journal_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    journal_entry_id UUID NOT NULL REFERENCES fi_journal_entries(id) ON DELETE CASCADE,
    line_number INT NOT NULL,
    account_id UUID NOT NULL REFERENCES fi_accounts(id),
    debit_amount DECIMAL(18,4) NOT NULL DEFAULT 0,
    credit_amount DECIMAL(18,4) NOT NULL DEFAULT 0,
    cost_center_id UUID,
    description TEXT
);

CREATE TABLE IF NOT EXISTS fi_ar_invoices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_number VARCHAR(30) NOT NULL UNIQUE,
    customer_id UUID,
    invoice_date DATE NOT NULL,
    due_date DATE NOT NULL,
    total_amount DECIMAL(18,4) NOT NULL,
    paid_amount DECIMAL(18,4) NOT NULL DEFAULT 0,
    status VARCHAR(20) NOT NULL DEFAULT 'OPEN',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS fi_ap_invoices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_number VARCHAR(30) NOT NULL UNIQUE,
    vendor_id UUID,
    invoice_date DATE NOT NULL,
    due_date DATE NOT NULL,
    total_amount DECIMAL(18,4) NOT NULL,
    paid_amount DECIMAL(18,4) NOT NULL DEFAULT 0,
    status VARCHAR(20) NOT NULL DEFAULT 'OPEN',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
