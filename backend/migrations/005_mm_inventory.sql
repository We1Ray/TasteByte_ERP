-- MM Module: Inventory, Purchase Orders

CREATE TABLE IF NOT EXISTS mm_plant_stock (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    material_id UUID NOT NULL REFERENCES mm_materials(id),
    warehouse_id UUID,
    quantity DECIMAL(18,4) NOT NULL DEFAULT 0,
    reserved_quantity DECIMAL(18,4) NOT NULL DEFAULT 0,
    uom_id UUID REFERENCES mm_uom(id),
    last_count_date DATE,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(material_id, warehouse_id)
);

CREATE TABLE IF NOT EXISTS mm_material_movements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_number VARCHAR(30) NOT NULL UNIQUE,
    movement_type VARCHAR(20) NOT NULL CHECK (movement_type IN ('GOODS_RECEIPT', 'GOODS_ISSUE', 'TRANSFER', 'ADJUSTMENT')),
    material_id UUID NOT NULL REFERENCES mm_materials(id),
    warehouse_id UUID,
    quantity DECIMAL(18,4) NOT NULL,
    uom_id UUID REFERENCES mm_uom(id),
    reference_type VARCHAR(50),
    reference_id UUID,
    posted_by UUID REFERENCES users(id),
    posted_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS mm_purchase_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    po_number VARCHAR(30) NOT NULL UNIQUE,
    vendor_id UUID NOT NULL REFERENCES mm_vendors(id),
    order_date DATE NOT NULL,
    delivery_date DATE,
    status VARCHAR(20) NOT NULL DEFAULT 'DRAFT',
    total_amount DECIMAL(18,4) NOT NULL DEFAULT 0,
    currency VARCHAR(3) NOT NULL DEFAULT 'TWD',
    notes TEXT,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS mm_purchase_order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    purchase_order_id UUID NOT NULL REFERENCES mm_purchase_orders(id) ON DELETE CASCADE,
    line_number INT NOT NULL,
    material_id UUID NOT NULL REFERENCES mm_materials(id),
    quantity DECIMAL(18,4) NOT NULL,
    unit_price DECIMAL(18,4) NOT NULL,
    total_price DECIMAL(18,4) NOT NULL,
    uom_id UUID REFERENCES mm_uom(id),
    delivery_date DATE,
    received_quantity DECIMAL(18,4) NOT NULL DEFAULT 0
);
