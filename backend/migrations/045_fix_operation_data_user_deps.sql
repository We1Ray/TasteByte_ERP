-- 045_fix_operation_data_user_deps.sql
-- Fix: migration 038 inserts lc_operation_data rows referencing test users that
-- are only created in migration 041. On a clean migration run, 038's sample data
-- section is skipped (guarded by a user-existence check). This migration ensures
-- the required users exist, then re-inserts the sample data with ON CONFLICT DO
-- NOTHING so it is idempotent regardless of execution order.

-- ============================================================
-- 1. Ensure required test users exist
-- ============================================================
DO $$
DECLARE
    v_pwd_hash TEXT := '$argon2id$v=19$m=19456,t=2,p=1$YWRtaW4xMjNzYWx0$JCe3F3pO7VPMqnLPSxcdYXaj7b1VFQhFOAaXlFmv77k';
BEGIN
    INSERT INTO users (id, username, email, password_hash, display_name, is_active) VALUES
        ('a0000000-0000-0000-0000-000000000003', 'lin.mei', 'lin.mei@tastebyte.com', v_pwd_hash, 'Lin Mei (Sales)', true),
        ('a0000000-0000-0000-0000-000000000004', 'wang.jun', 'wang.jun@tastebyte.com', v_pwd_hash, 'Wang Jun (Procurement)', true),
        ('a0000000-0000-0000-0000-000000000005', 'zhang.li', 'zhang.li@tastebyte.com', v_pwd_hash, 'Zhang Li (Production)', true),
        ('a0000000-0000-0000-0000-000000000007', 'liu.qm', 'liu.qm@tastebyte.com', v_pwd_hash, 'Liu QM (Quality)', true)
    ON CONFLICT (username) DO NOTHING;
END $$;

-- ============================================================
-- 2. Insert lc_operation_data sample rows (originally from migration 038)
-- ============================================================

-- GRN records (wang.jun)
INSERT INTO lc_operation_data (id, operation_id, data, created_by, created_at) VALUES
('dd000000-0000-0000-0000-000000000001', 'd1000000-0000-0000-0000-000000000001',
 '{"grn_number":"GRN-001","po_number":"PO-00001","vendor_name":"台灣農產供應商","receipt_date":"2026-03-20","warehouse":"WH-001","status":"CONFIRMED","material_name":"馬鈴薯粉","ordered_qty":2000,"received_qty":1980,"rejected_qty":20,"batch_number":"BAT-2026031001","expiry_date":"2027-03-20","inspection_result":"PASS","notes":"20kg包裝破損退回"}',
 'a0000000-0000-0000-0000-000000000004', '2026-03-20 09:00:00+08'),
('dd000000-0000-0000-0000-000000000002', 'd1000000-0000-0000-0000-000000000001',
 '{"grn_number":"GRN-002","po_number":"PO-00002","vendor_name":"濃縮汁進口商","receipt_date":"2026-03-22","warehouse":"WH-002","status":"DRAFT","material_name":"葡萄濃縮汁","ordered_qty":1000,"received_qty":1000,"rejected_qty":0,"batch_number":"BAT-2026032201","expiry_date":"2027-06-30","inspection_result":"PASS","notes":"冷藏品，已入冷藏倉"}',
 'a0000000-0000-0000-0000-000000000004', '2026-03-22 10:30:00+08')
ON CONFLICT DO NOTHING;

-- Delivery note (lin.mei)
INSERT INTO lc_operation_data (id, operation_id, data, created_by, created_at) VALUES
('dd000000-0000-0000-0000-000000000003', 'd2000000-0000-0000-0000-000000000001',
 '{"delivery_number":"DLV-001","so_number":"SO-00001","customer_name":"全聯實業","delivery_date":"2026-03-25","status":"SHIPPED","material_name":"脆薯片 (原味) 150g","ordered_qty":10000,"shipped_qty":10000,"batch_number":"BAT-P2026032201","carrier":"新竹物流","tracking_number":"HCT-20260325-001","shipping_address":"台北市中山區民生東路100號","receiver_signature":""}',
 'a0000000-0000-0000-0000-000000000003', '2026-03-25 08:00:00+08')
ON CONFLICT DO NOTHING;

-- Vendor evaluations (wang.jun)
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

-- Quality inspection (liu.qm)
INSERT INTO lc_operation_data (id, operation_id, data, created_by, created_at) VALUES
('dd000000-0000-0000-0000-000000000007', 'd5000000-0000-0000-0000-000000000001',
 '{"inspection_number":"QC-001","inspection_type":"INCOMING","material_name":"馬鈴薯粉","batch_number":"BAT-2026031001","sample_size":50,"inspection_date":"2026-03-20","appearance":"PASS","weight_check":"PASS","taste_check":"PASS","packaging_check":"FAIL","defect_count":2,"defect_description":"2袋外包裝破損，內容物未受影響","overall_result":"CONDITIONAL","disposition":"CONCESSION","inspector_notes":"破損品另行標記，優先使用"}',
 'a0000000-0000-0000-0000-000000000007', '2026-03-20 10:00:00+08')
ON CONFLICT DO NOTHING;

-- Leave requests (lin.mei, wang.jun)
INSERT INTO lc_operation_data (id, operation_id, data, created_by, created_at) VALUES
('dd000000-0000-0000-0000-000000000008', 'd6000000-0000-0000-0000-000000000001',
 '{"employee_name":"林美","department":"業務部","leave_type":"ANNUAL","start_date":"2026-04-01","end_date":"2026-04-03","days":3,"reason":"清明節連假出遊","status":"APPROVED","approver":"陳偉","approval_notes":"准假"}',
 'a0000000-0000-0000-0000-000000000003', '2026-03-18 09:00:00+08'),
('dd000000-0000-0000-0000-000000000009', 'd6000000-0000-0000-0000-000000000001',
 '{"employee_name":"王軍","department":"採購部","leave_type":"SICK","start_date":"2026-03-19","end_date":"2026-03-19","days":1,"reason":"身體不適","status":"PENDING","approver":"陳偉","approval_notes":""}',
 'a0000000-0000-0000-0000-000000000004', '2026-03-19 07:30:00+08')
ON CONFLICT DO NOTHING;

-- Production consumption (zhang.li)
INSERT INTO lc_operation_data (id, operation_id, data, created_by, created_at) VALUES
('dd000000-0000-0000-0000-000000000010', 'd7000000-0000-0000-0000-000000000001',
 '{"production_order":"PRD-00001","product_name":"脆薯片 (原味) 150g","work_date":"2026-03-22","shift":"DAY","material_consumed":"馬鈴薯粉","planned_qty":100,"actual_qty":105,"variance":5,"output_qty":980,"defect_qty":20,"yield_rate":98,"production_notes":"馬鈴薯粉含水率略高，用量增加5kg"}',
 'a0000000-0000-0000-0000-000000000005', '2026-03-22 17:00:00+08')
ON CONFLICT DO NOTHING;

-- Inventory movements (wang.jun, zhang.li)
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
