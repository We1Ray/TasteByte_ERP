-- WM Module: Warehouses, Storage Bins, Transfers, Stock Counts

CREATE TABLE IF NOT EXISTS wm_warehouses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(20) NOT NULL UNIQUE,
    name VARCHAR(200) NOT NULL,
    address TEXT,
    warehouse_type VARCHAR(20) NOT NULL DEFAULT 'STANDARD',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS wm_storage_bins (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    warehouse_id UUID NOT NULL REFERENCES wm_warehouses(id),
    bin_code VARCHAR(30) NOT NULL,
    zone VARCHAR(20),
    aisle VARCHAR(20),
    rack VARCHAR(20),
    level VARCHAR(20),
    max_weight DECIMAL(12,4),
    is_active BOOLEAN NOT NULL DEFAULT true,
    UNIQUE(warehouse_id, bin_code)
);

CREATE TABLE IF NOT EXISTS wm_stock_transfers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transfer_number VARCHAR(30) NOT NULL UNIQUE,
    from_warehouse_id UUID NOT NULL REFERENCES wm_warehouses(id),
    to_warehouse_id UUID NOT NULL REFERENCES wm_warehouses(id),
    material_id UUID NOT NULL REFERENCES mm_materials(id),
    quantity DECIMAL(18,4) NOT NULL,
    uom_id UUID REFERENCES mm_uom(id),
    status VARCHAR(20) NOT NULL DEFAULT 'CREATED',
    requested_by UUID REFERENCES users(id),
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS wm_stock_counts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    count_number VARCHAR(30) NOT NULL UNIQUE,
    warehouse_id UUID NOT NULL REFERENCES wm_warehouses(id),
    count_date DATE NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'PLANNED',
    counted_by UUID REFERENCES users(id),
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS wm_stock_count_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    stock_count_id UUID NOT NULL REFERENCES wm_stock_counts(id) ON DELETE CASCADE,
    material_id UUID NOT NULL REFERENCES mm_materials(id),
    storage_bin_id UUID REFERENCES wm_storage_bins(id),
    book_quantity DECIMAL(18,4) NOT NULL,
    counted_quantity DECIMAL(18,4),
    difference DECIMAL(18,4)
);
