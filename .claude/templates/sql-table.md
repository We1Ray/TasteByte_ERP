# SQL Table Template (PostgreSQL)

> Use this template when creating a new database table for TasteByte ERP.
> Database: PostgreSQL 17, port 5432, database: TastyByte

## File Naming Convention

```
backend/migrations/NNN_{module}_{description}.sql
```

Where `NNN` is a 3-digit sequence number.

## ERP Master Data Table Template

```sql
-- =====================================================
-- Table: {module}_{table_name}
-- Module: {FI|CO|MM|SD|PP|HR|WM|QM}
-- Description: {Brief description}
-- Dependencies: {parent_tables}
-- =====================================================

-- =========================
-- 1. Table Definition
-- =========================
CREATE TABLE IF NOT EXISTS {module}_{table_name} (
    -- Primary Key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Master Data Fields
    {resource}_number VARCHAR(20) NOT NULL UNIQUE,  -- 編號 (e.g., MAT-00001)
    description TEXT NOT NULL,

    -- Classification
    {resource}_type VARCHAR(20) NOT NULL,
    {resource}_group VARCHAR(20),

    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,

    -- Audit Fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id)
);

-- =========================
-- 2. Indexes
-- =========================
CREATE INDEX idx_{module}_{table_name}_{resource}_number
    ON {module}_{table_name}({resource}_number);
CREATE INDEX idx_{module}_{table_name}_{resource}_type
    ON {module}_{table_name}({resource}_type);
CREATE INDEX idx_{module}_{table_name}_is_active
    ON {module}_{table_name}(is_active) WHERE is_active = true;

-- =========================
-- 3. Updated_at Trigger
-- =========================
CREATE TRIGGER update_{module}_{table_name}_updated_at
    BEFORE UPDATE ON {module}_{table_name}
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- =========================
-- 4. Comments
-- =========================
COMMENT ON TABLE {module}_{table_name} IS '{Module}: {Description}';
```

## ERP Transaction Data Table Template

```sql
-- =====================================================
-- Table: {module}_{table_name}
-- Module: {FI|CO|MM|SD|PP|HR|WM|QM}
-- Type: Transaction Data
-- Description: {Brief description}
-- =====================================================

CREATE TABLE IF NOT EXISTS {module}_{table_name} (
    -- Primary Key
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Document Control
    document_number VARCHAR(20) NOT NULL UNIQUE,
    status VARCHAR(20) NOT NULL DEFAULT 'DRAFT'
        CHECK (status IN ('DRAFT', 'CONFIRMED', 'IN_PROGRESS', 'COMPLETED', 'CANCELLED', 'CLOSED')),
    fiscal_year INTEGER NOT NULL,
    posting_date DATE NOT NULL,

    -- Foreign Keys
    {related}_id UUID NOT NULL REFERENCES {module}_{related_table}(id),

    -- Monetary Fields (always use DECIMAL, never FLOAT)
    total_amount DECIMAL(15, 2) NOT NULL DEFAULT 0,
    currency VARCHAR(3) NOT NULL DEFAULT 'TWD',

    -- Additional Fields
    notes TEXT,

    -- Audit Fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id)
);

-- Document Line Items
CREATE TABLE IF NOT EXISTS {module}_{table_name}_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    {parent}_id UUID NOT NULL REFERENCES {module}_{table_name}(id) ON DELETE CASCADE,
    item_number INTEGER NOT NULL,
    material_id UUID NOT NULL REFERENCES mm_materials(id),
    quantity DECIMAL(15, 3) NOT NULL,
    unit_price DECIMAL(15, 2) NOT NULL,
    amount DECIMAL(15, 2) NOT NULL,
    UNIQUE({parent}_id, item_number)
);

-- =========================
-- Indexes
-- =========================
CREATE INDEX idx_{module}_{table_name}_document_number
    ON {module}_{table_name}(document_number);
CREATE INDEX idx_{module}_{table_name}_status
    ON {module}_{table_name}(status);
CREATE INDEX idx_{module}_{table_name}_posting_date
    ON {module}_{table_name}(posting_date);
CREATE INDEX idx_{module}_{table_name}_{related}_id
    ON {module}_{table_name}({related}_id);
CREATE INDEX idx_{module}_{table_name}_items_{parent}_id
    ON {module}_{table_name}_items({parent}_id);

-- =========================
-- Triggers
-- =========================
CREATE TRIGGER update_{module}_{table_name}_updated_at
    BEFORE UPDATE ON {module}_{table_name}
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

## Common Trigger Function (in 001_init.sql)

```sql
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
```

## Checklist

- [ ] Table name uses `{module}_{resource}` format (snake_case)
- [ ] Has UUID primary key with gen_random_uuid()
- [ ] Transaction tables have document_number, status, fiscal_year, posting_date
- [ ] Monetary fields use DECIMAL(15, 2), never FLOAT
- [ ] Foreign keys have ON DELETE action specified
- [ ] All FK columns are indexed
- [ ] Has created_at, updated_at, created_by, updated_by
- [ ] updated_at trigger is applied
- [ ] Status fields have CHECK constraints
- [ ] Migration file numbered correctly (NNN_module_description.sql)
