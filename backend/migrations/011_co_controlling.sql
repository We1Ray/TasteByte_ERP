-- CO Module: Cost Centers, Profit Centers, Internal Orders, Cost Allocations

CREATE TABLE IF NOT EXISTS co_cost_centers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(20) NOT NULL UNIQUE,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    responsible_person UUID REFERENCES users(id),
    is_active BOOLEAN NOT NULL DEFAULT true,
    valid_from DATE,
    valid_to DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS co_profit_centers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(20) NOT NULL UNIQUE,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    responsible_person UUID REFERENCES users(id),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS co_internal_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_number VARCHAR(30) NOT NULL UNIQUE,
    name VARCHAR(200) NOT NULL,
    order_type VARCHAR(50) NOT NULL,
    cost_center_id UUID REFERENCES co_cost_centers(id),
    status VARCHAR(20) NOT NULL DEFAULT 'CREATED',
    budget DECIMAL(18,4) NOT NULL DEFAULT 0,
    actual_cost DECIMAL(18,4) NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS co_cost_allocations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_cost_center_id UUID NOT NULL REFERENCES co_cost_centers(id),
    to_cost_center_id UUID NOT NULL REFERENCES co_cost_centers(id),
    allocation_date DATE NOT NULL,
    amount DECIMAL(18,4) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
