-- SD Module: Sales, Deliveries, Invoices

CREATE TABLE IF NOT EXISTS sd_customers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    customer_number VARCHAR(30) NOT NULL UNIQUE,
    name VARCHAR(200) NOT NULL,
    contact_person VARCHAR(200),
    email VARCHAR(255),
    phone VARCHAR(50),
    address TEXT,
    payment_terms INT NOT NULL DEFAULT 30,
    credit_limit DECIMAL(18,4) NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS sd_sales_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_number VARCHAR(30) NOT NULL UNIQUE,
    customer_id UUID NOT NULL REFERENCES sd_customers(id),
    order_date DATE NOT NULL,
    requested_delivery_date DATE,
    status VARCHAR(20) NOT NULL DEFAULT 'DRAFT',
    total_amount DECIMAL(18,4) NOT NULL DEFAULT 0,
    currency VARCHAR(3) NOT NULL DEFAULT 'TWD',
    notes TEXT,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS sd_sales_order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sales_order_id UUID NOT NULL REFERENCES sd_sales_orders(id) ON DELETE CASCADE,
    line_number INT NOT NULL,
    material_id UUID NOT NULL REFERENCES mm_materials(id),
    quantity DECIMAL(18,4) NOT NULL,
    unit_price DECIMAL(18,4) NOT NULL,
    total_price DECIMAL(18,4) NOT NULL,
    uom_id UUID REFERENCES mm_uom(id),
    delivered_quantity DECIMAL(18,4) NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS sd_deliveries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    delivery_number VARCHAR(30) NOT NULL UNIQUE,
    sales_order_id UUID NOT NULL REFERENCES sd_sales_orders(id),
    delivery_date DATE NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'CREATED',
    shipped_by UUID REFERENCES users(id),
    shipped_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS sd_delivery_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    delivery_id UUID NOT NULL REFERENCES sd_deliveries(id) ON DELETE CASCADE,
    sales_order_item_id UUID NOT NULL REFERENCES sd_sales_order_items(id),
    quantity DECIMAL(18,4) NOT NULL
);

CREATE TABLE IF NOT EXISTS sd_invoices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    invoice_number VARCHAR(30) NOT NULL UNIQUE,
    sales_order_id UUID NOT NULL REFERENCES sd_sales_orders(id),
    delivery_id UUID REFERENCES sd_deliveries(id),
    customer_id UUID NOT NULL REFERENCES sd_customers(id),
    invoice_date DATE NOT NULL,
    due_date DATE NOT NULL,
    total_amount DECIMAL(18,4) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'CREATED',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
