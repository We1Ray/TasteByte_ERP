-- 039_test_seed_data.sql
-- Comprehensive test/demo data for TasteByte Foods ERP simulation.
-- All INSERTs use ON CONFLICT DO NOTHING for idempotency.

-- Note: no explicit BEGIN/COMMIT needed; the migrator wraps in a transaction.

-- ============================================================
-- 1. Test Users
-- ============================================================
-- Password hash is argon2id for 'password123'
DO $$
DECLARE
    v_pwd_hash TEXT := '$argon2id$v=19$m=19456,t=2,p=1$YWRtaW4xMjNzYWx0$JCe3F3pO7VPMqnLPSxcdYXaj7b1VFQhFOAaXlFmv77k';
BEGIN
    INSERT INTO users (id, username, email, password_hash, display_name, is_active) VALUES
        ('a0000000-0000-0000-0000-000000000002', 'chen.wei', 'chen.wei@tastebyte.com', v_pwd_hash, 'Chen Wei (Finance)', true),
        ('a0000000-0000-0000-0000-000000000003', 'lin.mei', 'lin.mei@tastebyte.com', v_pwd_hash, 'Lin Mei (Sales)', true),
        ('a0000000-0000-0000-0000-000000000004', 'wang.jun', 'wang.jun@tastebyte.com', v_pwd_hash, 'Wang Jun (Procurement)', true),
        ('a0000000-0000-0000-0000-000000000005', 'zhang.li', 'zhang.li@tastebyte.com', v_pwd_hash, 'Zhang Li (Production)', true),
        ('a0000000-0000-0000-0000-000000000006', 'huang.dev', 'huang.dev@tastebyte.com', v_pwd_hash, 'Huang Dev (Developer)', true),
        ('a0000000-0000-0000-0000-000000000007', 'liu.qm', 'liu.qm@tastebyte.com', v_pwd_hash, 'Liu QM (Quality)', true),
        ('a0000000-0000-0000-0000-000000000008', 'zhao.hr', 'zhao.hr@tastebyte.com', v_pwd_hash, 'Zhao HR (HR)', true)
    ON CONFLICT (username) DO NOTHING;

    -- Assign ERP roles
    INSERT INTO user_roles (user_id, role_id)
    SELECT 'a0000000-0000-0000-0000-000000000002'::uuid, id FROM roles WHERE name = 'FI_MANAGER'
    ON CONFLICT DO NOTHING;
    INSERT INTO user_roles (user_id, role_id)
    SELECT 'a0000000-0000-0000-0000-000000000003'::uuid, id FROM roles WHERE name = 'SD_MANAGER'
    ON CONFLICT DO NOTHING;
    INSERT INTO user_roles (user_id, role_id)
    SELECT 'a0000000-0000-0000-0000-000000000004'::uuid, id FROM roles WHERE name = 'MM_MANAGER'
    ON CONFLICT DO NOTHING;
    INSERT INTO user_roles (user_id, role_id)
    SELECT 'a0000000-0000-0000-0000-000000000005'::uuid, id FROM roles WHERE name = 'PP_MANAGER'
    ON CONFLICT DO NOTHING;
    INSERT INTO user_roles (user_id, role_id)
    SELECT 'a0000000-0000-0000-0000-000000000006'::uuid, id FROM roles WHERE name = 'ADMIN'
    ON CONFLICT DO NOTHING;
    INSERT INTO user_roles (user_id, role_id)
    SELECT 'a0000000-0000-0000-0000-000000000007'::uuid, id FROM roles WHERE name = 'QM_MANAGER'
    ON CONFLICT DO NOTHING;
    INSERT INTO user_roles (user_id, role_id)
    SELECT 'a0000000-0000-0000-0000-000000000008'::uuid, id FROM roles WHERE name = 'HR_MANAGER'
    ON CONFLICT DO NOTHING;
END $$;

-- ============================================================
-- 2. Material Groups & 3. Materials
-- Use DO $$ block to handle existing groups with different UUIDs
-- ============================================================
DO $$
DECLARE
    v_snacks UUID;
    v_beverages UUID;
    v_raw_food UUID;
    v_packaging UUID;
BEGIN
    -- Insert material groups (ON CONFLICT keeps existing row with its UUID)
    INSERT INTO mm_material_groups (id, code, name, description) VALUES
        ('90000000-0000-0000-0000-000000000001', 'SNACKS', 'Snack Products', 'Finished snack food products')
    ON CONFLICT (code) DO NOTHING;
    INSERT INTO mm_material_groups (id, code, name, description) VALUES
        ('90000000-0000-0000-0000-000000000002', 'BEVERAGES', 'Beverages', 'Finished beverage products')
    ON CONFLICT (code) DO NOTHING;
    INSERT INTO mm_material_groups (id, code, name, description) VALUES
        ('90000000-0000-0000-0000-000000000003', 'RAW-FOOD', 'Raw Food Ingredients', 'Raw materials for food production')
    ON CONFLICT (code) DO NOTHING;
    INSERT INTO mm_material_groups (id, code, name, description) VALUES
        ('90000000-0000-0000-0000-000000000004', 'PACKAGING', 'Packaging Materials', 'Packaging and containers')
    ON CONFLICT (code) DO NOTHING;

    -- Look up actual IDs (may differ from hardcoded if groups pre-existed)
    SELECT id INTO v_snacks FROM mm_material_groups WHERE code = 'SNACKS';
    SELECT id INTO v_beverages FROM mm_material_groups WHERE code = 'BEVERAGES';
    SELECT id INTO v_raw_food FROM mm_material_groups WHERE code = 'RAW-FOOD';
    SELECT id INTO v_packaging FROM mm_material_groups WHERE code = 'PACKAGING';

    -- Insert materials using looked-up group IDs
    INSERT INTO mm_materials (id, material_number, name, description, material_group_id, material_type, is_active) VALUES
        ('b2000000-0000-0000-0000-000000000001', 'MAT-00001', 'Crispy Chips (Original) 150g', 'Original flavor potato chips', v_snacks, 'FERT', true),
        ('b2000000-0000-0000-0000-000000000002', 'MAT-00002', 'Crispy Chips (BBQ) 150g', 'BBQ flavor potato chips', v_snacks, 'FERT', true),
        ('b2000000-0000-0000-0000-000000000003', 'MAT-00003', 'Sparkling Juice (Grape) 330ml', 'Grape sparkling juice drink', v_beverages, 'FERT', true),
        ('b2000000-0000-0000-0000-000000000004', 'MAT-00004', 'Sparkling Juice (Apple) 330ml', 'Apple sparkling juice drink', v_beverages, 'FERT', true),
        ('b2000000-0000-0000-0000-000000000005', 'MAT-00005', 'Potato Starch', 'Food-grade potato starch powder', v_raw_food, 'RAW', true),
        ('b2000000-0000-0000-0000-000000000006', 'MAT-00006', 'Grape Concentrate', 'Concentrated grape juice', v_raw_food, 'RAW', true),
        ('b2000000-0000-0000-0000-000000000007', 'MAT-00007', 'Apple Concentrate', 'Concentrated apple juice', v_raw_food, 'RAW', true),
        ('b2000000-0000-0000-0000-000000000008', 'MAT-00008', 'Cooking Oil', 'High-grade frying oil', v_raw_food, 'RAW', true),
        ('b2000000-0000-0000-0000-000000000009', 'MAT-00009', 'BBQ Seasoning', 'BBQ flavor seasoning powder', v_raw_food, 'RAW', true),
        ('b2000000-0000-0000-0000-000000000010', 'MAT-00010', 'Chip Packaging Bag', 'Foil packaging bag for chips', v_packaging, 'HIBE', true),
        ('b2000000-0000-0000-0000-000000000011', 'MAT-00011', 'Aluminum Can 330ml', '330ml aluminum beverage can', v_packaging, 'HIBE', true),
        ('b2000000-0000-0000-0000-000000000012', 'MAT-00012', 'Carbonated Water', 'Filtered carbonated water', v_raw_food, 'RAW', true),
        ('b2000000-0000-0000-0000-000000000013', 'MAT-00013', 'Sugar', 'Refined white sugar', v_raw_food, 'RAW', true)
    ON CONFLICT (material_number) DO NOTHING;
END $$;

-- ============================================================
-- 4. Vendors
-- ============================================================
INSERT INTO mm_vendors (id, vendor_number, name, contact_person, email, phone, address, payment_terms, is_active) VALUES
    ('c2000000-0000-0000-0000-000000000001', 'VND-00001', 'Taiwan Agri Supplier', 'Mr. Li', 'li@taiwan-agri.com', '02-2345-6789', 'No. 100 Zhongzheng Rd, Taipei', 30, true),
    ('c2000000-0000-0000-0000-000000000002', 'VND-00002', 'Juice Import Trading', 'Ms. Chen', 'chen@juice-import.com', '02-3456-7890', 'No. 200 Minsheng E Rd, Taipei', 45, true),
    ('c2000000-0000-0000-0000-000000000003', 'VND-00003', 'Pacific Packaging Co.', 'Mr. Wang', 'wang@pacific-pkg.com', '03-4567-8901', 'No. 500 Industrial Rd, Taoyuan', 30, true)
ON CONFLICT (vendor_number) DO NOTHING;

-- ============================================================
-- 5. Customers
-- ============================================================
INSERT INTO sd_customers (id, customer_number, name, contact_person, email, phone, address, payment_terms, credit_limit, is_active) VALUES
    ('c0000000-0000-0000-0000-000000000001', 'CUST-00001', 'PX Mart', 'Manager Li', 'li@pxmart.com', '02-2345-6789', 'No. 100 Minsheng E Rd, Taipei', 30, 5000000, true),
    ('c0000000-0000-0000-0000-000000000002', 'CUST-00002', '7-Eleven', 'Manager Wang', 'wang@7-11.com', '02-3456-7890', 'No. 1 Songzhi Rd, Taipei', 30, 10000000, true),
    ('c0000000-0000-0000-0000-000000000003', 'CUST-00003', 'Carrefour', 'Buyer Chen', 'chen@carrefour.com', '02-4567-8901', 'No. 800 Zhongzheng Rd, New Taipei', 45, 8000000, true),
    ('c0000000-0000-0000-0000-000000000004', 'CUST-00004', 'Costco', 'Manager Zhang', 'zhang@costco.com', '02-5678-9012', 'No. 100 Jiuzong Rd, Taipei', 30, 15000000, true),
    ('c0000000-0000-0000-0000-000000000005', 'CUST-00005', 'RT-Mart', 'Supervisor Liu', 'liu@rt-mart.com', '03-6789-0123', 'No. 500 Zhongzheng Rd, Taoyuan', 60, 3000000, true)
ON CONFLICT (customer_number) DO NOTHING;

-- ============================================================
-- 6. Warehouses
-- ============================================================
INSERT INTO wm_warehouses (id, code, name, address, warehouse_type, is_active) VALUES
    ('f1000000-0000-0000-0000-000000000001', 'WH-TPE', 'Taipei Main Warehouse', 'No. 200 Logistics Rd, Taipei', 'STANDARD', true),
    ('f1000000-0000-0000-0000-000000000002', 'WH-TYN', 'Taoyuan Cold Storage', 'No. 300 Cold Chain Rd, Taoyuan', 'COLD_STORAGE', true),
    ('f1000000-0000-0000-0000-000000000003', 'WH-TCH', 'Taichung Distribution Center', 'No. 150 Distribution Rd, Taichung', 'STANDARD', true)
ON CONFLICT (code) DO NOTHING;

-- Storage Bins for Taipei warehouse
INSERT INTO wm_storage_bins (warehouse_id, bin_code, zone, aisle, rack, level, is_active)
SELECT w.id, bin.code, bin.zone, bin.aisle, bin.rack, bin.lvl, true
FROM wm_warehouses w,
(VALUES
    ('A-01-01-1', 'A', '01', '01', '1'),
    ('A-01-01-2', 'A', '01', '01', '2'),
    ('A-01-02-1', 'A', '01', '02', '1'),
    ('B-01-01-1', 'B', '01', '01', '1'),
    ('B-01-02-1', 'B', '01', '02', '1'),
    ('C-01-01-1', 'C', '01', '01', '1')
) AS bin(code, zone, aisle, rack, lvl)
WHERE w.code = 'WH-TPE'
ON CONFLICT (warehouse_id, bin_code) DO NOTHING;

-- ============================================================
-- 7. Plant Stock (initial inventory)
-- ============================================================
-- Get UOM IDs
DO $$
DECLARE
    v_ea_uom UUID;
    v_kg_uom UUID;
    v_l_uom UUID;
    v_wh1 UUID := 'f1000000-0000-0000-0000-000000000001';
BEGIN
    SELECT id INTO v_ea_uom FROM mm_uom WHERE code = 'EA';
    SELECT id INTO v_kg_uom FROM mm_uom WHERE code = 'KG';
    SELECT id INTO v_l_uom FROM mm_uom WHERE code = 'L';

    -- Finished goods
    INSERT INTO mm_plant_stock (material_id, warehouse_id, quantity, reserved_quantity, uom_id)
    VALUES
        ('b2000000-0000-0000-0000-000000000001', v_wh1, 15000, 0, v_ea_uom),
        ('b2000000-0000-0000-0000-000000000002', v_wh1, 12000, 0, v_ea_uom),
        ('b2000000-0000-0000-0000-000000000003', v_wh1, 35000, 0, v_ea_uom),
        ('b2000000-0000-0000-0000-000000000004', v_wh1, 28000, 0, v_ea_uom),
        -- Raw materials
        ('b2000000-0000-0000-0000-000000000005', v_wh1, 3000, 0, v_kg_uom),
        ('b2000000-0000-0000-0000-000000000006', v_wh1, 1200, 0, v_l_uom),
        ('b2000000-0000-0000-0000-000000000007', v_wh1, 800, 0, v_l_uom),
        ('b2000000-0000-0000-0000-000000000008', v_wh1, 2500, 0, v_l_uom),
        ('b2000000-0000-0000-0000-000000000009', v_wh1, 500, 0, v_kg_uom),
        -- Packaging
        ('b2000000-0000-0000-0000-000000000010', v_wh1, 45000, 0, v_ea_uom),
        ('b2000000-0000-0000-0000-000000000011', v_wh1, 60000, 0, v_ea_uom),
        ('b2000000-0000-0000-0000-000000000012', v_wh1, 15000, 0, v_l_uom),
        ('b2000000-0000-0000-0000-000000000013', v_wh1, 4000, 0, v_kg_uom)
    ON CONFLICT (material_id, warehouse_id) DO NOTHING;
END $$;

-- ============================================================
-- 8. Cost Centers & Profit Centers
-- ============================================================
INSERT INTO co_cost_centers (id, code, name, description, is_active) VALUES
    ('cc000000-0000-0000-0000-000000000001', 'CC-PROD', 'Production', 'Production cost center', true),
    ('cc000000-0000-0000-0000-000000000002', 'CC-SALES', 'Sales', 'Sales department cost center', true),
    ('cc000000-0000-0000-0000-000000000003', 'CC-PROCUREMENT', 'Procurement', 'Procurement cost center', true),
    ('cc000000-0000-0000-0000-000000000004', 'CC-QC', 'Quality Control', 'Quality control cost center', true),
    ('cc000000-0000-0000-0000-000000000005', 'CC-ADMIN', 'Administration', 'General admin cost center', true)
ON CONFLICT (code) DO NOTHING;

INSERT INTO co_profit_centers (id, code, name, description, is_active) VALUES
    ('0c000000-0000-0000-0000-000000000001', 'PC-SNACKS', 'Snack Products', 'Snack product line profit center', true),
    ('0c000000-0000-0000-0000-000000000002', 'PC-BEVERAGES', 'Beverages', 'Beverage product line profit center', true),
    ('0c000000-0000-0000-0000-000000000003', 'PC-RETAIL', 'Retail Channel', 'Retail channel profit center', true)
ON CONFLICT (code) DO NOTHING;

-- ============================================================
-- 9. HR Departments & Employees
-- ============================================================
INSERT INTO hr_departments (id, code, name, is_active) VALUES
    ('d0000000-0000-0000-0000-000000000001', 'FIN', 'Finance Department', true),
    ('d0000000-0000-0000-0000-000000000002', 'SALES', 'Sales Department', true),
    ('d0000000-0000-0000-0000-000000000003', 'PROC', 'Procurement Department', true),
    ('d0000000-0000-0000-0000-000000000004', 'PROD', 'Production Department', true),
    ('d0000000-0000-0000-0000-000000000005', 'QC', 'Quality Control Department', true),
    ('d0000000-0000-0000-0000-000000000006', 'HRD', 'Human Resources Department', true)
ON CONFLICT (code) DO NOTHING;

INSERT INTO hr_employees (id, employee_number, user_id, first_name, last_name, email, phone, department_id, hire_date, status) VALUES
    ('e0000000-0000-0000-0000-000000000001', 'EMP-00001', 'a0000000-0000-0000-0000-000000000002', 'Wei', 'Chen', 'chen.wei@tastebyte.com', '0912-345-678', 'd0000000-0000-0000-0000-000000000001', '2020-01-15', 'ACTIVE'),
    ('e0000000-0000-0000-0000-000000000002', 'EMP-00002', 'a0000000-0000-0000-0000-000000000003', 'Mei', 'Lin', 'lin.mei@tastebyte.com', '0923-456-789', 'd0000000-0000-0000-0000-000000000002', '2019-06-01', 'ACTIVE'),
    ('e0000000-0000-0000-0000-000000000003', 'EMP-00003', 'a0000000-0000-0000-0000-000000000004', 'Jun', 'Wang', 'wang.jun@tastebyte.com', '0934-567-890', 'd0000000-0000-0000-0000-000000000003', '2021-03-10', 'ACTIVE'),
    ('e0000000-0000-0000-0000-000000000004', 'EMP-00004', 'a0000000-0000-0000-0000-000000000005', 'Li', 'Zhang', 'zhang.li@tastebyte.com', '0945-678-901', 'd0000000-0000-0000-0000-000000000004', '2018-11-20', 'ACTIVE'),
    ('e0000000-0000-0000-0000-000000000005', 'EMP-00005', 'a0000000-0000-0000-0000-000000000007', 'QM', 'Liu', 'liu.qm@tastebyte.com', '0956-789-012', 'd0000000-0000-0000-0000-000000000005', '2022-02-14', 'ACTIVE'),
    ('e0000000-0000-0000-0000-000000000006', 'EMP-00006', 'a0000000-0000-0000-0000-000000000008', 'HR', 'Zhao', 'zhao.hr@tastebyte.com', '0967-890-123', 'd0000000-0000-0000-0000-000000000006', '2020-08-01', 'ACTIVE')
ON CONFLICT (employee_number) DO NOTHING;

-- ============================================================
-- 10. BOM (Bill of Materials) - matching actual schema
-- ============================================================
INSERT INTO pp_boms (id, bom_number, material_id, name, version, status) VALUES
    ('b0000000-0000-0000-0000-000000000001', 'BOM-00001', 'b2000000-0000-0000-0000-000000000001', 'Crispy Chips (Original) BOM', 1, 'ACTIVE'),
    ('b0000000-0000-0000-0000-000000000002', 'BOM-00002', 'b2000000-0000-0000-0000-000000000003', 'Sparkling Juice (Grape) BOM', 1, 'ACTIVE')
ON CONFLICT (bom_number) DO NOTHING;

-- BOM items require line_number and component_material_id (not material_id)
DO $$
DECLARE
    v_ea_uom UUID;
    v_kg_uom UUID;
    v_l_uom UUID;
BEGIN
    SELECT id INTO v_ea_uom FROM mm_uom WHERE code = 'EA';
    SELECT id INTO v_kg_uom FROM mm_uom WHERE code = 'KG';
    SELECT id INTO v_l_uom FROM mm_uom WHERE code = 'L';

    -- Only insert if BOM items don't exist for these BOMs
    IF NOT EXISTS (SELECT 1 FROM pp_bom_items WHERE bom_id = 'b0000000-0000-0000-0000-000000000001') THEN
        INSERT INTO pp_bom_items (bom_id, line_number, component_material_id, quantity, uom_id, scrap_percentage) VALUES
            ('b0000000-0000-0000-0000-000000000001', 1, 'b2000000-0000-0000-0000-000000000005', 100, v_kg_uom, 2.0),
            ('b0000000-0000-0000-0000-000000000001', 2, 'b2000000-0000-0000-0000-000000000008', 50, v_l_uom, 1.0),
            ('b0000000-0000-0000-0000-000000000001', 3, 'b2000000-0000-0000-0000-000000000010', 1000, v_ea_uom, 3.0);
    END IF;

    IF NOT EXISTS (SELECT 1 FROM pp_bom_items WHERE bom_id = 'b0000000-0000-0000-0000-000000000002') THEN
        INSERT INTO pp_bom_items (bom_id, line_number, component_material_id, quantity, uom_id, scrap_percentage) VALUES
            ('b0000000-0000-0000-0000-000000000002', 1, 'b2000000-0000-0000-0000-000000000006', 30, v_l_uom, 1.0),
            ('b0000000-0000-0000-0000-000000000002', 2, 'b2000000-0000-0000-0000-000000000012', 250, v_l_uom, 0.5),
            ('b0000000-0000-0000-0000-000000000002', 3, 'b2000000-0000-0000-0000-000000000013', 20, v_kg_uom, 0.5),
            ('b0000000-0000-0000-0000-000000000002', 4, 'b2000000-0000-0000-0000-000000000011', 1000, v_ea_uom, 2.0);
    END IF;
END $$;

-- ============================================================
-- 11. Purchase Orders (matching actual schema: vendor_id FK, not vendor_name)
-- ============================================================
INSERT INTO mm_purchase_orders (id, po_number, vendor_id, order_date, delivery_date, status, total_amount, currency, notes, created_by) VALUES
    ('40000000-0000-0000-0000-000000000001', 'PO-00000001', 'c2000000-0000-0000-0000-000000000001', '2026-03-10', '2026-03-20', 'RELEASED', 320000, 'TWD', 'Monthly raw material replenishment', 'a0000000-0000-0000-0000-000000000004'),
    ('40000000-0000-0000-0000-000000000002', 'PO-00000002', 'c2000000-0000-0000-0000-000000000002', '2026-03-12', '2026-03-22', 'RELEASED', 280000, 'TWD', 'Juice concentrate order', 'a0000000-0000-0000-0000-000000000004')
ON CONFLICT (po_number) DO NOTHING;

DO $$
DECLARE
    v_kg_uom UUID;
    v_l_uom UUID;
    v_ea_uom UUID;
BEGIN
    SELECT id INTO v_kg_uom FROM mm_uom WHERE code = 'KG';
    SELECT id INTO v_l_uom FROM mm_uom WHERE code = 'L';
    SELECT id INTO v_ea_uom FROM mm_uom WHERE code = 'EA';

    IF NOT EXISTS (SELECT 1 FROM mm_purchase_order_items WHERE purchase_order_id = '40000000-0000-0000-0000-000000000001') THEN
        INSERT INTO mm_purchase_order_items (purchase_order_id, line_number, material_id, quantity, unit_price, total_price, uom_id, received_quantity) VALUES
            ('40000000-0000-0000-0000-000000000001', 1, 'b2000000-0000-0000-0000-000000000005', 2000, 80, 160000, v_kg_uom, 0),
            ('40000000-0000-0000-0000-000000000001', 2, 'b2000000-0000-0000-0000-000000000008', 2000, 45, 90000, v_l_uom, 0),
            ('40000000-0000-0000-0000-000000000001', 3, 'b2000000-0000-0000-0000-000000000013', 1000, 30, 30000, v_kg_uom, 0),
            ('40000000-0000-0000-0000-000000000001', 4, 'b2000000-0000-0000-0000-000000000010', 20000, 2, 40000, v_ea_uom, 0);
    END IF;

    IF NOT EXISTS (SELECT 1 FROM mm_purchase_order_items WHERE purchase_order_id = '40000000-0000-0000-0000-000000000002') THEN
        INSERT INTO mm_purchase_order_items (purchase_order_id, line_number, material_id, quantity, unit_price, total_price, uom_id, received_quantity) VALUES
            ('40000000-0000-0000-0000-000000000002', 1, 'b2000000-0000-0000-0000-000000000006', 1000, 150, 150000, v_l_uom, 0),
            ('40000000-0000-0000-0000-000000000002', 2, 'b2000000-0000-0000-0000-000000000007', 1000, 130, 130000, v_l_uom, 0);
    END IF;
END $$;

-- ============================================================
-- 12. Sales Orders (matching actual schema)
-- ============================================================
INSERT INTO sd_sales_orders (id, order_number, customer_id, order_date, requested_delivery_date, status, total_amount, currency, notes, created_by) VALUES
    ('50000000-0000-0000-0000-000000000001', 'SO-00000001', 'c0000000-0000-0000-0000-000000000001', '2026-03-15', '2026-03-25', 'CONFIRMED', 875000, 'TWD', 'PX Mart monthly order', 'a0000000-0000-0000-0000-000000000003'),
    ('50000000-0000-0000-0000-000000000002', 'SO-00000002', 'c0000000-0000-0000-0000-000000000002', '2026-03-16', '2026-03-28', 'CONFIRMED', 1250000, 'TWD', '7-Eleven promotion order', 'a0000000-0000-0000-0000-000000000003'),
    ('50000000-0000-0000-0000-000000000003', 'SO-00000003', 'c0000000-0000-0000-0000-000000000004', '2026-03-18', '2026-04-01', 'DRAFT', 2100000, 'TWD', 'Costco bulk order', 'a0000000-0000-0000-0000-000000000003')
ON CONFLICT (order_number) DO NOTHING;

DO $$
DECLARE
    v_ea_uom UUID;
BEGIN
    SELECT id INTO v_ea_uom FROM mm_uom WHERE code = 'EA';

    IF NOT EXISTS (SELECT 1 FROM sd_sales_order_items WHERE sales_order_id = '50000000-0000-0000-0000-000000000001') THEN
        INSERT INTO sd_sales_order_items (sales_order_id, line_number, material_id, quantity, unit_price, total_price, uom_id, delivered_quantity) VALUES
            ('50000000-0000-0000-0000-000000000001', 1, 'b2000000-0000-0000-0000-000000000001', 10000, 35, 350000, v_ea_uom, 0),
            ('50000000-0000-0000-0000-000000000001', 2, 'b2000000-0000-0000-0000-000000000003', 21000, 25, 525000, v_ea_uom, 0);
    END IF;

    IF NOT EXISTS (SELECT 1 FROM sd_sales_order_items WHERE sales_order_id = '50000000-0000-0000-0000-000000000002') THEN
        INSERT INTO sd_sales_order_items (sales_order_id, line_number, material_id, quantity, unit_price, total_price, uom_id, delivered_quantity) VALUES
            ('50000000-0000-0000-0000-000000000002', 1, 'b2000000-0000-0000-0000-000000000002', 15000, 35, 525000, v_ea_uom, 0),
            ('50000000-0000-0000-0000-000000000002', 2, 'b2000000-0000-0000-0000-000000000004', 29000, 25, 725000, v_ea_uom, 0);
    END IF;

    IF NOT EXISTS (SELECT 1 FROM sd_sales_order_items WHERE sales_order_id = '50000000-0000-0000-0000-000000000003') THEN
        INSERT INTO sd_sales_order_items (sales_order_id, line_number, material_id, quantity, unit_price, total_price, uom_id, delivered_quantity) VALUES
            ('50000000-0000-0000-0000-000000000003', 1, 'b2000000-0000-0000-0000-000000000001', 30000, 35, 1050000, v_ea_uom, 0),
            ('50000000-0000-0000-0000-000000000003', 2, 'b2000000-0000-0000-0000-000000000003', 42000, 25, 1050000, v_ea_uom, 0);
    END IF;
END $$;

-- ============================================================
-- 13. Production Orders (matching actual schema)
-- ============================================================
DO $$
DECLARE
    v_ea_uom UUID;
BEGIN
    SELECT id INTO v_ea_uom FROM mm_uom WHERE code = 'EA';

    INSERT INTO pp_production_orders (id, order_number, material_id, bom_id, planned_quantity, actual_quantity, uom_id, planned_start, planned_end, status, created_by) VALUES
        ('e2000000-0000-0000-0000-000000000001', 'PRD-00000001', 'b2000000-0000-0000-0000-000000000001', 'b0000000-0000-0000-0000-000000000001', 10000, 0, v_ea_uom, '2026-03-20', '2026-03-22', 'CREATED', 'a0000000-0000-0000-0000-000000000005'),
        ('e2000000-0000-0000-0000-000000000002', 'PRD-00000002', 'b2000000-0000-0000-0000-000000000003', 'b0000000-0000-0000-0000-000000000002', 21000, 0, v_ea_uom, '2026-03-21', '2026-03-24', 'CREATED', 'a0000000-0000-0000-0000-000000000005')
    ON CONFLICT (order_number) DO NOTHING;
END $$;

-- ============================================================
-- 14. Sample GRN (Goods Receipt Note) for testing the new table
-- ============================================================
DO $$
DECLARE
    v_kg_uom UUID;
    v_l_uom UUID;
    v_po1_item1 UUID;
    v_po1_item2 UUID;
BEGIN
    SELECT id INTO v_kg_uom FROM mm_uom WHERE code = 'KG';
    SELECT id INTO v_l_uom FROM mm_uom WHERE code = 'L';

    -- Get PO item IDs
    SELECT id INTO v_po1_item1 FROM mm_purchase_order_items
    WHERE purchase_order_id = '40000000-0000-0000-0000-000000000001' AND line_number = 1;
    SELECT id INTO v_po1_item2 FROM mm_purchase_order_items
    WHERE purchase_order_id = '40000000-0000-0000-0000-000000000001' AND line_number = 2;

    IF v_po1_item1 IS NOT NULL AND NOT EXISTS (SELECT 1 FROM mm_goods_receipts WHERE grn_number = 'GRN-00000001') THEN
        INSERT INTO mm_goods_receipts (id, grn_number, purchase_order_id, vendor_id, receipt_date, warehouse_id, status, notes, received_by)
        VALUES ('9e000000-0000-0000-0000-000000000001', 'GRN-00000001', '40000000-0000-0000-0000-000000000001', 'c2000000-0000-0000-0000-000000000001', '2026-03-20', 'f1000000-0000-0000-0000-000000000001', 'DRAFT', 'Partial receipt of PO-00000001', 'a0000000-0000-0000-0000-000000000004');

        INSERT INTO mm_goods_receipt_items (goods_receipt_id, po_item_id, material_id, ordered_quantity, received_quantity, rejected_quantity, uom_id, batch_number, notes)
        VALUES
            ('9e000000-0000-0000-0000-000000000001', v_po1_item1, 'b2000000-0000-0000-0000-000000000005', 2000, 1500, 0, v_kg_uom, 'BATCH-2026-03-001', 'First partial delivery'),
            ('9e000000-0000-0000-0000-000000000001', v_po1_item2, 'b2000000-0000-0000-0000-000000000008', 2000, 2000, 50, v_l_uom, 'BATCH-2026-03-002', 'Full delivery, 50L rejected');
    END IF;
END $$;

-- ============================================================
-- 15. Stock Reservations (for confirmed SOs)
-- ============================================================
INSERT INTO mm_stock_reservations (material_id, warehouse_id, reserved_quantity, reference_type, reference_id, status, reserved_by)
SELECT 'b2000000-0000-0000-0000-000000000001'::uuid, 'f1000000-0000-0000-0000-000000000001'::uuid, 10000, 'SALES_ORDER', '50000000-0000-0000-0000-000000000001'::uuid, 'ACTIVE', 'a0000000-0000-0000-0000-000000000003'::uuid
WHERE NOT EXISTS (SELECT 1 FROM mm_stock_reservations WHERE reference_id = '50000000-0000-0000-0000-000000000001' AND material_id = 'b2000000-0000-0000-0000-000000000001');

INSERT INTO mm_stock_reservations (material_id, warehouse_id, reserved_quantity, reference_type, reference_id, status, reserved_by)
SELECT 'b2000000-0000-0000-0000-000000000003'::uuid, 'f1000000-0000-0000-0000-000000000001'::uuid, 21000, 'SALES_ORDER', '50000000-0000-0000-0000-000000000001'::uuid, 'ACTIVE', 'a0000000-0000-0000-0000-000000000003'::uuid
WHERE NOT EXISTS (SELECT 1 FROM mm_stock_reservations WHERE reference_id = '50000000-0000-0000-0000-000000000001' AND material_id = 'b2000000-0000-0000-0000-000000000003');

-- ============================================================
-- 16. Update number range counters to reflect seeded data
-- ============================================================
UPDATE number_ranges SET current_number = GREATEST(current_number, 13) WHERE object_type = 'MAT';
UPDATE number_ranges SET current_number = GREATEST(current_number, 2) WHERE object_type = 'PO';
UPDATE number_ranges SET current_number = GREATEST(current_number, 3) WHERE object_type = 'SO';
UPDATE number_ranges SET current_number = GREATEST(current_number, 2) WHERE object_type = 'PRD';
UPDATE number_ranges SET current_number = GREATEST(current_number, 6) WHERE object_type = 'EMP';
UPDATE number_ranges SET current_number = GREATEST(current_number, 2) WHERE object_type = 'BOM';
UPDATE number_ranges SET current_number = GREATEST(current_number, 1) WHERE object_type = 'GRN';

-- Number range for VND (vendors) if missing
INSERT INTO number_ranges (object_type, prefix, current_number, pad_length) VALUES
    ('VND', 'VND', 3, 8),
    ('CUST', 'CUST', 5, 8)
ON CONFLICT (object_type) DO UPDATE SET current_number = GREATEST(number_ranges.current_number, EXCLUDED.current_number);
