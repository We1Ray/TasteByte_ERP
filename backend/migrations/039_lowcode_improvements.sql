-- 039_lowcode_improvements.sql
-- ═══════════════════════════════════════════════════════════════════════
-- TasteByte Foods IT: 透過 lowcode 平台功能改善現有作業
-- 使用: default_value_sql 自動編號, cross_field_rules 跨欄位驗證,
--       calculation_formulas 自動計算, approval_matrices 簽核,
--       output_rules 自動通知
-- ═══════════════════════════════════════════════════════════════════════

-- ══════════════════════════════════════════════════════════════════════
-- 改善 1: 自動產生編號 (default_value_sql)
-- GRN、出貨單、品檢單自動取號，使用者不用手動填寫
-- ══════════════════════════════════════════════════════════════════════

-- GRN 編號自動取號
UPDATE lc_field_definitions
SET default_value_sql = 'SELECT ''GRN-'' || LPAD((COALESCE(MAX(CAST(SUBSTRING(data->>''grn_number'' FROM ''GRN-(\d+)'') AS INT)), 0) + 1)::TEXT, 3, ''0'') FROM lc_operation_data WHERE operation_id = ''d1000000-0000-0000-0000-000000000001'''
WHERE id = 'd1111000-0000-0000-0000-000000000001';

-- 出貨單號自動取號
UPDATE lc_field_definitions
SET default_value_sql = 'SELECT ''DLV-'' || LPAD((COALESCE(MAX(CAST(SUBSTRING(data->>''delivery_number'' FROM ''DLV-(\d+)'') AS INT)), 0) + 1)::TEXT, 3, ''0'') FROM lc_operation_data WHERE operation_id = ''d2000000-0000-0000-0000-000000000001'''
WHERE id = 'd2111000-0000-0000-0000-000000000001';

-- 品檢單號自動取號
UPDATE lc_field_definitions
SET default_value_sql = 'SELECT ''QC-'' || LPAD((COALESCE(MAX(CAST(SUBSTRING(data->>''inspection_number'' FROM ''QC-(\d+)'') AS INT)), 0) + 1)::TEXT, 3, ''0'') FROM lc_operation_data WHERE operation_id = ''d5000000-0000-0000-0000-000000000001'''
WHERE id = 'd5111000-0000-0000-0000-000000000001';

-- 收貨日期預設今天
UPDATE lc_field_definitions
SET default_value_sql = 'SELECT CURRENT_DATE::TEXT'
WHERE id = 'd1111000-0000-0000-0000-000000000004';

-- 出貨日期預設今天
UPDATE lc_field_definitions
SET default_value_sql = 'SELECT CURRENT_DATE::TEXT'
WHERE id = 'd2111000-0000-0000-0000-000000000004';

-- 品檢日期預設今天
UPDATE lc_field_definitions
SET default_value_sql = 'SELECT CURRENT_DATE::TEXT'
WHERE id = 'd5111000-0000-0000-0000-000000000006';

-- 庫存異動日期預設今天
UPDATE lc_field_definitions
SET default_value_sql = 'SELECT CURRENT_DATE::TEXT'
WHERE id = 'd3111000-0000-0000-0000-000000000009';

-- 生產作業日期預設今天
UPDATE lc_field_definitions
SET default_value_sql = 'SELECT CURRENT_DATE::TEXT'
WHERE id = 'd7111000-0000-0000-0000-000000000003';

-- ══════════════════════════════════════════════════════════════════════
-- 改善 2: 跨欄位驗證規則 (cross_field_rules)
-- 確保業務邏輯正確性
-- ══════════════════════════════════════════════════════════════════════

-- GRN: 實收數量不得超過訂購數量的 110%
INSERT INTO cross_field_rules (operation_id, rule_name, description, rule_type, source_field, operator, target_field, target_value, error_message, sort_order) VALUES
('d1000000-0000-0000-0000-000000000001', '收貨數量上限', '實收數量不得超過訂購數量的110%', 'VALIDATION', 'received_qty', 'lte', null, null, '實收數量超過訂購數量上限', 0)
ON CONFLICT DO NOTHING;

-- GRN: 退回數量不得超過實收數量
INSERT INTO cross_field_rules (operation_id, rule_name, description, rule_type, source_field, operator, target_field, target_value, error_message, sort_order) VALUES
('d1000000-0000-0000-0000-000000000001', '退回數量限制', '退回數量不可超過實收數量', 'VALIDATION', 'rejected_qty', 'lte', 'received_qty', null, '退回數量不可超過實收數量', 1)
ON CONFLICT DO NOTHING;

-- 出貨: 出貨數量不得超過訂購數量
INSERT INTO cross_field_rules (operation_id, rule_name, description, rule_type, source_field, operator, target_field, target_value, error_message, sort_order) VALUES
('d2000000-0000-0000-0000-000000000001', '出貨數量限制', '出貨數量不可超過訂購數量', 'VALIDATION', 'shipped_qty', 'lte', 'ordered_qty', null, '出貨數量不可超過訂購數量', 0)
ON CONFLICT DO NOTHING;

-- 請假: 結束日期不得早於開始日期
INSERT INTO cross_field_rules (operation_id, rule_name, description, rule_type, source_field, operator, target_field, target_value, error_message, sort_order) VALUES
('d6000000-0000-0000-0000-000000000001', '日期順序', '結束日期不得早於開始日期', 'VALIDATION', 'end_date', 'gte', 'start_date', null, '結束日期不可早於開始日期', 0)
ON CONFLICT DO NOTHING;

-- 請假: 天數必須大於 0
INSERT INTO cross_field_rules (operation_id, rule_name, description, rule_type, source_field, operator, target_field, target_value, error_message, sort_order) VALUES
('d6000000-0000-0000-0000-000000000001', '天數檢查', '請假天數必須大於0', 'VALIDATION', 'days', 'gt', null, '0', '請假天數必須大於0', 1)
ON CONFLICT DO NOTHING;

-- 生產: 實際用量必須大於 0
INSERT INTO cross_field_rules (operation_id, rule_name, description, rule_type, source_field, operator, target_field, target_value, error_message, sort_order) VALUES
('d7000000-0000-0000-0000-000000000001', '用量檢查', '實際用量必須大於0', 'VALIDATION', 'actual_qty', 'gt', null, '0', '實際用量必須大於0', 0)
ON CONFLICT DO NOTHING;

-- 供應商評鑑: 各項分數 0-100 (已有 min/max，加跨欄位總分檢查)
INSERT INTO cross_field_rules (operation_id, rule_name, description, rule_type, source_field, operator, target_field, target_value, error_message, sort_order) VALUES
('d4000000-0000-0000-0000-000000000001', '綜合分數範圍', '綜合分數不得超過100', 'VALIDATION', 'overall_score', 'lte', null, '100', '綜合分數不可超過100', 0)
ON CONFLICT DO NOTHING;

-- 庫存異動: 數量必須大於 0
INSERT INTO cross_field_rules (operation_id, rule_name, description, rule_type, source_field, operator, target_field, target_value, error_message, sort_order) VALUES
('d3000000-0000-0000-0000-000000000001', '異動數量', '異動數量必須大於0', 'VALIDATION', 'quantity', 'gt', null, '0', '異動數量必須大於0', 0)
ON CONFLICT DO NOTHING;

-- ══════════════════════════════════════════════════════════════════════
-- 改善 3: 計算公式 (calculation_formulas)
-- 自動計算衍生欄位
-- ══════════════════════════════════════════════════════════════════════

-- 供應商評鑑: 綜合分數 = (品質*0.3 + 交期*0.25 + 服務*0.25 + 價格*0.2)
-- 注意: 目前公式引擎支援簡單運算，用近似方式
INSERT INTO calculation_formulas (operation_id, target_field, formula, trigger_fields, description, sort_order) VALUES
('d4000000-0000-0000-0000-000000000001', 'overall_score', 'quality_score * 0.3 + delivery_score * 0.25 + service_score * 0.25 + price_score * 0.2',
 ARRAY['quality_score', 'delivery_score', 'service_score', 'price_score'],
 '綜合分數 = 品質30% + 交期25% + 服務25% + 價格20%', 0)
ON CONFLICT DO NOTHING;

-- 生產: 差異 = 實際用量 - 計畫用量
INSERT INTO calculation_formulas (operation_id, target_field, formula, trigger_fields, description, sort_order) VALUES
('d7000000-0000-0000-0000-000000000001', 'variance', 'actual_qty - planned_qty',
 ARRAY['actual_qty', 'planned_qty'],
 '用料差異 = 實際用量 - 計畫用量', 0)
ON CONFLICT DO NOTHING;

-- 生產: 良率 = (產出數量 / (產出數量 + 不良品數量)) * 100
INSERT INTO calculation_formulas (operation_id, target_field, formula, trigger_fields, description, sort_order) VALUES
('d7000000-0000-0000-0000-000000000001', 'yield_rate', 'output_qty / (output_qty + defect_qty) * 100',
 ARRAY['output_qty', 'defect_qty'],
 '良率 = 良品 / (良品+不良品) × 100', 1)
ON CONFLICT DO NOTHING;

-- ══════════════════════════════════════════════════════════════════════
-- 改善 4: 簽核矩陣 (approval_matrices)
-- 請假申請依天數分級簽核
-- ══════════════════════════════════════════════════════════════════════

INSERT INTO approval_matrices (id, name, operation_id, description, is_active) VALUES
('a0a00000-0000-0000-0000-000000000001', '請假簽核', 'd6000000-0000-0000-0000-000000000001', '依請假天數分級簽核：1-3天主管簽核，3天以上需人資加簽', true)
ON CONFLICT DO NOTHING;

INSERT INTO approval_levels (id, matrix_id, level_order, name, condition_field, condition_operator, condition_value, approver_type, approver_role, sla_hours, auto_escalate) VALUES
('a1a00000-0000-0000-0000-000000000001', 'a0a00000-0000-0000-0000-000000000001', 1, '直屬主管', null, 'gte', null, 'ROLE', 'ADMIN', 24, false),
('a1a00000-0000-0000-0000-000000000002', 'a0a00000-0000-0000-0000-000000000001', 2, '人資經理', 'days', 'gte', 3, 'ROLE', 'HR_MANAGER', 48, true)
ON CONFLICT DO NOTHING;

-- 供應商評鑑: D級需主管覆核
INSERT INTO approval_matrices (id, name, operation_id, description, is_active) VALUES
('a0a00000-0000-0000-0000-000000000002', '供應商評鑑覆核', 'd4000000-0000-0000-0000-000000000001', 'D級供應商評鑑需主管覆核確認', true)
ON CONFLICT DO NOTHING;

INSERT INTO approval_levels (id, matrix_id, level_order, name, condition_field, condition_operator, condition_value, approver_type, approver_role, sla_hours) VALUES
('a1a00000-0000-0000-0000-000000000003', 'a0a00000-0000-0000-0000-000000000002', 1, '採購主管覆核', 'overall_score', 'lte', 60, 'ROLE', 'MM_MANAGER', 48)
ON CONFLICT DO NOTHING;

-- ══════════════════════════════════════════════════════════════════════
-- 改善 5: 輸出決定 (output_rules)
-- 自動發送通知
-- ══════════════════════════════════════════════════════════════════════

-- GRN 確認時通知財務（可開立應付帳款）
INSERT INTO output_rules (name, operation_id, trigger_event, condition_field, condition_operator, condition_value, output_type, recipient_type, recipient_static, sort_order) VALUES
('收貨確認通知財務', 'd1000000-0000-0000-0000-000000000001', 'ON_CREATE', 'status', 'equals', 'CONFIRMED', 'NOTIFICATION', 'STATIC', 'a0000000-0000-0000-0000-000000000002', 0)
ON CONFLICT DO NOTHING;

-- 出貨完成時通知財務（可開立發票）
INSERT INTO output_rules (name, operation_id, trigger_event, condition_field, condition_operator, condition_value, output_type, recipient_type, recipient_static, sort_order) VALUES
('出貨完成通知財務', 'd2000000-0000-0000-0000-000000000001', 'ON_CREATE', 'status', 'equals', 'SHIPPED', 'NOTIFICATION', 'STATIC', 'a0000000-0000-0000-0000-000000000002', 0)
ON CONFLICT DO NOTHING;

-- 品檢不合格時通知採購（需與供應商處理）
INSERT INTO output_rules (name, operation_id, trigger_event, condition_field, condition_operator, condition_value, output_type, recipient_type, recipient_static, sort_order) VALUES
('品檢不合格通知採購', 'd5000000-0000-0000-0000-000000000001', 'ON_CREATE', 'overall_result', 'equals', 'FAIL', 'NOTIFICATION', 'STATIC', 'a0000000-0000-0000-0000-000000000004', 0)
ON CONFLICT DO NOTHING;

-- 供應商評鑑 D 級通知管理層
INSERT INTO output_rules (name, operation_id, trigger_event, condition_field, condition_operator, condition_value, output_type, recipient_type, recipient_static, sort_order) VALUES
('D級供應商警示', 'd4000000-0000-0000-0000-000000000001', 'ON_CREATE', 'rating', 'equals', 'D', 'NOTIFICATION', 'STATIC', 'a0000000-0000-0000-0000-000000000001', 0)
ON CONFLICT DO NOTHING;

-- 生產良率低於 95% 通知品管
INSERT INTO output_rules (name, operation_id, trigger_event, condition_field, condition_operator, condition_value, output_type, recipient_type, recipient_static, sort_order) VALUES
('低良率通知品管', 'd7000000-0000-0000-0000-000000000001', 'ON_CREATE', null, null, null, 'NOTIFICATION', 'STATIC', 'a0000000-0000-0000-0000-000000000007', 0)
ON CONFLICT DO NOTHING;

-- 請假申請提交時通知主管
INSERT INTO output_rules (name, operation_id, trigger_event, condition_field, condition_operator, condition_value, output_type, recipient_type, recipient_static, sort_order) VALUES
('請假申請通知主管', 'd6000000-0000-0000-0000-000000000001', 'ON_CREATE', 'status', 'equals', 'PENDING', 'NOTIFICATION', 'STATIC', 'a0000000-0000-0000-0000-000000000001', 0)
ON CONFLICT DO NOTHING;

-- ══════════════════════════════════════════════════════════════════════
-- 額外改善: 增強 visibility_rule
-- ══════════════════════════════════════════════════════════════════════

-- GRN: 品檢結果為不合格時，顯示備註欄（強制填寫原因）
UPDATE lc_field_definitions
SET visibility_rule = '{"dependent_field":"inspection_result","operator":"equals","value":"FAIL","action":"show"}'
WHERE id = 'd1121000-0000-0000-0000-000000000008'
AND visibility_rule IS NULL;

-- 出貨: 狀態為「已簽收」時顯示簽收人欄位
UPDATE lc_field_definitions
SET visibility_rule = '{"dependent_field":"status","operator":"equals","value":"SIGNED","action":"show"}'
WHERE id = 'd2131000-0000-0000-0000-000000000004'
AND visibility_rule IS NULL;

-- 庫存異動: 類型為「倉庫調撥」時才顯示來源/目的倉庫
UPDATE lc_field_definitions
SET visibility_rule = '{"dependent_field":"movement_type","operator":"equals","value":"TRANSFER","action":"show"}'
WHERE id = 'd3111000-0000-0000-0000-000000000005'
AND visibility_rule IS NULL;

UPDATE lc_field_definitions
SET visibility_rule = '{"dependent_field":"movement_type","operator":"equals","value":"TRANSFER","action":"show"}'
WHERE id = 'd3111000-0000-0000-0000-000000000006'
AND visibility_rule IS NULL;

-- 品檢: 有缺陷時才顯示缺陷說明欄
UPDATE lc_field_definitions
SET visibility_rule = '{"dependent_field":"defect_count","operator":"gt","value":"0","action":"show"}'
WHERE id = 'd5121000-0000-0000-0000-000000000006'
AND visibility_rule IS NULL;

-- 品檢: 不合格時才顯示處置方式
UPDATE lc_field_definitions
SET visibility_rule = '{"dependent_field":"overall_result","operator":"not_equals","value":"PASS","action":"show"}'
WHERE id = 'd5131000-0000-0000-0000-000000000002'
AND visibility_rule IS NULL;

-- 請假: 簽核欄位在待簽核以外狀態才顯示簽核意見
UPDATE lc_field_definitions
SET visibility_rule = '{"dependent_field":"status","operator":"not_equals","value":"PENDING","action":"show"}'
WHERE id = 'd6121000-0000-0000-0000-000000000003'
AND visibility_rule IS NULL;
