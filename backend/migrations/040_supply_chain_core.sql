-- 038_supply_chain_core.sql
-- Core supply chain improvements: GRN documents, Stock Reservations ledger,
-- Stock Movements unified ledger, and Fiscal Period status control.
-- NOTE: sd_deliveries and sd_delivery_items already exist (migration 006).
-- NOTE: fi_fiscal_periods already exists (migration 002) with is_closed boolean.

-- ============================================================
-- 1. Goods Receipt Notes (GRN) - standalone document for PO receiving
-- ============================================================
CREATE TABLE IF NOT EXISTS mm_goods_receipts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    grn_number VARCHAR(50) UNIQUE NOT NULL,
    purchase_order_id UUID REFERENCES mm_purchase_orders(id),
    vendor_id UUID REFERENCES mm_vendors(id),
    receipt_date DATE NOT NULL DEFAULT CURRENT_DATE,
    warehouse_id UUID REFERENCES wm_warehouses(id),
    status VARCHAR(20) NOT NULL DEFAULT 'DRAFT',
    notes TEXT,
    received_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS mm_goods_receipt_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    goods_receipt_id UUID NOT NULL REFERENCES mm_goods_receipts(id) ON DELETE CASCADE,
    po_item_id UUID REFERENCES mm_purchase_order_items(id),
    material_id UUID NOT NULL REFERENCES mm_materials(id),
    ordered_quantity NUMERIC(18,4),
    received_quantity NUMERIC(18,4) NOT NULL,
    rejected_quantity NUMERIC(18,4) DEFAULT 0,
    uom_id UUID REFERENCES mm_uom(id),
    batch_number VARCHAR(100),
    expiry_date DATE,
    storage_bin VARCHAR(100),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================
-- 2. Stock Reservations ledger (complements mm_plant_stock.reserved_quantity)
-- ============================================================
CREATE TABLE IF NOT EXISTS mm_stock_reservations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    material_id UUID NOT NULL REFERENCES mm_materials(id),
    warehouse_id UUID REFERENCES wm_warehouses(id),
    reserved_quantity NUMERIC(18,4) NOT NULL,
    reference_type VARCHAR(30) NOT NULL,
    reference_id UUID NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'ACTIVE',
    reserved_by UUID REFERENCES users(id),
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================
-- 3. Stock Movements unified ledger (supplements mm_material_movements)
-- ============================================================
CREATE TABLE IF NOT EXISTS mm_stock_movements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    material_id UUID NOT NULL REFERENCES mm_materials(id),
    warehouse_id UUID REFERENCES wm_warehouses(id),
    movement_type VARCHAR(30) NOT NULL,
    quantity NUMERIC(18,4) NOT NULL,
    reference_type VARCHAR(30),
    reference_id UUID,
    batch_number VARCHAR(100),
    notes TEXT,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================
-- 4. Fiscal Period status control (add status column to existing table)
-- ============================================================
ALTER TABLE fi_fiscal_periods
    ADD COLUMN IF NOT EXISTS status VARCHAR(20) DEFAULT 'OPEN';

-- Backfill status from is_closed boolean
UPDATE fi_fiscal_periods
SET status = CASE WHEN is_closed THEN 'CLOSED' ELSE 'OPEN' END
WHERE status IS NULL OR status = 'OPEN';

-- ============================================================
-- 5. Enhance sd_deliveries with additional shipping fields
-- ============================================================
ALTER TABLE sd_deliveries
    ADD COLUMN IF NOT EXISTS shipping_address TEXT,
    ADD COLUMN IF NOT EXISTS carrier VARCHAR(200),
    ADD COLUMN IF NOT EXISTS tracking_number VARCHAR(200),
    ADD COLUMN IF NOT EXISTS notes TEXT;

-- ============================================================
-- 6. Number ranges for new document types
-- ============================================================
INSERT INTO number_ranges (object_type, prefix, current_number, pad_length) VALUES
    ('GRN', 'GRN', 0, 8)
ON CONFLICT (object_type) DO NOTHING;

-- ============================================================
-- 7. Seed fiscal periods for 2024-2026 (into existing fi_fiscal_periods table)
-- ============================================================
DO $$
DECLARE
    v_company_code_id UUID;
    v_fiscal_year_id UUID;
    v_year INT;
    v_month INT;
    v_start DATE;
    v_end DATE;
BEGIN
    SELECT id INTO v_company_code_id FROM fi_company_codes LIMIT 1;
    IF v_company_code_id IS NULL THEN
        RETURN;
    END IF;

    FOR v_year IN 2024..2026 LOOP
        -- Ensure fiscal year exists
        INSERT INTO fi_fiscal_years (company_code_id, year, start_date, end_date, is_closed)
        VALUES (v_company_code_id, v_year,
                (v_year || '-01-01')::date,
                (v_year || '-12-31')::date,
                CASE WHEN v_year < 2026 THEN true ELSE false END)
        ON CONFLICT DO NOTHING;

        SELECT id INTO v_fiscal_year_id
        FROM fi_fiscal_years
        WHERE company_code_id = v_company_code_id AND year = v_year;

        IF v_fiscal_year_id IS NOT NULL THEN
            FOR v_month IN 1..12 LOOP
                v_start := (v_year || '-' || LPAD(v_month::text, 2, '0') || '-01')::date;
                v_end := (v_start + INTERVAL '1 month' - INTERVAL '1 day')::date;

                INSERT INTO fi_fiscal_periods (fiscal_year_id, period, start_date, end_date, is_closed, status)
                VALUES (v_fiscal_year_id, v_month, v_start, v_end,
                        CASE WHEN v_start < CURRENT_DATE THEN true ELSE false END,
                        CASE WHEN v_start < CURRENT_DATE THEN 'CLOSED' ELSE 'OPEN' END)
                ON CONFLICT DO NOTHING;
            END LOOP;
        END IF;
    END LOOP;
END $$;

-- ============================================================
-- 8. Indexes
-- ============================================================
CREATE INDEX IF NOT EXISTS idx_grn_po ON mm_goods_receipts(purchase_order_id);
CREATE INDEX IF NOT EXISTS idx_grn_vendor ON mm_goods_receipts(vendor_id);
CREATE INDEX IF NOT EXISTS idx_grn_status ON mm_goods_receipts(status);
CREATE INDEX IF NOT EXISTS idx_grn_items_grn ON mm_goods_receipt_items(goods_receipt_id);
CREATE INDEX IF NOT EXISTS idx_grn_items_material ON mm_goods_receipt_items(material_id);
CREATE INDEX IF NOT EXISTS idx_stock_reservations_mat ON mm_stock_reservations(material_id, status);
CREATE INDEX IF NOT EXISTS idx_stock_reservations_ref ON mm_stock_reservations(reference_type, reference_id);
CREATE INDEX IF NOT EXISTS idx_stock_movements_mat ON mm_stock_movements(material_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_stock_movements_ref ON mm_stock_movements(reference_type, reference_id);
