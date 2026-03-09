-- MM Module: Materials, Vendors, UOMs

CREATE TABLE IF NOT EXISTS mm_uom (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(10) NOT NULL UNIQUE,
    name VARCHAR(50) NOT NULL,
    is_base BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS mm_material_groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(20) NOT NULL UNIQUE,
    name VARCHAR(200) NOT NULL,
    description TEXT
);

CREATE TABLE IF NOT EXISTS mm_materials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    material_number VARCHAR(30) NOT NULL UNIQUE,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    material_group_id UUID REFERENCES mm_material_groups(id),
    base_uom_id UUID REFERENCES mm_uom(id),
    material_type VARCHAR(20) NOT NULL DEFAULT 'RAW',
    weight DECIMAL(12,4),
    weight_uom VARCHAR(10),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS mm_vendors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    vendor_number VARCHAR(30) NOT NULL UNIQUE,
    name VARCHAR(200) NOT NULL,
    contact_person VARCHAR(200),
    email VARCHAR(255),
    phone VARCHAR(50),
    address TEXT,
    payment_terms INT NOT NULL DEFAULT 30,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
