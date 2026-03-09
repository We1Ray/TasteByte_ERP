-- 030: Cross-Module Integration
-- Closes 10 GAPs in ERP cross-module data flows:
-- GAP 1: HR->FI Payroll
-- GAP 2: HR->CO Department-CostCenter link
-- GAP 3: SD->CO Customer Profit Center
-- GAP 4: FI->CO Unified auto-posting (code only)
-- GAP 5: PP->FI Production Settlement
-- GAP 6: WM->FI Stock Count Adjustment
-- GAP 7: QM->PP Production Output Inspection (code only)
-- GAP 8: SD->WM Outbound Delivery Traceability
-- GAP 9: AR/AP Payment Processing
-- GAP 10: SO INVOICED Status

-- ============================================================
-- 1. New FI Accounts
-- ============================================================
INSERT INTO fi_accounts (account_number, name, account_type, is_reconciliation, is_active)
VALUES
    ('1410', 'Finished Goods Inventory', 'ASSET', false, true),
    ('2210', 'Payroll Payable', 'LIABILITY', false, true),
    ('5200', 'Manufacturing Overhead', 'EXPENSE', false, true),
    ('6600', 'Inventory Adjustment', 'EXPENSE', false, true)
ON CONFLICT (account_number) DO NOTHING;

-- ============================================================
-- 2. New Number Ranges
-- ============================================================
INSERT INTO number_ranges (object_type, prefix, current_number, pad_length) VALUES
    ('PAY', 'PAY', 0, 8),
    ('SAL', 'SAL', 0, 8),
    ('PR', 'PR', 0, 8)
ON CONFLICT (object_type) DO NOTHING;

-- ============================================================
-- 3. HR Salary Structures (GAP 1)
-- ============================================================
CREATE TABLE IF NOT EXISTS hr_salary_structures (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    structure_number VARCHAR(30) NOT NULL UNIQUE,
    employee_id UUID NOT NULL REFERENCES hr_employees(id),
    base_salary DECIMAL(18,4) NOT NULL DEFAULT 0,
    allowances DECIMAL(18,4) NOT NULL DEFAULT 0,
    deductions DECIMAL(18,4) NOT NULL DEFAULT 0,
    currency VARCHAR(10) NOT NULL DEFAULT 'TWD',
    effective_from DATE NOT NULL,
    effective_to DATE,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_hr_salary_employee ON hr_salary_structures(employee_id);
CREATE INDEX IF NOT EXISTS idx_hr_salary_active ON hr_salary_structures(employee_id, is_active)
    WHERE is_active = true;

-- ============================================================
-- 4. HR Payroll Runs (GAP 1)
-- ============================================================
CREATE TABLE IF NOT EXISTS hr_payroll_runs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    run_number VARCHAR(30) NOT NULL UNIQUE,
    period_year INTEGER NOT NULL,
    period_month INTEGER NOT NULL CHECK (period_month BETWEEN 1 AND 12),
    status VARCHAR(20) NOT NULL DEFAULT 'DRAFT'
        CHECK (status IN ('DRAFT', 'PROCESSING', 'COMPLETED', 'CANCELLED')),
    total_gross DECIMAL(18,4) NOT NULL DEFAULT 0,
    total_deductions DECIMAL(18,4) NOT NULL DEFAULT 0,
    total_net DECIMAL(18,4) NOT NULL DEFAULT 0,
    employee_count INTEGER NOT NULL DEFAULT 0,
    journal_entry_id UUID REFERENCES fi_journal_entries(id),
    executed_by UUID REFERENCES users(id),
    executed_at TIMESTAMPTZ,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(period_year, period_month)
);

-- ============================================================
-- 5. HR Payroll Items (GAP 1)
-- ============================================================
CREATE TABLE IF NOT EXISTS hr_payroll_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    payroll_run_id UUID NOT NULL REFERENCES hr_payroll_runs(id) ON DELETE CASCADE,
    employee_id UUID NOT NULL REFERENCES hr_employees(id),
    department_id UUID REFERENCES hr_departments(id),
    cost_center_id UUID REFERENCES co_cost_centers(id),
    base_salary DECIMAL(18,4) NOT NULL DEFAULT 0,
    allowances DECIMAL(18,4) NOT NULL DEFAULT 0,
    deductions DECIMAL(18,4) NOT NULL DEFAULT 0,
    net_salary DECIMAL(18,4) NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_hr_payroll_items_run ON hr_payroll_items(payroll_run_id);
CREATE INDEX IF NOT EXISTS idx_hr_payroll_items_emp ON hr_payroll_items(employee_id);

-- ============================================================
-- 6. HR Department-CostCenter Link (GAP 2)
-- ============================================================
ALTER TABLE hr_departments
    ADD COLUMN IF NOT EXISTS cost_center_id UUID REFERENCES co_cost_centers(id);

CREATE INDEX IF NOT EXISTS idx_hr_departments_cc ON hr_departments(cost_center_id)
    WHERE cost_center_id IS NOT NULL;

-- ============================================================
-- 7. SD Customer Profit Center Link (GAP 3)
-- ============================================================
ALTER TABLE sd_customers
    ADD COLUMN IF NOT EXISTS profit_center_id UUID REFERENCES co_profit_centers(id);

CREATE INDEX IF NOT EXISTS idx_sd_customers_pc ON sd_customers(profit_center_id)
    WHERE profit_center_id IS NOT NULL;

-- ============================================================
-- 8. CO Cost Allocations - Profit Center support (GAP 3)
-- ============================================================
ALTER TABLE co_cost_allocations
    ADD COLUMN IF NOT EXISTS profit_center_id UUID REFERENCES co_profit_centers(id);

CREATE INDEX IF NOT EXISTS idx_co_cost_allocations_pc
    ON co_cost_allocations(profit_center_id)
    WHERE profit_center_id IS NOT NULL;

-- ============================================================
-- 9. SD Delivery Items - Warehouse traceability (GAP 8)
-- ============================================================
ALTER TABLE sd_delivery_items
    ADD COLUMN IF NOT EXISTS warehouse_id UUID REFERENCES wm_warehouses(id);

-- ============================================================
-- 10. FI Payment Documents (GAP 9)
-- ============================================================
CREATE TABLE IF NOT EXISTS fi_payment_documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_number VARCHAR(30) NOT NULL UNIQUE,
    payment_type VARCHAR(10) NOT NULL CHECK (payment_type IN ('AR', 'AP')),
    invoice_id UUID NOT NULL,
    amount DECIMAL(18,4) NOT NULL,
    payment_date DATE NOT NULL,
    journal_entry_id UUID REFERENCES fi_journal_entries(id),
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_fi_payments_invoice ON fi_payment_documents(invoice_id);
CREATE INDEX IF NOT EXISTS idx_fi_payments_type ON fi_payment_documents(payment_type);

-- ============================================================
-- 11. Update CHECK constraints
-- ============================================================

-- AR Invoice: add PARTIALLY_PAID (GAP 9)
ALTER TABLE fi_ar_invoices DROP CONSTRAINT IF EXISTS chk_ar_status;
ALTER TABLE fi_ar_invoices ADD CONSTRAINT chk_ar_status
    CHECK (status IN ('OPEN', 'PARTIALLY_PAID', 'PAID', 'CANCELLED'));

-- AP Invoice: add PARTIALLY_PAID (GAP 9)
ALTER TABLE fi_ap_invoices DROP CONSTRAINT IF EXISTS chk_ap_status;
ALTER TABLE fi_ap_invoices ADD CONSTRAINT chk_ap_status
    CHECK (status IN ('OPEN', 'PARTIALLY_PAID', 'PAID', 'CANCELLED'));

-- Sales Order: add INVOICED (GAP 10)
ALTER TABLE sd_sales_orders DROP CONSTRAINT IF EXISTS chk_so_status;
ALTER TABLE sd_sales_orders ADD CONSTRAINT chk_so_status
    CHECK (status IN ('DRAFT', 'CONFIRMED', 'PARTIALLY_DELIVERED', 'DELIVERED', 'INVOICED', 'CLOSED', 'CANCELLED'));

-- Seed HR cost center for payroll
INSERT INTO co_cost_centers (code, name, description, is_active)
VALUES ('CC-HR', 'Human Resources', 'Default cost center for HR payroll', true)
ON CONFLICT (code) DO NOTHING;
