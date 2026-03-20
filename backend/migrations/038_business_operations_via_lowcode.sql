-- 038_business_operations_via_lowcode.sql
-- ═══════════════════════════════════════════════════════════════════════
-- TasteByte Foods IT Team: Using the lowcode platform to build all
-- missing business operations. No custom code — all through the platform.
-- ═══════════════════════════════════════════════════════════════════════

-- ── Project: Supply Chain Operations ──────────────────────────────────
INSERT INTO lc_projects (id, project_number, name, description, is_active)
VALUES ('d0000000-0000-0000-0000-000000000001', 'LCP-100', 'Supply Chain Operations', 'GRN, Delivery, Stock Movement, Supplier Evaluation — built via lowcode', true)
ON CONFLICT (project_number) DO NOTHING;

-- ══════════════════════════════════════════════════════════════════════
-- OPERATION 1: GRN 收貨單 (Goods Receipt Note)
-- Module: MM | Type: FORM | Used by: 採購經理 王軍
-- ══════════════════════════════════════════════════════════════════════
INSERT INTO lc_operations (id, operation_code, project_id, name, description, operation_type, is_published, version, module, sidebar_icon, sidebar_sort_order)
VALUES ('d1000000-0000-0000-0000-000000000001', 'MM-GRN', 'd0000000-0000-0000-0000-000000000001', '收貨單 (GRN)', '採購收貨確認，對應採購單數量核對，自動更新庫存', 'FORM', true, 1, 'MM', 'PackageCheck', 30)
ON CONFLICT (operation_code) DO NOTHING;

INSERT INTO lc_form_definitions (id, operation_id, layout_config, form_settings) VALUES
('d1100000-0000-0000-0000-000000000001', 'd1000000-0000-0000-0000-000000000001', '{}', '{}')
ON CONFLICT (operation_id) DO NOTHING;

-- Section 1: GRN Header
INSERT INTO lc_form_sections (id, form_id, title, description, columns, sort_order) VALUES
('d1110000-0000-0000-0000-000000000001', 'd1100000-0000-0000-0000-000000000001', '收貨單資訊', '基本收貨資訊', 2, 0)
ON CONFLICT DO NOTHING;

INSERT INTO lc_field_definitions (id, section_id, field_name, field_label, field_type, is_required, is_searchable, sort_order, column_span, placeholder, field_config) VALUES
('d1111000-0000-0000-0000-000000000001', 'd1110000-0000-0000-0000-000000000001', 'grn_number', 'GRN 編號', 'TEXT', true, true, 0, 1, '', '{"is_readonly": true}'),
('d1111000-0000-0000-0000-000000000002', 'd1110000-0000-0000-0000-000000000001', 'po_number', '對應採購單號', 'TEXT', true, true, 1, 1, '例：PO-00001', '{}'),
('d1111000-0000-0000-0000-000000000003', 'd1110000-0000-0000-0000-000000000001', 'vendor_name', '供應商', 'TEXT', true, true, 2, 1, '', '{}'),
('d1111000-0000-0000-0000-000000000004', 'd1110000-0000-0000-0000-000000000001', 'receipt_date', '收貨日期', 'DATE', true, false, 3, 1, '', '{}'),
('d1111000-0000-0000-0000-000000000005', 'd1110000-0000-0000-0000-000000000001', 'warehouse', '收貨倉庫', 'DROPDOWN', true, false, 4, 1, '', '{}'),
('d1111000-0000-0000-0000-000000000006', 'd1110000-0000-0000-0000-000000000001', 'status', '狀態', 'DROPDOWN', true, true, 5, 1, '', '{}')
ON CONFLICT DO NOTHING;

INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order, is_default) VALUES
('d1111000-0000-0000-0000-000000000005', '台北主倉庫', 'WH-001', 0, true),
('d1111000-0000-0000-0000-000000000005', '桃園冷藏倉', 'WH-002', 1, false),
('d1111000-0000-0000-0000-000000000005', '台中配送中心', 'WH-003', 2, false),
('d1111000-0000-0000-0000-000000000006', '草稿', 'DRAFT', 0, true),
('d1111000-0000-0000-0000-000000000006', '已確認', 'CONFIRMED', 1, false),
('d1111000-0000-0000-0000-000000000006', '已入庫', 'RECEIVED', 2, false),
('d1111000-0000-0000-0000-000000000006', '已關閉', 'CLOSED', 3, false)
ON CONFLICT DO NOTHING;

-- Section 2: GRN Items
INSERT INTO lc_form_sections (id, form_id, title, description, columns, sort_order) VALUES
('d1120000-0000-0000-0000-000000000001', 'd1100000-0000-0000-0000-000000000001', '收貨明細', '各品項收貨數量', 1, 1)
ON CONFLICT DO NOTHING;

INSERT INTO lc_field_definitions (id, section_id, field_name, field_label, field_type, is_required, sort_order, column_span, field_config) VALUES
('d1121000-0000-0000-0000-000000000001', 'd1120000-0000-0000-0000-000000000001', 'material_name', '物料名稱', 'TEXT', true, 0, 1, '{}'),
('d1121000-0000-0000-0000-000000000002', 'd1120000-0000-0000-0000-000000000001', 'ordered_qty', '訂購數量', 'NUMBER', true, 1, 1, '{}'),
('d1121000-0000-0000-0000-000000000003', 'd1120000-0000-0000-0000-000000000001', 'received_qty', '實收數量', 'NUMBER', true, 2, 1, '{}'),
('d1121000-0000-0000-0000-000000000004', 'd1120000-0000-0000-0000-000000000001', 'rejected_qty', '退回數量', 'NUMBER', false, 3, 1, '{}'),
('d1121000-0000-0000-0000-000000000005', 'd1120000-0000-0000-0000-000000000001', 'batch_number', '批號', 'TEXT', false, 4, 1, '{}'),
('d1121000-0000-0000-0000-000000000006', 'd1120000-0000-0000-0000-000000000001', 'expiry_date', '有效期限', 'DATE', false, 5, 1, '{}'),
('d1121000-0000-0000-0000-000000000007', 'd1120000-0000-0000-0000-000000000001', 'inspection_result', '品檢結果', 'DROPDOWN', false, 6, 1, '{}'),
('d1121000-0000-0000-0000-000000000008', 'd1120000-0000-0000-0000-000000000001', 'notes', '備註', 'TEXTAREA', false, 7, 1, '{}')
ON CONFLICT DO NOTHING;

INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
('d1121000-0000-0000-0000-000000000007', '合格', 'PASS', 0),
('d1121000-0000-0000-0000-000000000007', '條件允收', 'CONDITIONAL', 1),
('d1121000-0000-0000-0000-000000000007', '不合格', 'FAIL', 2)
ON CONFLICT DO NOTHING;

-- ══════════════════════════════════════════════════════════════════════
-- OPERATION 2: 出貨確認單 (Delivery Confirmation)
-- Module: SD | Type: FORM | Used by: 業務經理 林美
-- ══════════════════════════════════════════════════════════════════════
INSERT INTO lc_operations (id, operation_code, project_id, name, description, operation_type, is_published, version, module, sidebar_icon, sidebar_sort_order)
VALUES ('d2000000-0000-0000-0000-000000000001', 'SD-DELIVERY', 'd0000000-0000-0000-0000-000000000001', '出貨確認單', '銷售出貨確認，記錄出貨數量、運送資訊，出貨後才可開立發票', 'FORM', true, 1, 'SD', 'Truck', 30)
ON CONFLICT (operation_code) DO NOTHING;

INSERT INTO lc_form_definitions (id, operation_id) VALUES
('d2100000-0000-0000-0000-000000000001', 'd2000000-0000-0000-0000-000000000001')
ON CONFLICT (operation_id) DO NOTHING;

INSERT INTO lc_form_sections (id, form_id, title, columns, sort_order) VALUES
('d2110000-0000-0000-0000-000000000001', 'd2100000-0000-0000-0000-000000000001', '出貨資訊', 2, 0),
('d2120000-0000-0000-0000-000000000001', 'd2100000-0000-0000-0000-000000000001', '出貨明細', 1, 1),
('d2130000-0000-0000-0000-000000000001', 'd2100000-0000-0000-0000-000000000001', '運送資訊', 2, 2)
ON CONFLICT DO NOTHING;

INSERT INTO lc_field_definitions (id, section_id, field_name, field_label, field_type, is_required, is_searchable, sort_order, column_span, field_config) VALUES
-- Header
('d2111000-0000-0000-0000-000000000001', 'd2110000-0000-0000-0000-000000000001', 'delivery_number', '出貨單號', 'TEXT', true, true, 0, 1, '{"is_readonly":true}'),
('d2111000-0000-0000-0000-000000000002', 'd2110000-0000-0000-0000-000000000001', 'so_number', '銷售訂單號', 'TEXT', true, true, 1, 1, '{}'),
('d2111000-0000-0000-0000-000000000003', 'd2110000-0000-0000-0000-000000000001', 'customer_name', '客戶名稱', 'TEXT', true, true, 2, 1, '{}'),
('d2111000-0000-0000-0000-000000000004', 'd2110000-0000-0000-0000-000000000001', 'delivery_date', '出貨日期', 'DATE', true, false, 3, 1, '{}'),
('d2111000-0000-0000-0000-000000000005', 'd2110000-0000-0000-0000-000000000001', 'status', '狀態', 'DROPDOWN', true, true, 4, 1, '{}'),
-- Items
('d2121000-0000-0000-0000-000000000001', 'd2120000-0000-0000-0000-000000000001', 'material_name', '商品名稱', 'TEXT', true, false, 0, 1, '{}'),
('d2121000-0000-0000-0000-000000000002', 'd2120000-0000-0000-0000-000000000001', 'ordered_qty', '訂購數量', 'NUMBER', true, false, 1, 1, '{}'),
('d2121000-0000-0000-0000-000000000003', 'd2120000-0000-0000-0000-000000000001', 'shipped_qty', '出貨數量', 'NUMBER', true, false, 2, 1, '{}'),
('d2121000-0000-0000-0000-000000000004', 'd2120000-0000-0000-0000-000000000001', 'batch_number', '批號', 'TEXT', false, false, 3, 1, '{}'),
-- Shipping
('d2131000-0000-0000-0000-000000000001', 'd2130000-0000-0000-0000-000000000001', 'carrier', '物流公司', 'TEXT', false, false, 0, 1, '{}'),
('d2131000-0000-0000-0000-000000000002', 'd2130000-0000-0000-0000-000000000001', 'tracking_number', '物流追蹤號', 'TEXT', false, true, 1, 1, '{}'),
('d2131000-0000-0000-0000-000000000003', 'd2130000-0000-0000-0000-000000000001', 'shipping_address', '送貨地址', 'TEXTAREA', false, false, 2, 2, '{}'),
('d2131000-0000-0000-0000-000000000004', 'd2130000-0000-0000-0000-000000000001', 'receiver_signature', '簽收人', 'TEXT', false, false, 3, 1, '{}')
ON CONFLICT DO NOTHING;

INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
('d2111000-0000-0000-0000-000000000005', '準備中', 'PREPARING', 0),
('d2111000-0000-0000-0000-000000000005', '已出貨', 'SHIPPED', 1),
('d2111000-0000-0000-0000-000000000005', '運送中', 'IN_TRANSIT', 2),
('d2111000-0000-0000-0000-000000000005', '已送達', 'DELIVERED', 3),
('d2111000-0000-0000-0000-000000000005', '已簽收', 'SIGNED', 4)
ON CONFLICT DO NOTHING;

-- ══════════════════════════════════════════════════════════════════════
-- OPERATION 3: 庫存異動單 (Stock Movement)
-- Module: WM | Type: FORM | Used by: 倉管人員
-- ══════════════════════════════════════════════════════════════════════
INSERT INTO lc_operations (id, operation_code, project_id, name, description, operation_type, is_published, version, module, sidebar_icon, sidebar_sort_order)
VALUES ('d3000000-0000-0000-0000-000000000001', 'WM-MOVE', 'd0000000-0000-0000-0000-000000000001', '庫存異動單', '記錄入庫/出庫/調撥/盤點等庫存異動', 'FORM', true, 1, 'WM', 'ArrowLeftRight', 20)
ON CONFLICT (operation_code) DO NOTHING;

INSERT INTO lc_form_definitions (id, operation_id) VALUES
('d3100000-0000-0000-0000-000000000001', 'd3000000-0000-0000-0000-000000000001')
ON CONFLICT (operation_id) DO NOTHING;

INSERT INTO lc_form_sections (id, form_id, title, columns, sort_order) VALUES
('d3110000-0000-0000-0000-000000000001', 'd3100000-0000-0000-0000-000000000001', '異動資訊', 2, 0)
ON CONFLICT DO NOTHING;

INSERT INTO lc_field_definitions (id, section_id, field_name, field_label, field_type, is_required, is_searchable, sort_order, column_span, field_config) VALUES
('d3111000-0000-0000-0000-000000000001', 'd3110000-0000-0000-0000-000000000001', 'movement_type', '異動類型', 'DROPDOWN', true, true, 0, 1, '{}'),
('d3111000-0000-0000-0000-000000000002', 'd3110000-0000-0000-0000-000000000001', 'material_name', '物料名稱', 'TEXT', true, true, 1, 1, '{}'),
('d3111000-0000-0000-0000-000000000003', 'd3110000-0000-0000-0000-000000000001', 'quantity', '數量', 'NUMBER', true, false, 2, 1, '{}'),
('d3111000-0000-0000-0000-000000000004', 'd3110000-0000-0000-0000-000000000001', 'unit', '單位', 'TEXT', true, false, 3, 1, '{}'),
('d3111000-0000-0000-0000-000000000005', 'd3110000-0000-0000-0000-000000000001', 'from_warehouse', '來源倉庫', 'DROPDOWN', false, false, 4, 1, '{}'),
('d3111000-0000-0000-0000-000000000006', 'd3110000-0000-0000-0000-000000000001', 'to_warehouse', '目的倉庫', 'DROPDOWN', false, false, 5, 1, '{}'),
('d3111000-0000-0000-0000-000000000007', 'd3110000-0000-0000-0000-000000000001', 'batch_number', '批號', 'TEXT', false, true, 6, 1, '{}'),
('d3111000-0000-0000-0000-000000000008', 'd3110000-0000-0000-0000-000000000001', 'reference_doc', '參考單號', 'TEXT', false, true, 7, 1, '{}'),
('d3111000-0000-0000-0000-000000000009', 'd3110000-0000-0000-0000-000000000001', 'movement_date', '異動日期', 'DATE', true, false, 8, 1, '{}'),
('d3111000-0000-0000-0000-000000000010', 'd3110000-0000-0000-0000-000000000001', 'reason', '異動原因', 'TEXTAREA', false, false, 9, 2, '{}')
ON CONFLICT DO NOTHING;

INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
('d3111000-0000-0000-0000-000000000001', '入庫 (採購收貨)', 'GR_PO', 0),
('d3111000-0000-0000-0000-000000000001', '出庫 (銷售出貨)', 'GI_SO', 1),
('d3111000-0000-0000-0000-000000000001', '生產入庫', 'GR_PROD', 2),
('d3111000-0000-0000-0000-000000000001', '生產領料', 'GI_PROD', 3),
('d3111000-0000-0000-0000-000000000001', '倉庫調撥', 'TRANSFER', 4),
('d3111000-0000-0000-0000-000000000001', '盤點調整', 'ADJUSTMENT', 5),
('d3111000-0000-0000-0000-000000000001', '報廢', 'SCRAP', 6),
('d3111000-0000-0000-0000-000000000005', '台北主倉庫', 'WH-001', 0),
('d3111000-0000-0000-0000-000000000005', '桃園冷藏倉', 'WH-002', 1),
('d3111000-0000-0000-0000-000000000005', '台中配送中心', 'WH-003', 2),
('d3111000-0000-0000-0000-000000000006', '台北主倉庫', 'WH-001', 0),
('d3111000-0000-0000-0000-000000000006', '桃園冷藏倉', 'WH-002', 1),
('d3111000-0000-0000-0000-000000000006', '台中配送中心', 'WH-003', 2)
ON CONFLICT DO NOTHING;

-- ══════════════════════════════════════════════════════════════════════
-- OPERATION 4: 供應商評鑑 (Supplier Evaluation)
-- Module: MM | Type: FORM | Used by: 採購經理 王軍
-- ══════════════════════════════════════════════════════════════════════
INSERT INTO lc_operations (id, operation_code, project_id, name, description, operation_type, is_published, version, module, sidebar_icon, sidebar_sort_order)
VALUES ('d4000000-0000-0000-0000-000000000001', 'MM-EVAL', 'd0000000-0000-0000-0000-000000000001', '供應商評鑑', '定期評鑑供應商品質、交期、服務，分數低於60需提出改善計畫', 'FORM', true, 1, 'MM', 'ClipboardCheck', 40)
ON CONFLICT (operation_code) DO NOTHING;

INSERT INTO lc_form_definitions (id, operation_id) VALUES
('d4100000-0000-0000-0000-000000000001', 'd4000000-0000-0000-0000-000000000001')
ON CONFLICT (operation_id) DO NOTHING;

INSERT INTO lc_form_sections (id, form_id, title, columns, sort_order) VALUES
('d4110000-0000-0000-0000-000000000001', 'd4100000-0000-0000-0000-000000000001', '供應商資訊', 2, 0),
('d4120000-0000-0000-0000-000000000001', 'd4100000-0000-0000-0000-000000000001', '評分項目', 3, 1),
('d4130000-0000-0000-0000-000000000001', 'd4100000-0000-0000-0000-000000000001', '評鑑結果', 2, 2)
ON CONFLICT DO NOTHING;

INSERT INTO lc_field_definitions (id, section_id, field_name, field_label, field_type, is_required, is_searchable, sort_order, column_span, min_value, max_value, field_config, visibility_rule) VALUES
('d4111000-0000-0000-0000-000000000001', 'd4110000-0000-0000-0000-000000000001', 'vendor_name', '供應商名稱', 'TEXT', true, true, 0, 1, null, null, '{}', null),
('d4111000-0000-0000-0000-000000000002', 'd4110000-0000-0000-0000-000000000001', 'eval_date', '評鑑日期', 'DATE', true, false, 1, 1, null, null, '{}', null),
('d4111000-0000-0000-0000-000000000003', 'd4110000-0000-0000-0000-000000000001', 'eval_period', '評鑑期間', 'DROPDOWN', true, false, 2, 1, null, null, '{}', null),
('d4111000-0000-0000-0000-000000000004', 'd4110000-0000-0000-0000-000000000001', 'evaluator', '評鑑人', 'TEXT', true, false, 3, 1, null, null, '{}', null),
-- Scores (0-100)
('d4121000-0000-0000-0000-000000000001', 'd4120000-0000-0000-0000-000000000001', 'quality_score', '品質分數', 'NUMBER', true, false, 0, 1, 0, 100, '{}', null),
('d4121000-0000-0000-0000-000000000002', 'd4120000-0000-0000-0000-000000000001', 'delivery_score', '交期分數', 'NUMBER', true, false, 1, 1, 0, 100, '{}', null),
('d4121000-0000-0000-0000-000000000003', 'd4120000-0000-0000-0000-000000000001', 'service_score', '服務分數', 'NUMBER', true, false, 2, 1, 0, 100, '{}', null),
('d4121000-0000-0000-0000-000000000004', 'd4120000-0000-0000-0000-000000000001', 'price_score', '價格分數', 'NUMBER', true, false, 3, 1, 0, 100, '{}', null),
('d4121000-0000-0000-0000-000000000005', 'd4120000-0000-0000-0000-000000000001', 'overall_score', '綜合分數', 'NUMBER', false, true, 4, 1, 0, 100, '{}', null),
-- Results with visibility rules
('d4131000-0000-0000-0000-000000000001', 'd4130000-0000-0000-0000-000000000001', 'rating', '評等', 'DROPDOWN', true, true, 0, 1, null, null, '{}', null),
('d4131000-0000-0000-0000-000000000002', 'd4130000-0000-0000-0000-000000000001', 'excellence_note', '優良供應商備註', 'TEXTAREA', false, false, 1, 2, null, null, '{}', '{"dependent_field":"overall_score","operator":"gt","value":"79","action":"show"}'),
('d4131000-0000-0000-0000-000000000003', 'd4130000-0000-0000-0000-000000000001', 'improvement_plan', '改善計畫', 'TEXTAREA', false, false, 2, 2, null, null, '{}', '{"dependent_field":"overall_score","operator":"lt","value":"60","action":"show"}'),
('d4131000-0000-0000-0000-000000000004', 'd4130000-0000-0000-0000-000000000001', 'comments', '備註', 'TEXTAREA', false, false, 3, 2, null, null, '{}', null)
ON CONFLICT DO NOTHING;

INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
('d4111000-0000-0000-0000-000000000003', '2026 Q1', '2026Q1', 0),
('d4111000-0000-0000-0000-000000000003', '2026 Q2', '2026Q2', 1),
('d4111000-0000-0000-0000-000000000003', '2026 Q3', '2026Q3', 2),
('d4111000-0000-0000-0000-000000000003', '2026 Q4', '2026Q4', 3),
('d4131000-0000-0000-0000-000000000001', '優良 (A)', 'A', 0),
('d4131000-0000-0000-0000-000000000001', '合格 (B)', 'B', 1),
('d4131000-0000-0000-0000-000000000001', '待改善 (C)', 'C', 2),
('d4131000-0000-0000-0000-000000000001', '不合格 (D)', 'D', 3)
ON CONFLICT DO NOTHING;

-- ══════════════════════════════════════════════════════════════════════
-- OPERATION 5: 品質檢驗記錄 (Quality Inspection)
-- Module: QM | Type: FORM | Used by: 品管主任 劉品管
-- ══════════════════════════════════════════════════════════════════════
INSERT INTO lc_operations (id, operation_code, project_id, name, description, operation_type, is_published, version, module, sidebar_icon, sidebar_sort_order)
VALUES ('d5000000-0000-0000-0000-000000000001', 'QM-INSP', 'd0000000-0000-0000-0000-000000000001', '品質檢驗記錄', '記錄原料/成品品檢結果、缺陷、判定', 'FORM', true, 1, 'QM', 'SearchCheck', 20)
ON CONFLICT (operation_code) DO NOTHING;

INSERT INTO lc_form_definitions (id, operation_id) VALUES
('d5100000-0000-0000-0000-000000000001', 'd5000000-0000-0000-0000-000000000001')
ON CONFLICT (operation_id) DO NOTHING;

INSERT INTO lc_form_sections (id, form_id, title, columns, sort_order) VALUES
('d5110000-0000-0000-0000-000000000001', 'd5100000-0000-0000-0000-000000000001', '檢驗資訊', 2, 0),
('d5120000-0000-0000-0000-000000000001', 'd5100000-0000-0000-0000-000000000001', '檢驗項目', 2, 1),
('d5130000-0000-0000-0000-000000000001', 'd5100000-0000-0000-0000-000000000001', '判定與處置', 2, 2)
ON CONFLICT DO NOTHING;

INSERT INTO lc_field_definitions (id, section_id, field_name, field_label, field_type, is_required, is_searchable, sort_order, column_span, field_config) VALUES
('d5111000-0000-0000-0000-000000000001', 'd5110000-0000-0000-0000-000000000001', 'inspection_number', '檢驗單號', 'TEXT', true, true, 0, 1, '{"is_readonly":true}'),
('d5111000-0000-0000-0000-000000000002', 'd5110000-0000-0000-0000-000000000001', 'inspection_type', '檢驗類型', 'DROPDOWN', true, true, 1, 1, '{}'),
('d5111000-0000-0000-0000-000000000003', 'd5110000-0000-0000-0000-000000000001', 'material_name', '物料名稱', 'TEXT', true, true, 2, 1, '{}'),
('d5111000-0000-0000-0000-000000000004', 'd5110000-0000-0000-0000-000000000001', 'batch_number', '批號', 'TEXT', true, true, 3, 1, '{}'),
('d5111000-0000-0000-0000-000000000005', 'd5110000-0000-0000-0000-000000000001', 'sample_size', '抽樣數量', 'NUMBER', true, false, 4, 1, '{}'),
('d5111000-0000-0000-0000-000000000006', 'd5110000-0000-0000-0000-000000000001', 'inspection_date', '檢驗日期', 'DATE', true, false, 5, 1, '{}'),
('d5121000-0000-0000-0000-000000000001', 'd5120000-0000-0000-0000-000000000001', 'appearance', '外觀檢查', 'DROPDOWN', true, false, 0, 1, '{}'),
('d5121000-0000-0000-0000-000000000002', 'd5120000-0000-0000-0000-000000000001', 'weight_check', '重量檢查', 'DROPDOWN', true, false, 1, 1, '{}'),
('d5121000-0000-0000-0000-000000000003', 'd5120000-0000-0000-0000-000000000001', 'taste_check', '口味檢查', 'DROPDOWN', false, false, 2, 1, '{}'),
('d5121000-0000-0000-0000-000000000004', 'd5120000-0000-0000-0000-000000000001', 'packaging_check', '包裝檢查', 'DROPDOWN', true, false, 3, 1, '{}'),
('d5121000-0000-0000-0000-000000000005', 'd5120000-0000-0000-0000-000000000001', 'defect_count', '缺陷數量', 'NUMBER', false, false, 4, 1, '{}'),
('d5121000-0000-0000-0000-000000000006', 'd5120000-0000-0000-0000-000000000001', 'defect_description', '缺陷說明', 'TEXTAREA', false, false, 5, 2, '{}'),
('d5131000-0000-0000-0000-000000000001', 'd5130000-0000-0000-0000-000000000001', 'overall_result', '綜合判定', 'DROPDOWN', true, true, 0, 1, '{}'),
('d5131000-0000-0000-0000-000000000002', 'd5130000-0000-0000-0000-000000000001', 'disposition', '處置方式', 'DROPDOWN', false, false, 1, 1, '{}'),
('d5131000-0000-0000-0000-000000000003', 'd5130000-0000-0000-0000-000000000001', 'inspector_notes', '檢驗員備註', 'TEXTAREA', false, false, 2, 2, '{}')
ON CONFLICT DO NOTHING;

INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
('d5111000-0000-0000-0000-000000000002', '進料檢驗', 'INCOMING', 0),
('d5111000-0000-0000-0000-000000000002', '製程檢驗', 'IN_PROCESS', 1),
('d5111000-0000-0000-0000-000000000002', '成品檢驗', 'FINAL', 2),
('d5111000-0000-0000-0000-000000000002', '出貨檢驗', 'OUTGOING', 3),
('d5121000-0000-0000-0000-000000000001', '合格', 'PASS', 0),
('d5121000-0000-0000-0000-000000000001', '不合格', 'FAIL', 1),
('d5121000-0000-0000-0000-000000000002', '合格', 'PASS', 0),
('d5121000-0000-0000-0000-000000000002', '不合格', 'FAIL', 1),
('d5121000-0000-0000-0000-000000000003', '合格', 'PASS', 0),
('d5121000-0000-0000-0000-000000000003', '不合格', 'FAIL', 1),
('d5121000-0000-0000-0000-000000000004', '合格', 'PASS', 0),
('d5121000-0000-0000-0000-000000000004', '不合格', 'FAIL', 1),
('d5131000-0000-0000-0000-000000000001', '合格', 'PASS', 0),
('d5131000-0000-0000-0000-000000000001', '條件允收', 'CONDITIONAL', 1),
('d5131000-0000-0000-0000-000000000001', '不合格', 'FAIL', 2),
('d5131000-0000-0000-0000-000000000002', '入庫', 'ACCEPT', 0),
('d5131000-0000-0000-0000-000000000002', '退貨', 'RETURN', 1),
('d5131000-0000-0000-0000-000000000002', '報廢', 'SCRAP', 2),
('d5131000-0000-0000-0000-000000000002', '讓步接收', 'CONCESSION', 3)
ON CONFLICT DO NOTHING;

-- ══════════════════════════════════════════════════════════════════════
-- OPERATION 6: 請假申請 (Leave Request)
-- Module: HR | Type: FORM | Used by: 全體員工
-- ══════════════════════════════════════════════════════════════════════
INSERT INTO lc_operations (id, operation_code, project_id, name, description, operation_type, is_published, version, module, sidebar_icon, sidebar_sort_order)
VALUES ('d6000000-0000-0000-0000-000000000001', 'HR-LEAVE', 'd0000000-0000-0000-0000-000000000001', '請假申請', '員工請假申請，含假別、天數計算、主管簽核', 'FORM', true, 1, 'HR', 'CalendarOff', 30)
ON CONFLICT (operation_code) DO NOTHING;

INSERT INTO lc_form_definitions (id, operation_id) VALUES
('d6100000-0000-0000-0000-000000000001', 'd6000000-0000-0000-0000-000000000001')
ON CONFLICT (operation_id) DO NOTHING;

INSERT INTO lc_form_sections (id, form_id, title, columns, sort_order) VALUES
('d6110000-0000-0000-0000-000000000001', 'd6100000-0000-0000-0000-000000000001', '請假資訊', 2, 0),
('d6120000-0000-0000-0000-000000000001', 'd6100000-0000-0000-0000-000000000001', '簽核', 2, 1)
ON CONFLICT DO NOTHING;

INSERT INTO lc_field_definitions (id, section_id, field_name, field_label, field_type, is_required, is_searchable, sort_order, column_span, field_config) VALUES
('d6111000-0000-0000-0000-000000000001', 'd6110000-0000-0000-0000-000000000001', 'employee_name', '申請人', 'TEXT', true, true, 0, 1, '{}'),
('d6111000-0000-0000-0000-000000000002', 'd6110000-0000-0000-0000-000000000001', 'department', '部門', 'TEXT', true, false, 1, 1, '{}'),
('d6111000-0000-0000-0000-000000000003', 'd6110000-0000-0000-0000-000000000001', 'leave_type', '假別', 'DROPDOWN', true, true, 2, 1, '{}'),
('d6111000-0000-0000-0000-000000000004', 'd6110000-0000-0000-0000-000000000001', 'start_date', '開始日期', 'DATE', true, false, 3, 1, '{}'),
('d6111000-0000-0000-0000-000000000005', 'd6110000-0000-0000-0000-000000000001', 'end_date', '結束日期', 'DATE', true, false, 4, 1, '{}'),
('d6111000-0000-0000-0000-000000000006', 'd6110000-0000-0000-0000-000000000001', 'days', '天數', 'NUMBER', true, false, 5, 1, '{}'),
('d6111000-0000-0000-0000-000000000007', 'd6110000-0000-0000-0000-000000000001', 'reason', '請假事由', 'TEXTAREA', true, false, 6, 2, '{}'),
('d6121000-0000-0000-0000-000000000001', 'd6120000-0000-0000-0000-000000000001', 'status', '簽核狀態', 'DROPDOWN', true, true, 0, 1, '{}'),
('d6121000-0000-0000-0000-000000000002', 'd6120000-0000-0000-0000-000000000001', 'approver', '主管', 'TEXT', false, false, 1, 1, '{}'),
('d6121000-0000-0000-0000-000000000003', 'd6120000-0000-0000-0000-000000000001', 'approval_notes', '簽核意見', 'TEXTAREA', false, false, 2, 2, '{}')
ON CONFLICT DO NOTHING;

INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
('d6111000-0000-0000-0000-000000000003', '特休', 'ANNUAL', 0),
('d6111000-0000-0000-0000-000000000003', '事假', 'PERSONAL', 1),
('d6111000-0000-0000-0000-000000000003', '病假', 'SICK', 2),
('d6111000-0000-0000-0000-000000000003', '公假', 'OFFICIAL', 3),
('d6111000-0000-0000-0000-000000000003', '婚假', 'MARRIAGE', 4),
('d6111000-0000-0000-0000-000000000003', '喪假', 'BEREAVEMENT', 5),
('d6121000-0000-0000-0000-000000000001', '待簽核', 'PENDING', 0),
('d6121000-0000-0000-0000-000000000001', '已核准', 'APPROVED', 1),
('d6121000-0000-0000-0000-000000000001', '已退回', 'REJECTED', 2),
('d6121000-0000-0000-0000-000000000001', '已取消', 'CANCELLED', 3)
ON CONFLICT DO NOTHING;

-- ══════════════════════════════════════════════════════════════════════
-- OPERATION 7: 生產用料確認 (Production Material Consumption)
-- Module: PP | Type: FORM | Used by: 生產經理 張莉
-- ══════════════════════════════════════════════════════════════════════
INSERT INTO lc_operations (id, operation_code, project_id, name, description, operation_type, is_published, version, module, sidebar_icon, sidebar_sort_order)
VALUES ('d7000000-0000-0000-0000-000000000001', 'PP-CONSUME', 'd0000000-0000-0000-0000-000000000001', '生產用料確認', '記錄生產工單實際用料與產出，計算耗用差異', 'FORM', true, 1, 'PP', 'Factory', 30)
ON CONFLICT (operation_code) DO NOTHING;

INSERT INTO lc_form_definitions (id, operation_id) VALUES
('d7100000-0000-0000-0000-000000000001', 'd7000000-0000-0000-0000-000000000001')
ON CONFLICT (operation_id) DO NOTHING;

INSERT INTO lc_form_sections (id, form_id, title, columns, sort_order) VALUES
('d7110000-0000-0000-0000-000000000001', 'd7100000-0000-0000-0000-000000000001', '工單資訊', 2, 0),
('d7120000-0000-0000-0000-000000000001', 'd7100000-0000-0000-0000-000000000001', '用料記錄', 2, 1),
('d7130000-0000-0000-0000-000000000001', 'd7100000-0000-0000-0000-000000000001', '產出記錄', 2, 2)
ON CONFLICT DO NOTHING;

INSERT INTO lc_field_definitions (id, section_id, field_name, field_label, field_type, is_required, is_searchable, sort_order, column_span, field_config) VALUES
('d7111000-0000-0000-0000-000000000001', 'd7110000-0000-0000-0000-000000000001', 'production_order', '生產工單號', 'TEXT', true, true, 0, 1, '{}'),
('d7111000-0000-0000-0000-000000000002', 'd7110000-0000-0000-0000-000000000001', 'product_name', '產品名稱', 'TEXT', true, true, 1, 1, '{}'),
('d7111000-0000-0000-0000-000000000003', 'd7110000-0000-0000-0000-000000000001', 'work_date', '作業日期', 'DATE', true, false, 2, 1, '{}'),
('d7111000-0000-0000-0000-000000000004', 'd7110000-0000-0000-0000-000000000001', 'shift', '班別', 'DROPDOWN', true, false, 3, 1, '{}'),
('d7121000-0000-0000-0000-000000000001', 'd7120000-0000-0000-0000-000000000001', 'material_consumed', '投入原料', 'TEXT', true, false, 0, 1, '{}'),
('d7121000-0000-0000-0000-000000000002', 'd7120000-0000-0000-0000-000000000001', 'planned_qty', '計畫用量', 'NUMBER', true, false, 1, 1, '{}'),
('d7121000-0000-0000-0000-000000000003', 'd7120000-0000-0000-0000-000000000001', 'actual_qty', '實際用量', 'NUMBER', true, false, 2, 1, '{}'),
('d7121000-0000-0000-0000-000000000004', 'd7120000-0000-0000-0000-000000000001', 'variance', '差異', 'NUMBER', false, false, 3, 1, '{}'),
('d7131000-0000-0000-0000-000000000001', 'd7130000-0000-0000-0000-000000000001', 'output_qty', '產出數量', 'NUMBER', true, false, 0, 1, '{}'),
('d7131000-0000-0000-0000-000000000002', 'd7130000-0000-0000-0000-000000000001', 'defect_qty', '不良品數量', 'NUMBER', false, false, 1, 1, '{}'),
('d7131000-0000-0000-0000-000000000003', 'd7130000-0000-0000-0000-000000000001', 'yield_rate', '良率 (%)', 'NUMBER', false, false, 2, 1, '{}'),
('d7131000-0000-0000-0000-000000000004', 'd7130000-0000-0000-0000-000000000001', 'production_notes', '生產備註', 'TEXTAREA', false, false, 3, 2, '{}')
ON CONFLICT DO NOTHING;

INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
('d7111000-0000-0000-0000-000000000004', '早班', 'DAY', 0),
('d7111000-0000-0000-0000-000000000004', '晚班', 'NIGHT', 1),
('d7111000-0000-0000-0000-000000000004', '全天', 'FULL', 2)
ON CONFLICT DO NOTHING;

-- ══════════════════════════════════════════════════════════════════════
-- SAMPLE DATA: 各角色使用的測試資料
-- ══════════════════════════════════════════════════════════════════════

-- 王軍 (採購經理) 建立的 GRN
INSERT INTO lc_operation_data (id, operation_id, data, created_by, created_at) VALUES
('dd000000-0000-0000-0000-000000000001', 'd1000000-0000-0000-0000-000000000001',
 '{"grn_number":"GRN-001","po_number":"PO-00001","vendor_name":"台灣農產供應商","receipt_date":"2026-03-20","warehouse":"WH-001","status":"CONFIRMED","material_name":"馬鈴薯粉","ordered_qty":2000,"received_qty":1980,"rejected_qty":20,"batch_number":"BAT-2026031001","expiry_date":"2027-03-20","inspection_result":"PASS","notes":"20kg包裝破損退回"}',
 'a0000000-0000-0000-0000-000000000004', '2026-03-20 09:00:00+08'),
('dd000000-0000-0000-0000-000000000002', 'd1000000-0000-0000-0000-000000000001',
 '{"grn_number":"GRN-002","po_number":"PO-00002","vendor_name":"濃縮汁進口商","receipt_date":"2026-03-22","warehouse":"WH-002","status":"DRAFT","material_name":"葡萄濃縮汁","ordered_qty":1000,"received_qty":1000,"rejected_qty":0,"batch_number":"BAT-2026032201","expiry_date":"2027-06-30","inspection_result":"PASS","notes":"冷藏品，已入冷藏倉"}',
 'a0000000-0000-0000-0000-000000000004', '2026-03-22 10:30:00+08')
ON CONFLICT DO NOTHING;

-- 林美 (業務經理) 建立的出貨單
INSERT INTO lc_operation_data (id, operation_id, data, created_by, created_at) VALUES
('dd000000-0000-0000-0000-000000000003', 'd2000000-0000-0000-0000-000000000001',
 '{"delivery_number":"DLV-001","so_number":"SO-00001","customer_name":"全聯實業","delivery_date":"2026-03-25","status":"SHIPPED","material_name":"脆薯片 (原味) 150g","ordered_qty":10000,"shipped_qty":10000,"batch_number":"BAT-P2026032201","carrier":"新竹物流","tracking_number":"HCT-20260325-001","shipping_address":"台北市中山區民生東路100號","receiver_signature":""}',
 'a0000000-0000-0000-0000-000000000003', '2026-03-25 08:00:00+08')
ON CONFLICT DO NOTHING;

-- 王軍 建立的供應商評鑑
INSERT INTO lc_operation_data (id, operation_id, data, created_by, created_at) VALUES
('dd000000-0000-0000-0000-000000000004', 'd4000000-0000-0000-0000-000000000001',
 '{"vendor_name":"台灣農產供應商","eval_date":"2026-03-15","eval_period":"2026Q1","evaluator":"王軍","quality_score":85,"delivery_score":90,"service_score":80,"price_score":75,"overall_score":83,"rating":"A","excellence_note":"品質穩定，交期準時，建議續約","comments":"本季整體表現優良"}',
 'a0000000-0000-0000-0000-000000000004', '2026-03-15 14:00:00+08'),
('dd000000-0000-0000-0000-000000000005', 'd4000000-0000-0000-0000-000000000001',
 '{"vendor_name":"濃縮汁進口商","eval_date":"2026-03-15","eval_period":"2026Q1","evaluator":"王軍","quality_score":70,"delivery_score":55,"service_score":60,"price_score":80,"overall_score":66,"rating":"B","comments":"交期需改善，上季兩次延遲交貨"}',
 'a0000000-0000-0000-0000-000000000004', '2026-03-15 15:00:00+08'),
('dd000000-0000-0000-0000-000000000006', 'd4000000-0000-0000-0000-000000000001',
 '{"vendor_name":"劣質包材商","eval_date":"2026-03-15","eval_period":"2026Q1","evaluator":"王軍","quality_score":40,"delivery_score":50,"service_score":35,"price_score":90,"overall_score":54,"rating":"D","improvement_plan":"1. 品質：需提供出廠檢驗報告\n2. 交期：需改善至少95%準時率\n3. 服務：指定專人對應窗口\n4. 限期3個月內改善，否則終止合作","comments":"品質與服務嚴重不合格，已發出改善通知"}',
 'a0000000-0000-0000-0000-000000000004', '2026-03-15 16:00:00+08')
ON CONFLICT DO NOTHING;

-- 劉品管 建立的品檢記錄
INSERT INTO lc_operation_data (id, operation_id, data, created_by, created_at) VALUES
('dd000000-0000-0000-0000-000000000007', 'd5000000-0000-0000-0000-000000000001',
 '{"inspection_number":"QC-001","inspection_type":"INCOMING","material_name":"馬鈴薯粉","batch_number":"BAT-2026031001","sample_size":50,"inspection_date":"2026-03-20","appearance":"PASS","weight_check":"PASS","taste_check":"PASS","packaging_check":"FAIL","defect_count":2,"defect_description":"2袋外包裝破損，內容物未受影響","overall_result":"CONDITIONAL","disposition":"CONCESSION","inspector_notes":"破損品另行標記，優先使用"}',
 'a0000000-0000-0000-0000-000000000007', '2026-03-20 10:00:00+08')
ON CONFLICT DO NOTHING;

-- 請假申請
INSERT INTO lc_operation_data (id, operation_id, data, created_by, created_at) VALUES
('dd000000-0000-0000-0000-000000000008', 'd6000000-0000-0000-0000-000000000001',
 '{"employee_name":"林美","department":"業務部","leave_type":"ANNUAL","start_date":"2026-04-01","end_date":"2026-04-03","days":3,"reason":"清明節連假出遊","status":"APPROVED","approver":"陳偉","approval_notes":"准假"}',
 'a0000000-0000-0000-0000-000000000003', '2026-03-18 09:00:00+08'),
('dd000000-0000-0000-0000-000000000009', 'd6000000-0000-0000-0000-000000000001',
 '{"employee_name":"王軍","department":"採購部","leave_type":"SICK","start_date":"2026-03-19","end_date":"2026-03-19","days":1,"reason":"身體不適","status":"PENDING","approver":"陳偉","approval_notes":""}',
 'a0000000-0000-0000-0000-000000000004', '2026-03-19 07:30:00+08')
ON CONFLICT DO NOTHING;

-- 生產用料確認
INSERT INTO lc_operation_data (id, operation_id, data, created_by, created_at) VALUES
('dd000000-0000-0000-0000-000000000010', 'd7000000-0000-0000-0000-000000000001',
 '{"production_order":"PRD-00001","product_name":"脆薯片 (原味) 150g","work_date":"2026-03-22","shift":"DAY","material_consumed":"馬鈴薯粉","planned_qty":100,"actual_qty":105,"variance":5,"output_qty":980,"defect_qty":20,"yield_rate":98,"production_notes":"馬鈴薯粉含水率略高，用量增加5kg"}',
 'a0000000-0000-0000-0000-000000000005', '2026-03-22 17:00:00+08')
ON CONFLICT DO NOTHING;

-- 庫存異動
INSERT INTO lc_operation_data (id, operation_id, data, created_by, created_at) VALUES
('dd000000-0000-0000-0000-000000000011', 'd3000000-0000-0000-0000-000000000001',
 '{"movement_type":"GR_PO","material_name":"馬鈴薯粉","quantity":1980,"unit":"KG","from_warehouse":"","to_warehouse":"WH-001","batch_number":"BAT-2026031001","reference_doc":"GRN-001","movement_date":"2026-03-20","reason":"採購收貨入庫"}',
 'a0000000-0000-0000-0000-000000000004', '2026-03-20 11:00:00+08'),
('dd000000-0000-0000-0000-000000000012', 'd3000000-0000-0000-0000-000000000001',
 '{"movement_type":"GI_PROD","material_name":"馬鈴薯粉","quantity":105,"unit":"KG","from_warehouse":"WH-001","to_warehouse":"","batch_number":"BAT-2026031001","reference_doc":"PRD-00001","movement_date":"2026-03-22","reason":"生產領料 - 脆薯片原味"}',
 'a0000000-0000-0000-0000-000000000005', '2026-03-22 08:00:00+08'),
('dd000000-0000-0000-0000-000000000013', 'd3000000-0000-0000-0000-000000000001',
 '{"movement_type":"GR_PROD","material_name":"脆薯片 (原味) 150g","quantity":980,"unit":"PCS","from_warehouse":"","to_warehouse":"WH-001","batch_number":"BAT-P2026032201","reference_doc":"PRD-00001","movement_date":"2026-03-22","reason":"生產入庫 - 脆薯片原味 (良品980/不良品20)"}',
 'a0000000-0000-0000-0000-000000000005', '2026-03-22 17:30:00+08'),
('dd000000-0000-0000-0000-000000000014', 'd3000000-0000-0000-0000-000000000001',
 '{"movement_type":"GI_SO","material_name":"脆薯片 (原味) 150g","quantity":10000,"unit":"PCS","from_warehouse":"WH-001","to_warehouse":"","batch_number":"BAT-P2026032201","reference_doc":"DLV-001","movement_date":"2026-03-25","reason":"銷售出貨 - 全聯實業 SO-00001"}',
 'a0000000-0000-0000-0000-000000000004', '2026-03-25 07:30:00+08')
ON CONFLICT DO NOTHING;
