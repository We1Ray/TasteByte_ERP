-- 043_100_operations_bulk.sql
-- Bulk create 100+ lowcode operations across all modules using helper function
-- Each operation: form definition + sections + fields + options + sample data

BEGIN;

-- Helper: Create a complete operation with one call
CREATE OR REPLACE FUNCTION _create_op(
    p_code TEXT, p_name TEXT, p_desc TEXT, p_module TEXT, p_icon TEXT, p_sort INT,
    p_fields JSONB -- array of {section, field_name, label, type, required, searchable, options[], config}
) RETURNS UUID AS $$
DECLARE
    v_op_id UUID := gen_random_uuid();
    v_form_id UUID := gen_random_uuid();
    v_proj_id UUID := 'd0000000-0000-0000-0000-000000000001'::UUID;
    v_sec_id UUID;
    v_fld_id UUID;
    v_sec TEXT;
    v_last_sec TEXT := '';
    v_sec_idx INT := 0;
    v_fld_idx INT := 0;
    v_field JSONB;
    v_opt JSONB;
    v_opt_idx INT;
BEGIN
    -- Operation
    INSERT INTO lc_operations (id,operation_code,project_id,name,description,operation_type,is_published,version,module,sidebar_icon,sidebar_sort_order,is_yaml_managed)
    VALUES (v_op_id, p_code, v_proj_id, p_name, p_desc, 'FORM', true, 1, p_module, p_icon, p_sort, false)
    ON CONFLICT (operation_code) DO UPDATE SET name=p_name, description=p_desc, updated_at=NOW()
    RETURNING id INTO v_op_id;

    -- Form definition
    INSERT INTO lc_form_definitions (id, operation_id) VALUES (v_form_id, v_op_id)
    ON CONFLICT (operation_id) DO NOTHING;
    SELECT id INTO v_form_id FROM lc_form_definitions WHERE operation_id = v_op_id;

    -- Process fields (grouped by section)
    FOR v_field IN SELECT * FROM jsonb_array_elements(p_fields) LOOP
        v_sec := v_field->>'section';
        IF v_sec != v_last_sec THEN
            v_sec_id := gen_random_uuid();
            v_fld_idx := 0;
            INSERT INTO lc_form_sections (id, form_id, title, columns, sort_order)
            VALUES (v_sec_id, v_form_id, v_sec, 2, v_sec_idx)
            ON CONFLICT DO NOTHING;
            v_last_sec := v_sec;
            v_sec_idx := v_sec_idx + 1;
        END IF;

        v_fld_id := gen_random_uuid();
        INSERT INTO lc_field_definitions (id, section_id, field_name, field_label, field_type,
            is_required, is_searchable, sort_order, column_span, default_value,
            min_value, max_value, visibility_rule, field_config)
        VALUES (v_fld_id, v_sec_id,
            v_field->>'field_name', v_field->>'label', COALESCE(v_field->>'type','TEXT'),
            COALESCE((v_field->>'required')::boolean, false),
            COALESCE((v_field->>'searchable')::boolean, false),
            v_fld_idx, COALESCE((v_field->>'span')::int, 1),
            v_field->>'default_value',
            (v_field->>'min')::numeric, (v_field->>'max')::numeric,
            v_field->'visibility_rule', COALESCE(v_field->'config', '{}')
        ) ON CONFLICT (section_id, field_name) DO NOTHING;

        -- Options
        IF v_field->'options' IS NOT NULL THEN
            v_opt_idx := 0;
            FOR v_opt IN SELECT * FROM jsonb_array_elements(v_field->'options') LOOP
                INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order, is_default)
                VALUES (v_fld_id, v_opt->>'label', v_opt->>'value', v_opt_idx,
                    COALESCE((v_opt->>'is_default')::boolean, false))
                ON CONFLICT DO NOTHING;
                v_opt_idx := v_opt_idx + 1;
            END LOOP;
        END IF;
        v_fld_idx := v_fld_idx + 1;
    END LOOP;

    RETURN v_op_id;
END;
$$ LANGUAGE plpgsql;

-- Helper: Insert sample data
CREATE OR REPLACE FUNCTION _sample(p_code TEXT, p_data JSONB) RETURNS VOID AS $$
BEGIN
    INSERT INTO lc_operation_data (operation_id, data, created_by)
    SELECT o.id, p_data, 'a0000000-0000-0000-0000-000000000004'::UUID
    FROM lc_operations o WHERE o.operation_code = p_code;
END;
$$ LANGUAGE plpgsql;

-- ═══════════════════════════════════════════════════════════════════
-- FI MODULE (12 operations)
-- ═══════════════════════════════════════════════════════════════════

SELECT _create_op('FI-AP-INV','應付發票登錄','登錄供應商發票，對應採購單驗證','FI','FileText',10, '[
  {"section":"發票資訊","field_name":"invoice_no","label":"發票號碼","type":"TEXT","required":true,"searchable":true},
  {"section":"發票資訊","field_name":"vendor","label":"供應商","type":"TEXT","required":true,"searchable":true},
  {"section":"發票資訊","field_name":"invoice_date","label":"發票日期","type":"DATE","required":true},
  {"section":"發票資訊","field_name":"due_date","label":"到期日","type":"DATE","required":true},
  {"section":"發票資訊","field_name":"po_reference","label":"採購單號","type":"TEXT","searchable":true},
  {"section":"金額","field_name":"amount","label":"發票金額","type":"NUMBER","required":true},
  {"section":"金額","field_name":"tax_amount","label":"稅額","type":"NUMBER"},
  {"section":"金額","field_name":"net_amount","label":"淨額","type":"NUMBER"},
  {"section":"金額","field_name":"currency","label":"幣別","type":"DROPDOWN","options":[{"label":"TWD","value":"TWD","is_default":"true"},{"label":"USD","value":"USD"},{"label":"EUR","value":"EUR"}]},
  {"section":"狀態","field_name":"status","label":"狀態","type":"DROPDOWN","required":true,"searchable":true,"options":[{"label":"草稿","value":"DRAFT","is_default":"true"},{"label":"已過帳","value":"POSTED"},{"label":"已付款","value":"PAID"}]}
]'::jsonb);
SELECT _sample('FI-AP-INV','{"invoice_no":"INV-V-001","vendor":"台灣農產供應商","invoice_date":"2026-03-20","due_date":"2026-04-20","po_reference":"PO-00001","amount":320000,"tax_amount":16000,"net_amount":304000,"currency":"TWD","status":"POSTED"}');
SELECT _sample('FI-AP-INV','{"invoice_no":"INV-V-002","vendor":"濃縮汁進口商","invoice_date":"2026-03-22","due_date":"2026-05-22","po_reference":"PO-00002","amount":280000,"tax_amount":14000,"net_amount":266000,"currency":"TWD","status":"DRAFT"}');

SELECT _create_op('FI-AP-PAY','付款作業','對供應商進行付款','FI','Banknote',11, '[
  {"section":"付款資訊","field_name":"payment_no","label":"付款編號","type":"TEXT","required":true,"searchable":true},
  {"section":"付款資訊","field_name":"vendor","label":"供應商","type":"TEXT","required":true,"searchable":true},
  {"section":"付款資訊","field_name":"payment_date","label":"付款日期","type":"DATE","required":true},
  {"section":"付款資訊","field_name":"amount","label":"付款金額","type":"NUMBER","required":true},
  {"section":"付款資訊","field_name":"currency","label":"幣別","type":"DROPDOWN","options":[{"label":"TWD","value":"TWD","is_default":"true"},{"label":"USD","value":"USD"}]},
  {"section":"付款資訊","field_name":"payment_method","label":"付款方式","type":"DROPDOWN","required":true,"options":[{"label":"銀行轉帳","value":"TRANSFER","is_default":"true"},{"label":"支票","value":"CHECK"},{"label":"現金","value":"CASH"}]},
  {"section":"付款資訊","field_name":"bank_account","label":"銀行帳號","type":"TEXT"},
  {"section":"付款資訊","field_name":"reference","label":"參考單號","type":"TEXT","searchable":true},
  {"section":"付款資訊","field_name":"status","label":"狀態","type":"DROPDOWN","required":true,"options":[{"label":"待處理","value":"PENDING","is_default":"true"},{"label":"已完成","value":"COMPLETED"},{"label":"已取消","value":"CANCELLED"}]},
  {"section":"付款資訊","field_name":"notes","label":"備註","type":"TEXTAREA"}
]'::jsonb);
SELECT _sample('FI-AP-PAY','{"payment_no":"PAY-001","vendor":"台灣農產供應商","payment_date":"2026-04-15","amount":320000,"currency":"TWD","payment_method":"TRANSFER","bank_account":"012-345678-9","reference":"INV-V-001","status":"COMPLETED","notes":"月結付款"}');

SELECT _create_op('FI-AR-REC','收款作業','記錄客戶收款','FI','HandCoins',12, '[
  {"section":"收款資訊","field_name":"receipt_no","label":"收款編號","type":"TEXT","required":true,"searchable":true},
  {"section":"收款資訊","field_name":"customer","label":"客戶","type":"TEXT","required":true,"searchable":true},
  {"section":"收款資訊","field_name":"receipt_date","label":"收款日期","type":"DATE","required":true},
  {"section":"收款資訊","field_name":"amount","label":"收款金額","type":"NUMBER","required":true},
  {"section":"收款資訊","field_name":"invoice_reference","label":"對應發票","type":"TEXT","searchable":true},
  {"section":"收款資訊","field_name":"payment_method","label":"收款方式","type":"DROPDOWN","options":[{"label":"銀行轉帳","value":"TRANSFER","is_default":"true"},{"label":"支票","value":"CHECK"},{"label":"現金","value":"CASH"}]},
  {"section":"收款資訊","field_name":"status","label":"狀態","type":"DROPDOWN","required":true,"options":[{"label":"待確認","value":"PENDING","is_default":"true"},{"label":"已確認","value":"CONFIRMED"},{"label":"已沖帳","value":"APPLIED"}]}
]'::jsonb);
SELECT _sample('FI-AR-REC','{"receipt_no":"REC-001","customer":"全聯實業","receipt_date":"2026-04-10","amount":875000,"invoice_reference":"INV-C-001","payment_method":"TRANSFER","status":"CONFIRMED"}');

SELECT _create_op('FI-EXPENSE','費用報銷','員工費用報銷申請','FI','Receipt',13, '[
  {"section":"申請資訊","field_name":"expense_no","label":"報銷單號","type":"TEXT","required":true,"searchable":true},
  {"section":"申請資訊","field_name":"applicant","label":"申請人","type":"TEXT","required":true,"searchable":true},
  {"section":"申請資訊","field_name":"department","label":"部門","type":"TEXT","required":true},
  {"section":"申請資訊","field_name":"expense_date","label":"費用日期","type":"DATE","required":true},
  {"section":"費用明細","field_name":"category","label":"費用類別","type":"DROPDOWN","required":true,"options":[{"label":"交通","value":"TRANSPORT"},{"label":"餐飲","value":"MEAL"},{"label":"住宿","value":"HOTEL"},{"label":"文具","value":"OFFICE"},{"label":"其他","value":"OTHER"}]},
  {"section":"費用明細","field_name":"amount","label":"金額","type":"NUMBER","required":true,"min":0},
  {"section":"費用明細","field_name":"receipt_count","label":"收據張數","type":"NUMBER","required":true,"min":1},
  {"section":"費用明細","field_name":"description","label":"說明","type":"TEXTAREA"},
  {"section":"審核","field_name":"status","label":"狀態","type":"DROPDOWN","required":true,"searchable":true,"options":[{"label":"待審核","value":"PENDING","is_default":"true"},{"label":"已核准","value":"APPROVED"},{"label":"已退回","value":"REJECTED"},{"label":"已付款","value":"PAID"}]},
  {"section":"審核","field_name":"approver","label":"審核人","type":"TEXT"}
]'::jsonb);
SELECT _sample('FI-EXPENSE','{"expense_no":"EXP-001","applicant":"林美","department":"業務部","expense_date":"2026-03-18","category":"TRANSPORT","amount":2500,"receipt_count":3,"description":"客戶拜訪交通費","status":"APPROVED","approver":"陳偉"}');
SELECT _sample('FI-EXPENSE','{"expense_no":"EXP-002","applicant":"王軍","department":"採購部","expense_date":"2026-03-19","category":"MEAL","amount":850,"receipt_count":1,"description":"供應商午餐會議","status":"PENDING","approver":""}');

SELECT _create_op('FI-BUDGET','預算編列','部門預算管理','FI','PiggyBank',14, '[
  {"section":"預算資訊","field_name":"budget_code","label":"預算代碼","type":"TEXT","required":true,"searchable":true},
  {"section":"預算資訊","field_name":"department","label":"部門","type":"TEXT","required":true,"searchable":true},
  {"section":"預算資訊","field_name":"fiscal_year","label":"會計年度","type":"TEXT","required":true},
  {"section":"預算資訊","field_name":"period","label":"期間","type":"TEXT","required":true},
  {"section":"預算資訊","field_name":"account","label":"會計科目","type":"TEXT","required":true},
  {"section":"金額","field_name":"planned_amount","label":"預算金額","type":"NUMBER","required":true},
  {"section":"金額","field_name":"actual_amount","label":"實際金額","type":"NUMBER"},
  {"section":"金額","field_name":"variance","label":"差異","type":"NUMBER"},
  {"section":"金額","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"編列中","value":"DRAFT"},{"label":"已核定","value":"APPROVED"},{"label":"已凍結","value":"FROZEN"}]}
]'::jsonb);
SELECT _sample('FI-BUDGET','{"budget_code":"BUD-2026-SALES","department":"業務部","fiscal_year":"2026","period":"Q1","account":"6100","planned_amount":500000,"actual_amount":380000,"variance":120000,"status":"APPROVED"}');

SELECT _create_op('FI-FIXED-ASSET','固定資產登記','固定資產新增與管理','FI','Building',15, '[
  {"section":"資產資訊","field_name":"asset_no","label":"資產編號","type":"TEXT","required":true,"searchable":true},
  {"section":"資產資訊","field_name":"asset_name","label":"資產名稱","type":"TEXT","required":true,"searchable":true},
  {"section":"資產資訊","field_name":"category","label":"資產類別","type":"DROPDOWN","required":true,"options":[{"label":"房屋建築","value":"BUILDING"},{"label":"機器設備","value":"EQUIPMENT"},{"label":"運輸設備","value":"VEHICLE"},{"label":"辦公傢俱","value":"FURNITURE"},{"label":"資訊設備","value":"IT"}]},
  {"section":"資產資訊","field_name":"acquisition_date","label":"取得日期","type":"DATE","required":true},
  {"section":"資產資訊","field_name":"acquisition_cost","label":"取得成本","type":"NUMBER","required":true},
  {"section":"折舊","field_name":"useful_life","label":"耐用年限","type":"NUMBER","required":true},
  {"section":"折舊","field_name":"depreciation_method","label":"折舊方法","type":"DROPDOWN","options":[{"label":"直線法","value":"STRAIGHT_LINE","is_default":"true"},{"label":"定率遞減法","value":"DECLINING"}]},
  {"section":"折舊","field_name":"current_value","label":"帳面價值","type":"NUMBER"},
  {"section":"位置","field_name":"location","label":"使用位置","type":"TEXT"},
  {"section":"位置","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"使用中","value":"ACTIVE","is_default":"true"},{"label":"已報廢","value":"DISPOSED"},{"label":"維修中","value":"MAINTENANCE"}]}
]'::jsonb);
SELECT _sample('FI-FIXED-ASSET','{"asset_no":"FA-001","asset_name":"薯片生產線A","category":"EQUIPMENT","acquisition_date":"2022-06-15","acquisition_cost":5000000,"useful_life":10,"depreciation_method":"STRAIGHT_LINE","current_value":3500000,"location":"桃園廠","status":"ACTIVE"}');
SELECT _sample('FI-FIXED-ASSET','{"asset_no":"FA-002","asset_name":"冷藏庫","category":"BUILDING","acquisition_date":"2021-01-10","acquisition_cost":8000000,"useful_life":20,"depreciation_method":"STRAIGHT_LINE","current_value":6800000,"location":"桃園冷藏倉","status":"ACTIVE"}');

SELECT _create_op('FI-BANK-REC','銀行對帳','銀行餘額與帳面對帳','FI','Landmark',16, '[
  {"section":"對帳資訊","field_name":"reconciliation_date","label":"對帳日期","type":"DATE","required":true},
  {"section":"對帳資訊","field_name":"bank_account","label":"銀行帳號","type":"TEXT","required":true,"searchable":true},
  {"section":"餘額","field_name":"bank_balance","label":"銀行餘額","type":"NUMBER","required":true},
  {"section":"餘額","field_name":"book_balance","label":"帳面餘額","type":"NUMBER","required":true},
  {"section":"餘額","field_name":"difference","label":"差異金額","type":"NUMBER"},
  {"section":"餘額","field_name":"unmatched_items","label":"未沖帳筆數","type":"NUMBER"},
  {"section":"狀態","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"草稿","value":"DRAFT"},{"label":"已核對","value":"MATCHED"},{"label":"已完成","value":"COMPLETED"}]},
  {"section":"狀態","field_name":"notes","label":"備註","type":"TEXTAREA"}
]'::jsonb);
SELECT _sample('FI-BANK-REC','{"reconciliation_date":"2026-03-31","bank_account":"012-345678-9","bank_balance":15800000,"book_balance":15650000,"difference":150000,"unmatched_items":3,"status":"MATCHED","notes":"3筆在途支票尚未兌現"}');

SELECT _create_op('FI-TAX-DEC','稅務申報','營業稅/所得稅申報','FI','Scale',17, '[
  {"section":"申報資訊","field_name":"declaration_no","label":"申報編號","type":"TEXT","required":true,"searchable":true},
  {"section":"申報資訊","field_name":"tax_type","label":"稅務類型","type":"DROPDOWN","required":true,"options":[{"label":"營業稅","value":"VAT"},{"label":"營所稅","value":"INCOME"},{"label":"扣繳稅","value":"WITHHOLDING"}]},
  {"section":"申報資訊","field_name":"period","label":"申報期間","type":"TEXT","required":true},
  {"section":"金額","field_name":"taxable_amount","label":"課稅金額","type":"NUMBER","required":true},
  {"section":"金額","field_name":"tax_rate","label":"稅率(%)","type":"NUMBER"},
  {"section":"金額","field_name":"tax_amount","label":"應繳稅額","type":"NUMBER"},
  {"section":"金額","field_name":"filing_date","label":"申報日期","type":"DATE"},
  {"section":"金額","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"準備中","value":"PREPARING"},{"label":"已申報","value":"FILED"},{"label":"已繳納","value":"PAID"}]}
]'::jsonb);
SELECT _sample('FI-TAX-DEC','{"declaration_no":"TAX-2026-03","tax_type":"VAT","period":"2026-03","taxable_amount":12000000,"tax_rate":5,"tax_amount":600000,"filing_date":"2026-04-15","status":"PREPARING"}');

SELECT _create_op('FI-CLOSE','月結作業','會計期間月結檢核','FI','Lock',18, '[
  {"section":"月結資訊","field_name":"period","label":"會計期間","type":"TEXT","required":true,"searchable":true},
  {"section":"月結資訊","field_name":"fiscal_year","label":"會計年度","type":"TEXT","required":true},
  {"section":"月結資訊","field_name":"close_date","label":"結帳日期","type":"DATE"},
  {"section":"檢核","field_name":"checklist_items","label":"檢核項目數","type":"NUMBER"},
  {"section":"檢核","field_name":"completed_items","label":"已完成項目","type":"NUMBER"},
  {"section":"檢核","field_name":"completion_rate","label":"完成率(%)","type":"NUMBER"},
  {"section":"檢核","field_name":"status","label":"狀態","type":"DROPDOWN","required":true,"options":[{"label":"開放","value":"OPEN","is_default":"true"},{"label":"結帳中","value":"IN_PROGRESS"},{"label":"已結帳","value":"CLOSED"}]},
  {"section":"檢核","field_name":"closed_by","label":"結帳人","type":"TEXT"},
  {"section":"檢核","field_name":"notes","label":"備註","type":"TEXTAREA"}
]'::jsonb);
SELECT _sample('FI-CLOSE','{"period":"2026-02","fiscal_year":"2026","close_date":"2026-03-05","checklist_items":12,"completed_items":12,"completion_rate":100,"status":"CLOSED","closed_by":"陳偉","notes":"2月份帳務已全部結清"}');

SELECT _create_op('FI-PETTY-CASH','零用金管理','零用金收支記錄','FI','Wallet',19, '[
  {"section":"交易資訊","field_name":"transaction_date","label":"交易日期","type":"DATE","required":true},
  {"section":"交易資訊","field_name":"transaction_type","label":"交易類型","type":"DROPDOWN","required":true,"options":[{"label":"補充","value":"REPLENISH"},{"label":"支出","value":"EXPENSE"}]},
  {"section":"交易資訊","field_name":"amount","label":"金額","type":"NUMBER","required":true},
  {"section":"交易資訊","field_name":"balance","label":"餘額","type":"NUMBER"},
  {"section":"交易資訊","field_name":"category","label":"用途","type":"TEXT"},
  {"section":"交易資訊","field_name":"description","label":"說明","type":"TEXTAREA"},
  {"section":"交易資訊","field_name":"handler","label":"經手人","type":"TEXT","required":true}
]'::jsonb);
SELECT _sample('FI-PETTY-CASH','{"transaction_date":"2026-03-20","transaction_type":"EXPENSE","amount":350,"balance":4650,"category":"文具","description":"購買A4影印紙5包","handler":"行政部小李"}');

SELECT _create_op('FI-CREDIT-NOTE','折讓單','銷售折讓/退貨折讓','FI','FileX',20, '[
  {"section":"折讓資訊","field_name":"credit_note_no","label":"折讓單號","type":"TEXT","required":true,"searchable":true},
  {"section":"折讓資訊","field_name":"customer","label":"客戶","type":"TEXT","required":true,"searchable":true},
  {"section":"折讓資訊","field_name":"original_invoice","label":"原始發票","type":"TEXT","searchable":true},
  {"section":"折讓資訊","field_name":"reason","label":"折讓原因","type":"DROPDOWN","required":true,"options":[{"label":"退貨","value":"RETURN"},{"label":"折扣","value":"DISCOUNT"},{"label":"錯誤更正","value":"ERROR"},{"label":"商品瑕疵","value":"DAMAGE"}]},
  {"section":"折讓資訊","field_name":"amount","label":"折讓金額","type":"NUMBER","required":true},
  {"section":"折讓資訊","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"草稿","value":"DRAFT"},{"label":"已核准","value":"APPROVED"},{"label":"已開立","value":"ISSUED"}]},
  {"section":"折讓資訊","field_name":"notes","label":"備註","type":"TEXTAREA"}
]'::jsonb);
SELECT _sample('FI-CREDIT-NOTE','{"credit_note_no":"CN-001","customer":"大潤發","original_invoice":"INV-C-003","reason":"DAMAGE","amount":15000,"status":"APPROVED","notes":"運送途中破損20箱薯片"}');

-- ═══════════════════════════════════════════════════════════════════
-- CO MODULE (8 operations)
-- ═══════════════════════════════════════════════════════════════════

SELECT _create_op('CO-COST-ALLOC','成本分攤','成本中心間費用分攤','CO','GitFork',10, '[
  {"section":"分攤資訊","field_name":"allocation_no","label":"分攤編號","type":"TEXT","required":true,"searchable":true},
  {"section":"分攤資訊","field_name":"period","label":"期間","type":"TEXT","required":true},
  {"section":"分攤資訊","field_name":"from_cost_center","label":"分攤來源","type":"TEXT","required":true},
  {"section":"分攤資訊","field_name":"to_cost_center","label":"分攤對象","type":"TEXT","required":true},
  {"section":"分攤資訊","field_name":"amount","label":"分攤金額","type":"NUMBER","required":true},
  {"section":"分攤資訊","field_name":"allocation_basis","label":"分攤基礎","type":"DROPDOWN","options":[{"label":"人數","value":"HEADCOUNT"},{"label":"面積","value":"AREA"},{"label":"營收","value":"REVENUE"},{"label":"使用量","value":"USAGE"}]},
  {"section":"分攤資訊","field_name":"percentage","label":"分攤比例(%)","type":"NUMBER"},
  {"section":"分攤資訊","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"草稿","value":"DRAFT"},{"label":"已過帳","value":"POSTED"}]}
]'::jsonb);
SELECT _sample('CO-COST-ALLOC','{"allocation_no":"CA-2026-03","period":"2026-03","from_cost_center":"管理部","to_cost_center":"生產部","amount":120000,"allocation_basis":"HEADCOUNT","percentage":45,"status":"POSTED"}');

SELECT _create_op('CO-PROFIT-RPT','利潤分析','產品線利潤分析報告','CO','TrendingUp',11, '[
  {"section":"報告資訊","field_name":"report_no","label":"報告編號","type":"TEXT","required":true,"searchable":true},
  {"section":"報告資訊","field_name":"product_line","label":"產品線","type":"TEXT","required":true,"searchable":true},
  {"section":"報告資訊","field_name":"period","label":"期間","type":"TEXT","required":true},
  {"section":"損益","field_name":"revenue","label":"營收","type":"NUMBER","required":true},
  {"section":"損益","field_name":"cogs","label":"銷貨成本","type":"NUMBER","required":true},
  {"section":"損益","field_name":"gross_profit","label":"毛利","type":"NUMBER"},
  {"section":"損益","field_name":"operating_expense","label":"營業費用","type":"NUMBER"},
  {"section":"損益","field_name":"net_profit","label":"淨利","type":"NUMBER"},
  {"section":"損益","field_name":"margin_pct","label":"淨利率(%)","type":"NUMBER"}
]'::jsonb);
SELECT _sample('CO-PROFIT-RPT','{"report_no":"PL-2026Q1-SNACK","product_line":"零食系列","period":"2026 Q1","revenue":8500000,"cogs":5100000,"gross_profit":3400000,"operating_expense":1200000,"net_profit":2200000,"margin_pct":25.9}');

SELECT _create_op('CO-INTERNAL-ORD','內部訂單','內部專案/維修/行銷訂單','CO','ClipboardList',12, '[
  {"section":"訂單資訊","field_name":"order_no","label":"內部訂單號","type":"TEXT","required":true,"searchable":true},
  {"section":"訂單資訊","field_name":"order_type","label":"訂單類型","type":"DROPDOWN","required":true,"options":[{"label":"專案","value":"PROJECT"},{"label":"維修","value":"MAINTENANCE"},{"label":"行銷","value":"MARKETING"},{"label":"研發","value":"RD"}]},
  {"section":"訂單資訊","field_name":"description","label":"說明","type":"TEXTAREA","required":true},
  {"section":"訂單資訊","field_name":"responsible","label":"負責人","type":"TEXT","required":true},
  {"section":"預算","field_name":"budget","label":"預算","type":"NUMBER","required":true},
  {"section":"預算","field_name":"actual_cost","label":"實際成本","type":"NUMBER"},
  {"section":"預算","field_name":"variance","label":"差異","type":"NUMBER"},
  {"section":"時程","field_name":"start_date","label":"開始日期","type":"DATE","required":true},
  {"section":"時程","field_name":"end_date","label":"結束日期","type":"DATE"},
  {"section":"時程","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"開立","value":"OPEN"},{"label":"進行中","value":"IN_PROGRESS"},{"label":"已關閉","value":"CLOSED"},{"label":"已取消","value":"CANCELLED"}]}
]'::jsonb);
SELECT _sample('CO-INTERNAL-ORD','{"order_no":"IO-2026-001","order_type":"MARKETING","description":"2026春季促銷活動","responsible":"林美","budget":200000,"actual_cost":165000,"variance":35000,"start_date":"2026-03-01","end_date":"2026-04-30","status":"IN_PROGRESS"}');

SELECT _create_op('CO-OVERHEAD','間接費用分配','間接費用按基礎分配','CO','Calculator',13, '[
  {"section":"分配資訊","field_name":"period","label":"期間","type":"TEXT","required":true},
  {"section":"分配資訊","field_name":"cost_center","label":"成本中心","type":"TEXT","required":true,"searchable":true},
  {"section":"分配資訊","field_name":"overhead_type","label":"費用類型","type":"DROPDOWN","required":true,"options":[{"label":"租金","value":"RENT"},{"label":"水電","value":"UTILITIES"},{"label":"資訊","value":"IT"},{"label":"人事","value":"HR"},{"label":"行政","value":"ADMIN"}]},
  {"section":"計算","field_name":"base_amount","label":"基礎金額","type":"NUMBER","required":true},
  {"section":"計算","field_name":"rate","label":"分配率(%)","type":"NUMBER","required":true},
  {"section":"計算","field_name":"allocated_amount","label":"分配金額","type":"NUMBER"},
  {"section":"計算","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"計算中","value":"CALCULATING"},{"label":"已確認","value":"CONFIRMED"},{"label":"已過帳","value":"POSTED"}]}
]'::jsonb);
SELECT _sample('CO-OVERHEAD','{"period":"2026-03","cost_center":"生產部","overhead_type":"RENT","base_amount":300000,"rate":60,"allocated_amount":180000,"status":"POSTED"}');

SELECT _create_op('CO-VARIANCE','差異分析','標準成本與實際成本差異','CO','BarChart2',14, '[
  {"section":"分析","field_name":"analysis_no","label":"分析編號","type":"TEXT","required":true,"searchable":true},
  {"section":"分析","field_name":"product","label":"產品","type":"TEXT","required":true,"searchable":true},
  {"section":"分析","field_name":"period","label":"期間","type":"TEXT","required":true},
  {"section":"成本","field_name":"standard_cost","label":"標準成本","type":"NUMBER","required":true},
  {"section":"成本","field_name":"actual_cost","label":"實際成本","type":"NUMBER","required":true},
  {"section":"差異明細","field_name":"material_variance","label":"材料差異","type":"NUMBER"},
  {"section":"差異明細","field_name":"labor_variance","label":"人工差異","type":"NUMBER"},
  {"section":"差異明細","field_name":"overhead_variance","label":"製費差異","type":"NUMBER"},
  {"section":"差異明細","field_name":"total_variance","label":"總差異","type":"NUMBER"}
]'::jsonb);
SELECT _sample('CO-VARIANCE','{"analysis_no":"VA-2026Q1-001","product":"脆薯片(原味)","period":"2026 Q1","standard_cost":22,"actual_cost":23.5,"material_variance":0.8,"labor_variance":0.3,"overhead_variance":0.4,"total_variance":1.5}');

SELECT _create_op('CO-PLAN','成本計畫','年度成本計畫編制','CO','Target',15, '[
  {"section":"計畫","field_name":"plan_no","label":"計畫編號","type":"TEXT","required":true,"searchable":true},
  {"section":"計畫","field_name":"cost_center","label":"成本中心","type":"TEXT","required":true,"searchable":true},
  {"section":"計畫","field_name":"period","label":"期間","type":"TEXT","required":true},
  {"section":"計畫","field_name":"cost_element","label":"成本要素","type":"TEXT","required":true},
  {"section":"金額","field_name":"planned_amount","label":"計畫金額","type":"NUMBER","required":true},
  {"section":"金額","field_name":"prior_year","label":"去年同期","type":"NUMBER"},
  {"section":"金額","field_name":"change_pct","label":"變動率(%)","type":"NUMBER"},
  {"section":"金額","field_name":"justification","label":"變動說明","type":"TEXTAREA"},
  {"section":"金額","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"規劃中","value":"PLANNING"},{"label":"已提交","value":"SUBMITTED"},{"label":"已核定","value":"APPROVED"}]}
]'::jsonb);
SELECT _sample('CO-PLAN','{"plan_no":"CP-2026-PROD","cost_center":"生產部","period":"2026","cost_element":"直接材料","planned_amount":15000000,"prior_year":13500000,"change_pct":11.1,"justification":"原物料價格上漲及產量增加","status":"APPROVED"}');

SELECT _create_op('CO-ABC','作業基礎成本','ABC成本分析','CO','Layers',16, '[
  {"section":"作業","field_name":"activity","label":"作業名稱","type":"TEXT","required":true,"searchable":true},
  {"section":"作業","field_name":"cost_driver","label":"成本動因","type":"TEXT","required":true},
  {"section":"作業","field_name":"volume","label":"動因數量","type":"NUMBER","required":true},
  {"section":"作業","field_name":"unit_cost","label":"單位成本","type":"NUMBER","required":true},
  {"section":"作業","field_name":"total_cost","label":"總成本","type":"NUMBER"},
  {"section":"分配","field_name":"product","label":"分配產品","type":"TEXT"},
  {"section":"分配","field_name":"allocated_to","label":"分配對象","type":"TEXT"},
  {"section":"分配","field_name":"period","label":"期間","type":"TEXT","required":true}
]'::jsonb);
SELECT _sample('CO-ABC','{"activity":"品質檢驗","cost_driver":"檢驗次數","volume":500,"unit_cost":120,"total_cost":60000,"product":"脆薯片系列","allocated_to":"QM部門","period":"2026-03"}');

SELECT _create_op('CO-BENCHMARK','成本標竿','成本標竿比較分析','CO','Award',17, '[
  {"section":"比較","field_name":"product","label":"產品","type":"TEXT","required":true,"searchable":true},
  {"section":"比較","field_name":"benchmark_type","label":"標竿類型","type":"DROPDOWN","options":[{"label":"內部","value":"INTERNAL"},{"label":"同業","value":"INDUSTRY"},{"label":"競品","value":"COMPETITOR"}]},
  {"section":"比較","field_name":"our_cost","label":"我方成本","type":"NUMBER","required":true},
  {"section":"比較","field_name":"benchmark_cost","label":"標竿成本","type":"NUMBER","required":true},
  {"section":"比較","field_name":"gap","label":"差距","type":"NUMBER"},
  {"section":"比較","field_name":"gap_pct","label":"差距率(%)","type":"NUMBER"},
  {"section":"改善","field_name":"improvement_target","label":"改善目標","type":"TEXT"},
  {"section":"改善","field_name":"notes","label":"備註","type":"TEXTAREA"}
]'::jsonb);
SELECT _sample('CO-BENCHMARK','{"product":"脆薯片(原味)","benchmark_type":"INDUSTRY","our_cost":23.5,"benchmark_cost":21,"gap":2.5,"gap_pct":11.9,"improvement_target":"2026年底降至22元","notes":"主要差距在包材成本"}');

-- ═══════════════════════════════════════════════════════════════════
-- MM MODULE (additional 15 operations)
-- ═══════════════════════════════════════════════════════════════════

SELECT _create_op('MM-VENDOR','供應商主檔','供應商基本資料管理','MM','Users',50, '[
  {"section":"基本資料","field_name":"vendor_code","label":"供應商代碼","type":"TEXT","required":true,"searchable":true},
  {"section":"基本資料","field_name":"vendor_name","label":"供應商名稱","type":"TEXT","required":true,"searchable":true},
  {"section":"基本資料","field_name":"contact","label":"聯絡人","type":"TEXT"},
  {"section":"基本資料","field_name":"phone","label":"電話","type":"TEXT"},
  {"section":"基本資料","field_name":"email","label":"Email","type":"TEXT"},
  {"section":"基本資料","field_name":"address","label":"地址","type":"TEXTAREA"},
  {"section":"商務","field_name":"payment_terms","label":"付款條件","type":"DROPDOWN","options":[{"label":"月結30天","value":"NET30","is_default":"true"},{"label":"月結60天","value":"NET60"},{"label":"月結90天","value":"NET90"},{"label":"貨到付款","value":"COD"}]},
  {"section":"商務","field_name":"tax_id","label":"統一編號","type":"TEXT","searchable":true},
  {"section":"商務","field_name":"category","label":"供應商分類","type":"DROPDOWN","options":[{"label":"原物料","value":"RAW"},{"label":"包材","value":"PACKAGING"},{"label":"服務","value":"SERVICE"},{"label":"設備","value":"EQUIPMENT"}]},
  {"section":"商務","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"有效","value":"ACTIVE","is_default":"true"},{"label":"暫停","value":"BLOCKED"},{"label":"停用","value":"INACTIVE"}]}
]'::jsonb);
SELECT _sample('MM-VENDOR','{"vendor_code":"V-001","vendor_name":"台灣農產供應商","contact":"黃先生","phone":"03-1234567","email":"huang@twagri.com","address":"桃園市中壢區農業路100號","payment_terms":"NET30","tax_id":"12345678","category":"RAW","status":"ACTIVE"}');
SELECT _sample('MM-VENDOR','{"vendor_code":"V-002","vendor_name":"濃縮汁進口商","contact":"張小姐","phone":"02-9876543","email":"zhang@juiceimport.com","address":"台北市大安區進口路50號","payment_terms":"NET60","tax_id":"87654321","category":"RAW","status":"ACTIVE"}');

SELECT _create_op('MM-PURCH-REQ','請購單','物料請購申請','MM','ShoppingCart',51, '[
  {"section":"請購資訊","field_name":"pr_no","label":"請購單號","type":"TEXT","required":true,"searchable":true},
  {"section":"請購資訊","field_name":"requester","label":"請購人","type":"TEXT","required":true,"searchable":true},
  {"section":"請購資訊","field_name":"department","label":"部門","type":"TEXT","required":true},
  {"section":"品項","field_name":"material","label":"物料名稱","type":"TEXT","required":true,"searchable":true},
  {"section":"品項","field_name":"quantity","label":"數量","type":"NUMBER","required":true,"min":1},
  {"section":"品項","field_name":"unit","label":"單位","type":"TEXT","required":true},
  {"section":"品項","field_name":"estimated_price","label":"預估單價","type":"NUMBER"},
  {"section":"品項","field_name":"total_amount","label":"預估總額","type":"NUMBER"},
  {"section":"品項","field_name":"required_date","label":"需求日期","type":"DATE","required":true},
  {"section":"品項","field_name":"purpose","label":"用途說明","type":"TEXTAREA"},
  {"section":"審核","field_name":"priority","label":"優先順序","type":"DROPDOWN","options":[{"label":"緊急","value":"URGENT"},{"label":"高","value":"HIGH"},{"label":"一般","value":"NORMAL","is_default":"true"},{"label":"低","value":"LOW"}]},
  {"section":"審核","field_name":"status","label":"狀態","type":"DROPDOWN","required":true,"searchable":true,"options":[{"label":"草稿","value":"DRAFT","is_default":"true"},{"label":"已提交","value":"SUBMITTED"},{"label":"已核准","value":"APPROVED"},{"label":"已轉採購","value":"ORDERED"},{"label":"已退回","value":"REJECTED"}]},
  {"section":"審核","field_name":"approver","label":"核准人","type":"TEXT"}
]'::jsonb);
SELECT _sample('MM-PURCH-REQ','{"pr_no":"PR-001","requester":"張莉","department":"生產部","material":"馬鈴薯粉","quantity":3000,"unit":"KG","estimated_price":80,"total_amount":240000,"required_date":"2026-04-01","purpose":"4月份生產用料","priority":"HIGH","status":"APPROVED","approver":"王軍"}');

SELECT _create_op('MM-RFQ','詢價單','向供應商詢價','MM','Search',52, '[
  {"section":"詢價","field_name":"rfq_no","label":"詢價單號","type":"TEXT","required":true,"searchable":true},
  {"section":"詢價","field_name":"item_description","label":"品項說明","type":"TEXT","required":true},
  {"section":"詢價","field_name":"quantity","label":"數量","type":"NUMBER","required":true},
  {"section":"詢價","field_name":"unit","label":"單位","type":"TEXT"},
  {"section":"詢價","field_name":"required_date","label":"需求日期","type":"DATE"},
  {"section":"報價比較","field_name":"vendor_1","label":"供應商1","type":"TEXT"},
  {"section":"報價比較","field_name":"price_1","label":"報價1","type":"NUMBER"},
  {"section":"報價比較","field_name":"vendor_2","label":"供應商2","type":"TEXT"},
  {"section":"報價比較","field_name":"price_2","label":"報價2","type":"NUMBER"},
  {"section":"報價比較","field_name":"vendor_3","label":"供應商3","type":"TEXT"},
  {"section":"報價比較","field_name":"price_3","label":"報價3","type":"NUMBER"},
  {"section":"決標","field_name":"selected_vendor","label":"得標供應商","type":"TEXT","searchable":true},
  {"section":"決標","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"草稿","value":"DRAFT"},{"label":"已發出","value":"SENT"},{"label":"已收到報價","value":"RECEIVED"},{"label":"已決標","value":"AWARDED"}]}
]'::jsonb);
SELECT _sample('MM-RFQ','{"rfq_no":"RFQ-001","item_description":"食用油 (大豆沙拉油)","quantity":5000,"unit":"L","required_date":"2026-04-15","vendor_1":"油品A商","price_1":42,"vendor_2":"油品B商","price_2":45,"vendor_3":"油品C商","price_3":40,"selected_vendor":"油品C商","status":"AWARDED"}');

SELECT _create_op('MM-CONTRACT','採購合約','長期採購合約管理','MM','FileSignature',53, '[
  {"section":"合約資訊","field_name":"contract_no","label":"合約編號","type":"TEXT","required":true,"searchable":true},
  {"section":"合約資訊","field_name":"vendor","label":"供應商","type":"TEXT","required":true,"searchable":true},
  {"section":"合約資訊","field_name":"start_date","label":"開始日期","type":"DATE","required":true},
  {"section":"合約資訊","field_name":"end_date","label":"結束日期","type":"DATE","required":true},
  {"section":"品項","field_name":"material","label":"物料","type":"TEXT","required":true},
  {"section":"品項","field_name":"agreed_price","label":"合約單價","type":"NUMBER","required":true},
  {"section":"品項","field_name":"min_quantity","label":"最低數量","type":"NUMBER"},
  {"section":"品項","field_name":"max_quantity","label":"最高數量","type":"NUMBER"},
  {"section":"執行","field_name":"total_ordered","label":"已訂購量","type":"NUMBER"},
  {"section":"執行","field_name":"remaining_qty","label":"剩餘量","type":"NUMBER"},
  {"section":"執行","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"有效","value":"ACTIVE","is_default":"true"},{"label":"已到期","value":"EXPIRED"},{"label":"已終止","value":"TERMINATED"}]}
]'::jsonb);
SELECT _sample('MM-CONTRACT','{"contract_no":"CTR-001","vendor":"台灣農產供應商","start_date":"2026-01-01","end_date":"2026-12-31","material":"馬鈴薯粉","agreed_price":78,"min_quantity":5000,"max_quantity":50000,"total_ordered":12000,"remaining_qty":38000,"status":"ACTIVE"}');

SELECT _create_op('MM-RETURN','採購退貨','採購退貨處理','MM','Undo2',54, '[
  {"section":"退貨資訊","field_name":"return_no","label":"退貨單號","type":"TEXT","required":true,"searchable":true},
  {"section":"退貨資訊","field_name":"vendor","label":"供應商","type":"TEXT","required":true,"searchable":true},
  {"section":"退貨資訊","field_name":"po_reference","label":"採購單號","type":"TEXT","searchable":true},
  {"section":"退貨資訊","field_name":"grn_reference","label":"收貨單號","type":"TEXT"},
  {"section":"品項","field_name":"material","label":"物料","type":"TEXT","required":true},
  {"section":"品項","field_name":"return_qty","label":"退貨數量","type":"NUMBER","required":true},
  {"section":"品項","field_name":"reason","label":"退貨原因","type":"DROPDOWN","required":true,"options":[{"label":"品質不良","value":"DEFECTIVE"},{"label":"交錯貨","value":"WRONG_ITEM"},{"label":"數量過多","value":"EXCESS"},{"label":"已過期","value":"EXPIRED"},{"label":"包裝損壞","value":"DAMAGED"}]},
  {"section":"處理","field_name":"return_date","label":"退貨日期","type":"DATE","required":true},
  {"section":"處理","field_name":"credit_amount","label":"退款金額","type":"NUMBER"},
  {"section":"處理","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"待處理","value":"PENDING"},{"label":"已出貨","value":"SHIPPED"},{"label":"已退款","value":"CREDITED"},{"label":"已關閉","value":"CLOSED"}]}
]'::jsonb);
SELECT _sample('MM-RETURN','{"return_no":"RET-001","vendor":"劣質包材商","po_reference":"PO-00003","grn_reference":"GRN-003","material":"包裝袋","return_qty":5000,"reason":"DEFECTIVE","return_date":"2026-03-25","credit_amount":12500,"status":"SHIPPED"}');

SELECT _create_op('MM-INV-VERIFY','發票驗證','三方匹配驗證','MM','ShieldCheck',55, '[
  {"section":"驗證資訊","field_name":"verification_no","label":"驗證編號","type":"TEXT","required":true,"searchable":true},
  {"section":"驗證資訊","field_name":"vendor","label":"供應商","type":"TEXT","required":true,"searchable":true},
  {"section":"單據","field_name":"po_number","label":"採購單號","type":"TEXT","required":true,"searchable":true},
  {"section":"單據","field_name":"grn_number","label":"收貨單號","type":"TEXT","required":true},
  {"section":"單據","field_name":"invoice_number","label":"發票號碼","type":"TEXT","required":true},
  {"section":"金額比較","field_name":"po_amount","label":"採購金額","type":"NUMBER","required":true},
  {"section":"金額比較","field_name":"grn_amount","label":"收貨金額","type":"NUMBER"},
  {"section":"金額比較","field_name":"invoice_amount","label":"發票金額","type":"NUMBER","required":true},
  {"section":"金額比較","field_name":"difference","label":"差異金額","type":"NUMBER"},
  {"section":"結果","field_name":"match_result","label":"匹配結果","type":"DROPDOWN","options":[{"label":"三方一致","value":"3WAY_MATCH"},{"label":"有差異","value":"DISCREPANCY"},{"label":"已凍結","value":"BLOCKED"}]},
  {"section":"結果","field_name":"status","label":"處理狀態","type":"DROPDOWN","options":[{"label":"待驗證","value":"PENDING"},{"label":"已通過","value":"PASSED"},{"label":"待釐清","value":"INVESTIGATING"},{"label":"已關閉","value":"CLOSED"}]}
]'::jsonb);
SELECT _sample('MM-INV-VERIFY','{"verification_no":"IVF-001","vendor":"台灣農產供應商","po_number":"PO-00001","grn_number":"GRN-001","invoice_number":"INV-V-001","po_amount":320000,"grn_amount":316800,"invoice_amount":320000,"difference":3200,"match_result":"DISCREPANCY","status":"INVESTIGATING"}');

SELECT _create_op('MM-PRICE-CHG','物料調價','物料價格變動通知','MM','TrendingUp',56, '[
  {"section":"調價資訊","field_name":"notice_no","label":"通知編號","type":"TEXT","required":true,"searchable":true},
  {"section":"調價資訊","field_name":"material","label":"物料","type":"TEXT","required":true,"searchable":true},
  {"section":"價格","field_name":"current_price","label":"現行價格","type":"NUMBER","required":true},
  {"section":"價格","field_name":"new_price","label":"新價格","type":"NUMBER","required":true},
  {"section":"價格","field_name":"change_pct","label":"漲跌幅(%)","type":"NUMBER"},
  {"section":"價格","field_name":"effective_date","label":"生效日期","type":"DATE","required":true},
  {"section":"審核","field_name":"reason","label":"調價原因","type":"TEXTAREA","required":true},
  {"section":"審核","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"待審核","value":"PENDING"},{"label":"已核准","value":"APPROVED"},{"label":"已退回","value":"REJECTED"},{"label":"已生效","value":"APPLIED"}]},
  {"section":"審核","field_name":"approved_by","label":"核准人","type":"TEXT"}
]'::jsonb);
SELECT _sample('MM-PRICE-CHG','{"notice_no":"PC-001","material":"馬鈴薯粉","current_price":80,"new_price":85,"change_pct":6.25,"effective_date":"2026-04-01","reason":"國際原物料價格上漲","status":"APPROVED","approved_by":"王軍"}');

SELECT _create_op('MM-REORDER','補貨建議','自動補貨建議產生','MM','RefreshCw',57, '[
  {"section":"物料","field_name":"material","label":"物料","type":"TEXT","required":true,"searchable":true},
  {"section":"庫存","field_name":"current_stock","label":"目前庫存","type":"NUMBER"},
  {"section":"庫存","field_name":"min_stock","label":"安全庫存","type":"NUMBER"},
  {"section":"庫存","field_name":"max_stock","label":"最高庫存","type":"NUMBER"},
  {"section":"建議","field_name":"reorder_qty","label":"建議補貨量","type":"NUMBER"},
  {"section":"建議","field_name":"unit","label":"單位","type":"TEXT"},
  {"section":"建議","field_name":"suggested_vendor","label":"建議供應商","type":"TEXT"},
  {"section":"建議","field_name":"unit_price","label":"單價","type":"NUMBER"},
  {"section":"建議","field_name":"total_cost","label":"總金額","type":"NUMBER"},
  {"section":"建議","field_name":"urgency","label":"緊急度","type":"DROPDOWN","options":[{"label":"高","value":"HIGH"},{"label":"中","value":"MEDIUM"},{"label":"低","value":"LOW"}]},
  {"section":"建議","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"建議","value":"SUGGESTED"},{"label":"已下單","value":"ORDERED"},{"label":"已取消","value":"CANCELLED"}]}
]'::jsonb);
SELECT _sample('MM-REORDER','{"material":"馬鈴薯粉","current_stock":1875,"min_stock":500,"max_stock":10000,"reorder_qty":8125,"unit":"KG","suggested_vendor":"台灣農產供應商","unit_price":78,"total_cost":633750,"urgency":"MEDIUM","status":"SUGGESTED"}');

SELECT _create_op('MM-BATCH','批次管理','物料批次追溯','MM','Barcode',58, '[
  {"section":"批次資訊","field_name":"batch_no","label":"批號","type":"TEXT","required":true,"searchable":true},
  {"section":"批次資訊","field_name":"material","label":"物料","type":"TEXT","required":true,"searchable":true},
  {"section":"批次資訊","field_name":"production_date","label":"生產日期","type":"DATE","required":true},
  {"section":"批次資訊","field_name":"expiry_date","label":"有效期限","type":"DATE","required":true},
  {"section":"批次資訊","field_name":"quantity","label":"數量","type":"NUMBER","required":true},
  {"section":"批次資訊","field_name":"unit","label":"單位","type":"TEXT"},
  {"section":"狀態","field_name":"status","label":"狀態","type":"DROPDOWN","required":true,"options":[{"label":"可用","value":"AVAILABLE","is_default":"true"},{"label":"已預留","value":"RESERVED"},{"label":"已耗用","value":"CONSUMED"},{"label":"已過期","value":"EXPIRED"},{"label":"已召回","value":"RECALLED"}]},
  {"section":"狀態","field_name":"location","label":"儲位","type":"TEXT"},
  {"section":"狀態","field_name":"quality_status","label":"品質狀態","type":"DROPDOWN","options":[{"label":"合格","value":"PASSED"},{"label":"不合格","value":"FAILED"},{"label":"待檢","value":"PENDING"}]}
]'::jsonb);
SELECT _sample('MM-BATCH','{"batch_no":"BAT-2026031001","material":"馬鈴薯粉","production_date":"2026-03-10","expiry_date":"2027-03-10","quantity":1980,"unit":"KG","status":"AVAILABLE","location":"WH-001-A-01","quality_status":"PASSED"}');
SELECT _sample('MM-BATCH','{"batch_no":"BAT-2026032201","material":"葡萄濃縮汁","production_date":"2026-03-01","expiry_date":"2027-06-30","quantity":1000,"unit":"L","status":"AVAILABLE","location":"WH-002-C-01","quality_status":"PASSED"}');

SELECT _create_op('MM-STOCK-COUNT','庫存盤點','庫存盤點差異調整','MM','ClipboardCheck',59, '[
  {"section":"盤點資訊","field_name":"count_no","label":"盤點單號","type":"TEXT","required":true,"searchable":true},
  {"section":"盤點資訊","field_name":"material","label":"物料","type":"TEXT","required":true,"searchable":true},
  {"section":"盤點資訊","field_name":"location","label":"儲位","type":"TEXT","required":true},
  {"section":"數量","field_name":"system_qty","label":"帳面數量","type":"NUMBER","required":true},
  {"section":"數量","field_name":"actual_qty","label":"實際數量","type":"NUMBER","required":true},
  {"section":"數量","field_name":"difference","label":"差異","type":"NUMBER"},
  {"section":"數量","field_name":"unit","label":"單位","type":"TEXT"},
  {"section":"處理","field_name":"count_date","label":"盤點日期","type":"DATE","required":true},
  {"section":"處理","field_name":"counter","label":"盤點人","type":"TEXT","required":true},
  {"section":"處理","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"計畫中","value":"PLANNED"},{"label":"盤點中","value":"COUNTING"},{"label":"已完成","value":"COMPLETED"},{"label":"已調整","value":"ADJUSTED"}]},
  {"section":"處理","field_name":"adjustment_reason","label":"調整原因","type":"TEXTAREA"}
]'::jsonb);
SELECT _sample('MM-STOCK-COUNT','{"count_no":"SC-001","material":"脆薯片(原味)","location":"WH-001-B-03","system_qty":15000,"actual_qty":14850,"difference":-150,"unit":"PCS","count_date":"2026-03-31","counter":"倉管小陳","status":"COMPLETED","adjustment_reason":"150片包裝破損報廢未入帳"}');

SELECT _create_op('MM-GOODS-ISSUE','發料單','生產/維修領料','MM','LogOut',60, '[
  {"section":"發料資訊","field_name":"issue_no","label":"發料單號","type":"TEXT","required":true,"searchable":true},
  {"section":"發料資訊","field_name":"requester","label":"領料人","type":"TEXT","required":true},
  {"section":"發料資訊","field_name":"department","label":"部門","type":"TEXT","required":true},
  {"section":"品項","field_name":"material","label":"物料","type":"TEXT","required":true,"searchable":true},
  {"section":"品項","field_name":"quantity","label":"數量","type":"NUMBER","required":true,"min":1},
  {"section":"品項","field_name":"unit","label":"單位","type":"TEXT","required":true},
  {"section":"品項","field_name":"purpose","label":"用途","type":"DROPDOWN","required":true,"options":[{"label":"生產","value":"PRODUCTION"},{"label":"維修","value":"MAINTENANCE"},{"label":"樣品","value":"SAMPLE"},{"label":"研發","value":"RD"},{"label":"其他","value":"OTHER"}]},
  {"section":"品項","field_name":"batch_number","label":"批號","type":"TEXT"},
  {"section":"品項","field_name":"cost_center","label":"成本中心","type":"TEXT"},
  {"section":"品項","field_name":"issue_date","label":"發料日期","type":"DATE","required":true},
  {"section":"品項","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"草稿","value":"DRAFT"},{"label":"已發料","value":"ISSUED"},{"label":"已取消","value":"CANCELLED"}]}
]'::jsonb);
SELECT _sample('MM-GOODS-ISSUE','{"issue_no":"GI-001","requester":"張莉","department":"生產部","material":"馬鈴薯粉","quantity":105,"unit":"KG","purpose":"PRODUCTION","batch_number":"BAT-2026031001","cost_center":"生產部","issue_date":"2026-03-22","status":"ISSUED"}');

SELECT _create_op('MM-VENDOR-CERT','供應商認證','供應商品質認證管理','MM','Award',61, '[
  {"section":"認證資訊","field_name":"cert_no","label":"認證編號","type":"TEXT","required":true,"searchable":true},
  {"section":"認證資訊","field_name":"vendor","label":"供應商","type":"TEXT","required":true,"searchable":true},
  {"section":"認證資訊","field_name":"cert_type","label":"認證類型","type":"DROPDOWN","required":true,"options":[{"label":"ISO 9001","value":"ISO9001"},{"label":"ISO 14001","value":"ISO14001"},{"label":"HACCP","value":"HACCP"},{"label":"GMP","value":"GMP"},{"label":"清真","value":"HALAL"},{"label":"有機","value":"ORGANIC"}]},
  {"section":"認證資訊","field_name":"issue_date","label":"核發日期","type":"DATE","required":true},
  {"section":"認證資訊","field_name":"expiry_date","label":"到期日","type":"DATE","required":true},
  {"section":"認證資訊","field_name":"cert_body","label":"認證機構","type":"TEXT"},
  {"section":"認證資訊","field_name":"cert_number","label":"證書號碼","type":"TEXT"},
  {"section":"狀態","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"有效","value":"VALID","is_default":"true"},{"label":"即將到期","value":"EXPIRING"},{"label":"已過期","value":"EXPIRED"},{"label":"已撤銷","value":"REVOKED"}]}
]'::jsonb);
SELECT _sample('MM-VENDOR-CERT','{"cert_no":"VC-001","vendor":"台灣農產供應商","cert_type":"HACCP","issue_date":"2025-06-01","expiry_date":"2028-05-31","cert_body":"SGS台灣","cert_number":"HACCP-2025-1234","status":"VALID"}');

-- ═══════════════════════════════════════════════════════════════════
-- SD MODULE (15 operations)
-- ═══════════════════════════════════════════════════════════════════

SELECT _create_op('SD-QUOTATION','報價單','客戶報價管理','SD','FileText',50, '[
  {"section":"報價資訊","field_name":"quote_no","label":"報價單號","type":"TEXT","required":true,"searchable":true},
  {"section":"報價資訊","field_name":"customer","label":"客戶","type":"TEXT","required":true,"searchable":true},
  {"section":"報價資訊","field_name":"valid_until","label":"有效期限","type":"DATE","required":true},
  {"section":"品項","field_name":"item_description","label":"品項說明","type":"TEXT","required":true},
  {"section":"品項","field_name":"quantity","label":"數量","type":"NUMBER","required":true},
  {"section":"品項","field_name":"unit_price","label":"單價","type":"NUMBER","required":true},
  {"section":"品項","field_name":"total_amount","label":"總金額","type":"NUMBER"},
  {"section":"品項","field_name":"discount_pct","label":"折扣(%)","type":"NUMBER"},
  {"section":"品項","field_name":"final_amount","label":"折後金額","type":"NUMBER"},
  {"section":"狀態","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"草稿","value":"DRAFT"},{"label":"已報價","value":"SENT"},{"label":"已接受","value":"ACCEPTED"},{"label":"已拒絕","value":"REJECTED"},{"label":"已過期","value":"EXPIRED"}]}
]'::jsonb);
SELECT _sample('SD-QUOTATION','{"quote_no":"QT-001","customer":"好市多","valid_until":"2026-04-15","item_description":"脆薯片(原味) 150g x 30000箱","quantity":30000,"unit_price":35,"total_amount":1050000,"discount_pct":5,"final_amount":997500,"status":"SENT"}');

SELECT _create_op('SD-COMPLAINT','客訴管理','客戶投訴處理追蹤','SD','AlertTriangle',51, '[
  {"section":"客訴資訊","field_name":"complaint_no","label":"客訴編號","type":"TEXT","required":true,"searchable":true},
  {"section":"客訴資訊","field_name":"customer","label":"客戶","type":"TEXT","required":true,"searchable":true},
  {"section":"客訴資訊","field_name":"product","label":"相關產品","type":"TEXT","required":true},
  {"section":"客訴資訊","field_name":"complaint_date","label":"客訴日期","type":"DATE","required":true},
  {"section":"客訴資訊","field_name":"category","label":"客訴類別","type":"DROPDOWN","required":true,"options":[{"label":"品質","value":"QUALITY"},{"label":"交期","value":"DELIVERY"},{"label":"服務","value":"SERVICE"},{"label":"價格","value":"PRICING"},{"label":"其他","value":"OTHER"}]},
  {"section":"客訴資訊","field_name":"severity","label":"嚴重度","type":"DROPDOWN","required":true,"options":[{"label":"低","value":"LOW"},{"label":"中","value":"MEDIUM"},{"label":"高","value":"HIGH"},{"label":"緊急","value":"CRITICAL"}]},
  {"section":"處理","field_name":"description","label":"問題描述","type":"TEXTAREA","required":true},
  {"section":"處理","field_name":"root_cause","label":"根因分析","type":"TEXTAREA"},
  {"section":"處理","field_name":"corrective_action","label":"矯正措施","type":"TEXTAREA"},
  {"section":"處理","field_name":"resolution_date","label":"解決日期","type":"DATE"},
  {"section":"處理","field_name":"status","label":"狀態","type":"DROPDOWN","required":true,"searchable":true,"options":[{"label":"開立","value":"OPEN","is_default":"true"},{"label":"調查中","value":"INVESTIGATING"},{"label":"已解決","value":"RESOLVED"},{"label":"已關閉","value":"CLOSED"}]},
  {"section":"處理","field_name":"assigned_to","label":"負責人","type":"TEXT"}
]'::jsonb);
SELECT _sample('SD-COMPLAINT','{"complaint_no":"CMP-001","customer":"全聯實業","product":"脆薯片(BBQ)","complaint_date":"2026-03-22","category":"QUALITY","severity":"MEDIUM","description":"部分薯片有異味，疑似調味料問題","root_cause":"BBQ調味料供應商批次品質不穩定","corrective_action":"1.更換調味料批次 2.加強進料檢驗 3.通知供應商改善","resolution_date":"2026-03-28","status":"RESOLVED","assigned_to":"劉品管"}');

SELECT _create_op('SD-RETURN','銷售退貨','客戶退貨處理','SD','RotateCcw',52, '[
  {"section":"退貨資訊","field_name":"return_no","label":"退貨單號","type":"TEXT","required":true,"searchable":true},
  {"section":"退貨資訊","field_name":"customer","label":"客戶","type":"TEXT","required":true,"searchable":true},
  {"section":"退貨資訊","field_name":"original_so","label":"原始訂單","type":"TEXT","searchable":true},
  {"section":"品項","field_name":"material","label":"商品","type":"TEXT","required":true},
  {"section":"品項","field_name":"return_qty","label":"退貨數量","type":"NUMBER","required":true},
  {"section":"品項","field_name":"reason","label":"退貨原因","type":"DROPDOWN","required":true,"options":[{"label":"品質不良","value":"DEFECTIVE"},{"label":"交錯貨","value":"WRONG_ITEM"},{"label":"客戶變更","value":"CUSTOMER_CHANGE"},{"label":"運送損壞","value":"DAMAGED"}]},
  {"section":"處理","field_name":"return_date","label":"退貨日期","type":"DATE","required":true},
  {"section":"處理","field_name":"refund_amount","label":"退款金額","type":"NUMBER"},
  {"section":"處理","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"待處理","value":"PENDING"},{"label":"已收貨","value":"RECEIVED"},{"label":"已檢驗","value":"INSPECTED"},{"label":"已退款","value":"REFUNDED"},{"label":"已關閉","value":"CLOSED"}]}
]'::jsonb);
SELECT _sample('SD-RETURN','{"return_no":"SR-001","customer":"大潤發","original_so":"SO-00003","material":"脆薯片(原味)","return_qty":500,"reason":"DAMAGED","return_date":"2026-03-26","refund_amount":17500,"status":"RECEIVED"}');

SELECT _create_op('SD-COMMISSION','佣金計算','業務人員佣金計算','SD','DollarSign',53, '[
  {"section":"佣金資訊","field_name":"salesperson","label":"業務人員","type":"TEXT","required":true,"searchable":true},
  {"section":"佣金資訊","field_name":"period","label":"期間","type":"TEXT","required":true},
  {"section":"金額","field_name":"sales_amount","label":"銷售金額","type":"NUMBER","required":true},
  {"section":"金額","field_name":"commission_rate","label":"佣金率(%)","type":"NUMBER","required":true},
  {"section":"金額","field_name":"commission_amount","label":"佣金金額","type":"NUMBER"},
  {"section":"金額","field_name":"bonus_amount","label":"獎金","type":"NUMBER"},
  {"section":"金額","field_name":"total_payout","label":"總發放","type":"NUMBER"},
  {"section":"狀態","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"已計算","value":"CALCULATED"},{"label":"已核准","value":"APPROVED"},{"label":"已發放","value":"PAID"}]}
]'::jsonb);
SELECT _sample('SD-COMMISSION','{"salesperson":"林美","period":"2026-03","sales_amount":4225000,"commission_rate":1.5,"commission_amount":63375,"bonus_amount":10000,"total_payout":73375,"status":"CALCULATED"}');

SELECT _create_op('SD-FORECAST','銷售預測','產品銷售預測管理','SD','LineChart',54, '[
  {"section":"預測","field_name":"product","label":"產品","type":"TEXT","required":true,"searchable":true},
  {"section":"預測","field_name":"period","label":"期間","type":"TEXT","required":true},
  {"section":"預測","field_name":"forecast_qty","label":"預測數量","type":"NUMBER","required":true},
  {"section":"預測","field_name":"actual_qty","label":"實際數量","type":"NUMBER"},
  {"section":"預測","field_name":"accuracy_pct","label":"準確率(%)","type":"NUMBER"},
  {"section":"預測","field_name":"region","label":"區域","type":"TEXT"},
  {"section":"預測","field_name":"channel","label":"通路","type":"TEXT"},
  {"section":"預測","field_name":"trend","label":"趨勢","type":"DROPDOWN","options":[{"label":"上升","value":"UP"},{"label":"下降","value":"DOWN"},{"label":"持平","value":"STABLE"}]}
]'::jsonb);
SELECT _sample('SD-FORECAST','{"product":"脆薯片(原味)","period":"2026-04","forecast_qty":50000,"actual_qty":null,"accuracy_pct":null,"region":"北區","channel":"量販店","trend":"UP"}');

SELECT _create_op('SD-TARGET','業績目標','業務業績目標追蹤','SD','Target',55, '[
  {"section":"目標","field_name":"salesperson","label":"業務人員","type":"TEXT","required":true,"searchable":true},
  {"section":"目標","field_name":"period","label":"期間","type":"TEXT","required":true},
  {"section":"目標","field_name":"target_amount","label":"目標金額","type":"NUMBER","required":true},
  {"section":"目標","field_name":"achieved_amount","label":"達成金額","type":"NUMBER"},
  {"section":"目標","field_name":"achievement_pct","label":"達成率(%)","type":"NUMBER"},
  {"section":"目標","field_name":"product_mix","label":"產品組合","type":"TEXT"},
  {"section":"目標","field_name":"region","label":"區域","type":"TEXT"},
  {"section":"目標","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"進行中","value":"IN_PROGRESS"},{"label":"已達成","value":"ACHIEVED"},{"label":"未達成","value":"MISSED"}]}
]'::jsonb);
SELECT _sample('SD-TARGET','{"salesperson":"林美","period":"2026 Q1","target_amount":5000000,"achieved_amount":4225000,"achievement_pct":84.5,"product_mix":"零食60%/飲料40%","region":"北區","status":"IN_PROGRESS"}');

SELECT _create_op('SD-CONTRACT','銷售合約','客戶長期合約管理','SD','FileSignature',56, '[
  {"section":"合約","field_name":"contract_no","label":"合約編號","type":"TEXT","required":true,"searchable":true},
  {"section":"合約","field_name":"customer","label":"客戶","type":"TEXT","required":true,"searchable":true},
  {"section":"合約","field_name":"start_date","label":"起始日","type":"DATE","required":true},
  {"section":"合約","field_name":"end_date","label":"結束日","type":"DATE","required":true},
  {"section":"合約","field_name":"agreed_volume","label":"合約數量","type":"NUMBER"},
  {"section":"合約","field_name":"fulfilled_volume","label":"已交數量","type":"NUMBER"},
  {"section":"合約","field_name":"fulfillment_pct","label":"履約率(%)","type":"NUMBER"},
  {"section":"合約","field_name":"price_terms","label":"價格條件","type":"TEXT"},
  {"section":"合約","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"有效","value":"ACTIVE"},{"label":"到期","value":"EXPIRED"},{"label":"終止","value":"TERMINATED"}]}
]'::jsonb);
SELECT _sample('SD-CONTRACT','{"contract_no":"SC-001","customer":"統一超商","start_date":"2026-01-01","end_date":"2026-12-31","agreed_volume":500000,"fulfilled_volume":125000,"fulfillment_pct":25,"price_terms":"固定單價35元/包","status":"ACTIVE"}');

SELECT _create_op('SD-PROMO','促銷活動','促銷活動規劃管理','SD','Megaphone',57, '[
  {"section":"活動","field_name":"promo_code","label":"活動代碼","type":"TEXT","required":true,"searchable":true},
  {"section":"活動","field_name":"promo_name","label":"活動名稱","type":"TEXT","required":true,"searchable":true},
  {"section":"活動","field_name":"start_date","label":"開始日期","type":"DATE","required":true},
  {"section":"活動","field_name":"end_date","label":"結束日期","type":"DATE","required":true},
  {"section":"活動","field_name":"target_product","label":"適用產品","type":"TEXT"},
  {"section":"折扣","field_name":"discount_type","label":"折扣類型","type":"DROPDOWN","options":[{"label":"百分比折扣","value":"PERCENTAGE"},{"label":"固定折扣","value":"FIXED"},{"label":"買一送一","value":"BOGO"}]},
  {"section":"折扣","field_name":"discount_value","label":"折扣值","type":"NUMBER"},
  {"section":"預算","field_name":"budget","label":"預算","type":"NUMBER"},
  {"section":"預算","field_name":"actual_spend","label":"實際花費","type":"NUMBER"},
  {"section":"預算","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"規劃中","value":"PLANNED"},{"label":"進行中","value":"ACTIVE"},{"label":"已結束","value":"ENDED"},{"label":"已取消","value":"CANCELLED"}]}
]'::jsonb);
SELECT _sample('SD-PROMO','{"promo_code":"SPRING2026","promo_name":"春季大促銷","start_date":"2026-03-15","end_date":"2026-04-15","target_product":"全系列零食","discount_type":"PERCENTAGE","discount_value":10,"budget":100000,"actual_spend":65000,"status":"ACTIVE"}');

SELECT _create_op('SD-CUST-VISIT','客戶拜訪','業務客戶拜訪記錄','SD','MapPin',58, '[
  {"section":"拜訪","field_name":"visit_date","label":"拜訪日期","type":"DATE","required":true},
  {"section":"拜訪","field_name":"customer","label":"客戶","type":"TEXT","required":true,"searchable":true},
  {"section":"拜訪","field_name":"contact_person","label":"接洽人","type":"TEXT"},
  {"section":"拜訪","field_name":"salesperson","label":"業務人員","type":"TEXT","required":true},
  {"section":"拜訪","field_name":"visit_type","label":"拜訪類型","type":"DROPDOWN","options":[{"label":"例行拜訪","value":"REGULAR"},{"label":"客訴處理","value":"COMPLAINT"},{"label":"新客開發","value":"NEW_BUSINESS"},{"label":"合約續約","value":"RENEWAL"}]},
  {"section":"內容","field_name":"topics_discussed","label":"討論主題","type":"TEXTAREA"},
  {"section":"內容","field_name":"action_items","label":"後續事項","type":"TEXTAREA"},
  {"section":"追蹤","field_name":"next_visit_date","label":"下次拜訪日","type":"DATE"},
  {"section":"追蹤","field_name":"opportunity_amount","label":"商機金額","type":"NUMBER"},
  {"section":"追蹤","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"已完成","value":"COMPLETED"},{"label":"待追蹤","value":"FOLLOW_UP"},{"label":"已取消","value":"CANCELLED"}]}
]'::jsonb);
SELECT _sample('SD-CUST-VISIT','{"visit_date":"2026-03-18","customer":"好市多","contact_person":"張經理","salesperson":"林美","visit_type":"NEW_BUSINESS","topics_discussed":"討論2026年度合作方案，好市多有意增加飲料品項","action_items":"1.準備飲料系列報價 2.安排試吃活動 3.提供品質認證文件","next_visit_date":"2026-04-01","opportunity_amount":2000000,"status":"FOLLOW_UP"}');

SELECT _create_op('SD-SHIP-TRACK','出貨追蹤','物流出貨追蹤','SD','MapPinned',59, '[
  {"section":"追蹤","field_name":"tracking_no","label":"追蹤號碼","type":"TEXT","required":true,"searchable":true},
  {"section":"追蹤","field_name":"delivery_no","label":"出貨單號","type":"TEXT","searchable":true},
  {"section":"追蹤","field_name":"carrier","label":"物流公司","type":"TEXT"},
  {"section":"追蹤","field_name":"ship_date","label":"出貨日期","type":"DATE"},
  {"section":"追蹤","field_name":"estimated_arrival","label":"預計到達","type":"DATE"},
  {"section":"追蹤","field_name":"actual_arrival","label":"實際到達","type":"DATE"},
  {"section":"追蹤","field_name":"current_location","label":"目前位置","type":"TEXT"},
  {"section":"狀態","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"已取件","value":"PICKED_UP"},{"label":"運送中","value":"IN_TRANSIT"},{"label":"配送中","value":"OUT_FOR_DELIVERY"},{"label":"已送達","value":"DELIVERED"},{"label":"異常","value":"EXCEPTION"}]},
  {"section":"狀態","field_name":"delay_days","label":"延遲天數","type":"NUMBER"},
  {"section":"狀態","field_name":"customer_notified","label":"已通知客戶","type":"DROPDOWN","options":[{"label":"是","value":"YES"},{"label":"否","value":"NO"}]}
]'::jsonb);
SELECT _sample('SD-SHIP-TRACK','{"tracking_no":"HCT-20260325-001","delivery_no":"DLV-001","carrier":"新竹物流","ship_date":"2026-03-25","estimated_arrival":"2026-03-26","actual_arrival":"2026-03-26","current_location":"台北市中山區","status":"DELIVERED","delay_days":0,"customer_notified":"YES"}');

SELECT _create_op('SD-CREDIT-CHK','信用檢查','客戶信用額度檢查','SD','Shield',60, '[
  {"section":"客戶","field_name":"customer","label":"客戶","type":"TEXT","required":true,"searchable":true},
  {"section":"信用","field_name":"credit_limit","label":"信用額度","type":"NUMBER","required":true},
  {"section":"信用","field_name":"current_balance","label":"目前餘額","type":"NUMBER"},
  {"section":"信用","field_name":"available_credit","label":"可用額度","type":"NUMBER"},
  {"section":"信用","field_name":"order_amount","label":"訂單金額","type":"NUMBER"},
  {"section":"檢查","field_name":"check_result","label":"檢查結果","type":"DROPDOWN","options":[{"label":"通過","value":"PASS"},{"label":"警告","value":"WARNING"},{"label":"凍結","value":"BLOCK"}]},
  {"section":"檢查","field_name":"risk_level","label":"風險等級","type":"DROPDOWN","options":[{"label":"低","value":"LOW"},{"label":"中","value":"MEDIUM"},{"label":"高","value":"HIGH"},{"label":"危急","value":"CRITICAL"}]},
  {"section":"帳齡","field_name":"last_payment_date","label":"最近付款日","type":"DATE"},
  {"section":"帳齡","field_name":"days_overdue","label":"逾期天數","type":"NUMBER"}
]'::jsonb);
SELECT _sample('SD-CREDIT-CHK','{"customer":"全聯實業","credit_limit":5000000,"current_balance":1875000,"available_credit":3125000,"order_amount":875000,"check_result":"PASS","risk_level":"LOW","last_payment_date":"2026-03-10","days_overdue":0}');
SELECT _sample('SD-CREDIT-CHK','{"customer":"大潤發","credit_limit":3000000,"current_balance":2800000,"available_credit":200000,"order_amount":500000,"check_result":"WARNING","risk_level":"HIGH","last_payment_date":"2026-02-15","days_overdue":15}');

SELECT _create_op('SD-REBATE','返利管理','客戶返利計算','SD','Gift',61, '[
  {"section":"返利","field_name":"rebate_no","label":"返利編號","type":"TEXT","required":true,"searchable":true},
  {"section":"返利","field_name":"customer","label":"客戶","type":"TEXT","required":true,"searchable":true},
  {"section":"返利","field_name":"period","label":"期間","type":"TEXT","required":true},
  {"section":"計算","field_name":"sales_volume","label":"銷售金額","type":"NUMBER","required":true},
  {"section":"計算","field_name":"tier","label":"等級","type":"DROPDOWN","options":[{"label":"銅","value":"BRONZE"},{"label":"銀","value":"SILVER"},{"label":"金","value":"GOLD"},{"label":"白金","value":"PLATINUM"}]},
  {"section":"計算","field_name":"rebate_rate","label":"返利率(%)","type":"NUMBER"},
  {"section":"計算","field_name":"rebate_amount","label":"返利金額","type":"NUMBER"},
  {"section":"狀態","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"已計算","value":"CALCULATED"},{"label":"已核准","value":"APPROVED"},{"label":"已開立","value":"ISSUED"}]}
]'::jsonb);
SELECT _sample('SD-REBATE','{"rebate_no":"RB-2026Q1-001","customer":"統一超商","period":"2026 Q1","sales_volume":3500000,"tier":"GOLD","rebate_rate":3,"rebate_amount":105000,"status":"CALCULATED"}');

-- Continue with PP, QM, HR, WM, GEN modules...
-- (Using same _create_op pattern)

-- ═══════════════════════════════════════════════════════════════════
-- PP MODULE (12 operations)
-- ═══════════════════════════════════════════════════════════════════

SELECT _create_op('PP-ROUTING','工藝路線','產品製程工藝路線','PP','Route',50, '[
  {"section":"路線","field_name":"routing_no","label":"路線編號","type":"TEXT","required":true,"searchable":true},
  {"section":"路線","field_name":"product","label":"產品","type":"TEXT","required":true,"searchable":true},
  {"section":"路線","field_name":"operation_seq","label":"工序順序","type":"NUMBER","required":true},
  {"section":"路線","field_name":"work_center","label":"工作中心","type":"TEXT","required":true},
  {"section":"路線","field_name":"description","label":"工序說明","type":"TEXT"},
  {"section":"工時","field_name":"setup_time","label":"準備時間(分)","type":"NUMBER"},
  {"section":"工時","field_name":"run_time_per_unit","label":"單位工時(分)","type":"NUMBER"},
  {"section":"成本","field_name":"labor_cost","label":"人工成本","type":"NUMBER"},
  {"section":"成本","field_name":"machine_cost","label":"機器成本","type":"NUMBER"},
  {"section":"成本","field_name":"total_cost","label":"工序總成本","type":"NUMBER"}
]'::jsonb);
SELECT _sample('PP-ROUTING','{"routing_no":"RT-001","product":"脆薯片(原味)","operation_seq":1,"work_center":"混合區","description":"原料混合與調味","setup_time":30,"run_time_per_unit":0.5,"labor_cost":500,"machine_cost":300,"total_cost":800}');
SELECT _sample('PP-ROUTING','{"routing_no":"RT-001","product":"脆薯片(原味)","operation_seq":2,"work_center":"油炸區","description":"油炸成型","setup_time":20,"run_time_per_unit":0.3,"labor_cost":400,"machine_cost":600,"total_cost":1000}');

SELECT _create_op('PP-WORK-CTR','工作中心','生產工作中心管理','PP','Factory',51, '[
  {"section":"基本","field_name":"wc_code","label":"工作中心代碼","type":"TEXT","required":true,"searchable":true},
  {"section":"基本","field_name":"wc_name","label":"工作中心名稱","type":"TEXT","required":true,"searchable":true},
  {"section":"基本","field_name":"category","label":"類別","type":"DROPDOWN","options":[{"label":"混合","value":"MIXING"},{"label":"烹調","value":"COOKING"},{"label":"包裝","value":"PACKAGING"},{"label":"測試","value":"TESTING"},{"label":"倉儲","value":"STORAGE"}]},
  {"section":"產能","field_name":"capacity_per_hour","label":"每小時產能","type":"NUMBER"},
  {"section":"產能","field_name":"available_hours","label":"可用小時/天","type":"NUMBER"},
  {"section":"產能","field_name":"utilization_pct","label":"利用率(%)","type":"NUMBER"},
  {"section":"產能","field_name":"cost_rate","label":"小時費率","type":"NUMBER"},
  {"section":"產能","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"運作中","value":"ACTIVE"},{"label":"維修中","value":"MAINTENANCE"},{"label":"停用","value":"INACTIVE"}]},
  {"section":"產能","field_name":"supervisor","label":"負責人","type":"TEXT"}
]'::jsonb);
SELECT _sample('PP-WORK-CTR','{"wc_code":"WC-MIX","wc_name":"混合區","category":"MIXING","capacity_per_hour":500,"available_hours":16,"utilization_pct":85,"cost_rate":800,"status":"ACTIVE","supervisor":"張莉"}');

SELECT _create_op('PP-MRP','物料需求計畫','MRP計算結果','PP','Calculator',52, '[
  {"section":"需求","field_name":"material","label":"物料","type":"TEXT","required":true,"searchable":true},
  {"section":"需求","field_name":"required_qty","label":"需求數量","type":"NUMBER","required":true},
  {"section":"供給","field_name":"on_hand","label":"現有庫存","type":"NUMBER"},
  {"section":"供給","field_name":"on_order","label":"在途數量","type":"NUMBER"},
  {"section":"供給","field_name":"available","label":"可用量","type":"NUMBER"},
  {"section":"建議","field_name":"net_requirement","label":"淨需求","type":"NUMBER"},
  {"section":"建議","field_name":"lead_time","label":"前置天數","type":"NUMBER"},
  {"section":"建議","field_name":"suggested_date","label":"建議下單日","type":"DATE"},
  {"section":"建議","field_name":"demand_source","label":"需求來源","type":"DROPDOWN","options":[{"label":"銷售訂單","value":"SO"},{"label":"預測","value":"FORECAST"},{"label":"安全庫存","value":"SAFETY_STOCK"}]},
  {"section":"建議","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"建議","value":"SUGGESTED"},{"label":"已確認","value":"CONFIRMED"},{"label":"已下單","value":"ORDERED"}]}
]'::jsonb);
SELECT _sample('PP-MRP','{"material":"馬鈴薯粉","required_qty":5000,"on_hand":1875,"on_order":2000,"available":3875,"net_requirement":1125,"lead_time":7,"suggested_date":"2026-04-08","demand_source":"SO","status":"SUGGESTED"}');

SELECT _create_op('PP-SCHED','生產排程','生產排程管理','PP','Calendar',53, '[
  {"section":"排程","field_name":"schedule_no","label":"排程編號","type":"TEXT","required":true,"searchable":true},
  {"section":"排程","field_name":"product","label":"產品","type":"TEXT","required":true,"searchable":true},
  {"section":"排程","field_name":"quantity","label":"數量","type":"NUMBER","required":true},
  {"section":"排程","field_name":"start_date","label":"開始日期","type":"DATE","required":true},
  {"section":"排程","field_name":"end_date","label":"結束日期","type":"DATE","required":true},
  {"section":"排程","field_name":"work_center","label":"工作中心","type":"TEXT","required":true},
  {"section":"排程","field_name":"priority","label":"優先順序","type":"DROPDOWN","options":[{"label":"1(最高)","value":"1"},{"label":"2","value":"2"},{"label":"3(一般)","value":"3"},{"label":"4","value":"4"},{"label":"5(最低)","value":"5"}]},
  {"section":"排程","field_name":"assigned_to","label":"負責人","type":"TEXT"},
  {"section":"排程","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"已排程","value":"SCHEDULED"},{"label":"進行中","value":"IN_PROGRESS"},{"label":"已完成","value":"COMPLETED"},{"label":"延遲","value":"DELAYED"}]}
]'::jsonb);
SELECT _sample('PP-SCHED','{"schedule_no":"SCH-001","product":"脆薯片(原味)","quantity":10000,"start_date":"2026-03-20","end_date":"2026-03-22","work_center":"WC-MIX","priority":"2","assigned_to":"張莉","status":"COMPLETED"}');

SELECT _create_op('PP-CONFIRM','生產報工','生產工序完工報告','PP','CheckCircle',54, '[
  {"section":"報工","field_name":"confirmation_no","label":"報工編號","type":"TEXT","required":true,"searchable":true},
  {"section":"報工","field_name":"production_order","label":"生產工單","type":"TEXT","required":true,"searchable":true},
  {"section":"報工","field_name":"work_center","label":"工作中心","type":"TEXT","required":true},
  {"section":"報工","field_name":"operator","label":"操作員","type":"TEXT","required":true},
  {"section":"數量","field_name":"quantity_good","label":"良品數量","type":"NUMBER","required":true},
  {"section":"數量","field_name":"quantity_scrap","label":"報廢數量","type":"NUMBER"},
  {"section":"數量","field_name":"yield_pct","label":"良率(%)","type":"NUMBER"},
  {"section":"工時","field_name":"start_time","label":"開始時間","type":"TEXT"},
  {"section":"工時","field_name":"end_time","label":"結束時間","type":"TEXT"},
  {"section":"工時","field_name":"notes","label":"備註","type":"TEXTAREA"}
]'::jsonb);
SELECT _sample('PP-CONFIRM','{"confirmation_no":"CNF-001","production_order":"PRD-00001","work_center":"WC-MIX","operator":"作業員小王","quantity_good":9800,"quantity_scrap":200,"yield_pct":98,"start_time":"08:00","end_time":"16:30","notes":"混合工序完成，少量結塊報廢"}');

SELECT _create_op('PP-SCRAP','報廢處理','生產報廢記錄','PP','Trash2',55, '[
  {"section":"報廢","field_name":"scrap_no","label":"報廢單號","type":"TEXT","required":true,"searchable":true},
  {"section":"報廢","field_name":"material","label":"物料","type":"TEXT","required":true,"searchable":true},
  {"section":"報廢","field_name":"batch_number","label":"批號","type":"TEXT","searchable":true},
  {"section":"報廢","field_name":"quantity","label":"報廢數量","type":"NUMBER","required":true},
  {"section":"報廢","field_name":"unit","label":"單位","type":"TEXT"},
  {"section":"報廢","field_name":"scrap_reason","label":"報廢原因","type":"DROPDOWN","required":true,"options":[{"label":"已過期","value":"EXPIRED"},{"label":"品質不良","value":"DEFECTIVE"},{"label":"受污染","value":"CONTAMINATED"},{"label":"損壞","value":"DAMAGED"},{"label":"淘汰","value":"OBSOLETE"}]},
  {"section":"處理","field_name":"scrap_value","label":"報廢金額","type":"NUMBER"},
  {"section":"處理","field_name":"disposal_method","label":"處置方式","type":"DROPDOWN","options":[{"label":"銷毀","value":"DESTROY"},{"label":"回收","value":"RECYCLE"},{"label":"退回","value":"RETURN"}]},
  {"section":"處理","field_name":"scrap_date","label":"報廢日期","type":"DATE","required":true},
  {"section":"處理","field_name":"approved_by","label":"核准人","type":"TEXT"},
  {"section":"處理","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"待核准","value":"PENDING"},{"label":"已核准","value":"APPROVED"},{"label":"已處置","value":"DISPOSED"}]}
]'::jsonb);
SELECT _sample('PP-SCRAP','{"scrap_no":"SCP-001","material":"脆薯片(BBQ)","batch_number":"BAT-P20260315","quantity":500,"unit":"PCS","scrap_reason":"DEFECTIVE","scrap_value":17500,"disposal_method":"DESTROY","scrap_date":"2026-03-22","approved_by":"張莉","status":"DISPOSED"}');

SELECT _create_op('PP-MAINT-REQ','設備維修申請','設備維修保養申請','PP','Wrench',56, '[
  {"section":"申請","field_name":"request_no","label":"申請單號","type":"TEXT","required":true,"searchable":true},
  {"section":"申請","field_name":"equipment_name","label":"設備名稱","type":"TEXT","required":true,"searchable":true},
  {"section":"申請","field_name":"equipment_code","label":"設備編號","type":"TEXT","searchable":true},
  {"section":"申請","field_name":"issue_description","label":"故障描述","type":"TEXTAREA","required":true},
  {"section":"申請","field_name":"urgency","label":"緊急度","type":"DROPDOWN","required":true,"options":[{"label":"緊急","value":"EMERGENCY"},{"label":"高","value":"HIGH"},{"label":"一般","value":"NORMAL"},{"label":"低","value":"LOW"}]},
  {"section":"處理","field_name":"reported_by","label":"報修人","type":"TEXT","required":true},
  {"section":"處理","field_name":"reported_date","label":"報修日期","type":"DATE","required":true},
  {"section":"處理","field_name":"assigned_to","label":"指派維修員","type":"TEXT"},
  {"section":"處理","field_name":"estimated_downtime","label":"預估停機(小時)","type":"NUMBER"},
  {"section":"處理","field_name":"actual_downtime","label":"實際停機(小時)","type":"NUMBER"},
  {"section":"處理","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"已報修","value":"REPORTED"},{"label":"已指派","value":"ASSIGNED"},{"label":"維修中","value":"IN_PROGRESS"},{"label":"已完成","value":"COMPLETED"}]}
]'::jsonb);
SELECT _sample('PP-MAINT-REQ','{"request_no":"MR-001","equipment_name":"薯片油炸機A","equipment_code":"EQ-FRY-001","issue_description":"油溫控制器異常，溫度波動超過±5度","urgency":"HIGH","reported_by":"張莉","reported_date":"2026-03-21","assigned_to":"維修組李師傅","estimated_downtime":4,"actual_downtime":3,"status":"COMPLETED"}');

SELECT _create_op('PP-OEE','設備綜合效率','OEE指標追蹤','PP','Gauge',57, '[
  {"section":"設備","field_name":"equipment","label":"設備名稱","type":"TEXT","required":true,"searchable":true},
  {"section":"設備","field_name":"period","label":"期間","type":"TEXT","required":true},
  {"section":"指標","field_name":"availability_pct","label":"稼動率(%)","type":"NUMBER","required":true},
  {"section":"指標","field_name":"performance_pct","label":"性能率(%)","type":"NUMBER","required":true},
  {"section":"指標","field_name":"quality_pct","label":"良率(%)","type":"NUMBER","required":true},
  {"section":"指標","field_name":"oee_pct","label":"OEE(%)","type":"NUMBER"},
  {"section":"明細","field_name":"planned_run_time","label":"計畫運轉時間","type":"NUMBER"},
  {"section":"明細","field_name":"actual_run_time","label":"實際運轉時間","type":"NUMBER"},
  {"section":"明細","field_name":"total_pieces","label":"總生產數","type":"NUMBER"},
  {"section":"明細","field_name":"good_pieces","label":"良品數","type":"NUMBER"}
]'::jsonb);
SELECT _sample('PP-OEE','{"equipment":"薯片生產線A","period":"2026-03","availability_pct":92,"performance_pct":88,"quality_pct":98,"oee_pct":79.4,"planned_run_time":480,"actual_run_time":442,"total_pieces":50000,"good_pieces":49000}');

SELECT _create_op('PP-DOWNTIME','停機記錄','設備停機原因記錄','PP','Pause',58, '[
  {"section":"停機","field_name":"record_no","label":"記錄編號","type":"TEXT","required":true,"searchable":true},
  {"section":"停機","field_name":"equipment","label":"設備","type":"TEXT","required":true,"searchable":true},
  {"section":"停機","field_name":"start_time","label":"開始時間","type":"TEXT","required":true},
  {"section":"停機","field_name":"end_time","label":"結束時間","type":"TEXT"},
  {"section":"停機","field_name":"duration_hours","label":"停機時數","type":"NUMBER"},
  {"section":"原因","field_name":"reason","label":"停機原因","type":"DROPDOWN","required":true,"options":[{"label":"故障","value":"BREAKDOWN"},{"label":"保養","value":"MAINTENANCE"},{"label":"換線","value":"CHANGEOVER"},{"label":"缺料","value":"NO_MATERIAL"},{"label":"無訂單","value":"NO_ORDER"}]},
  {"section":"原因","field_name":"impact_qty","label":"影響產量","type":"NUMBER"},
  {"section":"原因","field_name":"cost_impact","label":"損失金額","type":"NUMBER"},
  {"section":"原因","field_name":"resolution","label":"處理方式","type":"TEXTAREA"}
]'::jsonb);
SELECT _sample('PP-DOWNTIME','{"record_no":"DT-001","equipment":"薯片油炸機A","start_time":"2026-03-21 10:00","end_time":"2026-03-21 13:00","duration_hours":3,"reason":"BREAKDOWN","impact_qty":1500,"cost_impact":52500,"resolution":"更換油溫控制器模組"}');

SELECT _create_op('PP-SHIFT','班次管理','生產班次產能管理','PP','Clock',59, '[
  {"section":"班次","field_name":"shift_date","label":"日期","type":"DATE","required":true},
  {"section":"班次","field_name":"shift_type","label":"班別","type":"DROPDOWN","required":true,"options":[{"label":"早班","value":"DAY"},{"label":"晚班","value":"NIGHT"},{"label":"全天","value":"FULL"}]},
  {"section":"班次","field_name":"work_center","label":"工作中心","type":"TEXT","required":true,"searchable":true},
  {"section":"人力","field_name":"planned_headcount","label":"計畫人數","type":"NUMBER"},
  {"section":"人力","field_name":"actual_headcount","label":"實際人數","type":"NUMBER"},
  {"section":"產量","field_name":"production_target","label":"目標產量","type":"NUMBER"},
  {"section":"產量","field_name":"actual_production","label":"實際產量","type":"NUMBER"},
  {"section":"產量","field_name":"efficiency_pct","label":"效率(%)","type":"NUMBER"}
]'::jsonb);
SELECT _sample('PP-SHIFT','{"shift_date":"2026-03-22","shift_type":"DAY","work_center":"WC-MIX","planned_headcount":8,"actual_headcount":7,"production_target":5000,"actual_production":4600,"efficiency_pct":92}');

SELECT _create_op('PP-REWORK','重工單','不良品重工處理','PP','RotateCw',60, '[
  {"section":"重工","field_name":"rework_no","label":"重工單號","type":"TEXT","required":true,"searchable":true},
  {"section":"重工","field_name":"original_order","label":"原始工單","type":"TEXT","searchable":true},
  {"section":"重工","field_name":"material","label":"產品","type":"TEXT","required":true},
  {"section":"重工","field_name":"quantity","label":"重工數量","type":"NUMBER","required":true},
  {"section":"重工","field_name":"defect_type","label":"缺陷類型","type":"TEXT","required":true},
  {"section":"重工","field_name":"rework_procedure","label":"重工程序","type":"TEXTAREA"},
  {"section":"成本","field_name":"estimated_cost","label":"預估成本","type":"NUMBER"},
  {"section":"成本","field_name":"actual_cost","label":"實際成本","type":"NUMBER"},
  {"section":"時程","field_name":"start_date","label":"開始日","type":"DATE"},
  {"section":"時程","field_name":"completion_date","label":"完成日","type":"DATE"},
  {"section":"時程","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"待處理","value":"PENDING"},{"label":"進行中","value":"IN_PROGRESS"},{"label":"已完成","value":"COMPLETED"},{"label":"已取消","value":"CANCELLED"}]}
]'::jsonb);
SELECT _sample('PP-REWORK','{"rework_no":"RW-001","original_order":"PRD-00001","material":"脆薯片(原味)","quantity":200,"defect_type":"調味不均","rework_procedure":"重新過篩並二次調味","estimated_cost":3000,"actual_cost":2500,"start_date":"2026-03-23","completion_date":"2026-03-23","status":"COMPLETED"}');

-- ═══════════════════════════════════════════════════════════════════
-- QM MODULE (10 operations, additional to QM-INSP)
-- ═══════════════════════════════════════════════════════════════════

SELECT _create_op('QM-NONCONF','不合格品報告','NCR不合格品處理','QM','XCircle',30, '[
  {"section":"NCR","field_name":"ncr_no","label":"NCR編號","type":"TEXT","required":true,"searchable":true},
  {"section":"NCR","field_name":"product","label":"產品","type":"TEXT","required":true,"searchable":true},
  {"section":"NCR","field_name":"batch_number","label":"批號","type":"TEXT","searchable":true},
  {"section":"缺陷","field_name":"defect_type","label":"缺陷類型","type":"TEXT","required":true},
  {"section":"缺陷","field_name":"defect_qty","label":"不合格數","type":"NUMBER","required":true},
  {"section":"缺陷","field_name":"total_qty","label":"總數量","type":"NUMBER"},
  {"section":"缺陷","field_name":"defect_rate","label":"不合格率(%)","type":"NUMBER"},
  {"section":"處理","field_name":"severity","label":"嚴重度","type":"DROPDOWN","options":[{"label":"輕微","value":"MINOR"},{"label":"重大","value":"MAJOR"},{"label":"致命","value":"CRITICAL"}]},
  {"section":"處理","field_name":"root_cause","label":"根本原因","type":"TEXTAREA"},
  {"section":"處理","field_name":"disposition","label":"處置方式","type":"DROPDOWN","options":[{"label":"照用","value":"USE_AS_IS"},{"label":"重工","value":"REWORK"},{"label":"報廢","value":"SCRAP"},{"label":"退回","value":"RETURN"}]},
  {"section":"處理","field_name":"cost_impact","label":"損失金額","type":"NUMBER"},
  {"section":"處理","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"開立","value":"OPEN"},{"label":"調查中","value":"INVESTIGATING"},{"label":"已處置","value":"DISPOSED"},{"label":"已關閉","value":"CLOSED"}]}
]'::jsonb);
SELECT _sample('QM-NONCONF','{"ncr_no":"NCR-001","product":"脆薯片(BBQ)","batch_number":"BAT-P20260315","defect_type":"調味異常","defect_qty":500,"total_qty":10000,"defect_rate":5,"severity":"MAJOR","root_cause":"BBQ調味料批次不穩定，含水率過高","disposition":"SCRAP","cost_impact":17500,"status":"CLOSED"}');

SELECT _create_op('QM-CAPA','矯正預防措施','CAPA矯正預防管理','QM','Shield',31, '[
  {"section":"CAPA","field_name":"capa_no","label":"CAPA編號","type":"TEXT","required":true,"searchable":true},
  {"section":"CAPA","field_name":"type","label":"類型","type":"DROPDOWN","required":true,"options":[{"label":"矯正措施","value":"CORRECTIVE"},{"label":"預防措施","value":"PREVENTIVE"}]},
  {"section":"CAPA","field_name":"source","label":"來源","type":"DROPDOWN","options":[{"label":"客訴","value":"COMPLAINT"},{"label":"稽核","value":"AUDIT"},{"label":"NCR","value":"NCR"},{"label":"觀察","value":"OBSERVATION"}]},
  {"section":"CAPA","field_name":"description","label":"問題描述","type":"TEXTAREA","required":true},
  {"section":"分析","field_name":"root_cause_analysis","label":"根因分析","type":"TEXTAREA"},
  {"section":"分析","field_name":"action_plan","label":"行動計畫","type":"TEXTAREA","required":true},
  {"section":"追蹤","field_name":"responsible","label":"負責人","type":"TEXT","required":true},
  {"section":"追蹤","field_name":"due_date","label":"到期日","type":"DATE","required":true},
  {"section":"追蹤","field_name":"completion_date","label":"完成日","type":"DATE"},
  {"section":"追蹤","field_name":"effectiveness_check","label":"有效性確認","type":"TEXTAREA"},
  {"section":"追蹤","field_name":"status","label":"狀態","type":"DROPDOWN","required":true,"options":[{"label":"開立","value":"OPEN"},{"label":"進行中","value":"IN_PROGRESS"},{"label":"已完成","value":"COMPLETED"},{"label":"已驗證","value":"VERIFIED"},{"label":"已關閉","value":"CLOSED"}]}
]'::jsonb);
SELECT _sample('QM-CAPA','{"capa_no":"CAPA-001","type":"CORRECTIVE","source":"COMPLAINT","description":"客戶反映薯片有異味問題","root_cause_analysis":"BBQ調味料供應商品質不穩定，進料檢驗未覆蓋風味測試","action_plan":"1.增加進料感官檢測 2.建立調味料風味標準 3.供應商改善追蹤","responsible":"劉品管","due_date":"2026-04-15","completion_date":null,"effectiveness_check":"","status":"IN_PROGRESS"}');

SELECT _create_op('QM-AUDIT','內部稽核','品質內部稽核管理','QM','Search',32, '[
  {"section":"稽核","field_name":"audit_no","label":"稽核編號","type":"TEXT","required":true,"searchable":true},
  {"section":"稽核","field_name":"audit_type","label":"稽核類型","type":"DROPDOWN","required":true,"options":[{"label":"系統稽核","value":"SYSTEM"},{"label":"製程稽核","value":"PROCESS"},{"label":"產品稽核","value":"PRODUCT"},{"label":"供應商稽核","value":"SUPPLIER"}]},
  {"section":"稽核","field_name":"scope","label":"稽核範圍","type":"TEXT","required":true},
  {"section":"稽核","field_name":"lead_auditor","label":"主稽核員","type":"TEXT","required":true},
  {"section":"稽核","field_name":"audit_date","label":"稽核日期","type":"DATE","required":true},
  {"section":"發現","field_name":"findings_count","label":"發現總數","type":"NUMBER"},
  {"section":"發現","field_name":"major_findings","label":"重大發現","type":"NUMBER"},
  {"section":"發現","field_name":"minor_findings","label":"輕微發現","type":"NUMBER"},
  {"section":"結果","field_name":"overall_result","label":"整體結果","type":"DROPDOWN","options":[{"label":"通過","value":"PASS"},{"label":"有條件通過","value":"CONDITIONAL"},{"label":"不通過","value":"FAIL"}]},
  {"section":"結果","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"計畫中","value":"PLANNED"},{"label":"執行中","value":"IN_PROGRESS"},{"label":"已完成","value":"COMPLETED"},{"label":"追蹤中","value":"FOLLOW_UP"}]}
]'::jsonb);
SELECT _sample('QM-AUDIT','{"audit_no":"AUD-001","audit_type":"PROCESS","scope":"薯片生產線品質管控","lead_auditor":"劉品管","audit_date":"2026-03-15","findings_count":5,"major_findings":1,"minor_findings":4,"overall_result":"CONDITIONAL","status":"FOLLOW_UP"}');

SELECT _create_op('QM-FOOD-SAFE','食品安全檢查','每日食品安全巡檢','QM','Utensils',33, '[
  {"section":"檢查","field_name":"check_no","label":"檢查編號","type":"TEXT","required":true,"searchable":true},
  {"section":"檢查","field_name":"area","label":"檢查區域","type":"TEXT","required":true},
  {"section":"檢查","field_name":"check_date","label":"檢查日期","type":"DATE","required":true},
  {"section":"環境","field_name":"temperature","label":"溫度(°C)","type":"NUMBER"},
  {"section":"環境","field_name":"humidity","label":"濕度(%)","type":"NUMBER"},
  {"section":"項目","field_name":"cleanliness","label":"清潔度","type":"DROPDOWN","options":[{"label":"合格","value":"PASS"},{"label":"不合格","value":"FAIL"}]},
  {"section":"項目","field_name":"pest_control","label":"蟲害防治","type":"DROPDOWN","options":[{"label":"合格","value":"PASS"},{"label":"不合格","value":"FAIL"}]},
  {"section":"項目","field_name":"hand_hygiene","label":"手部衛生","type":"DROPDOWN","options":[{"label":"合格","value":"PASS"},{"label":"不合格","value":"FAIL"}]},
  {"section":"項目","field_name":"equipment_clean","label":"設備清潔","type":"DROPDOWN","options":[{"label":"合格","value":"PASS"},{"label":"不合格","value":"FAIL"}]},
  {"section":"結果","field_name":"overall_result","label":"總評","type":"DROPDOWN","options":[{"label":"合格","value":"PASS"},{"label":"不合格","value":"FAIL"}]},
  {"section":"結果","field_name":"inspector","label":"檢查員","type":"TEXT","required":true},
  {"section":"結果","field_name":"corrective_action","label":"矯正措施","type":"TEXTAREA"}
]'::jsonb);
SELECT _sample('QM-FOOD-SAFE','{"check_no":"FS-20260320","area":"油炸區","check_date":"2026-03-20","temperature":22,"humidity":55,"cleanliness":"PASS","pest_control":"PASS","hand_hygiene":"PASS","equipment_clean":"PASS","overall_result":"PASS","inspector":"劉品管","corrective_action":""}');

SELECT _create_op('QM-CALIBR','儀器校正','量測儀器校正管理','QM','Crosshair',34, '[
  {"section":"儀器","field_name":"instrument_no","label":"儀器編號","type":"TEXT","required":true,"searchable":true},
  {"section":"儀器","field_name":"instrument_name","label":"儀器名稱","type":"TEXT","required":true,"searchable":true},
  {"section":"儀器","field_name":"serial_number","label":"序號","type":"TEXT"},
  {"section":"儀器","field_name":"location","label":"位置","type":"TEXT"},
  {"section":"校正","field_name":"calibration_date","label":"校正日期","type":"DATE","required":true},
  {"section":"校正","field_name":"next_calibration","label":"下次校正","type":"DATE","required":true},
  {"section":"校正","field_name":"calibration_result","label":"校正結果","type":"DROPDOWN","options":[{"label":"合格","value":"PASS"},{"label":"不合格","value":"FAIL"},{"label":"已調整","value":"ADJUSTED"}]},
  {"section":"校正","field_name":"certificate_no","label":"證書編號","type":"TEXT"},
  {"section":"校正","field_name":"performed_by","label":"執行人","type":"TEXT"},
  {"section":"校正","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"已校正","value":"CALIBRATED"},{"label":"待校正","value":"DUE"},{"label":"逾期","value":"OVERDUE"},{"label":"停用","value":"OUT_OF_SERVICE"}]}
]'::jsonb);
SELECT _sample('QM-CALIBR','{"instrument_no":"CAL-001","instrument_name":"電子秤 (0.01g)","serial_number":"SN-20230101","location":"品管實驗室","calibration_date":"2026-03-01","next_calibration":"2026-09-01","calibration_result":"PASS","certificate_no":"CAL-2026-0301","performed_by":"SGS","status":"CALIBRATED"}');

-- ═══════════════════════════════════════════════════════════════════
-- HR MODULE (12 operations, additional to HR-LEAVE)
-- ═══════════════════════════════════════════════════════════════════

SELECT _create_op('HR-RECRUIT','招募申請','人才招募需求申請','HR','UserPlus',40, '[
  {"section":"需求","field_name":"requisition_no","label":"申請單號","type":"TEXT","required":true,"searchable":true},
  {"section":"需求","field_name":"position","label":"職位名稱","type":"TEXT","required":true,"searchable":true},
  {"section":"需求","field_name":"department","label":"部門","type":"TEXT","required":true},
  {"section":"需求","field_name":"headcount","label":"需求人數","type":"NUMBER","required":true},
  {"section":"需求","field_name":"job_level","label":"職級","type":"DROPDOWN","options":[{"label":"基層","value":"STAFF"},{"label":"資深","value":"SENIOR"},{"label":"主管","value":"MANAGER"},{"label":"主任","value":"DIRECTOR"}]},
  {"section":"薪資","field_name":"salary_range_min","label":"薪資下限","type":"NUMBER"},
  {"section":"薪資","field_name":"salary_range_max","label":"薪資上限","type":"NUMBER"},
  {"section":"時程","field_name":"required_date","label":"期望到職日","type":"DATE"},
  {"section":"時程","field_name":"justification","label":"需求說明","type":"TEXTAREA","required":true},
  {"section":"審核","field_name":"status","label":"狀態","type":"DROPDOWN","required":true,"searchable":true,"options":[{"label":"草稿","value":"DRAFT"},{"label":"已提交","value":"SUBMITTED"},{"label":"已核准","value":"APPROVED"},{"label":"招募中","value":"RECRUITING"},{"label":"已錄取","value":"FILLED"},{"label":"已取消","value":"CANCELLED"}]},
  {"section":"審核","field_name":"recruiter","label":"招募負責人","type":"TEXT"}
]'::jsonb);
SELECT _sample('HR-RECRUIT','{"requisition_no":"REQ-001","position":"品管工程師","department":"品管部","headcount":1,"job_level":"SENIOR","salary_range_min":55000,"salary_range_max":70000,"required_date":"2026-05-01","justification":"品管部人力不足，需增加一名資深品管工程師負責新產品線品質管控","status":"APPROVED","recruiter":"趙人資"}');

SELECT _create_op('HR-TRAINING','教育訓練','員工訓練課程管理','HR','GraduationCap',41, '[
  {"section":"課程","field_name":"training_no","label":"課程編號","type":"TEXT","required":true,"searchable":true},
  {"section":"課程","field_name":"training_name","label":"課程名稱","type":"TEXT","required":true,"searchable":true},
  {"section":"課程","field_name":"category","label":"類別","type":"DROPDOWN","required":true,"options":[{"label":"安全","value":"SAFETY"},{"label":"品質","value":"QUALITY"},{"label":"技能","value":"SKILL"},{"label":"法遵","value":"COMPLIANCE"},{"label":"管理","value":"LEADERSHIP"}]},
  {"section":"課程","field_name":"instructor","label":"講師","type":"TEXT","required":true},
  {"section":"課程","field_name":"training_date","label":"訓練日期","type":"DATE","required":true},
  {"section":"課程","field_name":"duration_hours","label":"時數","type":"NUMBER","required":true},
  {"section":"出席","field_name":"attendees_planned","label":"計畫人數","type":"NUMBER"},
  {"section":"出席","field_name":"attendees_actual","label":"實際人數","type":"NUMBER"},
  {"section":"出席","field_name":"attendance_rate","label":"出席率(%)","type":"NUMBER"},
  {"section":"評估","field_name":"evaluation_score","label":"評鑑分數","type":"NUMBER"},
  {"section":"評估","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"計畫中","value":"PLANNED"},{"label":"進行中","value":"IN_PROGRESS"},{"label":"已完成","value":"COMPLETED"},{"label":"已取消","value":"CANCELLED"}]}
]'::jsonb);
SELECT _sample('HR-TRAINING','{"training_no":"TR-001","training_name":"食品安全衛生教育","category":"SAFETY","instructor":"劉品管","training_date":"2026-03-10","duration_hours":4,"attendees_planned":30,"attendees_actual":28,"attendance_rate":93.3,"evaluation_score":4.5,"status":"COMPLETED"}');

SELECT _create_op('HR-PERF-EVAL','績效考核','員工績效評估','HR','Star',42, '[
  {"section":"考核","field_name":"employee","label":"員工姓名","type":"TEXT","required":true,"searchable":true},
  {"section":"考核","field_name":"evaluation_period","label":"考核期間","type":"TEXT","required":true},
  {"section":"考核","field_name":"evaluator","label":"考核人","type":"TEXT","required":true},
  {"section":"評分","field_name":"goal_achievement","label":"目標達成(1-100)","type":"NUMBER","required":true,"min":0,"max":100},
  {"section":"評分","field_name":"competency_score","label":"職能評分(1-100)","type":"NUMBER","required":true,"min":0,"max":100},
  {"section":"評分","field_name":"overall_score","label":"總分","type":"NUMBER"},
  {"section":"評分","field_name":"rating","label":"等級","type":"DROPDOWN","options":[{"label":"A (傑出)","value":"A"},{"label":"B (優良)","value":"B"},{"label":"C (稱職)","value":"C"},{"label":"D (待改善)","value":"D"}]},
  {"section":"回饋","field_name":"strengths","label":"優點","type":"TEXTAREA"},
  {"section":"回饋","field_name":"improvements","label":"待改善","type":"TEXTAREA"},
  {"section":"回饋","field_name":"development_plan","label":"發展計畫","type":"TEXTAREA"},
  {"section":"狀態","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"草稿","value":"DRAFT"},{"label":"已送出","value":"SUBMITTED"},{"label":"已面談","value":"REVIEWED"},{"label":"已定案","value":"FINALIZED"}]}
]'::jsonb);
SELECT _sample('HR-PERF-EVAL','{"employee":"林美","evaluation_period":"2025 H2","evaluator":"陳偉","goal_achievement":88,"competency_score":82,"overall_score":85.6,"rating":"B","strengths":"業務開發能力強，客戶關係維護優秀","improvements":"跨部門溝通可再加強","development_plan":"參加管理培訓課程","status":"FINALIZED"}');

SELECT _create_op('HR-OVERTIME','加班申請','員工加班申請','HR','Clock',43, '[
  {"section":"申請","field_name":"employee","label":"員工","type":"TEXT","required":true,"searchable":true},
  {"section":"申請","field_name":"department","label":"部門","type":"TEXT","required":true},
  {"section":"申請","field_name":"overtime_date","label":"加班日期","type":"DATE","required":true},
  {"section":"時間","field_name":"start_time","label":"開始時間","type":"TEXT","required":true},
  {"section":"時間","field_name":"end_time","label":"結束時間","type":"TEXT","required":true},
  {"section":"時間","field_name":"hours","label":"加班時數","type":"NUMBER","required":true},
  {"section":"時間","field_name":"reason","label":"加班原因","type":"TEXTAREA","required":true},
  {"section":"費用","field_name":"type","label":"加班類型","type":"DROPDOWN","options":[{"label":"平日","value":"WEEKDAY"},{"label":"假日","value":"WEEKEND"},{"label":"國定假日","value":"HOLIDAY"}]},
  {"section":"費用","field_name":"hourly_rate","label":"時薪","type":"NUMBER"},
  {"section":"費用","field_name":"overtime_pay","label":"加班費","type":"NUMBER"},
  {"section":"審核","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"待簽核","value":"PENDING"},{"label":"已核准","value":"APPROVED"},{"label":"已退回","value":"REJECTED"},{"label":"已補休","value":"COMPENSATED"}]}
]'::jsonb);
SELECT _sample('HR-OVERTIME','{"employee":"張莉","department":"生產部","overtime_date":"2026-03-22","start_time":"18:00","end_time":"21:00","hours":3,"reason":"趕製全聯訂單出貨","type":"WEEKDAY","hourly_rate":250,"overtime_pay":750,"status":"APPROVED"}');

SELECT _create_op('HR-TRAVEL','出差申請','員工出差申請','HR','Plane',44, '[
  {"section":"出差","field_name":"travel_no","label":"出差單號","type":"TEXT","required":true,"searchable":true},
  {"section":"出差","field_name":"employee","label":"員工","type":"TEXT","required":true,"searchable":true},
  {"section":"出差","field_name":"department","label":"部門","type":"TEXT","required":true},
  {"section":"出差","field_name":"destination","label":"目的地","type":"TEXT","required":true},
  {"section":"出差","field_name":"purpose","label":"出差目的","type":"TEXTAREA","required":true},
  {"section":"日期","field_name":"departure_date","label":"出發日","type":"DATE","required":true},
  {"section":"日期","field_name":"return_date","label":"返回日","type":"DATE","required":true},
  {"section":"日期","field_name":"days","label":"天數","type":"NUMBER"},
  {"section":"費用","field_name":"estimated_cost","label":"預估費用","type":"NUMBER"},
  {"section":"費用","field_name":"actual_cost","label":"實際費用","type":"NUMBER"},
  {"section":"審核","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"待簽核","value":"PENDING"},{"label":"已核准","value":"APPROVED"},{"label":"出差中","value":"ON_TRIP"},{"label":"已完成","value":"COMPLETED"},{"label":"已取消","value":"CANCELLED"}]},
  {"section":"審核","field_name":"approver","label":"核准人","type":"TEXT"}
]'::jsonb);
SELECT _sample('HR-TRAVEL','{"travel_no":"TRV-001","employee":"王軍","department":"採購部","destination":"高雄","purpose":"拜訪南部原料供應商","departure_date":"2026-04-05","return_date":"2026-04-06","days":2,"estimated_cost":8000,"actual_cost":null,"status":"APPROVED","approver":"陳偉"}');

SELECT _create_op('HR-ONBOARD','入職作業','新進人員到職作業','HR','UserCheck',45, '[
  {"section":"人員","field_name":"employee_name","label":"員工姓名","type":"TEXT","required":true,"searchable":true},
  {"section":"人員","field_name":"position","label":"職位","type":"TEXT","required":true},
  {"section":"人員","field_name":"department","label":"部門","type":"TEXT","required":true},
  {"section":"人員","field_name":"start_date","label":"到職日","type":"DATE","required":true},
  {"section":"人員","field_name":"mentor","label":"輔導人","type":"TEXT"},
  {"section":"準備","field_name":"it_setup","label":"IT設備","type":"DROPDOWN","options":[{"label":"待準備","value":"PENDING"},{"label":"已完成","value":"COMPLETED"}]},
  {"section":"準備","field_name":"badge_issued","label":"識別證","type":"DROPDOWN","options":[{"label":"是","value":"YES"},{"label":"否","value":"NO"}]},
  {"section":"準備","field_name":"training_scheduled","label":"訓練安排","type":"DROPDOWN","options":[{"label":"是","value":"YES"},{"label":"否","value":"NO"}]},
  {"section":"準備","field_name":"documents_received","label":"文件齊全","type":"DROPDOWN","options":[{"label":"是","value":"YES"},{"label":"否","value":"NO"}]},
  {"section":"準備","field_name":"onboard_status","label":"到職狀態","type":"DROPDOWN","options":[{"label":"進行中","value":"IN_PROGRESS"},{"label":"已完成","value":"COMPLETED"}]}
]'::jsonb);
SELECT _sample('HR-ONBOARD','{"employee_name":"新進品管員小林","position":"品管工程師","department":"品管部","start_date":"2026-05-01","mentor":"劉品管","it_setup":"PENDING","badge_issued":"NO","training_scheduled":"YES","documents_received":"YES","onboard_status":"IN_PROGRESS"}');

SELECT _create_op('HR-OFFBOARD','離職作業','員工離職作業處理','HR','UserMinus',46, '[
  {"section":"人員","field_name":"employee","label":"員工姓名","type":"TEXT","required":true,"searchable":true},
  {"section":"人員","field_name":"department","label":"部門","type":"TEXT","required":true},
  {"section":"人員","field_name":"position","label":"職位","type":"TEXT"},
  {"section":"離職","field_name":"last_day","label":"最後工作日","type":"DATE","required":true},
  {"section":"離職","field_name":"resignation_date","label":"申請日期","type":"DATE","required":true},
  {"section":"離職","field_name":"reason","label":"離職原因","type":"DROPDOWN","options":[{"label":"自願離職","value":"RESIGN"},{"label":"退休","value":"RETIREMENT"},{"label":"資遣","value":"TERMINATION"},{"label":"合約到期","value":"CONTRACT_END"}]},
  {"section":"作業","field_name":"exit_interview","label":"離職面談","type":"DROPDOWN","options":[{"label":"已完成","value":"YES"},{"label":"未完成","value":"NO"}]},
  {"section":"作業","field_name":"assets_returned","label":"資產歸還","type":"DROPDOWN","options":[{"label":"已歸還","value":"YES"},{"label":"未歸還","value":"NO"}]},
  {"section":"作業","field_name":"knowledge_transfer","label":"交接完成","type":"DROPDOWN","options":[{"label":"已完成","value":"YES"},{"label":"未完成","value":"NO"}]},
  {"section":"作業","field_name":"final_payment_status","label":"結算狀態","type":"TEXT"}
]'::jsonb);
SELECT _sample('HR-OFFBOARD','{"employee":"前員工老陳","department":"倉庫部","position":"倉管員","last_day":"2026-03-15","resignation_date":"2026-02-15","reason":"RESIGN","exit_interview":"YES","assets_returned":"YES","knowledge_transfer":"YES","final_payment_status":"已結算完成"}');

SELECT _create_op('HR-WELFARE','福利申請','員工福利金申請','HR','Gift',47, '[
  {"section":"申請","field_name":"application_no","label":"申請單號","type":"TEXT","required":true,"searchable":true},
  {"section":"申請","field_name":"employee","label":"員工","type":"TEXT","required":true,"searchable":true},
  {"section":"申請","field_name":"welfare_type","label":"福利類型","type":"DROPDOWN","required":true,"options":[{"label":"生日禮金","value":"BIRTHDAY"},{"label":"結婚禮金","value":"MARRIAGE"},{"label":"生育補助","value":"BIRTH"},{"label":"喪儀慰問","value":"FUNERAL"},{"label":"教育補助","value":"EDUCATION"},{"label":"健康檢查","value":"HEALTH"}]},
  {"section":"申請","field_name":"amount","label":"申請金額","type":"NUMBER","required":true},
  {"section":"申請","field_name":"application_date","label":"申請日期","type":"DATE","required":true},
  {"section":"審核","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"待審核","value":"SUBMITTED"},{"label":"已核准","value":"APPROVED"},{"label":"已發放","value":"PAID"},{"label":"已退回","value":"REJECTED"}]},
  {"section":"審核","field_name":"notes","label":"備註","type":"TEXTAREA"}
]'::jsonb);
SELECT _sample('HR-WELFARE','{"application_no":"WF-001","employee":"張莉","welfare_type":"BIRTHDAY","amount":2000,"application_date":"2026-03-15","status":"PAID","notes":"3月份壽星"}');

SELECT _create_op('HR-CERT-MGT','證照管理','員工專業證照管理','HR','Award',48, '[
  {"section":"證照","field_name":"employee","label":"員工","type":"TEXT","required":true,"searchable":true},
  {"section":"證照","field_name":"cert_name","label":"證照名稱","type":"TEXT","required":true,"searchable":true},
  {"section":"證照","field_name":"cert_type","label":"類型","type":"DROPDOWN","options":[{"label":"專業證照","value":"PROFESSIONAL"},{"label":"安全證照","value":"SAFETY"},{"label":"品質證照","value":"QUALITY"},{"label":"語文證照","value":"LANGUAGE"},{"label":"資訊證照","value":"IT"}]},
  {"section":"證照","field_name":"cert_number","label":"證書號碼","type":"TEXT"},
  {"section":"效期","field_name":"issue_date","label":"核發日期","type":"DATE","required":true},
  {"section":"效期","field_name":"expiry_date","label":"到期日","type":"DATE"},
  {"section":"效期","field_name":"issuing_body","label":"核發機構","type":"TEXT"},
  {"section":"效期","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"有效","value":"VALID"},{"label":"即將到期","value":"EXPIRING"},{"label":"已過期","value":"EXPIRED"}]},
  {"section":"效期","field_name":"renewal_required","label":"需續證","type":"DROPDOWN","options":[{"label":"是","value":"YES"},{"label":"否","value":"NO"}]}
]'::jsonb);
SELECT _sample('HR-CERT-MGT','{"employee":"劉品管","cert_name":"食品技師證照","cert_type":"PROFESSIONAL","cert_number":"FT-2024-5678","issue_date":"2024-06-15","expiry_date":"2027-06-14","issuing_body":"考試院","status":"VALID","renewal_required":"YES"}');

-- ═══════════════════════════════════════════════════════════════════
-- WM MODULE (8 operations, additional to WM-MOVE)
-- ═══════════════════════════════════════════════════════════════════

SELECT _create_op('WM-BIN','儲位管理','倉庫儲位設定','WM','Grid3x3',30, '[
  {"section":"儲位","field_name":"bin_code","label":"儲位代碼","type":"TEXT","required":true,"searchable":true},
  {"section":"儲位","field_name":"warehouse","label":"倉庫","type":"TEXT","required":true},
  {"section":"儲位","field_name":"zone","label":"區域","type":"DROPDOWN","options":[{"label":"A區","value":"A"},{"label":"B區","value":"B"},{"label":"C區","value":"C"},{"label":"D區","value":"D"}]},
  {"section":"儲位","field_name":"aisle","label":"走道","type":"TEXT"},
  {"section":"儲位","field_name":"rack","label":"貨架","type":"TEXT"},
  {"section":"儲位","field_name":"level","label":"層","type":"TEXT"},
  {"section":"容量","field_name":"capacity","label":"容量","type":"NUMBER"},
  {"section":"容量","field_name":"current_usage","label":"已用量","type":"NUMBER"},
  {"section":"容量","field_name":"usage_pct","label":"使用率(%)","type":"NUMBER"},
  {"section":"狀態","field_name":"material_type","label":"物料類型","type":"DROPDOWN","options":[{"label":"原料","value":"RAW"},{"label":"半成品","value":"SEMI"},{"label":"成品","value":"FINISHED"},{"label":"包材","value":"PACKAGING"}]},
  {"section":"狀態","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"可用","value":"AVAILABLE"},{"label":"已滿","value":"FULL"},{"label":"預留","value":"RESERVED"},{"label":"封鎖","value":"BLOCKED"}]}
]'::jsonb);
SELECT _sample('WM-BIN','{"bin_code":"WH001-A-01-01","warehouse":"台北主倉庫","zone":"A","aisle":"01","rack":"01","level":"1","capacity":1000,"current_usage":750,"usage_pct":75,"material_type":"FINISHED","status":"AVAILABLE"}');

SELECT _create_op('WM-PICK','揀貨單','出貨揀貨作業','WM','ClipboardList',31, '[
  {"section":"揀貨","field_name":"pick_no","label":"揀貨單號","type":"TEXT","required":true,"searchable":true},
  {"section":"揀貨","field_name":"delivery_reference","label":"出貨單號","type":"TEXT","searchable":true},
  {"section":"揀貨","field_name":"material","label":"物料","type":"TEXT","required":true},
  {"section":"揀貨","field_name":"quantity","label":"數量","type":"NUMBER","required":true},
  {"section":"揀貨","field_name":"unit","label":"單位","type":"TEXT"},
  {"section":"揀貨","field_name":"from_bin","label":"儲位","type":"TEXT","required":true},
  {"section":"揀貨","field_name":"batch_number","label":"批號","type":"TEXT"},
  {"section":"執行","field_name":"picker","label":"揀貨員","type":"TEXT","required":true},
  {"section":"執行","field_name":"start_time","label":"開始時間","type":"TEXT"},
  {"section":"執行","field_name":"end_time","label":"完成時間","type":"TEXT"},
  {"section":"執行","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"待揀","value":"PENDING"},{"label":"揀貨中","value":"IN_PROGRESS"},{"label":"已完成","value":"COMPLETED"},{"label":"缺貨","value":"SHORT"}]}
]'::jsonb);
SELECT _sample('WM-PICK','{"pick_no":"PICK-001","delivery_reference":"DLV-001","material":"脆薯片(原味) 150g","quantity":10000,"unit":"PCS","from_bin":"WH001-B-03-01","batch_number":"BAT-P2026032201","picker":"倉管小陳","start_time":"07:00","end_time":"07:45","status":"COMPLETED"}');

SELECT _create_op('WM-PACK','包裝作業','出貨包裝作業','WM','Package',32, '[
  {"section":"包裝","field_name":"pack_no","label":"包裝單號","type":"TEXT","required":true,"searchable":true},
  {"section":"包裝","field_name":"delivery_reference","label":"出貨單號","type":"TEXT","searchable":true},
  {"section":"包裝","field_name":"material","label":"商品","type":"TEXT","required":true},
  {"section":"包裝","field_name":"quantity","label":"數量","type":"NUMBER","required":true},
  {"section":"包裝","field_name":"package_type","label":"包裝方式","type":"DROPDOWN","options":[{"label":"紙箱","value":"CARTON"},{"label":"棧板","value":"PALLET"},{"label":"散裝","value":"BULK"},{"label":"收縮膜","value":"SHRINK_WRAP"}]},
  {"section":"包裝","field_name":"packages_count","label":"箱數","type":"NUMBER"},
  {"section":"重量","field_name":"gross_weight","label":"毛重(kg)","type":"NUMBER"},
  {"section":"重量","field_name":"net_weight","label":"淨重(kg)","type":"NUMBER"},
  {"section":"執行","field_name":"packer","label":"包裝員","type":"TEXT"},
  {"section":"執行","field_name":"pack_date","label":"包裝日期","type":"DATE"},
  {"section":"執行","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"待包裝","value":"PENDING"},{"label":"已包裝","value":"PACKED"},{"label":"已貼標","value":"LABELED"},{"label":"已出貨","value":"SHIPPED"}]}
]'::jsonb);
SELECT _sample('WM-PACK','{"pack_no":"PACK-001","delivery_reference":"DLV-001","material":"脆薯片(原味) 150g","quantity":10000,"package_type":"CARTON","packages_count":200,"gross_weight":1600,"net_weight":1500,"packer":"倉管小陳","pack_date":"2026-03-25","status":"SHIPPED"}');

SELECT _create_op('WM-CYCLE-CNT','循環盤點','定期循環盤點','WM','RefreshCw',33, '[
  {"section":"盤點","field_name":"count_no","label":"盤點編號","type":"TEXT","required":true,"searchable":true},
  {"section":"盤點","field_name":"bin_code","label":"儲位","type":"TEXT","required":true},
  {"section":"盤點","field_name":"material","label":"物料","type":"TEXT","required":true,"searchable":true},
  {"section":"數量","field_name":"system_qty","label":"帳面數量","type":"NUMBER"},
  {"section":"數量","field_name":"counted_qty","label":"盤點數量","type":"NUMBER","required":true},
  {"section":"數量","field_name":"variance","label":"差異","type":"NUMBER"},
  {"section":"數量","field_name":"variance_pct","label":"差異率(%)","type":"NUMBER"},
  {"section":"處理","field_name":"counter","label":"盤點人","type":"TEXT","required":true},
  {"section":"處理","field_name":"count_date","label":"盤點日期","type":"DATE","required":true},
  {"section":"處理","field_name":"adjustment_status","label":"調整狀態","type":"DROPDOWN","options":[{"label":"待調整","value":"PENDING"},{"label":"已調整","value":"ADJUSTED"},{"label":"調查中","value":"INVESTIGATED"}]}
]'::jsonb);
SELECT _sample('WM-CYCLE-CNT','{"count_no":"CC-001","bin_code":"WH001-A-01-01","material":"包裝袋(薯片用)","system_qty":45000,"counted_qty":44800,"variance":-200,"variance_pct":-0.44,"counter":"倉管小陳","count_date":"2026-03-28","adjustment_status":"ADJUSTED"}');

SELECT _create_op('WM-PUTAWAY','上架作業','收貨上架作業','WM','ArrowUp',34, '[
  {"section":"上架","field_name":"putaway_no","label":"上架單號","type":"TEXT","required":true,"searchable":true},
  {"section":"上架","field_name":"grn_reference","label":"收貨單號","type":"TEXT","searchable":true},
  {"section":"上架","field_name":"material","label":"物料","type":"TEXT","required":true},
  {"section":"上架","field_name":"quantity","label":"數量","type":"NUMBER","required":true},
  {"section":"上架","field_name":"unit","label":"單位","type":"TEXT"},
  {"section":"儲位","field_name":"suggested_bin","label":"建議儲位","type":"TEXT"},
  {"section":"儲位","field_name":"actual_bin","label":"實際儲位","type":"TEXT","required":true},
  {"section":"儲位","field_name":"batch_number","label":"批號","type":"TEXT"},
  {"section":"執行","field_name":"operator","label":"作業員","type":"TEXT","required":true},
  {"section":"執行","field_name":"putaway_date","label":"上架日期","type":"DATE","required":true},
  {"section":"執行","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"待上架","value":"PENDING"},{"label":"已完成","value":"COMPLETED"},{"label":"部分完成","value":"PARTIAL"}]}
]'::jsonb);
SELECT _sample('WM-PUTAWAY','{"putaway_no":"PUT-001","grn_reference":"GRN-001","material":"馬鈴薯粉","quantity":1980,"unit":"KG","suggested_bin":"WH001-A-01-01","actual_bin":"WH001-A-01-01","batch_number":"BAT-2026031001","operator":"倉管小陳","putaway_date":"2026-03-20","status":"COMPLETED"}');

SELECT _create_op('WM-TRANSFER','倉庫調撥','倉庫間物料調撥','WM','MoveHorizontal',35, '[
  {"section":"調撥","field_name":"transfer_no","label":"調撥單號","type":"TEXT","required":true,"searchable":true},
  {"section":"調撥","field_name":"material","label":"物料","type":"TEXT","required":true,"searchable":true},
  {"section":"調撥","field_name":"quantity","label":"數量","type":"NUMBER","required":true},
  {"section":"調撥","field_name":"unit","label":"單位","type":"TEXT"},
  {"section":"來源","field_name":"from_warehouse","label":"來源倉庫","type":"TEXT","required":true},
  {"section":"來源","field_name":"from_bin","label":"來源儲位","type":"TEXT"},
  {"section":"目的","field_name":"to_warehouse","label":"目的倉庫","type":"TEXT","required":true},
  {"section":"目的","field_name":"to_bin","label":"目的儲位","type":"TEXT"},
  {"section":"處理","field_name":"transfer_date","label":"調撥日期","type":"DATE","required":true},
  {"section":"處理","field_name":"batch_number","label":"批號","type":"TEXT"},
  {"section":"處理","field_name":"reason","label":"調撥原因","type":"TEXTAREA"},
  {"section":"處理","field_name":"requester","label":"申請人","type":"TEXT"},
  {"section":"處理","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"申請中","value":"REQUESTED"},{"label":"已核准","value":"APPROVED"},{"label":"運送中","value":"IN_TRANSIT"},{"label":"已收貨","value":"RECEIVED"}]}
]'::jsonb);
SELECT _sample('WM-TRANSFER','{"transfer_no":"TF-001","material":"脆薯片(原味) 150g","quantity":5000,"unit":"PCS","from_warehouse":"台北主倉庫","from_bin":"WH001-B-03","to_warehouse":"台中配送中心","to_bin":"WH003-A-01","transfer_date":"2026-03-26","batch_number":"BAT-P2026032201","reason":"台中配送中心庫存不足，補貨","requester":"倉管小陳","status":"IN_TRANSIT"}');

-- ═══════════════════════════════════════════════════════════════════
-- GEN MODULE (12 general/cross-module operations)
-- ═══════════════════════════════════════════════════════════════════

SELECT _create_op('GEN-ANNOUNCE','公告管理','公司公告發佈管理',null,'Megaphone',10, '[
  {"section":"公告","field_name":"announce_no","label":"公告編號","type":"TEXT","required":true,"searchable":true},
  {"section":"公告","field_name":"title","label":"標題","type":"TEXT","required":true,"searchable":true},
  {"section":"公告","field_name":"category","label":"類別","type":"DROPDOWN","required":true,"options":[{"label":"公司","value":"COMPANY"},{"label":"人事","value":"HR"},{"label":"系統","value":"SYSTEM"},{"label":"安全","value":"SAFETY"},{"label":"活動","value":"EVENT"}]},
  {"section":"公告","field_name":"content","label":"內容","type":"TEXTAREA","required":true},
  {"section":"公告","field_name":"target_audience","label":"對象","type":"DROPDOWN","options":[{"label":"全公司","value":"ALL"},{"label":"部門","value":"DEPARTMENT"},{"label":"管理層","value":"MANAGEMENT"}]},
  {"section":"時間","field_name":"effective_date","label":"生效日","type":"DATE","required":true},
  {"section":"時間","field_name":"expiry_date","label":"到期日","type":"DATE"},
  {"section":"時間","field_name":"author","label":"發佈人","type":"TEXT"},
  {"section":"時間","field_name":"priority","label":"重要性","type":"DROPDOWN","options":[{"label":"一般","value":"NORMAL"},{"label":"重要","value":"IMPORTANT"},{"label":"緊急","value":"URGENT"}]},
  {"section":"時間","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"草稿","value":"DRAFT"},{"label":"已發佈","value":"PUBLISHED"},{"label":"已歸檔","value":"ARCHIVED"}]}
]'::jsonb);
SELECT _sample('GEN-ANNOUNCE','{"announce_no":"ANN-001","title":"2026年度清明節連假通知","category":"HR","content":"依據人事行政局公告，清明連假為4/3-4/6共4天，4/7(一)正常上班。請各部門提前安排工作交接。","target_audience":"ALL","effective_date":"2026-03-25","expiry_date":"2026-04-07","author":"趙人資","priority":"IMPORTANT","status":"PUBLISHED"}');

SELECT _create_op('GEN-MEETING','會議記錄','會議紀錄管理',null,'Users',11, '[
  {"section":"會議","field_name":"meeting_no","label":"會議編號","type":"TEXT","required":true,"searchable":true},
  {"section":"會議","field_name":"title","label":"會議主題","type":"TEXT","required":true,"searchable":true},
  {"section":"會議","field_name":"meeting_date","label":"會議日期","type":"DATE","required":true},
  {"section":"會議","field_name":"start_time","label":"開始時間","type":"TEXT"},
  {"section":"會議","field_name":"end_time","label":"結束時間","type":"TEXT"},
  {"section":"會議","field_name":"location","label":"地點","type":"TEXT"},
  {"section":"會議","field_name":"organizer","label":"主持人","type":"TEXT","required":true},
  {"section":"內容","field_name":"attendees","label":"出席人員","type":"TEXTAREA"},
  {"section":"內容","field_name":"agenda","label":"議程","type":"TEXTAREA"},
  {"section":"內容","field_name":"minutes","label":"會議紀錄","type":"TEXTAREA"},
  {"section":"內容","field_name":"action_items","label":"決議事項","type":"TEXTAREA"},
  {"section":"追蹤","field_name":"next_meeting_date","label":"下次會議","type":"DATE"},
  {"section":"追蹤","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"已排程","value":"SCHEDULED"},{"label":"進行中","value":"IN_PROGRESS"},{"label":"已完成","value":"COMPLETED"},{"label":"已取消","value":"CANCELLED"}]}
]'::jsonb);
SELECT _sample('GEN-MEETING','{"meeting_no":"MTG-001","title":"2026年Q1營運檢討會議","meeting_date":"2026-03-28","start_time":"14:00","end_time":"16:00","location":"3F會議室","organizer":"陳偉","attendees":"陳偉、林美、王軍、張莉、劉品管、趙人資","agenda":"1.Q1業績檢討 2.生產效率報告 3.品質改善進度 4.Q2計畫","minutes":"Q1營收達成率85%，薯片系列表現優於預期，飲料系列需加強","action_items":"1.林美:提出Q2促銷方案(4/5前) 2.張莉:產能評估報告(4/3前) 3.劉品管:BBQ調味料改善報告(4/10前)","next_meeting_date":"2026-04-28","status":"COMPLETED"}');

SELECT _create_op('GEN-TODO','待辦事項','個人待辦事項管理',null,'CheckSquare',12, '[
  {"section":"任務","field_name":"task_title","label":"任務標題","type":"TEXT","required":true,"searchable":true},
  {"section":"任務","field_name":"description","label":"說明","type":"TEXTAREA"},
  {"section":"任務","field_name":"assigned_to","label":"負責人","type":"TEXT","required":true,"searchable":true},
  {"section":"任務","field_name":"assigned_by","label":"指派人","type":"TEXT"},
  {"section":"任務","field_name":"priority","label":"優先順序","type":"DROPDOWN","options":[{"label":"低","value":"LOW"},{"label":"一般","value":"NORMAL","is_default":"true"},{"label":"高","value":"HIGH"},{"label":"緊急","value":"URGENT"}]},
  {"section":"期限","field_name":"due_date","label":"到期日","type":"DATE","required":true},
  {"section":"期限","field_name":"category","label":"分類","type":"DROPDOWN","options":[{"label":"工作","value":"WORK"},{"label":"追蹤","value":"FOLLOW_UP"},{"label":"審核","value":"REVIEW"},{"label":"報告","value":"REPORT"}]},
  {"section":"期限","field_name":"status","label":"狀態","type":"DROPDOWN","required":true,"options":[{"label":"開放","value":"OPEN","is_default":"true"},{"label":"進行中","value":"IN_PROGRESS"},{"label":"已完成","value":"COMPLETED"},{"label":"逾期","value":"OVERDUE"}]},
  {"section":"期限","field_name":"completion_date","label":"完成日","type":"DATE"}
]'::jsonb);
SELECT _sample('GEN-TODO','{"task_title":"提出Q2促銷方案","description":"針對零食系列規劃Q2促銷活動方案","assigned_to":"林美","assigned_by":"陳偉","priority":"HIGH","due_date":"2026-04-05","category":"WORK","status":"IN_PROGRESS","completion_date":null}');
SELECT _sample('GEN-TODO','{"task_title":"BBQ調味料改善報告","description":"與供應商協調BBQ調味料品質改善措施","assigned_to":"劉品管","assigned_by":"陳偉","priority":"HIGH","due_date":"2026-04-10","category":"REPORT","status":"OPEN","completion_date":null}');

SELECT _create_op('GEN-INCIDENT','事件報告','異常事件報告管理',null,'AlertOctagon',13, '[
  {"section":"事件","field_name":"incident_no","label":"事件編號","type":"TEXT","required":true,"searchable":true},
  {"section":"事件","field_name":"incident_date","label":"發生日期","type":"DATE","required":true},
  {"section":"事件","field_name":"incident_time","label":"發生時間","type":"TEXT"},
  {"section":"事件","field_name":"location","label":"地點","type":"TEXT","required":true},
  {"section":"分類","field_name":"category","label":"事件類別","type":"DROPDOWN","required":true,"options":[{"label":"安全","value":"SAFETY"},{"label":"品質","value":"QUALITY"},{"label":"環保","value":"ENVIRONMENT"},{"label":"資安","value":"SECURITY"},{"label":"IT","value":"IT"}]},
  {"section":"分類","field_name":"severity","label":"嚴重度","type":"DROPDOWN","required":true,"options":[{"label":"低","value":"LOW"},{"label":"中","value":"MEDIUM"},{"label":"高","value":"HIGH"},{"label":"緊急","value":"CRITICAL"}]},
  {"section":"處理","field_name":"description","label":"事件描述","type":"TEXTAREA","required":true},
  {"section":"處理","field_name":"immediate_action","label":"立即處置","type":"TEXTAREA"},
  {"section":"處理","field_name":"root_cause","label":"根因分析","type":"TEXTAREA"},
  {"section":"處理","field_name":"corrective_action","label":"矯正措施","type":"TEXTAREA"},
  {"section":"處理","field_name":"reported_by","label":"通報人","type":"TEXT","required":true},
  {"section":"處理","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"已通報","value":"REPORTED"},{"label":"調查中","value":"INVESTIGATING"},{"label":"已解決","value":"RESOLVED"},{"label":"已關閉","value":"CLOSED"}]}
]'::jsonb);
SELECT _sample('GEN-INCIDENT','{"incident_no":"INC-001","incident_date":"2026-03-21","incident_time":"10:15","location":"油炸區","category":"SAFETY","severity":"MEDIUM","description":"油炸機溫控器故障導致油溫異常升高，作業員及時發現並緊急停機","immediate_action":"立即停機斷電，通知維修組","root_cause":"溫控器老化，感測元件失靈","corrective_action":"更換全新溫控器，建立預防性更換週期","reported_by":"張莉","status":"RESOLVED"}');

SELECT _create_op('GEN-KPI','KPI追蹤','關鍵績效指標追蹤',null,'BarChart3',14, '[
  {"section":"指標","field_name":"kpi_name","label":"KPI名稱","type":"TEXT","required":true,"searchable":true},
  {"section":"指標","field_name":"department","label":"部門","type":"TEXT","required":true,"searchable":true},
  {"section":"指標","field_name":"period","label":"期間","type":"TEXT","required":true},
  {"section":"數值","field_name":"target_value","label":"目標值","type":"NUMBER","required":true},
  {"section":"數值","field_name":"actual_value","label":"實際值","type":"NUMBER"},
  {"section":"數值","field_name":"achievement_pct","label":"達成率(%)","type":"NUMBER"},
  {"section":"數值","field_name":"unit","label":"單位","type":"TEXT"},
  {"section":"分析","field_name":"trend","label":"趨勢","type":"DROPDOWN","options":[{"label":"上升","value":"UP"},{"label":"下降","value":"DOWN"},{"label":"持平","value":"STABLE"}]},
  {"section":"分析","field_name":"responsible","label":"負責人","type":"TEXT"},
  {"section":"分析","field_name":"status","label":"達成狀態","type":"DROPDOWN","options":[{"label":"達標","value":"ON_TRACK"},{"label":"有風險","value":"AT_RISK"},{"label":"未達標","value":"OFF_TRACK"}]}
]'::jsonb);
SELECT _sample('GEN-KPI','{"kpi_name":"月營收","department":"業務部","period":"2026-03","target_value":5000000,"actual_value":4225000,"achievement_pct":84.5,"unit":"TWD","trend":"UP","responsible":"林美","status":"AT_RISK"}');
SELECT _sample('GEN-KPI','{"kpi_name":"生產良率","department":"生產部","period":"2026-03","target_value":98,"actual_value":98.2,"achievement_pct":100.2,"unit":"%","trend":"STABLE","responsible":"張莉","status":"ON_TRACK"}');
SELECT _sample('GEN-KPI','{"kpi_name":"客訴件數","department":"品管部","period":"2026-03","target_value":3,"actual_value":1,"achievement_pct":300,"unit":"件","trend":"DOWN","responsible":"劉品管","status":"ON_TRACK"}');

SELECT _create_op('GEN-PROJECT','專案管理','跨部門專案管理',null,'FolderKanban',15, '[
  {"section":"專案","field_name":"project_no","label":"專案編號","type":"TEXT","required":true,"searchable":true},
  {"section":"專案","field_name":"project_name","label":"專案名稱","type":"TEXT","required":true,"searchable":true},
  {"section":"專案","field_name":"sponsor","label":"專案發起人","type":"TEXT"},
  {"section":"專案","field_name":"manager","label":"專案經理","type":"TEXT","required":true},
  {"section":"時程","field_name":"start_date","label":"開始日期","type":"DATE","required":true},
  {"section":"時程","field_name":"end_date","label":"結束日期","type":"DATE"},
  {"section":"預算","field_name":"budget","label":"預算","type":"NUMBER"},
  {"section":"預算","field_name":"actual_cost","label":"實際成本","type":"NUMBER"},
  {"section":"進度","field_name":"completion_pct","label":"完成率(%)","type":"NUMBER"},
  {"section":"進度","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"規劃中","value":"PLANNING"},{"label":"進行中","value":"ACTIVE"},{"label":"暫停","value":"ON_HOLD"},{"label":"已完成","value":"COMPLETED"},{"label":"已取消","value":"CANCELLED"}]},
  {"section":"進度","field_name":"risk_level","label":"風險等級","type":"DROPDOWN","options":[{"label":"低","value":"LOW"},{"label":"中","value":"MEDIUM"},{"label":"高","value":"HIGH"}]}
]'::jsonb);
SELECT _sample('GEN-PROJECT','{"project_no":"PROJ-001","project_name":"新飲料產品線開發","sponsor":"陳偉","manager":"張莉","start_date":"2026-02-01","end_date":"2026-08-31","budget":2000000,"actual_cost":450000,"completion_pct":25,"status":"ACTIVE","risk_level":"MEDIUM"}');

SELECT _create_op('GEN-RISK','風險評估','企業風險識別評估',null,'ShieldAlert',16, '[
  {"section":"風險","field_name":"risk_no","label":"風險編號","type":"TEXT","required":true,"searchable":true},
  {"section":"風險","field_name":"risk_category","label":"風險類別","type":"DROPDOWN","required":true,"options":[{"label":"營運","value":"OPERATIONAL"},{"label":"財務","value":"FINANCIAL"},{"label":"法遵","value":"COMPLIANCE"},{"label":"策略","value":"STRATEGIC"},{"label":"技術","value":"TECHNICAL"}]},
  {"section":"風險","field_name":"description","label":"風險描述","type":"TEXTAREA","required":true},
  {"section":"評估","field_name":"likelihood","label":"可能性(1-5)","type":"NUMBER","required":true,"min":1,"max":5},
  {"section":"評估","field_name":"impact","label":"影響度(1-5)","type":"NUMBER","required":true,"min":1,"max":5},
  {"section":"評估","field_name":"risk_score","label":"風險分數","type":"NUMBER"},
  {"section":"應對","field_name":"mitigation_plan","label":"緩解措施","type":"TEXTAREA"},
  {"section":"應對","field_name":"owner","label":"負責人","type":"TEXT","required":true},
  {"section":"應對","field_name":"review_date","label":"檢討日期","type":"DATE"},
  {"section":"應對","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"已識別","value":"IDENTIFIED"},{"label":"緩解中","value":"MITIGATING"},{"label":"已接受","value":"ACCEPTED"},{"label":"已關閉","value":"CLOSED"}]}
]'::jsonb);
SELECT _sample('GEN-RISK','{"risk_no":"RISK-001","risk_category":"OPERATIONAL","description":"關鍵原料(馬鈴薯粉)供應商集中度過高，80%來自單一供應商","likelihood":3,"impact":4,"risk_score":12,"mitigation_plan":"1.開發第二供應來源 2.提高安全庫存 3.簽訂長期合約鎖定供應","owner":"王軍","review_date":"2026-06-30","status":"MITIGATING"}');

SELECT _create_op('GEN-SURVEY','問卷調查','員工/客戶滿意度調查',null,'ClipboardEdit',17, '[
  {"section":"調查","field_name":"survey_no","label":"調查編號","type":"TEXT","required":true,"searchable":true},
  {"section":"調查","field_name":"title","label":"調查名稱","type":"TEXT","required":true,"searchable":true},
  {"section":"調查","field_name":"target_group","label":"調查對象","type":"TEXT","required":true},
  {"section":"調查","field_name":"start_date","label":"開始日期","type":"DATE","required":true},
  {"section":"調查","field_name":"end_date","label":"截止日期","type":"DATE","required":true},
  {"section":"結果","field_name":"total_responses","label":"回收數","type":"NUMBER"},
  {"section":"結果","field_name":"satisfaction_score","label":"滿意度(1-5)","type":"NUMBER"},
  {"section":"結果","field_name":"nps_score","label":"NPS分數","type":"NUMBER"},
  {"section":"結果","field_name":"key_findings","label":"主要發現","type":"TEXTAREA"},
  {"section":"結果","field_name":"action_plan","label":"改善方案","type":"TEXTAREA"},
  {"section":"狀態","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"草稿","value":"DRAFT"},{"label":"進行中","value":"ACTIVE"},{"label":"已截止","value":"CLOSED"},{"label":"已分析","value":"ANALYZED"}]}
]'::jsonb);
SELECT _sample('GEN-SURVEY','{"survey_no":"SUR-001","title":"2026 Q1 員工滿意度調查","target_group":"全體員工","start_date":"2026-03-15","end_date":"2026-03-25","total_responses":128,"satisfaction_score":4.1,"nps_score":35,"key_findings":"整體滿意度良好，福利滿意度最高(4.5)，工作壓力為主要改善項目(3.6)","action_plan":"1.檢討工作量分配 2.增加員工紓壓活動 3.改善加班制度","status":"ANALYZED"}');

SELECT _create_op('GEN-CONTRACT','合約管理','各類合約集中管理',null,'FileSignature',18, '[
  {"section":"合約","field_name":"contract_no","label":"合約編號","type":"TEXT","required":true,"searchable":true},
  {"section":"合約","field_name":"contract_type","label":"合約類型","type":"DROPDOWN","required":true,"options":[{"label":"供應商合約","value":"VENDOR"},{"label":"客戶合約","value":"CUSTOMER"},{"label":"租賃合約","value":"LEASE"},{"label":"服務合約","value":"SERVICE"},{"label":"保密合約","value":"NDA"}]},
  {"section":"合約","field_name":"counterparty","label":"對象","type":"TEXT","required":true,"searchable":true},
  {"section":"效期","field_name":"start_date","label":"起始日","type":"DATE","required":true},
  {"section":"效期","field_name":"end_date","label":"到期日","type":"DATE","required":true},
  {"section":"效期","field_name":"value","label":"合約金額","type":"NUMBER"},
  {"section":"條款","field_name":"payment_terms","label":"付款條件","type":"TEXT"},
  {"section":"條款","field_name":"key_terms","label":"重要條款","type":"TEXTAREA"},
  {"section":"條款","field_name":"renewal_date","label":"續約日","type":"DATE"},
  {"section":"管理","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"草稿","value":"DRAFT"},{"label":"有效","value":"ACTIVE"},{"label":"到期","value":"EXPIRED"},{"label":"已終止","value":"TERMINATED"}]},
  {"section":"管理","field_name":"owner","label":"管理人","type":"TEXT"}
]'::jsonb);
SELECT _sample('GEN-CONTRACT','{"contract_no":"GC-001","contract_type":"LEASE","counterparty":"桃園工業區管委會","start_date":"2024-01-01","end_date":"2028-12-31","value":3600000,"payment_terms":"每月支付","key_terms":"租期5年，每年調漲3%，提前終止需付6個月違約金","renewal_date":"2028-06-30","status":"ACTIVE","owner":"陳偉"}');

SELECT _create_op('GEN-DOC-MGT','文件管理','公司文件版本管理',null,'FileArchive',19, '[
  {"section":"文件","field_name":"doc_no","label":"文件編號","type":"TEXT","required":true,"searchable":true},
  {"section":"文件","field_name":"doc_title","label":"文件名稱","type":"TEXT","required":true,"searchable":true},
  {"section":"文件","field_name":"doc_type","label":"文件類型","type":"DROPDOWN","required":true,"options":[{"label":"政策","value":"POLICY"},{"label":"程序","value":"PROCEDURE"},{"label":"指引","value":"GUIDELINE"},{"label":"表單","value":"FORM"},{"label":"記錄","value":"RECORD"}]},
  {"section":"文件","field_name":"version","label":"版本","type":"TEXT","required":true},
  {"section":"文件","field_name":"department","label":"部門","type":"TEXT"},
  {"section":"管理","field_name":"owner","label":"文件管理人","type":"TEXT","required":true},
  {"section":"管理","field_name":"review_date","label":"審查日期","type":"DATE"},
  {"section":"管理","field_name":"next_review","label":"下次審查日","type":"DATE"},
  {"section":"管理","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"草稿","value":"DRAFT"},{"label":"審查中","value":"REVIEW"},{"label":"已核定","value":"APPROVED"},{"label":"已廢止","value":"OBSOLETE"}]},
  {"section":"管理","field_name":"change_description","label":"變更說明","type":"TEXTAREA"}
]'::jsonb);
SELECT _sample('GEN-DOC-MGT','{"doc_no":"QP-001","doc_title":"品質管理手冊","doc_type":"POLICY","version":"3.0","department":"品管部","owner":"劉品管","review_date":"2026-01-15","next_review":"2027-01-15","status":"APPROVED","change_description":"更新第5章食品安全管理程序，配合ISO 22000:2018要求"}');

-- Cleanup helper functions
DROP FUNCTION IF EXISTS _create_op(TEXT,TEXT,TEXT,TEXT,TEXT,INT,JSONB);
DROP FUNCTION IF EXISTS _sample(TEXT,JSONB);

-- (moved to end)

-- Patch: Add 8 more operations to reach 100+ total
-- These are appended BEFORE the COMMIT (need to re-add helpers)

CREATE OR REPLACE FUNCTION _create_op(p_code TEXT,p_name TEXT,p_desc TEXT,p_module TEXT,p_icon TEXT,p_sort INT,p_fields JSONB) RETURNS UUID AS $$
DECLARE v_op_id UUID:=gen_random_uuid(); v_form_id UUID:=gen_random_uuid(); v_proj_id UUID:='d0000000-0000-0000-0000-000000000001'::UUID; v_sec_id UUID; v_fld_id UUID; v_sec TEXT; v_last_sec TEXT:=''; v_sec_idx INT:=0; v_fld_idx INT:=0; v_field JSONB; v_opt JSONB; v_opt_idx INT;
BEGIN
INSERT INTO lc_operations (id,operation_code,project_id,name,description,operation_type,is_published,version,module,sidebar_icon,sidebar_sort_order,is_yaml_managed) VALUES (v_op_id,p_code,v_proj_id,p_name,p_desc,'FORM',true,1,p_module,p_icon,p_sort,false) ON CONFLICT (operation_code) DO UPDATE SET name=p_name RETURNING id INTO v_op_id;
INSERT INTO lc_form_definitions (id,operation_id) VALUES (v_form_id,v_op_id) ON CONFLICT (operation_id) DO NOTHING;
SELECT id INTO v_form_id FROM lc_form_definitions WHERE operation_id=v_op_id;
FOR v_field IN SELECT * FROM jsonb_array_elements(p_fields) LOOP
v_sec:=v_field->>'section'; IF v_sec!=v_last_sec THEN v_sec_id:=gen_random_uuid(); v_fld_idx:=0;
INSERT INTO lc_form_sections (id,form_id,title,columns,sort_order) VALUES (v_sec_id,v_form_id,v_sec,2,v_sec_idx) ON CONFLICT DO NOTHING;
v_last_sec:=v_sec; v_sec_idx:=v_sec_idx+1; END IF;
v_fld_id:=gen_random_uuid();
INSERT INTO lc_field_definitions (id,section_id,field_name,field_label,field_type,is_required,is_searchable,sort_order,column_span,min_value,max_value,visibility_rule,field_config) VALUES (v_fld_id,v_sec_id,v_field->>'field_name',v_field->>'label',COALESCE(v_field->>'type','TEXT'),COALESCE((v_field->>'required')::boolean,false),COALESCE((v_field->>'searchable')::boolean,false),v_fld_idx,COALESCE((v_field->>'span')::int,1),(v_field->>'min')::numeric,(v_field->>'max')::numeric,v_field->'visibility_rule',COALESCE(v_field->'config','{}')) ON CONFLICT (section_id,field_name) DO NOTHING;
IF v_field->'options' IS NOT NULL THEN v_opt_idx:=0;
FOR v_opt IN SELECT * FROM jsonb_array_elements(v_field->'options') LOOP
INSERT INTO lc_field_options (field_id,option_label,option_value,sort_order,is_default) VALUES (v_fld_id,v_opt->>'label',v_opt->>'value',v_opt_idx,COALESCE((v_opt->>'is_default')::boolean,false)) ON CONFLICT DO NOTHING;
v_opt_idx:=v_opt_idx+1; END LOOP; END IF; v_fld_idx:=v_fld_idx+1; END LOOP; RETURN v_op_id;
END; $$ LANGUAGE plpgsql;
CREATE OR REPLACE FUNCTION _sample(p_code TEXT,p_data JSONB) RETURNS VOID AS $$ BEGIN INSERT INTO lc_operation_data (operation_id,data,created_by) SELECT o.id,p_data,'a0000000-0000-0000-0000-000000000004'::UUID FROM lc_operations o WHERE o.operation_code=p_code; END; $$ LANGUAGE plpgsql;

SELECT _create_op('GEN-CHANGE-REQ','變更申請','流程/產品/系統變更申請',null,'GitPullRequest',20,'[
  {"section":"變更","field_name":"change_no","label":"變更編號","type":"TEXT","required":true,"searchable":true},
  {"section":"變更","field_name":"change_type","label":"變更類型","type":"DROPDOWN","required":true,"options":[{"label":"流程","value":"PROCESS"},{"label":"產品","value":"PRODUCT"},{"label":"系統","value":"SYSTEM"},{"label":"組織","value":"ORGANIZATION"}]},
  {"section":"變更","field_name":"title","label":"變更標題","type":"TEXT","required":true,"searchable":true},
  {"section":"變更","field_name":"description","label":"變更說明","type":"TEXTAREA","required":true},
  {"section":"評估","field_name":"impact_analysis","label":"影響分析","type":"TEXTAREA"},
  {"section":"評估","field_name":"risk_level","label":"風險等級","type":"DROPDOWN","options":[{"label":"低","value":"LOW"},{"label":"中","value":"MEDIUM"},{"label":"高","value":"HIGH"}]},
  {"section":"審核","field_name":"requested_by","label":"申請人","type":"TEXT","required":true},
  {"section":"審核","field_name":"effective_date","label":"生效日期","type":"DATE"},
  {"section":"審核","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"已提交","value":"SUBMITTED"},{"label":"審查中","value":"REVIEWING"},{"label":"已核准","value":"APPROVED"},{"label":"已實施","value":"IMPLEMENTED"},{"label":"已退回","value":"REJECTED"}]}
]'::jsonb);
SELECT _sample('GEN-CHANGE-REQ','{"change_no":"CR-001","change_type":"PROCESS","title":"薯片包裝流程自動化升級","description":"將現有半自動包裝線升級為全自動，預計提升30%效率","impact_analysis":"需停機3天進行改裝，影響當週產能","risk_level":"MEDIUM","requested_by":"張莉","effective_date":"2026-05-01","status":"REVIEWING"}');

SELECT _create_op('GEN-APPROVAL','通用簽核','跨部門通用簽核表單',null,'Stamp',21,'[
  {"section":"申請","field_name":"request_no","label":"申請編號","type":"TEXT","required":true,"searchable":true},
  {"section":"申請","field_name":"request_type","label":"申請類型","type":"TEXT","required":true},
  {"section":"申請","field_name":"requester","label":"申請人","type":"TEXT","required":true,"searchable":true},
  {"section":"申請","field_name":"department","label":"部門","type":"TEXT"},
  {"section":"申請","field_name":"title","label":"申請主題","type":"TEXT","required":true,"searchable":true},
  {"section":"申請","field_name":"description","label":"申請說明","type":"TEXTAREA","required":true},
  {"section":"申請","field_name":"amount","label":"涉及金額","type":"NUMBER"},
  {"section":"簽核","field_name":"urgency","label":"緊急度","type":"DROPDOWN","options":[{"label":"一般","value":"NORMAL"},{"label":"高","value":"HIGH"},{"label":"緊急","value":"URGENT"}]},
  {"section":"簽核","field_name":"approver","label":"簽核人","type":"TEXT"},
  {"section":"簽核","field_name":"status","label":"狀態","type":"DROPDOWN","required":true,"options":[{"label":"待簽核","value":"PENDING"},{"label":"已核准","value":"APPROVED"},{"label":"已退回","value":"REJECTED"},{"label":"已完成","value":"COMPLETED"}]},
  {"section":"簽核","field_name":"rejection_reason","label":"退回原因","type":"TEXTAREA"}
]'::jsonb);
SELECT _sample('GEN-APPROVAL','{"request_no":"GA-001","request_type":"設備採購","requester":"張莉","department":"生產部","title":"申請購買自動包裝機","description":"為配合產能擴充需求，申請購買全自動包裝機一台","amount":1500000,"urgency":"HIGH","approver":"陳偉","status":"PENDING","rejection_reason":""}');

SELECT _create_op('SD-AGED-AR','應收帳齡','客戶應收帳款帳齡分析','SD','Clock',62,'[
  {"section":"客戶","field_name":"customer","label":"客戶","type":"TEXT","required":true,"searchable":true},
  {"section":"帳齡","field_name":"total_ar","label":"應收總額","type":"NUMBER","required":true},
  {"section":"帳齡","field_name":"current_amount","label":"未到期","type":"NUMBER"},
  {"section":"帳齡","field_name":"days_30","label":"1-30天","type":"NUMBER"},
  {"section":"帳齡","field_name":"days_60","label":"31-60天","type":"NUMBER"},
  {"section":"帳齡","field_name":"days_90","label":"61-90天","type":"NUMBER"},
  {"section":"帳齡","field_name":"days_over90","label":"90天以上","type":"NUMBER"},
  {"section":"催收","field_name":"contact_status","label":"聯繫狀態","type":"TEXT"},
  {"section":"催收","field_name":"last_contact_date","label":"最近聯繫日","type":"DATE"},
  {"section":"催收","field_name":"collection_action","label":"催收動作","type":"TEXTAREA"}
]'::jsonb);
SELECT _sample('SD-AGED-AR','{"customer":"大潤發","total_ar":2800000,"current_amount":1500000,"days_30":800000,"days_60":300000,"days_90":150000,"days_over90":50000,"contact_status":"已聯繫業務窗口","last_contact_date":"2026-03-18","collection_action":"已發催款函，業務部門持續跟進"}');

SELECT _create_op('SD-ORDER-CHG','訂單變更','銷售訂單變更管理','SD','RefreshCw',63,'[
  {"section":"變更","field_name":"change_no","label":"變更編號","type":"TEXT","required":true,"searchable":true},
  {"section":"變更","field_name":"original_so","label":"原始訂單","type":"TEXT","required":true,"searchable":true},
  {"section":"變更","field_name":"change_type","label":"變更類型","type":"DROPDOWN","required":true,"options":[{"label":"數量變更","value":"QTY_CHANGE"},{"label":"日期變更","value":"DATE_CHANGE"},{"label":"取消品項","value":"CANCEL_ITEM"},{"label":"新增品項","value":"ADD_ITEM"}]},
  {"section":"變更","field_name":"change_description","label":"變更說明","type":"TEXTAREA","required":true},
  {"section":"影響","field_name":"old_value","label":"原始值","type":"TEXT"},
  {"section":"影響","field_name":"new_value","label":"新值","type":"TEXT"},
  {"section":"影響","field_name":"impact_amount","label":"金額影響","type":"NUMBER"},
  {"section":"審核","field_name":"requested_by","label":"申請人","type":"TEXT"},
  {"section":"審核","field_name":"approved_by","label":"核准人","type":"TEXT"},
  {"section":"審核","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"申請中","value":"REQUESTED"},{"label":"已核准","value":"APPROVED"},{"label":"已退回","value":"REJECTED"},{"label":"已執行","value":"APPLIED"}]}
]'::jsonb);
SELECT _sample('SD-ORDER-CHG','{"change_no":"CHG-001","original_so":"SO-00003","change_type":"QTY_CHANGE","change_description":"好市多要求將脆薯片數量從30000調整為25000","old_value":"30000","new_value":"25000","impact_amount":-175000,"requested_by":"林美","approved_by":"","status":"REQUESTED"}');

SELECT _create_op('SD-PRICING','定價管理','產品定價與折扣管理','SD','Tag',64,'[
  {"section":"定價","field_name":"material","label":"產品","type":"TEXT","required":true,"searchable":true},
  {"section":"定價","field_name":"price_list","label":"價格表","type":"TEXT","required":true},
  {"section":"定價","field_name":"base_price","label":"基礎價格","type":"NUMBER","required":true},
  {"section":"折扣","field_name":"discount_group","label":"折扣群組","type":"TEXT"},
  {"section":"折扣","field_name":"discount_pct","label":"折扣率(%)","type":"NUMBER"},
  {"section":"折扣","field_name":"net_price","label":"淨價","type":"NUMBER"},
  {"section":"效期","field_name":"effective_from","label":"生效日","type":"DATE","required":true},
  {"section":"效期","field_name":"effective_until","label":"截止日","type":"DATE"},
  {"section":"效期","field_name":"currency","label":"幣別","type":"DROPDOWN","options":[{"label":"TWD","value":"TWD","is_default":"true"},{"label":"USD","value":"USD"}]},
  {"section":"效期","field_name":"min_qty","label":"最低數量","type":"NUMBER"},
  {"section":"效期","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"有效","value":"ACTIVE"},{"label":"即將生效","value":"PENDING"},{"label":"已失效","value":"EXPIRED"}]}
]'::jsonb);
SELECT _sample('SD-PRICING','{"material":"脆薯片(原味) 150g","price_list":"量販通路","base_price":35,"discount_group":"VOLUME","discount_pct":5,"net_price":33.25,"effective_from":"2026-01-01","effective_until":"2026-12-31","currency":"TWD","min_qty":1000,"status":"ACTIVE"}');

SELECT _create_op('HR-WORK-INJURY','工傷報告','職業災害報告','HR','Heart',49,'[
  {"section":"事故","field_name":"report_no","label":"報告編號","type":"TEXT","required":true,"searchable":true},
  {"section":"事故","field_name":"employee","label":"員工","type":"TEXT","required":true,"searchable":true},
  {"section":"事故","field_name":"injury_date","label":"受傷日期","type":"DATE","required":true},
  {"section":"事故","field_name":"injury_time","label":"受傷時間","type":"TEXT"},
  {"section":"事故","field_name":"location","label":"地點","type":"TEXT","required":true},
  {"section":"傷害","field_name":"injury_type","label":"傷害程度","type":"DROPDOWN","required":true,"options":[{"label":"輕微","value":"MINOR"},{"label":"中等","value":"MODERATE"},{"label":"嚴重","value":"SEVERE"}]},
  {"section":"傷害","field_name":"description","label":"事故描述","type":"TEXTAREA","required":true},
  {"section":"傷害","field_name":"witness","label":"目擊者","type":"TEXT"},
  {"section":"處理","field_name":"first_aid","label":"急救處理","type":"TEXTAREA"},
  {"section":"處理","field_name":"hospital_visit","label":"就醫","type":"DROPDOWN","options":[{"label":"是","value":"YES"},{"label":"否","value":"NO"}]},
  {"section":"處理","field_name":"lost_days","label":"損失工日","type":"NUMBER"},
  {"section":"處理","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"已通報","value":"REPORTED"},{"label":"調查中","value":"INVESTIGATING"},{"label":"已結案","value":"CLOSED"}]},
  {"section":"處理","field_name":"preventive_measures","label":"預防措施","type":"TEXTAREA"}
]'::jsonb);
SELECT _sample('HR-WORK-INJURY','{"report_no":"WI-001","employee":"作業員小王","injury_date":"2026-03-21","injury_time":"10:20","location":"油炸區","injury_type":"MINOR","description":"設備維修時手指輕微燙傷","witness":"張莉","first_aid":"冰敷處理，塗抹燙傷藥膏","hospital_visit":"NO","lost_days":0,"status":"CLOSED","preventive_measures":"加強防護手套配戴規定，設備維修前確認冷卻完成"}');

SELECT _create_op('QM-RECALL','產品召回','產品召回管理','QM','AlertTriangle',35,'[
  {"section":"召回","field_name":"recall_no","label":"召回編號","type":"TEXT","required":true,"searchable":true},
  {"section":"召回","field_name":"product","label":"產品","type":"TEXT","required":true,"searchable":true},
  {"section":"召回","field_name":"batch_numbers","label":"涉及批號","type":"TEXTAREA","required":true},
  {"section":"召回","field_name":"reason","label":"召回原因","type":"TEXTAREA","required":true},
  {"section":"召回","field_name":"severity","label":"嚴重等級","type":"DROPDOWN","required":true,"options":[{"label":"第一級(危害健康)","value":"CLASS_I"},{"label":"第二級(可能危害)","value":"CLASS_II"},{"label":"第三級(不會危害)","value":"CLASS_III"}]},
  {"section":"執行","field_name":"affected_quantity","label":"影響數量","type":"NUMBER","required":true},
  {"section":"執行","field_name":"recalled_quantity","label":"已回收數量","type":"NUMBER"},
  {"section":"執行","field_name":"recall_rate","label":"回收率(%)","type":"NUMBER"},
  {"section":"執行","field_name":"notification_date","label":"通知日期","type":"DATE","required":true},
  {"section":"執行","field_name":"completion_date","label":"完成日期","type":"DATE"},
  {"section":"執行","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"啟動","value":"INITIATED"},{"label":"執行中","value":"IN_PROGRESS"},{"label":"已完成","value":"COMPLETED"},{"label":"已結案","value":"CLOSED"}]}
]'::jsonb);
SELECT _sample('QM-RECALL','{"recall_no":"RCL-001","product":"脆薯片(BBQ) 150g","batch_numbers":"BAT-P20260315, BAT-P20260316","reason":"調味料異味問題，預防性召回","severity":"CLASS_III","affected_quantity":20000,"recalled_quantity":18500,"recall_rate":92.5,"notification_date":"2026-03-23","completion_date":null,"status":"IN_PROGRESS"}');

SELECT _create_op('FI-INTERCO','關企沖帳','關係企業間交易沖帳','FI','ArrowRightLeft',21,'[
  {"section":"交易","field_name":"transaction_no","label":"交易編號","type":"TEXT","required":true,"searchable":true},
  {"section":"交易","field_name":"from_company","label":"出帳公司","type":"TEXT","required":true},
  {"section":"交易","field_name":"to_company","label":"入帳公司","type":"TEXT","required":true},
  {"section":"交易","field_name":"amount","label":"金額","type":"NUMBER","required":true},
  {"section":"交易","field_name":"currency","label":"幣別","type":"DROPDOWN","options":[{"label":"TWD","value":"TWD","is_default":"true"},{"label":"USD","value":"USD"}]},
  {"section":"交易","field_name":"transaction_date","label":"交易日期","type":"DATE","required":true},
  {"section":"交易","field_name":"description","label":"說明","type":"TEXTAREA"},
  {"section":"交易","field_name":"status","label":"狀態","type":"DROPDOWN","options":[{"label":"待確認","value":"PENDING"},{"label":"已確認","value":"CONFIRMED"},{"label":"已沖帳","value":"SETTLED"}]}
]'::jsonb);
SELECT _sample('FI-INTERCO','{"transaction_no":"IC-001","from_company":"TasteByte Foods","to_company":"TasteByte Trading","amount":500000,"currency":"TWD","transaction_date":"2026-03-25","description":"代工生產費用","status":"CONFIRMED"}');

DROP FUNCTION IF EXISTS _create_op(TEXT,TEXT,TEXT,TEXT,TEXT,INT,JSONB);
DROP FUNCTION IF EXISTS _sample(TEXT,JSONB);

COMMIT;
