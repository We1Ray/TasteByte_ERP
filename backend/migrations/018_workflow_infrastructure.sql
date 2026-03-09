-- 018: Workflow infrastructure - status history, CHECK constraints, audit indexes, updated_at

-- ============================================================
-- 1. Document Status History Table
-- ============================================================

CREATE TABLE IF NOT EXISTS document_status_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_type VARCHAR(50) NOT NULL,
    document_id UUID NOT NULL,
    from_status VARCHAR(30),
    to_status VARCHAR(30) NOT NULL,
    changed_by UUID NOT NULL REFERENCES users(id),
    change_reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_status_history_doc ON document_status_history(document_type, document_id);
CREATE INDEX IF NOT EXISTS idx_status_history_time ON document_status_history(created_at);

-- ============================================================
-- 2. CHECK Constraints for Status Columns
-- ============================================================

-- Sales Orders
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM sd_sales_orders WHERE status NOT IN ('DRAFT','CONFIRMED','PARTIALLY_DELIVERED','DELIVERED','CLOSED','CANCELLED')) THEN
    ALTER TABLE sd_sales_orders ADD CONSTRAINT chk_so_status CHECK (status IN ('DRAFT','CONFIRMED','PARTIALLY_DELIVERED','DELIVERED','CLOSED','CANCELLED'));
  ELSE
    RAISE NOTICE 'Skipping chk_so_status: existing data violates constraint';
  END IF;
EXCEPTION WHEN duplicate_object THEN
  NULL; -- constraint already exists
END $$;

-- Purchase Orders
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM mm_purchase_orders WHERE status NOT IN ('DRAFT','RELEASED','PARTIALLY_RECEIVED','RECEIVED','CLOSED','CANCELLED')) THEN
    ALTER TABLE mm_purchase_orders ADD CONSTRAINT chk_po_status CHECK (status IN ('DRAFT','RELEASED','PARTIALLY_RECEIVED','RECEIVED','CLOSED','CANCELLED'));
  ELSE
    RAISE NOTICE 'Skipping chk_po_status: existing data violates constraint';
  END IF;
EXCEPTION WHEN duplicate_object THEN
  NULL; -- constraint already exists
END $$;

-- Production Orders
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pp_production_orders WHERE status NOT IN ('CREATED','RELEASED','IN_PROGRESS','COMPLETED','CLOSED','CANCELLED')) THEN
    ALTER TABLE pp_production_orders ADD CONSTRAINT chk_prod_status CHECK (status IN ('CREATED','RELEASED','IN_PROGRESS','COMPLETED','CLOSED','CANCELLED'));
  ELSE
    RAISE NOTICE 'Skipping chk_prod_status: existing data violates constraint';
  END IF;
EXCEPTION WHEN duplicate_object THEN
  NULL; -- constraint already exists
END $$;

-- Journal Entries
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM fi_journal_entries WHERE status NOT IN ('DRAFT','POSTED')) THEN
    ALTER TABLE fi_journal_entries ADD CONSTRAINT chk_je_status CHECK (status IN ('DRAFT','POSTED'));
  ELSE
    RAISE NOTICE 'Skipping chk_je_status: existing data violates constraint';
  END IF;
EXCEPTION WHEN duplicate_object THEN
  NULL; -- constraint already exists
END $$;

-- Deliveries
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM sd_deliveries WHERE status NOT IN ('CREATED','SHIPPED','DELIVERED','CANCELLED')) THEN
    ALTER TABLE sd_deliveries ADD CONSTRAINT chk_del_status CHECK (status IN ('CREATED','SHIPPED','DELIVERED','CANCELLED'));
  ELSE
    RAISE NOTICE 'Skipping chk_del_status: existing data violates constraint';
  END IF;
EXCEPTION WHEN duplicate_object THEN
  NULL; -- constraint already exists
END $$;

-- Invoices
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM sd_invoices WHERE status NOT IN ('CREATED','POSTED','PAID','CANCELLED')) THEN
    ALTER TABLE sd_invoices ADD CONSTRAINT chk_inv_status CHECK (status IN ('CREATED','POSTED','PAID','CANCELLED'));
  ELSE
    RAISE NOTICE 'Skipping chk_inv_status: existing data violates constraint';
  END IF;
EXCEPTION WHEN duplicate_object THEN
  NULL; -- constraint already exists
END $$;

-- AR Invoices
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM fi_ar_invoices WHERE status NOT IN ('OPEN','PAID','CANCELLED')) THEN
    ALTER TABLE fi_ar_invoices ADD CONSTRAINT chk_ar_status CHECK (status IN ('OPEN','PAID','CANCELLED'));
  ELSE
    RAISE NOTICE 'Skipping chk_ar_status: existing data violates constraint';
  END IF;
EXCEPTION WHEN duplicate_object THEN
  NULL; -- constraint already exists
END $$;

-- AP Invoices
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM fi_ap_invoices WHERE status NOT IN ('OPEN','PAID','CANCELLED')) THEN
    ALTER TABLE fi_ap_invoices ADD CONSTRAINT chk_ap_status CHECK (status IN ('OPEN','PAID','CANCELLED'));
  ELSE
    RAISE NOTICE 'Skipping chk_ap_status: existing data violates constraint';
  END IF;
EXCEPTION WHEN duplicate_object THEN
  NULL; -- constraint already exists
END $$;

-- ============================================================
-- 3. Audit Log Indexes
-- ============================================================

CREATE INDEX IF NOT EXISTS idx_audit_log_entity ON audit_log(table_name, record_id);
CREATE INDEX IF NOT EXISTS idx_audit_log_user ON audit_log(changed_by);
CREATE INDEX IF NOT EXISTS idx_audit_log_time ON audit_log(changed_at);

-- ============================================================
-- 4. Add updated_at Columns to Tables Missing Them
-- ============================================================

ALTER TABLE sd_deliveries ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();
ALTER TABLE sd_invoices ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();
ALTER TABLE fi_ar_invoices ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();
ALTER TABLE fi_ap_invoices ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();
ALTER TABLE wm_stock_transfers ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();
ALTER TABLE wm_stock_counts ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();
