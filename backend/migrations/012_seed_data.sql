-- Seed Data: Number Ranges, UOMs, Roles, Account Groups, Admin User

-- Number Ranges
INSERT INTO number_ranges (object_type, prefix, current_number, pad_length) VALUES
    ('MAT', 'MAT', 0, 8),
    ('PO', 'PO', 0, 8),
    ('SO', 'SO', 0, 8),
    ('DO', 'DO', 0, 8),
    ('INV', 'INV', 0, 8),
    ('PRD', 'PRD', 0, 8),
    ('EMP', 'EMP', 0, 8),
    ('BOM', 'BOM', 0, 8),
    ('TRF', 'TRF', 0, 8),
    ('CNT', 'CNT', 0, 8),
    ('QIL', 'QIL', 0, 8),
    ('QN', 'QN', 0, 8),
    ('IO', 'IO', 0, 8),
    ('JE', 'JE', 0, 8)
ON CONFLICT (object_type) DO NOTHING;

-- UOMs
INSERT INTO mm_uom (code, name, is_base) VALUES
    ('EA', 'Each', true),
    ('KG', 'Kilogram', true),
    ('L', 'Liter', true),
    ('M', 'Meter', true),
    ('BOX', 'Box', false),
    ('PAL', 'Pallet', false),
    ('SET', 'Set', false),
    ('HR', 'Hour', true)
ON CONFLICT (code) DO NOTHING;

-- Roles
INSERT INTO roles (name, description, is_system) VALUES
    ('ADMIN', 'System Administrator', true),
    ('FI_MANAGER', 'Financial Manager', true),
    ('MM_MANAGER', 'Materials Management Manager', true),
    ('SD_MANAGER', 'Sales & Distribution Manager', true),
    ('PP_MANAGER', 'Production Planning Manager', true),
    ('HR_MANAGER', 'Human Resources Manager', true),
    ('WM_MANAGER', 'Warehouse Management Manager', true),
    ('QM_MANAGER', 'Quality Management Manager', true),
    ('OPERATOR', 'Standard Operator', true)
ON CONFLICT (name) DO NOTHING;

-- Account Groups
INSERT INTO fi_account_groups (code, name, account_type) VALUES
    ('1000', 'Current Assets', 'ASSET'),
    ('1500', 'Fixed Assets', 'ASSET'),
    ('2000', 'Current Liabilities', 'LIABILITY'),
    ('2500', 'Long-term Liabilities', 'LIABILITY'),
    ('3000', 'Equity', 'EQUITY'),
    ('4000', 'Revenue', 'REVENUE'),
    ('5000', 'Cost of Goods Sold', 'EXPENSE'),
    ('6000', 'Operating Expenses', 'EXPENSE');

-- Default Company Code
INSERT INTO fi_company_codes (code, name, currency, country) VALUES
    ('TB01', 'TasteByte Corporation', 'TWD', 'TW')
ON CONFLICT (code) DO NOTHING;

-- Basic Chart of Accounts
INSERT INTO fi_accounts (account_number, name, account_type, is_reconciliation, is_active)
VALUES
    ('1100', 'Cash and Cash Equivalents', 'ASSET', false, true),
    ('1200', 'Accounts Receivable', 'ASSET', true, true),
    ('1300', 'Inventory', 'ASSET', false, true),
    ('1400', 'Prepaid Expenses', 'ASSET', false, true),
    ('1500', 'Fixed Assets', 'ASSET', false, true),
    ('1510', 'Accumulated Depreciation', 'ASSET', false, true),
    ('2100', 'Accounts Payable', 'LIABILITY', true, true),
    ('2200', 'Accrued Expenses', 'LIABILITY', false, true),
    ('2300', 'Tax Payable', 'LIABILITY', false, true),
    ('2500', 'Long-term Debt', 'LIABILITY', false, true),
    ('3100', 'Common Stock', 'EQUITY', false, true),
    ('3200', 'Retained Earnings', 'EQUITY', false, true),
    ('4100', 'Sales Revenue', 'REVENUE', false, true),
    ('4200', 'Service Revenue', 'REVENUE', false, true),
    ('5100', 'Cost of Goods Sold', 'EXPENSE', false, true),
    ('6100', 'Salaries Expense', 'EXPENSE', false, true),
    ('6200', 'Rent Expense', 'EXPENSE', false, true),
    ('6300', 'Utilities Expense', 'EXPENSE', false, true),
    ('6400', 'Depreciation Expense', 'EXPENSE', false, true),
    ('6500', 'General & Administrative', 'EXPENSE', false, true)
ON CONFLICT (account_number) DO NOTHING;

-- Admin user (password: admin123, hashed with argon2)
-- The actual hash will be generated at runtime during first startup
-- For seed data we use a placeholder that the migration runner will handle
INSERT INTO users (username, email, password_hash, display_name, is_active)
VALUES ('admin', 'admin@tastebyte.com', '$argon2id$v=19$m=19456,t=2,p=1$YWRtaW4xMjNzYWx0$JCe3F3pO7VPMqnLPSxcdYXaj7b1VFQhFOAaXlFmv77k', 'System Admin', true)
ON CONFLICT (username) DO NOTHING;

-- Assign admin role to admin user
INSERT INTO user_roles (user_id, role_id)
SELECT u.id, r.id FROM users u, roles r WHERE u.username = 'admin' AND r.name = 'ADMIN'
ON CONFLICT DO NOTHING;
