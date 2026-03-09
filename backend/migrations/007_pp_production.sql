-- PP Module: Production, BOMs, Routings

CREATE TABLE IF NOT EXISTS pp_boms (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bom_number VARCHAR(30) NOT NULL UNIQUE,
    material_id UUID NOT NULL REFERENCES mm_materials(id),
    name VARCHAR(200) NOT NULL,
    version INT NOT NULL DEFAULT 1,
    status VARCHAR(20) NOT NULL DEFAULT 'ACTIVE',
    valid_from DATE,
    valid_to DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS pp_bom_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    bom_id UUID NOT NULL REFERENCES pp_boms(id) ON DELETE CASCADE,
    line_number INT NOT NULL,
    component_material_id UUID NOT NULL REFERENCES mm_materials(id),
    quantity DECIMAL(18,4) NOT NULL,
    uom_id UUID REFERENCES mm_uom(id),
    scrap_percentage DECIMAL(5,2) NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS pp_routings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    routing_number VARCHAR(30) NOT NULL UNIQUE,
    material_id UUID NOT NULL REFERENCES mm_materials(id),
    name VARCHAR(200) NOT NULL,
    version INT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS pp_routing_operations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    routing_id UUID NOT NULL REFERENCES pp_routings(id) ON DELETE CASCADE,
    operation_number INT NOT NULL,
    work_center VARCHAR(100) NOT NULL,
    description TEXT,
    setup_time_minutes INT NOT NULL DEFAULT 0,
    run_time_minutes INT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS pp_production_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_number VARCHAR(30) NOT NULL UNIQUE,
    material_id UUID NOT NULL REFERENCES mm_materials(id),
    bom_id UUID NOT NULL REFERENCES pp_boms(id),
    routing_id UUID REFERENCES pp_routings(id),
    planned_quantity DECIMAL(18,4) NOT NULL,
    actual_quantity DECIMAL(18,4) NOT NULL DEFAULT 0,
    uom_id UUID REFERENCES mm_uom(id),
    planned_start DATE,
    planned_end DATE,
    actual_start DATE,
    actual_end DATE,
    status VARCHAR(20) NOT NULL DEFAULT 'CREATED',
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
