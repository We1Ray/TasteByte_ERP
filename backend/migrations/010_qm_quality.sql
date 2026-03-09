-- QM Module: Quality Inspections, Notifications

CREATE TABLE IF NOT EXISTS qm_inspection_lots (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lot_number VARCHAR(30) NOT NULL UNIQUE,
    material_id UUID NOT NULL REFERENCES mm_materials(id),
    reference_type VARCHAR(50),
    reference_id UUID,
    inspection_type VARCHAR(20) NOT NULL DEFAULT 'INCOMING',
    planned_quantity DECIMAL(18,4) NOT NULL,
    inspected_quantity DECIMAL(18,4) NOT NULL DEFAULT 0,
    status VARCHAR(20) NOT NULL DEFAULT 'CREATED',
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS qm_inspection_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    inspection_lot_id UUID NOT NULL REFERENCES qm_inspection_lots(id) ON DELETE CASCADE,
    characteristic VARCHAR(200) NOT NULL,
    target_value VARCHAR(200),
    actual_value VARCHAR(200),
    is_conforming BOOLEAN,
    inspected_by UUID REFERENCES users(id),
    inspected_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS qm_quality_notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    notification_number VARCHAR(30) NOT NULL UNIQUE,
    notification_type VARCHAR(50) NOT NULL,
    material_id UUID REFERENCES mm_materials(id),
    description TEXT NOT NULL,
    priority VARCHAR(10) NOT NULL DEFAULT 'MEDIUM',
    status VARCHAR(20) NOT NULL DEFAULT 'OPEN',
    reported_by UUID REFERENCES users(id),
    assigned_to UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
