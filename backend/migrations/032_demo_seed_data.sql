-- 032: Demo Seed Data - Realistic food & beverage industry data
-- This migration cleans up test data and inserts realistic demo records.
-- Designed to be idempotent (safe to run multiple times).

BEGIN;

-- ============================================================================
-- PHASE 1: CLEAN UP TEST DATA
-- ============================================================================

-- Delete test UOMs
DELETE FROM mm_uom WHERE code IN ('TEST', 'TST2');

-- Delete test accounts (account_number starting with 'T')
DELETE FROM fi_journal_items WHERE account_id IN (
    SELECT id FROM fi_accounts WHERE account_number LIKE 'T%'
);
DELETE FROM fi_accounts WHERE account_number LIKE 'T%';

-- Delete test cost centers (code starting with 'TC')
DELETE FROM co_cost_allocations WHERE from_cost_center_id IN (
    SELECT id FROM co_cost_centers WHERE code LIKE 'TC%'
) OR to_cost_center_id IN (
    SELECT id FROM co_cost_centers WHERE code LIKE 'TC%'
);
DELETE FROM co_internal_orders WHERE cost_center_id IN (
    SELECT id FROM co_cost_centers WHERE code LIKE 'TC%'
);
DELETE FROM co_cost_centers WHERE code LIKE 'TC%';

-- Delete test profit centers (code starting with 'TP')
DELETE FROM co_profit_centers WHERE code LIKE 'TP%';

-- Delete test warehouses (code not 'WH01')
DELETE FROM wm_stock_count_items WHERE stock_count_id IN (
    SELECT id FROM wm_stock_counts WHERE warehouse_id IN (
        SELECT id FROM wm_warehouses WHERE code != 'WH01'
    )
);
DELETE FROM wm_stock_counts WHERE warehouse_id IN (
    SELECT id FROM wm_warehouses WHERE code != 'WH01'
);
DELETE FROM wm_stock_transfers WHERE from_warehouse_id IN (
    SELECT id FROM wm_warehouses WHERE code != 'WH01'
) OR to_warehouse_id IN (
    SELECT id FROM wm_warehouses WHERE code != 'WH01'
);
DELETE FROM wm_storage_bins WHERE warehouse_id IN (
    SELECT id FROM wm_warehouses WHERE code != 'WH01'
);
DELETE FROM wm_warehouses WHERE code != 'WH01';

-- Delete test HR data
DELETE FROM hr_attendance WHERE employee_id IN (
    SELECT id FROM hr_employees WHERE first_name IN ('Test', 'Attendance')
);
DELETE FROM hr_employees WHERE first_name IN ('Test', 'Attendance');
DELETE FROM hr_positions WHERE code LIKE 'PO%' AND title = 'Integration Test Position';
DELETE FROM hr_departments WHERE code LIKE 'TD%';

-- Clean up duplicate/test materials (keep MAT00000001 through MAT00000005, delete the rest)
-- First, clean up references to test materials

-- Delete QM data referencing test materials
DELETE FROM qm_inspection_results WHERE inspection_lot_id IN (
    SELECT id FROM qm_inspection_lots WHERE material_id IN (
        SELECT id FROM mm_materials WHERE material_number NOT IN (
            'MAT00000001','MAT00000002','MAT00000003','MAT00000004','MAT00000005'
        )
    )
);
DELETE FROM qm_inspection_lots WHERE material_id IN (
    SELECT id FROM mm_materials WHERE material_number NOT IN (
        'MAT00000001','MAT00000002','MAT00000003','MAT00000004','MAT00000005'
    )
);

-- Delete quality notifications referencing test materials
DELETE FROM qm_quality_notifications WHERE material_id IN (
    SELECT id FROM mm_materials WHERE material_number NOT IN (
        'MAT00000001','MAT00000002','MAT00000003','MAT00000004','MAT00000005'
    )
);

-- Delete production orders referencing test materials (and their BOMs)
DELETE FROM pp_production_orders WHERE material_id IN (
    SELECT id FROM mm_materials WHERE material_number NOT IN (
        'MAT00000001','MAT00000002','MAT00000003','MAT00000004','MAT00000005'
    )
);

-- Delete routings referencing test materials
DELETE FROM pp_routing_operations WHERE routing_id IN (
    SELECT id FROM pp_routings WHERE material_id IN (
        SELECT id FROM mm_materials WHERE material_number NOT IN (
            'MAT00000001','MAT00000002','MAT00000003','MAT00000004','MAT00000005'
        )
    )
);
DELETE FROM pp_routings WHERE material_id IN (
    SELECT id FROM mm_materials WHERE material_number NOT IN (
        'MAT00000001','MAT00000002','MAT00000003','MAT00000004','MAT00000005'
    )
);

-- Delete BOM items and BOMs referencing test materials
DELETE FROM pp_bom_items WHERE bom_id IN (
    SELECT id FROM pp_boms WHERE material_id IN (
        SELECT id FROM mm_materials WHERE material_number NOT IN (
            'MAT00000001','MAT00000002','MAT00000003','MAT00000004','MAT00000005'
        )
    )
);
DELETE FROM pp_bom_items WHERE component_material_id IN (
    SELECT id FROM mm_materials WHERE material_number NOT IN (
        'MAT00000001','MAT00000002','MAT00000003','MAT00000004','MAT00000005'
    )
);
DELETE FROM pp_boms WHERE material_id IN (
    SELECT id FROM mm_materials WHERE material_number NOT IN (
        'MAT00000001','MAT00000002','MAT00000003','MAT00000004','MAT00000005'
    )
);

-- Delete SO items and SOs referencing test materials
DELETE FROM sd_delivery_items WHERE sales_order_item_id IN (
    SELECT id FROM sd_sales_order_items WHERE material_id IN (
        SELECT id FROM mm_materials WHERE material_number NOT IN (
            'MAT00000001','MAT00000002','MAT00000003','MAT00000004','MAT00000005'
        )
    )
);
DELETE FROM sd_invoices WHERE sales_order_id IN (
    SELECT id FROM sd_sales_orders WHERE customer_id IN (
        SELECT id FROM sd_customers WHERE name LIKE '%Test%' OR name LIKE 'SD %'
    )
);
DELETE FROM sd_deliveries WHERE sales_order_id IN (
    SELECT id FROM sd_sales_orders WHERE customer_id IN (
        SELECT id FROM sd_customers WHERE name LIKE '%Test%' OR name LIKE 'SD %'
    )
);
DELETE FROM sd_sales_order_items WHERE material_id IN (
    SELECT id FROM mm_materials WHERE material_number NOT IN (
        'MAT00000001','MAT00000002','MAT00000003','MAT00000004','MAT00000005'
    )
);
DELETE FROM sd_sales_order_items WHERE sales_order_id IN (
    SELECT id FROM sd_sales_orders WHERE customer_id IN (
        SELECT id FROM sd_customers WHERE name LIKE '%Test%' OR name LIKE 'SD %'
    )
);
DELETE FROM sd_sales_orders WHERE customer_id IN (
    SELECT id FROM sd_customers WHERE name LIKE '%Test%' OR name LIKE 'SD %'
);

-- Delete PO items and POs referencing test materials
DELETE FROM mm_purchase_order_items WHERE material_id IN (
    SELECT id FROM mm_materials WHERE material_number NOT IN (
        'MAT00000001','MAT00000002','MAT00000003','MAT00000004','MAT00000005'
    )
);
DELETE FROM mm_purchase_order_items WHERE purchase_order_id IN (
    SELECT id FROM mm_purchase_orders WHERE vendor_id IN (
        SELECT id FROM mm_vendors WHERE name LIKE '%Test%' OR name LIKE 'RBAC%'
    )
);
DELETE FROM mm_purchase_orders WHERE vendor_id IN (
    SELECT id FROM mm_vendors WHERE name LIKE '%Test%' OR name LIKE 'RBAC%'
);

-- Delete plant stock for test materials
DELETE FROM mm_plant_stock WHERE material_id IN (
    SELECT id FROM mm_materials WHERE material_number NOT IN (
        'MAT00000001','MAT00000002','MAT00000003','MAT00000004','MAT00000005'
    )
);

-- Delete material movements for test materials
DELETE FROM mm_material_movements WHERE material_id IN (
    SELECT id FROM mm_materials WHERE material_number NOT IN (
        'MAT00000001','MAT00000002','MAT00000003','MAT00000004','MAT00000005'
    )
);

-- Delete WM stock transfers for test materials
DELETE FROM wm_stock_transfers WHERE material_id IN (
    SELECT id FROM mm_materials WHERE material_number NOT IN (
        'MAT00000001','MAT00000002','MAT00000003','MAT00000004','MAT00000005'
    )
);

-- Now delete the test materials themselves
DELETE FROM mm_materials WHERE material_number NOT IN (
    'MAT00000001','MAT00000002','MAT00000003','MAT00000004','MAT00000005'
);

-- Delete test vendors (keep VND00000001)
DELETE FROM mm_vendors WHERE name LIKE '%Test%' OR name LIKE 'RBAC%';

-- Delete test customers (keep SO00000001 = Gourmet Restaurant Tokyo)
DELETE FROM sd_customers WHERE name LIKE '%Test%' OR name LIKE 'SD %';

-- ============================================================================
-- PHASE 2: MATERIAL GROUPS
-- ============================================================================

INSERT INTO mm_material_groups (code, name, description) VALUES
    ('FLOUR', 'Flour & Grains', 'Wheat flour, rice flour, oats, grains'),
    ('DAIRY', 'Dairy Products', 'Milk, butter, cream, cheese'),
    ('SEAFOOD', 'Seafood', 'Fresh and frozen seafood'),
    ('PRODUCE', 'Fresh Produce', 'Fruits, vegetables, herbs'),
    ('OILS', 'Oils & Condiments', 'Cooking oils, vinegars, sauces'),
    ('SPICES', 'Spices & Seasonings', 'Dried spices, herbs, seasonings'),
    ('MEAT', 'Meat & Poultry', 'Fresh and frozen meat products'),
    ('PACKAGING', 'Packaging Materials', 'Boxes, bags, containers'),
    ('BAKERY', 'Bakery Products', 'Bread, pastries, cakes'),
    ('BEVERAGE', 'Beverages', 'Juices, teas, coffee')
ON CONFLICT (code) DO NOTHING;

-- ============================================================================
-- PHASE 3: UPDATE EXISTING MATERIALS WITH REALISTIC DATA
-- ============================================================================

UPDATE mm_materials SET
    name = 'All-Purpose Flour 25kg',
    description = 'High-quality all-purpose wheat flour, protein content 10-12%',
    material_type = 'RAW',
    weight = 25.0000,
    weight_uom = 'KG',
    base_uom_id = (SELECT id FROM mm_uom WHERE code = 'KG'),
    material_group_id = (SELECT id FROM mm_material_groups WHERE code = 'FLOUR')
WHERE material_number = 'MAT00000001';

UPDATE mm_materials SET
    name = 'Extra Virgin Olive Oil 500ml',
    description = 'Italian extra virgin olive oil, cold pressed, premium grade',
    material_type = 'RAW',
    weight = 0.5000,
    weight_uom = 'KG',
    base_uom_id = (SELECT id FROM mm_uom WHERE code = 'L'),
    material_group_id = (SELECT id FROM mm_material_groups WHERE code = 'OILS')
WHERE material_number = 'MAT00000002';

UPDATE mm_materials SET
    name = 'Premium Gift Box (Large)',
    description = 'Luxury presentation box with magnetic closure, 30x20x10cm',
    material_type = 'RAW',
    weight = 0.3500,
    weight_uom = 'KG',
    base_uom_id = (SELECT id FROM mm_uom WHERE code = 'EA'),
    material_group_id = (SELECT id FROM mm_material_groups WHERE code = 'PACKAGING')
WHERE material_number = 'MAT00000003';

UPDATE mm_materials SET
    name = 'Silk Ribbon Gold 25mm',
    description = 'Gold satin ribbon for gift wrapping, 25mm width, sold by meter',
    material_type = 'RAW',
    weight = 0.0100,
    weight_uom = 'KG',
    base_uom_id = (SELECT id FROM mm_uom WHERE code = 'M'),
    material_group_id = (SELECT id FROM mm_material_groups WHERE code = 'PACKAGING')
WHERE material_number = 'MAT00000004';

UPDATE mm_materials SET
    name = 'Artisan Olive Oil Gift Set',
    description = 'Gift set containing 3 bottles of premium olive oil with tasting notes',
    material_type = 'FERT',
    weight = 2.5000,
    weight_uom = 'KG',
    base_uom_id = (SELECT id FROM mm_uom WHERE code = 'SET'),
    material_group_id = (SELECT id FROM mm_material_groups WHERE code = 'OILS')
WHERE material_number = 'MAT00000005';

-- ============================================================================
-- PHASE 4: INSERT NEW MATERIALS
-- ============================================================================

INSERT INTO mm_materials (material_number, name, description, material_type, weight, weight_uom, base_uom_id, material_group_id) VALUES
    ('MAT00000102', 'Bread Flour 25kg', 'High-gluten bread flour for artisan bread making', 'RAW', 25.0000, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'KG'), (SELECT id FROM mm_material_groups WHERE code = 'FLOUR')),
    ('MAT00000103', 'Unsalted Butter 5kg Block', 'European-style unsalted butter, 82% fat content', 'RAW', 5.0000, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'KG'), (SELECT id FROM mm_material_groups WHERE code = 'DAIRY')),
    ('MAT00000104', 'Fresh Cream 10L', 'Heavy whipping cream, 35% fat content', 'RAW', 10.0000, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'L'), (SELECT id FROM mm_material_groups WHERE code = 'DAIRY')),
    ('MAT00000105', 'Atlantic Salmon Fillet 1kg', 'Fresh Norwegian salmon fillet, skinless, boneless', 'RAW', 1.0000, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'KG'), (SELECT id FROM mm_material_groups WHERE code = 'SEAFOOD')),
    ('MAT00000106', 'Tiger Prawns 1kg', 'Frozen tiger prawns, size 16/20, head-on', 'RAW', 1.0000, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'KG'), (SELECT id FROM mm_material_groups WHERE code = 'SEAFOOD')),
    ('MAT00000107', 'Organic Eggs (30 pack)', 'Free-range organic eggs, size L', 'RAW', 1.8000, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'BOX'), (SELECT id FROM mm_material_groups WHERE code = 'DAIRY')),
    ('MAT00000108', 'Granulated Sugar 25kg', 'Refined white granulated sugar', 'RAW', 25.0000, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'KG'), (SELECT id FROM mm_material_groups WHERE code = 'FLOUR')),
    ('MAT00000109', 'Vanilla Extract 500ml', 'Pure Madagascar vanilla extract', 'RAW', 0.5000, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'L'), (SELECT id FROM mm_material_groups WHERE code = 'SPICES')),
    ('MAT00000110', 'Dark Chocolate 70% 5kg', 'Belgian couverture dark chocolate, 70% cocoa', 'RAW', 5.0000, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'KG'), (SELECT id FROM mm_material_groups WHERE code = 'SPICES')),
    ('MAT00000111', 'Fresh Basil Bundle', 'Locally grown fresh sweet basil, 100g bunch', 'RAW', 0.1000, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'EA'), (SELECT id FROM mm_material_groups WHERE code = 'PRODUCE')),
    ('MAT00000112', 'Roma Tomatoes 5kg', 'Vine-ripened Roma tomatoes for sauce making', 'RAW', 5.0000, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'KG'), (SELECT id FROM mm_material_groups WHERE code = 'PRODUCE')),
    ('MAT00000113', 'Mozzarella Cheese 1kg', 'Fresh Italian mozzarella di bufala', 'RAW', 1.0000, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'KG'), (SELECT id FROM mm_material_groups WHERE code = 'DAIRY')),
    ('MAT00000114', 'Croissant (Finished)', 'Freshly baked butter croissant', 'FERT', 0.0800, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'EA'), (SELECT id FROM mm_material_groups WHERE code = 'BAKERY')),
    ('MAT00000115', 'Margherita Pizza (Finished)', 'Traditional Margherita pizza, 12 inch', 'FERT', 0.4500, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'EA'), (SELECT id FROM mm_material_groups WHERE code = 'BAKERY')),
    ('MAT00000116', 'Chocolate Cake (Finished)', 'Triple-layer dark chocolate cake, 8 inch', 'FERT', 1.2000, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'EA'), (SELECT id FROM mm_material_groups WHERE code = 'BAKERY')),
    ('MAT00000117', 'Salmon Sashimi Platter', 'Premium salmon sashimi platter, 20 pieces', 'FERT', 0.5000, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'SET'), (SELECT id FROM mm_material_groups WHERE code = 'SEAFOOD')),
    ('MAT00000118', 'Seafood Pasta (Finished)', 'Linguine with tiger prawns and clam sauce', 'FERT', 0.5500, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'EA'), (SELECT id FROM mm_material_groups WHERE code = 'BAKERY')),
    ('MAT00000119', 'Green Tea Matcha 500g', 'Ceremonial grade Japanese matcha powder', 'RAW', 0.5000, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'KG'), (SELECT id FROM mm_material_groups WHERE code = 'BEVERAGE')),
    ('MAT00000120', 'Jasmine Rice 25kg', 'Thai premium jasmine rice, long grain', 'RAW', 25.0000, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'KG'), (SELECT id FROM mm_material_groups WHERE code = 'FLOUR')),
    ('MAT00000121', 'Takeaway Box (Medium)', 'Eco-friendly kraft paper takeaway box', 'RAW', 0.0300, 'KG',
     (SELECT id FROM mm_uom WHERE code = 'EA'), (SELECT id FROM mm_material_groups WHERE code = 'PACKAGING'))
ON CONFLICT (material_number) DO NOTHING;

-- ============================================================================
-- PHASE 5: VENDORS
-- ============================================================================

-- Update existing vendor
UPDATE mm_vendors SET
    name = 'Tartufi Milano S.r.l.',
    contact_person = 'Marco Rossi',
    email = 'marco@tartufimilano.it',
    phone = '+39-02-8765-4321',
    address = 'Via Torino 42, 20123 Milano, Italy',
    payment_terms = 45
WHERE vendor_number = 'VND00000001';

-- Insert new vendors
INSERT INTO mm_vendors (vendor_number, name, contact_person, email, phone, address, payment_terms) VALUES
    ('VND00000017', 'Fresh Farms Co.', 'Chen Wei-Lin', 'weiling@freshfarms.com.tw', '+886-4-2345-6789', 'No. 88, Nongye Rd, Taichung, Taiwan', 30),
    ('VND00000018', 'Pacific Seafood Ltd.', 'Tanaka Yuki', 'yuki@pacificseafood.co.jp', '+81-3-9876-5432', '2-1-5 Tsukiji, Chuo-ku, Tokyo, Japan', 15),
    ('VND00000019', 'Golden Grain Mills', 'Lin Mei-Hua', 'meihua@goldengrain.com.tw', '+886-3-5678-1234', 'No. 56, Gongye Rd, Taoyuan, Taiwan', 30),
    ('VND00000020', 'Tropical Imports Inc.', 'Raj Patel', 'raj@tropicalimports.com', '+65-6543-2109', '10 Changi Business Park, Singapore 486030', 45),
    ('VND00000021', 'Local Dairy Farm', 'Wang Da-Ming', 'daming@localdairy.com.tw', '+886-2-8765-4321', 'No. 12, Mudan Rd, Yilan, Taiwan', 14)
ON CONFLICT (vendor_number) DO NOTHING;

-- ============================================================================
-- PHASE 6: CUSTOMERS
-- ============================================================================

-- Update existing Gourmet Restaurant Tokyo
UPDATE sd_customers SET
    contact_person = 'Sato Hiroshi',
    email = 'hiroshi@gourmet-tokyo.jp',
    phone = '+81-3-1234-5678',
    address = '3-5-7 Ginza, Chuo-ku, Tokyo 104-0061, Japan',
    payment_terms = 30,
    credit_limit = 5000000.0000
WHERE customer_number = 'SO00000001';

-- Insert new customers
INSERT INTO sd_customers (customer_number, name, contact_person, email, phone, address, payment_terms, credit_limit) VALUES
    ('CUST00000013', 'Cafe Deluxe Taipei', 'Lin Yu-Ting', 'yuting@cafedeluxe.com.tw', '+886-2-2771-8899', 'No. 15, Section 4, Zhongxiao E Rd, Da-an District, Taipei', 30, 2000000.0000),
    ('CUST00000014', 'Hotel Grand Palace', 'Chang Hsiao-Wen', 'hswen@grandpalace.com.tw', '+886-2-2550-1234', 'No. 100, Zhongshan N Rd, Zhongshan District, Taipei', 45, 10000000.0000),
    ('CUST00000015', 'Bakery Plus Chain', 'Wu Chih-Ming', 'chihming@bakeryplus.com.tw', '+886-2-2345-6780', 'No. 28, Dunhua S Rd, Songshan District, Taipei', 30, 3000000.0000),
    ('CUST00000016', 'Fresh Market Superstore', 'Huang Li-Chen', 'lichen@freshmarket.com.tw', '+886-4-2233-4455', 'No. 200, Wenxin Rd, Xitun District, Taichung', 60, 8000000.0000)
ON CONFLICT (customer_number) DO NOTHING;

-- ============================================================================
-- PHASE 7: WAREHOUSES
-- ============================================================================

INSERT INTO wm_warehouses (code, name, address, warehouse_type) VALUES
    ('WH02', 'Cold Storage Facility', 'No. 50, Gangqu Rd, Nangang District, Taipei', 'COLD'),
    ('WH03', 'Dry Goods Warehouse', 'No. 88, Zhongzheng Rd, Xinzhuang District, New Taipei City', 'STANDARD'),
    ('WH04', 'Production Staging Area', 'No. 10, Factory Rd, Tucheng District, New Taipei City', 'PRODUCTION')
ON CONFLICT (code) DO NOTHING;

-- Storage bins for Main Warehouse
INSERT INTO wm_storage_bins (warehouse_id, bin_code, zone, aisle, rack, level, max_weight) VALUES
    ((SELECT id FROM wm_warehouses WHERE code = 'WH01'), 'A-01-01', 'A', '01', '01', '1', 500.0000),
    ((SELECT id FROM wm_warehouses WHERE code = 'WH01'), 'A-01-02', 'A', '01', '01', '2', 500.0000),
    ((SELECT id FROM wm_warehouses WHERE code = 'WH01'), 'A-02-01', 'A', '02', '01', '1', 500.0000),
    ((SELECT id FROM wm_warehouses WHERE code = 'WH01'), 'B-01-01', 'B', '01', '01', '1', 1000.0000),
    ((SELECT id FROM wm_warehouses WHERE code = 'WH01'), 'B-01-02', 'B', '01', '01', '2', 1000.0000),
    ((SELECT id FROM wm_warehouses WHERE code = 'WH02'), 'C-01-01', 'C', '01', '01', '1', 300.0000),
    ((SELECT id FROM wm_warehouses WHERE code = 'WH02'), 'C-01-02', 'C', '01', '01', '2', 300.0000),
    ((SELECT id FROM wm_warehouses WHERE code = 'WH03'), 'D-01-01', 'D', '01', '01', '1', 800.0000),
    ((SELECT id FROM wm_warehouses WHERE code = 'WH03'), 'D-02-01', 'D', '02', '01', '1', 800.0000)
ON CONFLICT (warehouse_id, bin_code) DO NOTHING;

-- ============================================================================
-- PHASE 8: HR - DEPARTMENTS, POSITIONS, EMPLOYEES
-- ============================================================================

INSERT INTO hr_departments (code, name) VALUES
    ('DEPT-MGMT', 'Executive Management'),
    ('DEPT-FIN', 'Finance & Accounting'),
    ('DEPT-PROD', 'Production'),
    ('DEPT-QC', 'Quality Control'),
    ('DEPT-SALES', 'Sales & Marketing'),
    ('DEPT-PROC', 'Procurement'),
    ('DEPT-WH', 'Warehouse & Logistics'),
    ('DEPT-HR', 'Human Resources'),
    ('DEPT-RD', 'Research & Development')
ON CONFLICT (code) DO NOTHING;

INSERT INTO hr_positions (code, title, department_id, grade) VALUES
    ('POS-GM', 'General Manager', (SELECT id FROM hr_departments WHERE code = 'DEPT-MGMT'), 'S1'),
    ('POS-CFO', 'Chief Financial Officer', (SELECT id FROM hr_departments WHERE code = 'DEPT-FIN'), 'S1'),
    ('POS-ACCT', 'Senior Accountant', (SELECT id FROM hr_departments WHERE code = 'DEPT-FIN'), 'M2'),
    ('POS-CHEF', 'Head Chef', (SELECT id FROM hr_departments WHERE code = 'DEPT-PROD'), 'M1'),
    ('POS-COOK', 'Line Cook', (SELECT id FROM hr_departments WHERE code = 'DEPT-PROD'), 'J2'),
    ('POS-BAKER', 'Master Baker', (SELECT id FROM hr_departments WHERE code = 'DEPT-PROD'), 'M1'),
    ('POS-QCMGR', 'QC Manager', (SELECT id FROM hr_departments WHERE code = 'DEPT-QC'), 'M1'),
    ('POS-QCINSP', 'QC Inspector', (SELECT id FROM hr_departments WHERE code = 'DEPT-QC'), 'J2'),
    ('POS-SLMGR', 'Sales Manager', (SELECT id FROM hr_departments WHERE code = 'DEPT-SALES'), 'M1'),
    ('POS-SLREP', 'Sales Representative', (SELECT id FROM hr_departments WHERE code = 'DEPT-SALES'), 'J2'),
    ('POS-BUYER', 'Senior Buyer', (SELECT id FROM hr_departments WHERE code = 'DEPT-PROC'), 'M2'),
    ('POS-WHMGR', 'Warehouse Manager', (SELECT id FROM hr_departments WHERE code = 'DEPT-WH'), 'M1'),
    ('POS-WHOP', 'Warehouse Operator', (SELECT id FROM hr_departments WHERE code = 'DEPT-WH'), 'J1'),
    ('POS-HRMGR', 'HR Manager', (SELECT id FROM hr_departments WHERE code = 'DEPT-HR'), 'M1'),
    ('POS-RDCHEF', 'R&D Chef', (SELECT id FROM hr_departments WHERE code = 'DEPT-RD'), 'M2')
ON CONFLICT (code) DO NOTHING;

INSERT INTO hr_employees (employee_number, first_name, last_name, email, phone, department_id, position_id, hire_date, status) VALUES
    ('EMP00000015', 'Wei-Lin', 'Chen', 'weilin.chen@tastebyte.com', '+886-2-1001', (SELECT id FROM hr_departments WHERE code = 'DEPT-MGMT'), (SELECT id FROM hr_positions WHERE code = 'POS-GM'), '2020-03-15', 'ACTIVE'),
    ('EMP00000016', 'Mei-Hua', 'Lin', 'meihua.lin@tastebyte.com', '+886-2-1002', (SELECT id FROM hr_departments WHERE code = 'DEPT-FIN'), (SELECT id FROM hr_positions WHERE code = 'POS-CFO'), '2020-06-01', 'ACTIVE'),
    ('EMP00000017', 'Hsiao-Wen', 'Chang', 'hsiaowen.chang@tastebyte.com', '+886-2-1003', (SELECT id FROM hr_departments WHERE code = 'DEPT-FIN'), (SELECT id FROM hr_positions WHERE code = 'POS-ACCT'), '2021-02-15', 'ACTIVE'),
    ('EMP00000018', 'Hiroshi', 'Tanaka', 'hiroshi.tanaka@tastebyte.com', '+886-2-1004', (SELECT id FROM hr_departments WHERE code = 'DEPT-PROD'), (SELECT id FROM hr_positions WHERE code = 'POS-CHEF'), '2020-09-01', 'ACTIVE'),
    ('EMP00000019', 'Yu-Ting', 'Wu', 'yuting.wu@tastebyte.com', '+886-2-1005', (SELECT id FROM hr_departments WHERE code = 'DEPT-PROD'), (SELECT id FROM hr_positions WHERE code = 'POS-COOK'), '2022-01-10', 'ACTIVE'),
    ('EMP00000020', 'Da-Ming', 'Wang', 'daming.wang@tastebyte.com', '+886-2-1006', (SELECT id FROM hr_departments WHERE code = 'DEPT-PROD'), (SELECT id FROM hr_positions WHERE code = 'POS-BAKER'), '2021-04-01', 'ACTIVE'),
    ('EMP00000021', 'Li-Chen', 'Huang', 'lichen.huang@tastebyte.com', '+886-2-1007', (SELECT id FROM hr_departments WHERE code = 'DEPT-QC'), (SELECT id FROM hr_positions WHERE code = 'POS-QCMGR'), '2021-03-15', 'ACTIVE'),
    ('EMP00000022', 'Chih-Ming', 'Liu', 'chihming.liu@tastebyte.com', '+886-2-1008', (SELECT id FROM hr_departments WHERE code = 'DEPT-QC'), (SELECT id FROM hr_positions WHERE code = 'POS-QCINSP'), '2022-06-01', 'ACTIVE'),
    ('EMP00000023', 'Shu-Fen', 'Tsai', 'shufen.tsai@tastebyte.com', '+886-2-1009', (SELECT id FROM hr_departments WHERE code = 'DEPT-SALES'), (SELECT id FROM hr_positions WHERE code = 'POS-SLMGR'), '2021-01-15', 'ACTIVE'),
    ('EMP00000024', 'Jia-Wei', 'Yang', 'jiawei.yang@tastebyte.com', '+886-2-1010', (SELECT id FROM hr_departments WHERE code = 'DEPT-SALES'), (SELECT id FROM hr_positions WHERE code = 'POS-SLREP'), '2023-03-01', 'ACTIVE'),
    ('EMP00000025', 'Min-Jie', 'Hsu', 'minjie.hsu@tastebyte.com', '+886-2-1011', (SELECT id FROM hr_departments WHERE code = 'DEPT-PROC'), (SELECT id FROM hr_positions WHERE code = 'POS-BUYER'), '2021-07-01', 'ACTIVE'),
    ('EMP00000026', 'Tzu-Hsuan', 'Chou', 'tzuhsuan.chou@tastebyte.com', '+886-2-1012', (SELECT id FROM hr_departments WHERE code = 'DEPT-WH'), (SELECT id FROM hr_positions WHERE code = 'POS-WHMGR'), '2020-11-15', 'ACTIVE'),
    ('EMP00000027', 'Kuan-Yu', 'Cheng', 'kuanyu.cheng@tastebyte.com', '+886-2-1013', (SELECT id FROM hr_departments WHERE code = 'DEPT-WH'), (SELECT id FROM hr_positions WHERE code = 'POS-WHOP'), '2023-01-10', 'ACTIVE'),
    ('EMP00000028', 'Pei-Yu', 'Sun', 'peiyu.sun@tastebyte.com', '+886-2-1014', (SELECT id FROM hr_departments WHERE code = 'DEPT-HR'), (SELECT id FROM hr_positions WHERE code = 'POS-HRMGR'), '2021-05-01', 'ACTIVE'),
    ('EMP00000029', 'Yen-Ju', 'Ho', 'yenju.ho@tastebyte.com', '+886-2-1015', (SELECT id FROM hr_departments WHERE code = 'DEPT-RD'), (SELECT id FROM hr_positions WHERE code = 'POS-RDCHEF'), '2022-08-15', 'ACTIVE'),
    ('EMP00000030', 'Shin-Yi', 'Pan', 'shinyi.pan@tastebyte.com', '+886-2-1016', (SELECT id FROM hr_departments WHERE code = 'DEPT-PROD'), (SELECT id FROM hr_positions WHERE code = 'POS-COOK'), '2023-06-01', 'ACTIVE')
ON CONFLICT (employee_number) DO NOTHING;

-- ============================================================================
-- PHASE 9: COST CENTERS & PROFIT CENTERS
-- ============================================================================

INSERT INTO co_cost_centers (code, name, description, valid_from) VALUES
    ('CC-SALES', 'Sales & Marketing', 'Sales and marketing department cost center', '2025-01-01'),
    ('CC-RD', 'Research & Development', 'R&D and product development', '2025-01-01'),
    ('CC-LOGISTICS', 'Logistics & Distribution', 'Shipping, delivery and logistics', '2025-01-01'),
    ('CC-QC', 'Quality Control', 'Quality assurance and inspection', '2025-01-01'),
    ('CC-ADMIN', 'Administration', 'General administration and office', '2025-01-01')
ON CONFLICT (code) DO NOTHING;

-- Update existing cost centers with valid_from
UPDATE co_cost_centers SET valid_from = '2025-01-01', description = 'General overhead and shared services' WHERE code = 'CC-GENERAL' AND valid_from IS NULL;
UPDATE co_cost_centers SET valid_from = '2025-01-01', description = 'Raw material and vendor procurement' WHERE code = 'CC-PROCUREMENT' AND valid_from IS NULL;
UPDATE co_cost_centers SET valid_from = '2025-01-01', description = 'Food production and manufacturing' WHERE code = 'CC-PRODUCTION' AND valid_from IS NULL;
UPDATE co_cost_centers SET valid_from = '2025-01-01', description = 'Human resources and talent management' WHERE code = 'CC-HR' AND valid_from IS NULL;

INSERT INTO co_profit_centers (code, name, description) VALUES
    ('PC-BAKERY', 'Bakery Products', 'Profit center for bread, pastry and cake products'),
    ('PC-SEAFOOD', 'Seafood Products', 'Profit center for seafood dishes and platters'),
    ('PC-CATERING', 'Catering Services', 'Profit center for B2B catering and events'),
    ('PC-RETAIL', 'Retail Sales', 'Profit center for direct-to-consumer retail'),
    ('PC-WHOLESALE', 'Wholesale Distribution', 'Profit center for wholesale and bulk orders')
ON CONFLICT (code) DO NOTHING;

-- ============================================================================
-- PHASE 10: PURCHASE ORDERS (update existing + add new)
-- ============================================================================

-- Update existing PO00000001
UPDATE mm_purchase_orders SET
    delivery_date = '2026-01-20',
    notes = 'Monthly olive oil order - premium Italian import'
WHERE po_number = 'PO00000001';

-- Delete remaining test POs that we could not clean by vendor
DELETE FROM mm_purchase_order_items WHERE purchase_order_id IN (
    SELECT id FROM mm_purchase_orders WHERE po_number != 'PO00000001'
);
DELETE FROM mm_purchase_orders WHERE po_number != 'PO00000001';

-- Insert new realistic POs
INSERT INTO mm_purchase_orders (po_number, vendor_id, order_date, delivery_date, status, total_amount, currency, notes) VALUES
    ('PO00000009', (SELECT id FROM mm_vendors WHERE vendor_number = 'VND00000019'), '2026-01-10', '2026-01-17', 'RECEIVED', 37500.0000, 'TWD', 'Monthly flour and grain supplies'),
    ('PO00000010', (SELECT id FROM mm_vendors WHERE vendor_number = 'VND00000021'), '2026-01-12', '2026-01-14', 'RECEIVED', 18500.0000, 'TWD', 'Weekly dairy products delivery'),
    ('PO00000011', (SELECT id FROM mm_vendors WHERE vendor_number = 'VND00000018'), '2026-01-25', '2026-01-28', 'RECEIVED', 42000.0000, 'TWD', 'Bi-weekly fresh seafood order'),
    ('PO00000012', (SELECT id FROM mm_vendors WHERE vendor_number = 'VND00000017'), '2026-02-01', '2026-02-04', 'RECEIVED', 8200.0000, 'TWD', 'Fresh produce - tomatoes, basil, herbs'),
    ('PO00000013', (SELECT id FROM mm_vendors WHERE vendor_number = 'VND00000020'), '2026-02-10', '2026-02-20', 'RELEASED', 28000.0000, 'TWD', 'Imported spices and specialty ingredients'),
    ('PO00000014', (SELECT id FROM mm_vendors WHERE vendor_number = 'VND00000019'), '2026-02-15', '2026-02-22', 'RELEASED', 43750.0000, 'TWD', 'Monthly flour restock + jasmine rice'),
    ('PO00000015', (SELECT id FROM mm_vendors WHERE vendor_number = 'VND00000021'), '2026-03-01', '2026-03-03', 'RELEASED', 22800.0000, 'TWD', 'Weekly dairy - butter, cream, eggs'),
    ('PO00000016', (SELECT id FROM mm_vendors WHERE vendor_number = 'VND00000018'), '2026-03-10', '2026-03-13', 'DRAFT', 35000.0000, 'TWD', 'Seafood order for spring menu launch'),
    ('PO00000017', (SELECT id FROM mm_vendors WHERE vendor_number = 'VND00000017'), '2026-03-15', '2026-03-18', 'DRAFT', 12500.0000, 'TWD', 'Fresh produce for weekend prep')
ON CONFLICT (po_number) DO NOTHING;

-- PO Line Items
INSERT INTO mm_purchase_order_items (purchase_order_id, line_number, material_id, quantity, unit_price, total_price, uom_id) VALUES
    -- PO00000009: Flour & Grains (Golden Grain Mills)
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000009'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000001'), 10.0000, 850.0000, 8500.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000009'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000102'), 8.0000, 950.0000, 7600.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000009'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000108'), 6.0000, 680.0000, 4080.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000009'), 4, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000120'), 10.0000, 1732.0000, 17320.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    -- PO00000010: Dairy (Local Dairy Farm)
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000010'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000103'), 6.0000, 1200.0000, 7200.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000010'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000104'), 5.0000, 1400.0000, 7000.0000, (SELECT id FROM mm_uom WHERE code = 'L')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000010'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000107'), 20.0000, 215.0000, 4300.0000, (SELECT id FROM mm_uom WHERE code = 'BOX')),
    -- PO00000011: Seafood (Pacific Seafood)
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000011'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000105'), 20.0000, 1200.0000, 24000.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000011'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000106'), 15.0000, 1200.0000, 18000.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    -- PO00000012: Produce (Fresh Farms)
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000012'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000112'), 30.0000, 180.0000, 5400.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000012'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000111'), 40.0000, 35.0000, 1400.0000, (SELECT id FROM mm_uom WHERE code = 'EA')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000012'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000113'), 10.0000, 140.0000, 1400.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    -- PO00000013: Spices & Specialty (Tropical Imports)
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000013'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000109'), 8.0000, 1800.0000, 14400.0000, (SELECT id FROM mm_uom WHERE code = 'L')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000013'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000110'), 4.0000, 2200.0000, 8800.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000013'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000119'), 4.0000, 1200.0000, 4800.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    -- PO00000014: Flour restock
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000014'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000001'), 15.0000, 850.0000, 12750.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000014'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000102'), 10.0000, 950.0000, 9500.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000014'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000120'), 12.0000, 1500.0000, 18000.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000014'), 4, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000108'), 5.0000, 700.0000, 3500.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    -- PO00000015: Dairy weekly
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000015'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000103'), 8.0000, 1200.0000, 9600.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000015'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000104'), 6.0000, 1400.0000, 8400.0000, (SELECT id FROM mm_uom WHERE code = 'L')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000015'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000107'), 30.0000, 160.0000, 4800.0000, (SELECT id FROM mm_uom WHERE code = 'BOX')),
    -- PO00000016: Seafood spring menu
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000016'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000105'), 15.0000, 1200.0000, 18000.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000016'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000106'), 12.0000, 1200.0000, 14400.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    -- PO00000017: Fresh produce
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000017'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000112'), 25.0000, 180.0000, 4500.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000017'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000111'), 60.0000, 35.0000, 2100.0000, (SELECT id FROM mm_uom WHERE code = 'EA')),
    ((SELECT id FROM mm_purchase_orders WHERE po_number = 'PO00000017'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000113'), 15.0000, 140.0000, 2100.0000, (SELECT id FROM mm_uom WHERE code = 'KG'))
ON CONFLICT DO NOTHING;

-- ============================================================================
-- PHASE 11: SALES ORDERS (clean test + add new)
-- ============================================================================

-- Delete test SOs that remain
DELETE FROM sd_delivery_items WHERE delivery_id IN (
    SELECT id FROM sd_deliveries WHERE sales_order_id IN (
        SELECT id FROM sd_sales_orders WHERE order_number NOT IN ('SO00000002')
    )
);
DELETE FROM sd_invoices WHERE sales_order_id IN (
    SELECT id FROM sd_sales_orders WHERE order_number NOT IN ('SO00000002')
);
DELETE FROM sd_deliveries WHERE sales_order_id IN (
    SELECT id FROM sd_sales_orders WHERE order_number NOT IN ('SO00000002')
);
DELETE FROM sd_sales_order_items WHERE sales_order_id IN (
    SELECT id FROM sd_sales_orders WHERE order_number NOT IN ('SO00000002')
);
DELETE FROM sd_sales_orders WHERE order_number NOT IN ('SO00000002');

-- Update existing SO00000002
UPDATE sd_sales_orders SET
    customer_id = (SELECT id FROM sd_customers WHERE customer_number = 'SO00000001'),
    order_date = '2026-01-15',
    requested_delivery_date = '2026-01-20',
    total_amount = 45000.0000,
    notes = 'Monthly premium ingredient order for Gourmet Restaurant Tokyo'
WHERE order_number = 'SO00000002';

-- Update SO items for SO00000002
UPDATE sd_sales_order_items SET
    quantity = 50.0000,
    unit_price = 450.0000,
    total_price = 22500.0000,
    delivered_quantity = 50.0000
WHERE sales_order_id = (SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000002')
  AND line_number = 1;

-- New Sales Orders
INSERT INTO sd_sales_orders (order_number, customer_id, order_date, requested_delivery_date, status, total_amount, currency, notes) VALUES
    ('SO00000011', (SELECT id FROM sd_customers WHERE customer_number = 'CUST00000013'), '2026-01-20', '2026-01-25', 'DELIVERED', 32400.0000, 'TWD', 'Cafe Deluxe weekly bakery supplies'),
    ('SO00000012', (SELECT id FROM sd_customers WHERE customer_number = 'CUST00000014'), '2026-02-01', '2026-02-05', 'DELIVERED', 128000.0000, 'TWD', 'Hotel Grand Palace - February banquet menu'),
    ('SO00000013', (SELECT id FROM sd_customers WHERE customer_number = 'CUST00000015'), '2026-02-10', '2026-02-14', 'CONFIRMED', 56000.0000, 'TWD', 'Bakery Plus Chain - Valentine special order'),
    ('SO00000014', (SELECT id FROM sd_customers WHERE customer_number = 'CUST00000016'), '2026-02-20', '2026-02-25', 'CONFIRMED', 89500.0000, 'TWD', 'Fresh Market Superstore - weekly retail supply'),
    ('SO00000015', (SELECT id FROM sd_customers WHERE customer_number = 'SO00000001'), '2026-03-01', '2026-03-05', 'CONFIRMED', 67200.0000, 'TWD', 'Gourmet Restaurant Tokyo - spring menu items'),
    ('SO00000016', (SELECT id FROM sd_customers WHERE customer_number = 'CUST00000014'), '2026-03-05', '2026-03-10', 'CONFIRMED', 95000.0000, 'TWD', 'Hotel Grand Palace - spring conference catering'),
    ('SO00000017', (SELECT id FROM sd_customers WHERE customer_number = 'CUST00000013'), '2026-03-10', '2026-03-14', 'DRAFT', 28800.0000, 'TWD', 'Cafe Deluxe - matcha dessert collection'),
    ('SO00000018', (SELECT id FROM sd_customers WHERE customer_number = 'CUST00000016'), '2026-03-15', '2026-03-20', 'DRAFT', 42000.0000, 'TWD', 'Fresh Market - Easter holiday special')
ON CONFLICT (order_number) DO NOTHING;

-- SO Line Items
INSERT INTO sd_sales_order_items (sales_order_id, line_number, material_id, quantity, unit_price, total_price, uom_id, delivered_quantity) VALUES
    -- SO00000011: Cafe Deluxe bakery
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000011'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000114'), 200.0000, 85.0000, 17000.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 200.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000011'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000116'), 10.0000, 1200.0000, 12000.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 10.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000011'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000005'), 8.0000, 425.0000, 3400.0000, (SELECT id FROM mm_uom WHERE code = 'SET'), 8.0000),
    -- SO00000012: Hotel Grand Palace banquet
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000012'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000117'), 80.0000, 650.0000, 52000.0000, (SELECT id FROM mm_uom WHERE code = 'SET'), 80.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000012'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000118'), 120.0000, 380.0000, 45600.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 120.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000012'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000116'), 20.0000, 1200.0000, 24000.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 20.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000012'), 4, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000005'), 15.0000, 425.0000, 6375.0000, (SELECT id FROM mm_uom WHERE code = 'SET'), 15.0000),
    -- SO00000013: Bakery Plus Valentine
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000013'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000116'), 30.0000, 1200.0000, 36000.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 0.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000013'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000114'), 200.0000, 85.0000, 17000.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 0.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000013'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000005'), 5.0000, 600.0000, 3000.0000, (SELECT id FROM mm_uom WHERE code = 'SET'), 0.0000),
    -- SO00000014: Fresh Market retail
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000014'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000114'), 500.0000, 75.0000, 37500.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 0.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000014'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000115'), 100.0000, 280.0000, 28000.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 0.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000014'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000116'), 20.0000, 1200.0000, 24000.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 0.0000),
    -- SO00000015: Gourmet Tokyo spring
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000015'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000117'), 60.0000, 650.0000, 39000.0000, (SELECT id FROM mm_uom WHERE code = 'SET'), 0.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000015'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000118'), 50.0000, 380.0000, 19000.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 0.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000015'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000005'), 12.0000, 425.0000, 5100.0000, (SELECT id FROM mm_uom WHERE code = 'SET'), 0.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000015'), 4, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000002'), 20.0000, 205.0000, 4100.0000, (SELECT id FROM mm_uom WHERE code = 'L'), 0.0000),
    -- SO00000016: Hotel spring conference
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000016'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000114'), 300.0000, 85.0000, 25500.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 0.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000016'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000117'), 50.0000, 650.0000, 32500.0000, (SELECT id FROM mm_uom WHERE code = 'SET'), 0.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000016'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000118'), 60.0000, 380.0000, 22800.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 0.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000016'), 4, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000116'), 12.0000, 1200.0000, 14400.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 0.0000),
    -- SO00000017: Cafe Deluxe matcha
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000017'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000116'), 15.0000, 1200.0000, 18000.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 0.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000017'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000114'), 100.0000, 85.0000, 8500.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 0.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000017'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000005'), 4.0000, 575.0000, 2300.0000, (SELECT id FROM mm_uom WHERE code = 'SET'), 0.0000),
    -- SO00000018: Fresh Market Easter
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000018'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000114'), 400.0000, 75.0000, 30000.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 0.0000),
    ((SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000018'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000116'), 10.0000, 1200.0000, 12000.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 0.0000)
ON CONFLICT DO NOTHING;

-- Deliveries for delivered SOs
INSERT INTO sd_deliveries (delivery_number, sales_order_id, delivery_date, status) VALUES
    ('DO00000002', (SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000011'), '2026-01-24', 'DELIVERED'),
    ('DO00000003', (SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000012'), '2026-02-04', 'DELIVERED')
ON CONFLICT (delivery_number) DO NOTHING;

-- Invoices for delivered SOs
INSERT INTO sd_invoices (invoice_number, sales_order_id, customer_id, invoice_date, due_date, total_amount, status) VALUES
    ('INV00000005', (SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000002'), (SELECT id FROM sd_customers WHERE customer_number = 'SO00000001'), '2026-01-20', '2026-02-19', 45000.0000, 'PAID'),
    ('INV00000006', (SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000011'), (SELECT id FROM sd_customers WHERE customer_number = 'CUST00000013'), '2026-01-25', '2026-02-24', 32400.0000, 'PAID'),
    ('INV00000007', (SELECT id FROM sd_sales_orders WHERE order_number = 'SO00000012'), (SELECT id FROM sd_customers WHERE customer_number = 'CUST00000014'), '2026-02-05', '2026-03-22', 128000.0000, 'POSTED')
ON CONFLICT (invoice_number) DO NOTHING;

-- ============================================================================
-- PHASE 12: BOMs (clean test + add realistic)
-- ============================================================================

-- Clean all existing BOMs (already cleaned test material refs, but ensure clean slate)
DELETE FROM pp_bom_items WHERE bom_id IN (SELECT id FROM pp_boms WHERE bom_number = 'BOM00000001');
DELETE FROM pp_production_orders WHERE bom_id IN (SELECT id FROM pp_boms WHERE bom_number = 'BOM00000001');

-- Update existing BOM00000001 for Artisan Olive Oil Gift Set
UPDATE pp_boms SET
    material_id = (SELECT id FROM mm_materials WHERE material_number = 'MAT00000005'),
    name = 'Artisan Olive Oil Gift Set',
    valid_from = '2025-06-01',
    status = 'ACTIVE'
WHERE bom_number = 'BOM00000001';

INSERT INTO pp_bom_items (bom_id, line_number, component_material_id, quantity, uom_id, scrap_percentage) VALUES
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000001'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000002'), 3.0000, (SELECT id FROM mm_uom WHERE code = 'L'), 2.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000001'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000003'), 1.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 1.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000001'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000004'), 2.0000, (SELECT id FROM mm_uom WHERE code = 'M'), 5.00)
ON CONFLICT DO NOTHING;

-- New BOMs
INSERT INTO pp_boms (bom_number, material_id, name, version, status, valid_from) VALUES
    ('BOM00000024', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000114'), 'Butter Croissant', 1, 'ACTIVE', '2025-09-01'),
    ('BOM00000025', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000115'), 'Margherita Pizza 12"', 1, 'ACTIVE', '2025-09-01'),
    ('BOM00000026', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000116'), 'Triple Chocolate Cake', 1, 'ACTIVE', '2025-10-01'),
    ('BOM00000027', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000117'), 'Salmon Sashimi Platter', 1, 'ACTIVE', '2025-11-01'),
    ('BOM00000028', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000118'), 'Seafood Linguine', 1, 'ACTIVE', '2025-11-01')
ON CONFLICT (bom_number) DO NOTHING;

-- BOM Items for new BOMs
INSERT INTO pp_bom_items (bom_id, line_number, component_material_id, quantity, uom_id, scrap_percentage) VALUES
    -- Croissant BOM
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000024'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000001'), 0.0800, (SELECT id FROM mm_uom WHERE code = 'KG'), 3.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000024'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000103'), 0.0500, (SELECT id FROM mm_uom WHERE code = 'KG'), 2.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000024'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000107'), 0.0333, (SELECT id FROM mm_uom WHERE code = 'BOX'), 1.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000024'), 4, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000108'), 0.0100, (SELECT id FROM mm_uom WHERE code = 'KG'), 0.00),
    -- Pizza BOM
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000025'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000001'), 0.2500, (SELECT id FROM mm_uom WHERE code = 'KG'), 2.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000025'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000112'), 0.1500, (SELECT id FROM mm_uom WHERE code = 'KG'), 5.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000025'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000113'), 0.1200, (SELECT id FROM mm_uom WHERE code = 'KG'), 3.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000025'), 4, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000111'), 2.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 10.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000025'), 5, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000002'), 0.0200, (SELECT id FROM mm_uom WHERE code = 'L'), 0.00),
    -- Chocolate Cake BOM
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000026'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000001'), 0.3000, (SELECT id FROM mm_uom WHERE code = 'KG'), 2.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000026'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000110'), 0.4000, (SELECT id FROM mm_uom WHERE code = 'KG'), 3.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000026'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000103'), 0.2000, (SELECT id FROM mm_uom WHERE code = 'KG'), 2.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000026'), 4, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000107'), 0.1000, (SELECT id FROM mm_uom WHERE code = 'BOX'), 1.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000026'), 5, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000108'), 0.2500, (SELECT id FROM mm_uom WHERE code = 'KG'), 0.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000026'), 6, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000104'), 0.3000, (SELECT id FROM mm_uom WHERE code = 'L'), 2.00),
    -- Salmon Sashimi Platter BOM
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000027'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000105'), 0.5000, (SELECT id FROM mm_uom WHERE code = 'KG'), 15.00),
    -- Seafood Linguine BOM
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000028'), 1, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000106'), 0.1500, (SELECT id FROM mm_uom WHERE code = 'KG'), 5.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000028'), 2, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000001'), 0.1200, (SELECT id FROM mm_uom WHERE code = 'KG'), 2.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000028'), 3, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000002'), 0.0300, (SELECT id FROM mm_uom WHERE code = 'L'), 0.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000028'), 4, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000104'), 0.0500, (SELECT id FROM mm_uom WHERE code = 'L'), 2.00),
    ((SELECT id FROM pp_boms WHERE bom_number = 'BOM00000028'), 5, (SELECT id FROM mm_materials WHERE material_number = 'MAT00000111'), 1.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), 5.00)
ON CONFLICT DO NOTHING;

-- ============================================================================
-- PHASE 13: PRODUCTION ORDERS
-- ============================================================================

-- Update existing PRD00000001
UPDATE pp_production_orders SET
    material_id = (SELECT id FROM mm_materials WHERE material_number = 'MAT00000005'),
    bom_id = (SELECT id FROM pp_boms WHERE bom_number = 'BOM00000001'),
    planned_quantity = 20.0000,
    actual_quantity = 20.0000,
    uom_id = (SELECT id FROM mm_uom WHERE code = 'SET'),
    planned_start = '2026-01-05',
    planned_end = '2026-01-06',
    actual_start = '2026-01-05',
    actual_end = '2026-01-06',
    status = 'COMPLETED'
WHERE order_number = 'PRD00000001';

-- Delete remaining test production orders
DELETE FROM pp_production_orders WHERE order_number NOT IN ('PRD00000001');

-- Insert new production orders
INSERT INTO pp_production_orders (order_number, material_id, bom_id, planned_quantity, actual_quantity, uom_id, planned_start, planned_end, actual_start, actual_end, status) VALUES
    ('PRD00000009', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000114'), (SELECT id FROM pp_boms WHERE bom_number = 'BOM00000024'),
     500.0000, 500.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), '2026-01-15', '2026-01-15', '2026-01-15', '2026-01-15', 'COMPLETED'),
    ('PRD00000010', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000116'), (SELECT id FROM pp_boms WHERE bom_number = 'BOM00000026'),
     30.0000, 30.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), '2026-01-18', '2026-01-19', '2026-01-18', '2026-01-19', 'COMPLETED'),
    ('PRD00000011', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000117'), (SELECT id FROM pp_boms WHERE bom_number = 'BOM00000027'),
     80.0000, 80.0000, (SELECT id FROM mm_uom WHERE code = 'SET'), '2026-02-01', '2026-02-01', '2026-02-01', '2026-02-01', 'COMPLETED'),
    ('PRD00000012', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000118'), (SELECT id FROM pp_boms WHERE bom_number = 'BOM00000028'),
     120.0000, 120.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), '2026-02-02', '2026-02-03', '2026-02-02', '2026-02-03', 'COMPLETED'),
    ('PRD00000013', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000114'), (SELECT id FROM pp_boms WHERE bom_number = 'BOM00000024'),
     400.0000, 250.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), '2026-02-20', '2026-02-20', '2026-02-20', NULL, 'IN_PROGRESS'),
    ('PRD00000014', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000115'), (SELECT id FROM pp_boms WHERE bom_number = 'BOM00000025'),
     150.0000, 80.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), '2026-02-25', '2026-02-26', '2026-02-25', NULL, 'IN_PROGRESS'),
    ('PRD00000015', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000116'), (SELECT id FROM pp_boms WHERE bom_number = 'BOM00000026'),
     50.0000, 0.0000, (SELECT id FROM mm_uom WHERE code = 'EA'), '2026-03-10', '2026-03-12', NULL, NULL, 'RELEASED'),
    ('PRD00000016', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000117'), (SELECT id FROM pp_boms WHERE bom_number = 'BOM00000027'),
     100.0000, 0.0000, (SELECT id FROM mm_uom WHERE code = 'SET'), '2026-03-15', '2026-03-15', NULL, NULL, 'CREATED')
ON CONFLICT (order_number) DO NOTHING;

-- ============================================================================
-- PHASE 14: PLANT STOCK
-- ============================================================================

-- Update existing plant stock
UPDATE mm_plant_stock SET
    quantity = 200.0000,
    warehouse_id = (SELECT id FROM wm_warehouses WHERE code = 'WH01')
WHERE material_id = (SELECT id FROM mm_materials WHERE material_number = 'MAT00000002');

UPDATE mm_plant_stock SET
    quantity = 85.0000,
    warehouse_id = (SELECT id FROM wm_warehouses WHERE code = 'WH01')
WHERE material_id = (SELECT id FROM mm_materials WHERE material_number = 'MAT00000003');

UPDATE mm_plant_stock SET
    quantity = 120.0000,
    warehouse_id = (SELECT id FROM wm_warehouses WHERE code = 'WH01')
WHERE material_id = (SELECT id FROM mm_materials WHERE material_number = 'MAT00000004');

UPDATE mm_plant_stock SET
    quantity = 35.0000,
    warehouse_id = (SELECT id FROM wm_warehouses WHERE code = 'WH01')
WHERE material_id = (SELECT id FROM mm_materials WHERE material_number = 'MAT00000005');

-- Insert stock for new materials
INSERT INTO mm_plant_stock (material_id, warehouse_id, quantity, reserved_quantity, uom_id) VALUES
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000001'), (SELECT id FROM wm_warehouses WHERE code = 'WH03'), 350.0000, 50.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000102'), (SELECT id FROM wm_warehouses WHERE code = 'WH03'), 280.0000, 40.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000103'), (SELECT id FROM wm_warehouses WHERE code = 'WH02'), 42.0000, 8.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000104'), (SELECT id FROM wm_warehouses WHERE code = 'WH02'), 35.0000, 10.0000, (SELECT id FROM mm_uom WHERE code = 'L')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000105'), (SELECT id FROM wm_warehouses WHERE code = 'WH02'), 18.0000, 5.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000106'), (SELECT id FROM wm_warehouses WHERE code = 'WH02'), 12.0000, 3.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000107'), (SELECT id FROM wm_warehouses WHERE code = 'WH02'), 45.0000, 10.0000, (SELECT id FROM mm_uom WHERE code = 'BOX')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000108'), (SELECT id FROM wm_warehouses WHERE code = 'WH03'), 200.0000, 25.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000109'), (SELECT id FROM wm_warehouses WHERE code = 'WH03'), 6.0000, 1.0000, (SELECT id FROM mm_uom WHERE code = 'L')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000110'), (SELECT id FROM wm_warehouses WHERE code = 'WH03'), 15.0000, 4.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000111'), (SELECT id FROM wm_warehouses WHERE code = 'WH02'), 30.0000, 5.0000, (SELECT id FROM mm_uom WHERE code = 'EA')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000112'), (SELECT id FROM wm_warehouses WHERE code = 'WH02'), 80.0000, 15.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000113'), (SELECT id FROM wm_warehouses WHERE code = 'WH02'), 8.0000, 2.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000114'), (SELECT id FROM wm_warehouses WHERE code = 'WH04'), 150.0000, 0.0000, (SELECT id FROM mm_uom WHERE code = 'EA')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000115'), (SELECT id FROM wm_warehouses WHERE code = 'WH04'), 25.0000, 0.0000, (SELECT id FROM mm_uom WHERE code = 'EA')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000116'), (SELECT id FROM wm_warehouses WHERE code = 'WH04'), 12.0000, 0.0000, (SELECT id FROM mm_uom WHERE code = 'EA')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000117'), (SELECT id FROM wm_warehouses WHERE code = 'WH02'), 10.0000, 0.0000, (SELECT id FROM mm_uom WHERE code = 'SET')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000118'), (SELECT id FROM wm_warehouses WHERE code = 'WH04'), 20.0000, 0.0000, (SELECT id FROM mm_uom WHERE code = 'EA')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000119'), (SELECT id FROM wm_warehouses WHERE code = 'WH03'), 3.5000, 0.5000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000120'), (SELECT id FROM wm_warehouses WHERE code = 'WH03'), 300.0000, 50.0000, (SELECT id FROM mm_uom WHERE code = 'KG')),
    ((SELECT id FROM mm_materials WHERE material_number = 'MAT00000121'), (SELECT id FROM wm_warehouses WHERE code = 'WH01'), 500.0000, 0.0000, (SELECT id FROM mm_uom WHERE code = 'EA'))
ON CONFLICT (material_id, warehouse_id) DO NOTHING;

-- ============================================================================
-- PHASE 15: JOURNAL ENTRIES - Update existing and add more
-- ============================================================================

-- Update existing journal entry descriptions to be more meaningful
UPDATE fi_journal_entries SET
    description = 'Sales revenue recognition - Gourmet Restaurant Tokyo January order',
    reference = 'SO00000002'
WHERE document_number = 'JE00000001';

UPDATE fi_journal_entries SET
    description = 'Inventory receipt - Pacific Seafood raw materials PO',
    reference = 'PO00000001'
WHERE document_number = 'JE00000002';

UPDATE fi_journal_entries SET
    description = 'Inventory receipt - Golden Grain Mills flour and grains',
    reference = 'PO00000009'
WHERE document_number = 'JE00000003';

UPDATE fi_journal_entries SET
    description = 'Cash receipt - customer payment Gourmet Restaurant Tokyo',
    reference = 'INV00000005'
WHERE document_number = 'JE00000004';

UPDATE fi_journal_entries SET
    description = 'Cash receipt - customer payment Cafe Deluxe Taipei',
    reference = 'INV00000006'
WHERE document_number = 'JE00000005';

UPDATE fi_journal_entries SET
    description = 'January payroll accrual',
    reference = 'PAYROLL-2026-01'
WHERE document_number = 'JE00000006';

UPDATE fi_journal_entries SET
    description = 'February rent payment',
    reference = 'RENT-2026-02'
WHERE document_number = 'JE00000007';

-- Update journal item amounts to be realistic
-- JE00000001: Sales revenue 45,000
UPDATE fi_journal_items SET debit_amount = 45000.0000, credit_amount = 0.0000
WHERE journal_entry_id = (SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000001') AND line_number = 1;
UPDATE fi_journal_items SET debit_amount = 0.0000, credit_amount = 45000.0000
WHERE journal_entry_id = (SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000001') AND line_number = 2;

-- JE00000002: Inventory receipt 1,500
UPDATE fi_journal_items SET debit_amount = 1500.0000, credit_amount = 0.0000
WHERE journal_entry_id = (SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000002') AND line_number = 1;
UPDATE fi_journal_items SET debit_amount = 0.0000, credit_amount = 1500.0000
WHERE journal_entry_id = (SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000002') AND line_number = 2;

-- JE00000003: Flour purchase 37,500
UPDATE fi_journal_items SET debit_amount = 37500.0000, credit_amount = 0.0000
WHERE journal_entry_id = (SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000003') AND line_number = 1;
UPDATE fi_journal_items SET debit_amount = 0.0000, credit_amount = 37500.0000
WHERE journal_entry_id = (SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000003') AND line_number = 2;

-- JE00000004: Cash receipt 45,000
UPDATE fi_journal_items SET debit_amount = 45000.0000, credit_amount = 0.0000
WHERE journal_entry_id = (SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000004') AND line_number = 1;
UPDATE fi_journal_items SET debit_amount = 0.0000, credit_amount = 45000.0000
WHERE journal_entry_id = (SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000004') AND line_number = 2;

-- JE00000005: Cash receipt 32,400
UPDATE fi_journal_items SET debit_amount = 32400.0000, credit_amount = 0.0000
WHERE journal_entry_id = (SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000005') AND line_number = 1;
UPDATE fi_journal_items SET debit_amount = 0.0000, credit_amount = 32400.0000
WHERE journal_entry_id = (SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000005') AND line_number = 2;

-- JE00000006: Payroll 850,000
UPDATE fi_journal_items SET
    debit_amount = 850000.0000, credit_amount = 0.0000,
    account_id = (SELECT id FROM fi_accounts WHERE account_number = '6100')
WHERE journal_entry_id = (SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000006') AND line_number = 1;
UPDATE fi_journal_items SET
    debit_amount = 0.0000, credit_amount = 850000.0000,
    account_id = (SELECT id FROM fi_accounts WHERE account_number = '1100')
WHERE journal_entry_id = (SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000006') AND line_number = 2;

-- JE00000007: Rent 120,000
UPDATE fi_journal_items SET
    debit_amount = 120000.0000, credit_amount = 0.0000,
    account_id = (SELECT id FROM fi_accounts WHERE account_number = '6200')
WHERE journal_entry_id = (SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000007') AND line_number = 1;
UPDATE fi_journal_items SET
    debit_amount = 0.0000, credit_amount = 120000.0000,
    account_id = (SELECT id FROM fi_accounts WHERE account_number = '1100')
WHERE journal_entry_id = (SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000007') AND line_number = 2;

-- Add new journal entries for Feb/March 2026
INSERT INTO fi_journal_entries (document_number, company_code_id, fiscal_year, fiscal_period, posting_date, document_date, reference, description, status) VALUES
    ('JE00000008', (SELECT id FROM fi_company_codes WHERE code = 'TB01'), 2026, 2, '2026-02-05', '2026-02-05', 'SO00000012', 'Sales revenue - Hotel Grand Palace February banquet', 'POSTED'),
    ('JE00000009', (SELECT id FROM fi_company_codes WHERE code = 'TB01'), 2026, 2, '2026-02-10', '2026-02-10', 'PO00000012', 'Inventory receipt - Fresh Farms produce delivery', 'POSTED'),
    ('JE00000010', (SELECT id FROM fi_company_codes WHERE code = 'TB01'), 2026, 2, '2026-02-28', '2026-02-28', 'PAYROLL-2026-02', 'February payroll', 'POSTED'),
    ('JE00000011', (SELECT id FROM fi_company_codes WHERE code = 'TB01'), 2026, 2, '2026-02-28', '2026-02-28', 'COGS-2026-02', 'February cost of goods sold', 'POSTED'),
    ('JE00000012', (SELECT id FROM fi_company_codes WHERE code = 'TB01'), 2026, 3, '2026-03-01', '2026-03-01', 'RENT-2026-03', 'March rent payment', 'POSTED'),
    ('JE00000013', (SELECT id FROM fi_company_codes WHERE code = 'TB01'), 2026, 3, '2026-03-05', '2026-03-05', 'UTIL-2026-03', 'March utilities - gas, water, electricity', 'POSTED'),
    ('JE00000014', (SELECT id FROM fi_company_codes WHERE code = 'TB01'), 2026, 3, '2026-03-15', '2026-03-15', 'DEPR-2026-Q1', 'Q1 2026 depreciation of kitchen equipment', 'DRAFT')
ON CONFLICT (document_number) DO NOTHING;

-- Journal items for new entries
INSERT INTO fi_journal_items (journal_entry_id, line_number, account_id, debit_amount, credit_amount, description) VALUES
    -- JE00000008: Hotel banquet revenue 128,000
    ((SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000008'), 1, (SELECT id FROM fi_accounts WHERE account_number = '1200'), 128000.0000, 0.0000, 'Accounts receivable - Hotel Grand Palace'),
    ((SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000008'), 2, (SELECT id FROM fi_accounts WHERE account_number = '4100'), 0.0000, 128000.0000, 'Sales revenue - banquet catering'),
    -- JE00000009: Produce inventory receipt 8,200
    ((SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000009'), 1, (SELECT id FROM fi_accounts WHERE account_number = '1300'), 8200.0000, 0.0000, 'Raw materials inventory'),
    ((SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000009'), 2, (SELECT id FROM fi_accounts WHERE account_number = '2100'), 0.0000, 8200.0000, 'Accounts payable - Fresh Farms Co.'),
    -- JE00000010: February payroll 880,000
    ((SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000010'), 1, (SELECT id FROM fi_accounts WHERE account_number = '6100'), 880000.0000, 0.0000, 'Salaries expense - February'),
    ((SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000010'), 2, (SELECT id FROM fi_accounts WHERE account_number = '1100'), 0.0000, 880000.0000, 'Cash payment - payroll'),
    -- JE00000011: February COGS 95,000
    ((SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000011'), 1, (SELECT id FROM fi_accounts WHERE account_number = '5100'), 95000.0000, 0.0000, 'Cost of goods sold - February'),
    ((SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000011'), 2, (SELECT id FROM fi_accounts WHERE account_number = '1300'), 0.0000, 95000.0000, 'Inventory consumed'),
    -- JE00000012: March rent 120,000
    ((SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000012'), 1, (SELECT id FROM fi_accounts WHERE account_number = '6200'), 120000.0000, 0.0000, 'Rent expense - March'),
    ((SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000012'), 2, (SELECT id FROM fi_accounts WHERE account_number = '1100'), 0.0000, 120000.0000, 'Cash payment - rent'),
    -- JE00000013: Utilities 45,000
    ((SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000013'), 1, (SELECT id FROM fi_accounts WHERE account_number = '6300'), 45000.0000, 0.0000, 'Utilities expense - gas, water, electricity'),
    ((SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000013'), 2, (SELECT id FROM fi_accounts WHERE account_number = '1100'), 0.0000, 45000.0000, 'Cash payment - utilities'),
    -- JE00000014: Depreciation 75,000
    ((SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000014'), 1, (SELECT id FROM fi_accounts WHERE account_number = '6400'), 75000.0000, 0.0000, 'Depreciation - kitchen equipment Q1'),
    ((SELECT id FROM fi_journal_entries WHERE document_number = 'JE00000014'), 2, (SELECT id FROM fi_accounts WHERE account_number = '1510'), 0.0000, 75000.0000, 'Accumulated depreciation')
ON CONFLICT DO NOTHING;

-- ============================================================================
-- PHASE 16: FISCAL YEARS & PERIODS
-- ============================================================================

INSERT INTO fi_fiscal_years (company_code_id, year, start_date, end_date, is_closed) VALUES
    ((SELECT id FROM fi_company_codes WHERE code = 'TB01'), 2025, '2025-01-01', '2025-12-31', true),
    ((SELECT id FROM fi_company_codes WHERE code = 'TB01'), 2026, '2026-01-01', '2026-12-31', false)
ON CONFLICT DO NOTHING;

-- Fiscal periods for 2026
INSERT INTO fi_fiscal_periods (fiscal_year_id, period, start_date, end_date, is_closed)
SELECT fy.id, p.period, p.start_date, p.end_date, p.is_closed
FROM fi_fiscal_years fy
CROSS JOIN (VALUES
    (1, '2026-01-01'::date, '2026-01-31'::date, true),
    (2, '2026-02-01'::date, '2026-02-28'::date, true),
    (3, '2026-03-01'::date, '2026-03-31'::date, false),
    (4, '2026-04-01'::date, '2026-04-30'::date, false),
    (5, '2026-05-01'::date, '2026-05-31'::date, false),
    (6, '2026-06-01'::date, '2026-06-30'::date, false),
    (7, '2026-07-01'::date, '2026-07-31'::date, false),
    (8, '2026-08-01'::date, '2026-08-31'::date, false),
    (9, '2026-09-01'::date, '2026-09-30'::date, false),
    (10, '2026-10-01'::date, '2026-10-31'::date, false),
    (11, '2026-11-01'::date, '2026-11-30'::date, false),
    (12, '2026-12-01'::date, '2026-12-31'::date, false)
) AS p(period, start_date, end_date, is_closed)
WHERE fy.year = 2026
AND NOT EXISTS (
    SELECT 1 FROM fi_fiscal_periods fp WHERE fp.fiscal_year_id = fy.id AND fp.period = p.period
);

-- ============================================================================
-- PHASE 17: QM INSPECTION LOTS (clean + add realistic)
-- ============================================================================

-- Clean existing test inspection lots
DELETE FROM qm_inspection_results;
DELETE FROM qm_quality_notifications;
DELETE FROM qm_inspection_lots;

INSERT INTO qm_inspection_lots (lot_number, material_id, inspection_type, planned_quantity, inspected_quantity, status) VALUES
    ('QIL00000022', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000105'), 'INCOMING', 20.0000, 20.0000, 'COMPLETED'),
    ('QIL00000023', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000106'), 'INCOMING', 15.0000, 15.0000, 'COMPLETED'),
    ('QIL00000024', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000103'), 'INCOMING', 6.0000, 6.0000, 'COMPLETED'),
    ('QIL00000025', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000114'), 'PRODUCTION', 500.0000, 500.0000, 'COMPLETED'),
    ('QIL00000026', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000116'), 'PRODUCTION', 30.0000, 30.0000, 'COMPLETED'),
    ('QIL00000027', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000105'), 'INCOMING', 15.0000, 5.0000, 'IN_PROGRESS'),
    ('QIL00000028', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000001'), 'INCOMING', 15.0000, 0.0000, 'CREATED')
ON CONFLICT (lot_number) DO NOTHING;

-- Inspection results
INSERT INTO qm_inspection_results (inspection_lot_id, characteristic, target_value, actual_value, is_conforming) VALUES
    ((SELECT id FROM qm_inspection_lots WHERE lot_number = 'QIL00000022'), 'Freshness (days)', '< 3', '1', true),
    ((SELECT id FROM qm_inspection_lots WHERE lot_number = 'QIL00000022'), 'Temperature (C)', '< 4', '2.5', true),
    ((SELECT id FROM qm_inspection_lots WHERE lot_number = 'QIL00000022'), 'Color', 'Orange-pink', 'Orange-pink', true),
    ((SELECT id FROM qm_inspection_lots WHERE lot_number = 'QIL00000023'), 'Shell integrity', '100%', '98%', true),
    ((SELECT id FROM qm_inspection_lots WHERE lot_number = 'QIL00000023'), 'Temperature (C)', '< -18', '-22', true),
    ((SELECT id FROM qm_inspection_lots WHERE lot_number = 'QIL00000024'), 'Fat content (%)', '82 +/- 1', '82.3', true),
    ((SELECT id FROM qm_inspection_lots WHERE lot_number = 'QIL00000024'), 'Moisture (%)', '< 16', '15.2', true),
    ((SELECT id FROM qm_inspection_lots WHERE lot_number = 'QIL00000025'), 'Visual inspection', 'Golden brown', 'Golden brown', true),
    ((SELECT id FROM qm_inspection_lots WHERE lot_number = 'QIL00000025'), 'Weight (g)', '80 +/- 5', '82', true),
    ((SELECT id FROM qm_inspection_lots WHERE lot_number = 'QIL00000025'), 'Texture', 'Flaky, layered', 'Flaky, layered', true),
    ((SELECT id FROM qm_inspection_lots WHERE lot_number = 'QIL00000026'), 'Moisture content', '< 25%', '22%', true),
    ((SELECT id FROM qm_inspection_lots WHERE lot_number = 'QIL00000026'), 'Cocoa flavor', 'Rich, balanced', 'Rich, balanced', true),
    ((SELECT id FROM qm_inspection_lots WHERE lot_number = 'QIL00000027'), 'Freshness (days)', '< 3', '2', true),
    ((SELECT id FROM qm_inspection_lots WHERE lot_number = 'QIL00000027'), 'Temperature (C)', '< 4', '3.1', true);

-- Quality notifications
INSERT INTO qm_quality_notifications (notification_number, notification_type, material_id, description, priority, status) VALUES
    ('QN00000008', 'VENDOR_COMPLAINT', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000112'), 'Roma tomatoes received with minor bruising on 15% of batch', 'LOW', 'CLOSED'),
    ('QN00000009', 'INTERNAL', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000114'), 'Croissant batch PRD00000009 slightly underweight - adjusted oven temperature', 'MEDIUM', 'CLOSED'),
    ('QN00000010', 'CUSTOMER_COMPLAINT', (SELECT id FROM mm_materials WHERE material_number = 'MAT00000117'), 'Customer reported sashimi platter presentation inconsistency', 'HIGH', 'IN_PROGRESS')
ON CONFLICT (notification_number) DO NOTHING;

-- ============================================================================
-- PHASE 18: CO INTERNAL ORDERS
-- ============================================================================

DELETE FROM co_internal_orders WHERE order_number LIKE 'IO%' AND name LIKE '%Integration%';

INSERT INTO co_internal_orders (order_number, name, order_type, cost_center_id, status, budget, actual_cost) VALUES
    ('IO00000010', 'Spring Menu Development 2026', 'DEVELOPMENT', (SELECT id FROM co_cost_centers WHERE code = 'CC-RD'), 'RELEASED', 500000.0000, 180000.0000),
    ('IO00000011', 'Kitchen Equipment Maintenance Q1', 'MAINTENANCE', (SELECT id FROM co_cost_centers WHERE code = 'CC-PRODUCTION'), 'RELEASED', 200000.0000, 85000.0000),
    ('IO00000012', 'Marketing Campaign - Valentine', 'MARKETING', (SELECT id FROM co_cost_centers WHERE code = 'CC-SALES'), 'COMPLETED', 300000.0000, 275000.0000),
    ('IO00000013', 'Food Safety Certification Renewal', 'OVERHEAD', (SELECT id FROM co_cost_centers WHERE code = 'CC-QC'), 'RELEASED', 150000.0000, 42000.0000),
    ('IO00000014', 'Warehouse Cold Room Upgrade', 'INVESTMENT', (SELECT id FROM co_cost_centers WHERE code = 'CC-LOGISTICS'), 'CREATED', 1200000.0000, 0.0000)
ON CONFLICT (order_number) DO NOTHING;

-- ============================================================================
-- PHASE 19: UPDATE NUMBER RANGES
-- ============================================================================

UPDATE number_ranges SET current_number = GREATEST(current_number, 121) WHERE object_type = 'MAT';
UPDATE number_ranges SET current_number = GREATEST(current_number, 21) WHERE object_type = 'VND';
UPDATE number_ranges SET current_number = GREATEST(current_number, 16) WHERE object_type = 'CUST';
UPDATE number_ranges SET current_number = GREATEST(current_number, 18) WHERE object_type = 'SO';
UPDATE number_ranges SET current_number = GREATEST(current_number, 17) WHERE object_type = 'PO';
UPDATE number_ranges SET current_number = GREATEST(current_number, 3) WHERE object_type = 'DO';
UPDATE number_ranges SET current_number = GREATEST(current_number, 7) WHERE object_type = 'INV';
UPDATE number_ranges SET current_number = GREATEST(current_number, 28) WHERE object_type = 'BOM';
UPDATE number_ranges SET current_number = GREATEST(current_number, 16) WHERE object_type = 'PRD';
UPDATE number_ranges SET current_number = GREATEST(current_number, 30) WHERE object_type = 'EMP';
UPDATE number_ranges SET current_number = GREATEST(current_number, 28) WHERE object_type = 'QIL';
UPDATE number_ranges SET current_number = GREATEST(current_number, 10) WHERE object_type = 'QN';
UPDATE number_ranges SET current_number = GREATEST(current_number, 14) WHERE object_type = 'JE';
UPDATE number_ranges SET current_number = GREATEST(current_number, 14) WHERE object_type = 'IO';

COMMIT;
