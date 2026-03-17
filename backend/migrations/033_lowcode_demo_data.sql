-- 033: Low-Code Platform Demo Data
-- Populates realistic cross-module demo data for a food & beverage ERP low-code platform.
-- Includes projects, operations (forms), field definitions, sample records, releases, feedback, and dev journal.

BEGIN;

-- ============================================================
-- Helper: resolve admin user ID
-- ============================================================
DO $$
DECLARE
    v_admin_id UUID;

    -- Project IDs
    v_proj_core_id UUID := 'dd000000-0000-0000-0000-000000000001'::uuid;
    v_proj_qc_id   UUID := 'dd000000-0000-0000-0000-000000000002'::uuid;

    -- Operation IDs (6 forms)
    v_op_mat_req_id   UUID := 'dd100000-0000-0000-0000-000000000001'::uuid;
    v_op_cust_fb_id   UUID := 'dd100000-0000-0000-0000-000000000002'::uuid;
    v_op_equip_id     UUID := 'dd100000-0000-0000-0000-000000000003'::uuid;
    v_op_food_safe_id UUID := 'dd100000-0000-0000-0000-000000000004'::uuid;
    v_op_supplier_id  UUID := 'dd100000-0000-0000-0000-000000000005'::uuid;
    v_op_training_id  UUID := 'dd100000-0000-0000-0000-000000000006'::uuid;

    -- Form definition IDs
    v_fd_mat_req_id   UUID := 'dd200000-0000-0000-0000-000000000001'::uuid;
    v_fd_cust_fb_id   UUID := 'dd200000-0000-0000-0000-000000000002'::uuid;
    v_fd_equip_id     UUID := 'dd200000-0000-0000-0000-000000000003'::uuid;
    v_fd_food_safe_id UUID := 'dd200000-0000-0000-0000-000000000004'::uuid;
    v_fd_supplier_id  UUID := 'dd200000-0000-0000-0000-000000000005'::uuid;
    v_fd_training_id  UUID := 'dd200000-0000-0000-0000-000000000006'::uuid;

    -- Section IDs (each form has 2-3 sections)
    -- Material Request: Basic Info, Request Details
    v_sec_mr_basic  UUID := 'dd300000-0000-0000-0000-000000000001'::uuid;
    v_sec_mr_detail UUID := 'dd300000-0000-0000-0000-000000000002'::uuid;
    -- Customer Feedback: Customer Info, Feedback
    v_sec_cf_cust   UUID := 'dd300000-0000-0000-0000-000000000003'::uuid;
    v_sec_cf_fb     UUID := 'dd300000-0000-0000-0000-000000000004'::uuid;
    -- Equipment Maintenance: Equipment Info, Maintenance Details
    v_sec_eq_info   UUID := 'dd300000-0000-0000-0000-000000000005'::uuid;
    v_sec_eq_maint  UUID := 'dd300000-0000-0000-0000-000000000006'::uuid;
    -- Food Safety: Inspection Info, Temperature Checks, Hygiene
    v_sec_fs_insp   UUID := 'dd300000-0000-0000-0000-000000000007'::uuid;
    v_sec_fs_temp   UUID := 'dd300000-0000-0000-0000-000000000008'::uuid;
    v_sec_fs_hyg    UUID := 'dd300000-0000-0000-0000-000000000009'::uuid;
    -- Supplier Evaluation: Supplier Info, Evaluation Criteria, Summary
    v_sec_se_info   UUID := 'dd300000-0000-0000-0000-00000000000a'::uuid;
    v_sec_se_eval   UUID := 'dd300000-0000-0000-0000-00000000000b'::uuid;
    v_sec_se_summ   UUID := 'dd300000-0000-0000-0000-00000000000c'::uuid;
    -- Employee Training: Training Info, Attendee Details
    v_sec_tr_info   UUID := 'dd300000-0000-0000-0000-00000000000d'::uuid;
    v_sec_tr_att    UUID := 'dd300000-0000-0000-0000-00000000000e'::uuid;

    -- Field IDs (prefixed dd4 for fields)
    -- Material Request fields
    v_fld_mr_date       UUID := 'dd400000-0000-0000-0000-000000000001'::uuid;
    v_fld_mr_requester  UUID := 'dd400000-0000-0000-0000-000000000002'::uuid;
    v_fld_mr_dept       UUID := 'dd400000-0000-0000-0000-000000000003'::uuid;
    v_fld_mr_matname    UUID := 'dd400000-0000-0000-0000-000000000004'::uuid;
    v_fld_mr_mattype    UUID := 'dd400000-0000-0000-0000-000000000005'::uuid;
    v_fld_mr_qty        UUID := 'dd400000-0000-0000-0000-000000000006'::uuid;
    v_fld_mr_unit       UUID := 'dd400000-0000-0000-0000-000000000007'::uuid;
    v_fld_mr_urgency    UUID := 'dd400000-0000-0000-0000-000000000008'::uuid;
    v_fld_mr_reason     UUID := 'dd400000-0000-0000-0000-000000000009'::uuid;
    v_fld_mr_approved   UUID := 'dd400000-0000-0000-0000-00000000000a'::uuid;

    -- Customer Feedback fields
    v_fld_cf_date       UUID := 'dd400000-0000-0000-0000-000000000011'::uuid;
    v_fld_cf_name       UUID := 'dd400000-0000-0000-0000-000000000012'::uuid;
    v_fld_cf_order      UUID := 'dd400000-0000-0000-0000-000000000013'::uuid;
    v_fld_cf_rating     UUID := 'dd400000-0000-0000-0000-000000000014'::uuid;
    v_fld_cf_category   UUID := 'dd400000-0000-0000-0000-000000000015'::uuid;
    v_fld_cf_comments   UUID := 'dd400000-0000-0000-0000-000000000016'::uuid;
    v_fld_cf_followup   UUID := 'dd400000-0000-0000-0000-000000000017'::uuid;
    v_fld_cf_email      UUID := 'dd400000-0000-0000-0000-000000000018'::uuid;

    -- Equipment Maintenance fields
    v_fld_eq_date       UUID := 'dd400000-0000-0000-0000-000000000021'::uuid;
    v_fld_eq_name       UUID := 'dd400000-0000-0000-0000-000000000022'::uuid;
    v_fld_eq_eqid       UUID := 'dd400000-0000-0000-0000-000000000023'::uuid;
    v_fld_eq_location   UUID := 'dd400000-0000-0000-0000-000000000024'::uuid;
    v_fld_eq_type       UUID := 'dd400000-0000-0000-0000-000000000025'::uuid;
    v_fld_eq_tech       UUID := 'dd400000-0000-0000-0000-000000000026'::uuid;
    v_fld_eq_desc       UUID := 'dd400000-0000-0000-0000-000000000027'::uuid;
    v_fld_eq_parts      UUID := 'dd400000-0000-0000-0000-000000000028'::uuid;
    v_fld_eq_downtime   UUID := 'dd400000-0000-0000-0000-000000000029'::uuid;
    v_fld_eq_next       UUID := 'dd400000-0000-0000-0000-00000000002a'::uuid;
    v_fld_eq_status     UUID := 'dd400000-0000-0000-0000-00000000002b'::uuid;

    -- Food Safety Checklist fields
    v_fld_fs_date       UUID := 'dd400000-0000-0000-0000-000000000031'::uuid;
    v_fld_fs_inspector  UUID := 'dd400000-0000-0000-0000-000000000032'::uuid;
    v_fld_fs_shift      UUID := 'dd400000-0000-0000-0000-000000000033'::uuid;
    v_fld_fs_area       UUID := 'dd400000-0000-0000-0000-000000000034'::uuid;
    v_fld_fs_fridge     UUID := 'dd400000-0000-0000-0000-000000000035'::uuid;
    v_fld_fs_freezer    UUID := 'dd400000-0000-0000-0000-000000000036'::uuid;
    v_fld_fs_hothold    UUID := 'dd400000-0000-0000-0000-000000000037'::uuid;
    v_fld_fs_handwash   UUID := 'dd400000-0000-0000-0000-000000000038'::uuid;
    v_fld_fs_surfaces   UUID := 'dd400000-0000-0000-0000-000000000039'::uuid;
    v_fld_fs_pest       UUID := 'dd400000-0000-0000-0000-00000000003a'::uuid;
    v_fld_fs_labeling   UUID := 'dd400000-0000-0000-0000-00000000003b'::uuid;
    v_fld_fs_result     UUID := 'dd400000-0000-0000-0000-00000000003c'::uuid;
    v_fld_fs_actions    UUID := 'dd400000-0000-0000-0000-00000000003d'::uuid;

    -- Supplier Evaluation fields
    v_fld_se_date       UUID := 'dd400000-0000-0000-0000-000000000041'::uuid;
    v_fld_se_supplier   UUID := 'dd400000-0000-0000-0000-000000000042'::uuid;
    v_fld_se_evaluator  UUID := 'dd400000-0000-0000-0000-000000000043'::uuid;
    v_fld_se_delivery   UUID := 'dd400000-0000-0000-0000-000000000044'::uuid;
    v_fld_se_quality    UUID := 'dd400000-0000-0000-0000-000000000045'::uuid;
    v_fld_se_price      UUID := 'dd400000-0000-0000-0000-000000000046'::uuid;
    v_fld_se_comm       UUID := 'dd400000-0000-0000-0000-000000000047'::uuid;
    v_fld_se_overall    UUID := 'dd400000-0000-0000-0000-000000000048'::uuid;
    v_fld_se_recommend  UUID := 'dd400000-0000-0000-0000-000000000049'::uuid;
    v_fld_se_notes      UUID := 'dd400000-0000-0000-0000-00000000004a'::uuid;

    -- Employee Training fields
    v_fld_tr_date       UUID := 'dd400000-0000-0000-0000-000000000051'::uuid;
    v_fld_tr_title      UUID := 'dd400000-0000-0000-0000-000000000052'::uuid;
    v_fld_tr_trainer    UUID := 'dd400000-0000-0000-0000-000000000053'::uuid;
    v_fld_tr_type       UUID := 'dd400000-0000-0000-0000-000000000054'::uuid;
    v_fld_tr_duration   UUID := 'dd400000-0000-0000-0000-000000000055'::uuid;
    v_fld_tr_empname    UUID := 'dd400000-0000-0000-0000-000000000056'::uuid;
    v_fld_tr_dept       UUID := 'dd400000-0000-0000-0000-000000000057'::uuid;
    v_fld_tr_score      UUID := 'dd400000-0000-0000-0000-000000000058'::uuid;
    v_fld_tr_passed     UUID := 'dd400000-0000-0000-0000-000000000059'::uuid;
    v_fld_tr_cert       UUID := 'dd400000-0000-0000-0000-00000000005a'::uuid;
    v_fld_tr_expiry     UUID := 'dd400000-0000-0000-0000-00000000005b'::uuid;

    -- Release IDs
    v_rel_1 UUID := 'dd500000-0000-0000-0000-000000000001'::uuid;
    v_rel_2 UUID := 'dd500000-0000-0000-0000-000000000002'::uuid;
    v_rel_3 UUID := 'dd500000-0000-0000-0000-000000000003'::uuid;

    -- Feedback IDs
    v_fb_1 UUID := 'dd600000-0000-0000-0000-000000000001'::uuid;
    v_fb_2 UUID := 'dd600000-0000-0000-0000-000000000002'::uuid;
    v_fb_3 UUID := 'dd600000-0000-0000-0000-000000000003'::uuid;
    v_fb_4 UUID := 'dd600000-0000-0000-0000-000000000004'::uuid;

BEGIN
    -- Resolve admin user
    SELECT id INTO v_admin_id FROM users WHERE username = 'admin' LIMIT 1;
    IF v_admin_id IS NULL THEN
        RAISE EXCEPTION 'Admin user not found. Cannot proceed with demo data.';
    END IF;

    -- ============================================================
    -- 0. CLEANUP old lowcode data that conflicts
    -- ============================================================
    DELETE FROM lc_operation_data WHERE operation_id IN (SELECT id FROM lc_operations WHERE project_id IN (SELECT id FROM lc_projects WHERE project_number IN ('LCP00000001','LCP00000002')));
    DELETE FROM lc_field_options WHERE field_id IN (SELECT fd.id FROM lc_field_definitions fd JOIN lc_form_sections fs ON fd.section_id = fs.id JOIN lc_form_definitions fdef ON fs.form_id = fdef.id JOIN lc_operations o ON fdef.operation_id = o.id WHERE o.project_id IN (SELECT id FROM lc_projects WHERE project_number IN ('LCP00000001','LCP00000002')));
    DELETE FROM lc_field_definitions WHERE section_id IN (SELECT fs.id FROM lc_form_sections fs JOIN lc_form_definitions fdef ON fs.form_id = fdef.id JOIN lc_operations o ON fdef.operation_id = o.id WHERE o.project_id IN (SELECT id FROM lc_projects WHERE project_number IN ('LCP00000001','LCP00000002')));
    DELETE FROM lc_form_sections WHERE form_id IN (SELECT fdef.id FROM lc_form_definitions fdef JOIN lc_operations o ON fdef.operation_id = o.id WHERE o.project_id IN (SELECT id FROM lc_projects WHERE project_number IN ('LCP00000001','LCP00000002')));
    DELETE FROM lc_form_definitions WHERE operation_id IN (SELECT id FROM lc_operations WHERE project_id IN (SELECT id FROM lc_projects WHERE project_number IN ('LCP00000001','LCP00000002')));
    DELETE FROM lc_dev_journal WHERE operation_id IN (SELECT id FROM lc_operations WHERE project_id IN (SELECT id FROM lc_projects WHERE project_number IN ('LCP00000001','LCP00000002')));
    DELETE FROM lc_release_feedback_links WHERE release_id IN (SELECT id FROM lc_releases WHERE operation_id IN (SELECT id FROM lc_operations WHERE project_id IN (SELECT id FROM lc_projects WHERE project_number IN ('LCP00000001','LCP00000002'))));
    DELETE FROM lc_feedback_comments WHERE feedback_id IN (SELECT id FROM lc_feedback WHERE operation_id IN (SELECT id FROM lc_operations WHERE project_id IN (SELECT id FROM lc_projects WHERE project_number IN ('LCP00000001','LCP00000002'))));
    DELETE FROM lc_feedback WHERE operation_id IN (SELECT id FROM lc_operations WHERE project_id IN (SELECT id FROM lc_projects WHERE project_number IN ('LCP00000001','LCP00000002')));
    DELETE FROM lc_releases WHERE operation_id IN (SELECT id FROM lc_operations WHERE project_id IN (SELECT id FROM lc_projects WHERE project_number IN ('LCP00000001','LCP00000002')));
    DELETE FROM lc_operations WHERE project_id IN (SELECT id FROM lc_projects WHERE project_number IN ('LCP00000001','LCP00000002'));
    DELETE FROM lc_project_developers WHERE project_id IN (SELECT id FROM lc_projects WHERE project_number IN ('LCP00000001','LCP00000002'));
    DELETE FROM lc_projects WHERE project_number IN ('LCP00000001','LCP00000002');

    -- ============================================================
    -- 1. PROJECTS
    -- ============================================================
    INSERT INTO lc_projects (id, project_number, name, description, is_active, created_by)
    VALUES
        (v_proj_core_id, 'LCP00000001', 'ERP Core Extensions',
         'Extensions for standard ERP modules including material management, sales, and production forms.',
         true, v_admin_id),
        (v_proj_qc_id, 'LCP00000002', 'Quality & Compliance',
         'Quality management and compliance forms for food safety, supplier evaluation, and employee training.',
         true, v_admin_id);

    -- Project developers
    INSERT INTO lc_project_developers (project_id, user_id, role)
    VALUES
        (v_proj_core_id, v_admin_id, 'LEAD'),
        (v_proj_qc_id, v_admin_id, 'LEAD')
    ON CONFLICT (project_id, user_id) DO NOTHING;

    -- ============================================================
    -- 2. OPERATIONS (6 forms)
    -- ============================================================
    INSERT INTO lc_operations (id, operation_code, project_id, name, description, operation_type, is_published, version, module, sidebar_icon, sidebar_sort_order, created_by)
    VALUES
        (v_op_mat_req_id, 'LCO00000003', v_proj_core_id,
         'Material Request Form', 'For requesting new materials to be added to inventory',
         'FORM', true, 2, 'MM', 'PackagePlus', 10, v_admin_id),
        (v_op_cust_fb_id, 'LCO00000004', v_proj_core_id,
         'Customer Feedback Form', 'For collecting customer satisfaction feedback',
         'FORM', true, 1, 'SD', 'MessageCircle', 20, v_admin_id),
        (v_op_equip_id, 'LCO00000005', v_proj_core_id,
         'Equipment Maintenance Log', 'For logging production equipment maintenance activities',
         'FORM', true, 1, 'PP', 'Wrench', 30, v_admin_id),
        (v_op_food_safe_id, 'LCO00000006', v_proj_qc_id,
         'Food Safety Checklist', 'Daily food safety inspection checklist for all areas',
         'FORM', true, 2, 'QM', 'ShieldCheck', 10, v_admin_id),
        (v_op_supplier_id, 'LCO00000007', v_proj_qc_id,
         'Supplier Evaluation Form', 'For evaluating supplier performance on delivery, quality, and pricing',
         'FORM', true, 1, 'MM', 'ClipboardCheck', 20, v_admin_id),
        (v_op_training_id, 'LCO00000008', v_proj_qc_id,
         'Employee Training Record', 'For tracking employee training completion and certifications',
         'FORM', true, 1, 'HR', 'GraduationCap', 30, v_admin_id)
    ON CONFLICT (id) DO NOTHING;

    -- ============================================================
    -- 3. FORM DEFINITIONS
    -- ============================================================
    INSERT INTO lc_form_definitions (id, operation_id, layout_config, form_settings, version)
    VALUES
        (v_fd_mat_req_id, v_op_mat_req_id,
         '{"maxWidth": "900px", "padding": "24px"}',
         '{"submitLabel": "Submit Request", "successMessage": "Material request submitted successfully"}',
         2),
        (v_fd_cust_fb_id, v_op_cust_fb_id,
         '{"maxWidth": "800px", "padding": "24px"}',
         '{"submitLabel": "Submit Feedback", "successMessage": "Thank you for your feedback"}',
         1),
        (v_fd_equip_id, v_op_equip_id,
         '{"maxWidth": "900px", "padding": "24px"}',
         '{"submitLabel": "Save Log Entry", "successMessage": "Maintenance log saved"}',
         1),
        (v_fd_food_safe_id, v_op_food_safe_id,
         '{"maxWidth": "900px", "padding": "24px"}',
         '{"submitLabel": "Submit Inspection", "successMessage": "Food safety inspection recorded"}',
         2),
        (v_fd_supplier_id, v_op_supplier_id,
         '{"maxWidth": "900px", "padding": "24px"}',
         '{"submitLabel": "Submit Evaluation", "successMessage": "Supplier evaluation saved"}',
         1),
        (v_fd_training_id, v_op_training_id,
         '{"maxWidth": "900px", "padding": "24px"}',
         '{"submitLabel": "Save Training Record", "successMessage": "Training record saved"}',
         1)
    ON CONFLICT (operation_id) DO NOTHING;

    -- ============================================================
    -- 4. FORM SECTIONS
    -- ============================================================
    INSERT INTO lc_form_sections (id, form_id, title, description, columns, sort_order, is_collapsible)
    VALUES
        -- Material Request
        (v_sec_mr_basic,  v_fd_mat_req_id, 'Basic Info', 'Requester and department information', 2, 0, false),
        (v_sec_mr_detail, v_fd_mat_req_id, 'Request Details', 'Material specification and urgency', 2, 1, false),
        -- Customer Feedback
        (v_sec_cf_cust, v_fd_cust_fb_id, 'Customer Info', 'Customer identification', 2, 0, false),
        (v_sec_cf_fb,   v_fd_cust_fb_id, 'Feedback', 'Feedback details and rating', 1, 1, false),
        -- Equipment Maintenance
        (v_sec_eq_info,  v_fd_equip_id, 'Equipment Info', 'Equipment identification and location', 2, 0, false),
        (v_sec_eq_maint, v_fd_equip_id, 'Maintenance Details', 'Work performed and scheduling', 2, 1, false),
        -- Food Safety
        (v_sec_fs_insp, v_fd_food_safe_id, 'Inspection Info', 'Inspector and shift details', 2, 0, false),
        (v_sec_fs_temp, v_fd_food_safe_id, 'Temperature Checks', 'Equipment temperature readings', 2, 1, false),
        (v_sec_fs_hyg,  v_fd_food_safe_id, 'Hygiene', 'Hygiene and compliance checks', 2, 2, false),
        -- Supplier Evaluation
        (v_sec_se_info, v_fd_supplier_id, 'Supplier Info', 'Supplier identification', 2, 0, false),
        (v_sec_se_eval, v_fd_supplier_id, 'Evaluation Criteria', 'Performance scoring', 2, 1, false),
        (v_sec_se_summ, v_fd_supplier_id, 'Summary', 'Overall assessment and recommendation', 1, 2, false),
        -- Employee Training
        (v_sec_tr_info, v_fd_training_id, 'Training Info', 'Training session details', 2, 0, false),
        (v_sec_tr_att,  v_fd_training_id, 'Attendee Details', 'Employee performance and certification', 2, 1, false)
    ON CONFLICT (id) DO NOTHING;

    -- ============================================================
    -- 5. FIELD DEFINITIONS
    -- ============================================================

    -- ---- Material Request Form ----
    INSERT INTO lc_field_definitions (id, section_id, field_name, field_label, field_type, is_required, placeholder, sort_order, column_span) VALUES
        (v_fld_mr_date,      v_sec_mr_basic,  'request_date',   'Request Date',    'DATE',     true,  NULL,                    0, 1),
        (v_fld_mr_requester, v_sec_mr_basic,  'requester_name', 'Requester Name',  'TEXT',     true,  'Enter your full name',  1, 1),
        (v_fld_mr_dept,      v_sec_mr_basic,  'department',     'Department',       'DROPDOWN', true,  NULL,                   2, 1),
        (v_fld_mr_matname,   v_sec_mr_detail, 'material_name',  'Material Name',   'TEXT',     true,  'e.g. Olive Oil, Sugar', 0, 1),
        (v_fld_mr_mattype,   v_sec_mr_detail, 'material_type',  'Material Type',   'DROPDOWN', true,  NULL,                   1, 1),
        (v_fld_mr_qty,       v_sec_mr_detail, 'quantity_needed', 'Quantity Needed', 'NUMBER',   true,  '0',                    2, 1),
        (v_fld_mr_unit,      v_sec_mr_detail, 'unit',           'Unit',            'DROPDOWN', true,  NULL,                    3, 1),
        (v_fld_mr_urgency,   v_sec_mr_detail, 'urgency',        'Urgency',         'DROPDOWN', true,  NULL,                    4, 1),
        (v_fld_mr_reason,    v_sec_mr_detail, 'reason',         'Reason',          'TEXTAREA', false, 'Why is this material needed?', 5, 2),
        (v_fld_mr_approved,  v_sec_mr_detail, 'approved_by',    'Approved By',     'TEXT',     false, 'Manager name',          6, 1)
    ON CONFLICT (section_id, field_name) DO NOTHING;

    -- ---- Customer Feedback Form ----
    INSERT INTO lc_field_definitions (id, section_id, field_name, field_label, field_type, is_required, placeholder, sort_order, column_span) VALUES
        (v_fld_cf_date,     v_sec_cf_cust, 'feedback_date',      'Feedback Date',      'DATE',     true,  NULL,                     0, 1),
        (v_fld_cf_name,     v_sec_cf_cust, 'customer_name',      'Customer Name',      'TEXT',     true,  'Customer or company name', 1, 1),
        (v_fld_cf_order,    v_sec_cf_cust, 'order_number',       'Order Number',       'TEXT',     false, 'SO-XXXXXXXX',            2, 1),
        (v_fld_cf_rating,   v_sec_cf_fb,   'rating',             'Rating',             'DROPDOWN', true,  NULL,                     0, 1),
        (v_fld_cf_category, v_sec_cf_fb,   'category',           'Category',           'DROPDOWN', true,  NULL,                     1, 1),
        (v_fld_cf_comments, v_sec_cf_fb,   'comments',           'Comments',           'TEXTAREA', false, 'Please describe your experience', 2, 1),
        (v_fld_cf_followup, v_sec_cf_fb,   'follow_up_required', 'Follow-up Required', 'CHECKBOX', false, NULL,                     3, 1),
        (v_fld_cf_email,    v_sec_cf_fb,   'contact_email',      'Contact Email',      'TEXT',     false, 'customer@example.com',   4, 1)
    ON CONFLICT (section_id, field_name) DO NOTHING;

    -- ---- Equipment Maintenance Log ----
    INSERT INTO lc_field_definitions (id, section_id, field_name, field_label, field_type, is_required, placeholder, sort_order, column_span) VALUES
        (v_fld_eq_date,     v_sec_eq_info,  'maintenance_date', 'Maintenance Date',  'DATE',     true,  NULL,                   0, 1),
        (v_fld_eq_name,     v_sec_eq_info,  'equipment_name',   'Equipment Name',    'TEXT',     true,  'e.g. Industrial Oven', 1, 1),
        (v_fld_eq_eqid,     v_sec_eq_info,  'equipment_id',     'Equipment ID',      'TEXT',     true,  'e.g. EQ-001',          2, 1),
        (v_fld_eq_location, v_sec_eq_info,  'location',         'Location',          'DROPDOWN', true,  NULL,                   3, 1),
        (v_fld_eq_type,     v_sec_eq_maint, 'maintenance_type', 'Maintenance Type',  'DROPDOWN', true,  NULL,                   0, 1),
        (v_fld_eq_tech,     v_sec_eq_maint, 'technician',       'Technician',        'TEXT',     true,  'Technician name',      1, 1),
        (v_fld_eq_desc,     v_sec_eq_maint, 'description',      'Description',       'TEXTAREA', false, 'Describe work performed', 2, 2),
        (v_fld_eq_parts,    v_sec_eq_maint, 'parts_replaced',   'Parts Replaced',    'TEXT',     false, 'List parts if any',    3, 1),
        (v_fld_eq_downtime, v_sec_eq_maint, 'downtime_hours',   'Downtime (Hours)',  'NUMBER',   false, '0',                    4, 1),
        (v_fld_eq_next,     v_sec_eq_maint, 'next_scheduled',   'Next Scheduled',    'DATE',     false, NULL,                   5, 1),
        (v_fld_eq_status,   v_sec_eq_maint, 'status',           'Status',            'DROPDOWN', true,  NULL,                   6, 1)
    ON CONFLICT (section_id, field_name) DO NOTHING;

    -- ---- Food Safety Checklist ----
    INSERT INTO lc_field_definitions (id, section_id, field_name, field_label, field_type, is_required, placeholder, sort_order, column_span) VALUES
        (v_fld_fs_date,      v_sec_fs_insp, 'inspection_date', 'Inspection Date', 'DATE',     true,  NULL,  0, 1),
        (v_fld_fs_inspector, v_sec_fs_insp, 'inspector_name',  'Inspector Name',  'TEXT',     true,  'Inspector full name', 1, 1),
        (v_fld_fs_shift,     v_sec_fs_insp, 'shift',           'Shift',           'DROPDOWN', true,  NULL,  2, 1),
        (v_fld_fs_area,      v_sec_fs_insp, 'area',            'Area',            'DROPDOWN', true,  NULL,  3, 1),
        (v_fld_fs_fridge,    v_sec_fs_temp, 'fridge_temp',     'Fridge Temp',     'NUMBER',   true,  'degrees C',  0, 1),
        (v_fld_fs_freezer,   v_sec_fs_temp, 'freezer_temp',    'Freezer Temp',    'NUMBER',   true,  'degrees C',  1, 1),
        (v_fld_fs_hothold,   v_sec_fs_temp, 'hot_holding_temp','Hot Holding Temp','NUMBER',   false, 'degrees C',  2, 1),
        (v_fld_fs_handwash,  v_sec_fs_hyg,  'hand_washing_ok', 'Hand Washing OK', 'CHECKBOX', false, NULL,  0, 1),
        (v_fld_fs_surfaces,  v_sec_fs_hyg,  'surfaces_clean',  'Surfaces Clean',  'CHECKBOX', false, NULL,  1, 1),
        (v_fld_fs_pest,      v_sec_fs_hyg,  'pest_control_ok', 'Pest Control OK', 'CHECKBOX', false, NULL,  2, 1),
        (v_fld_fs_labeling,  v_sec_fs_hyg,  'food_labeling_ok','Food Labeling OK','CHECKBOX', false, NULL,  3, 1),
        (v_fld_fs_result,    v_sec_fs_hyg,  'overall_result',  'Overall Result',  'DROPDOWN', true,  NULL,  4, 1),
        (v_fld_fs_actions,   v_sec_fs_hyg,  'corrective_actions','Corrective Actions','TEXTAREA', false, 'Describe any corrective actions taken', 5, 2)
    ON CONFLICT (section_id, field_name) DO NOTHING;

    -- ---- Supplier Evaluation Form ----
    INSERT INTO lc_field_definitions (id, section_id, field_name, field_label, field_type, is_required, placeholder, sort_order, column_span) VALUES
        (v_fld_se_date,      v_sec_se_info, 'evaluation_date',    'Evaluation Date',    'DATE',     true,  NULL,              0, 1),
        (v_fld_se_supplier,  v_sec_se_info, 'supplier_name',      'Supplier Name',      'TEXT',     true,  'Supplier company name', 1, 1),
        (v_fld_se_evaluator, v_sec_se_info, 'evaluator',          'Evaluator',          'TEXT',     true,  'Your name',       2, 1),
        (v_fld_se_delivery,  v_sec_se_eval, 'delivery_score',     'Delivery Score',     'DROPDOWN', true,  NULL,              0, 1),
        (v_fld_se_quality,   v_sec_se_eval, 'quality_score',      'Quality Score',      'DROPDOWN', true,  NULL,              1, 1),
        (v_fld_se_price,     v_sec_se_eval, 'price_score',        'Price Score',        'DROPDOWN', true,  NULL,              2, 1),
        (v_fld_se_comm,      v_sec_se_eval, 'communication_score','Communication Score','DROPDOWN', true,  NULL,              3, 1),
        (v_fld_se_overall,   v_sec_se_summ, 'overall_rating',     'Overall Rating',     'DROPDOWN', true,  NULL,              0, 1),
        (v_fld_se_recommend, v_sec_se_summ, 'recommendation',     'Recommendation',     'DROPDOWN', true,  NULL,              1, 1),
        (v_fld_se_notes,     v_sec_se_summ, 'notes',              'Notes',              'TEXTAREA', false, 'Additional comments', 2, 1)
    ON CONFLICT (section_id, field_name) DO NOTHING;

    -- ---- Employee Training Record ----
    INSERT INTO lc_field_definitions (id, section_id, field_name, field_label, field_type, is_required, placeholder, sort_order, column_span) VALUES
        (v_fld_tr_date,    v_sec_tr_info, 'training_date',      'Training Date',      'DATE',     true,  NULL,                   0, 1),
        (v_fld_tr_title,   v_sec_tr_info, 'training_title',     'Training Title',     'TEXT',     true,  'e.g. HACCP Level 2',   1, 1),
        (v_fld_tr_trainer, v_sec_tr_info, 'trainer',            'Trainer',            'TEXT',     true,  'Trainer name',         2, 1),
        (v_fld_tr_type,    v_sec_tr_info, 'training_type',      'Training Type',      'DROPDOWN', true,  NULL,                   3, 1),
        (v_fld_tr_duration,v_sec_tr_info, 'duration_hours',     'Duration (Hours)',   'NUMBER',   true,  '0',                    4, 1),
        (v_fld_tr_empname, v_sec_tr_att,  'employee_name',      'Employee Name',      'TEXT',     true,  'Employee full name',   0, 1),
        (v_fld_tr_dept,    v_sec_tr_att,  'department',         'Department',         'DROPDOWN', true,  NULL,                   1, 1),
        (v_fld_tr_score,   v_sec_tr_att,  'score',             'Score',              'NUMBER',   false, '0-100',                2, 1),
        (v_fld_tr_passed,  v_sec_tr_att,  'passed',            'Passed',             'CHECKBOX', false, NULL,                   3, 1),
        (v_fld_tr_cert,    v_sec_tr_att,  'certificate_number','Certificate Number', 'TEXT',     false, 'e.g. CERT-2026-001',   4, 1),
        (v_fld_tr_expiry,  v_sec_tr_att,  'expiry_date',       'Expiry Date',        'DATE',     false, NULL,                   5, 1)
    ON CONFLICT (section_id, field_name) DO NOTHING;

    -- ============================================================
    -- 6. FIELD OPTIONS (DROPDOWN values)
    -- ============================================================

    -- Material Request: department
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_mr_dept, 'Production',  'production',  0),
        (v_fld_mr_dept, 'Kitchen',     'kitchen',     1),
        (v_fld_mr_dept, 'Warehouse',   'warehouse',   2),
        (v_fld_mr_dept, 'Admin',       'admin',       3)
    ON CONFLICT DO NOTHING;

    -- Material Request: material_type
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_mr_mattype, 'Raw Material',    'raw_material',    0),
        (v_fld_mr_mattype, 'Packaging',       'packaging',       1),
        (v_fld_mr_mattype, 'Finished Good',   'finished_good',   2),
        (v_fld_mr_mattype, 'Semi-finished',   'semi_finished',   3)
    ON CONFLICT DO NOTHING;

    -- Material Request: unit
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_mr_unit, 'KG',  'KG',  0),
        (v_fld_mr_unit, 'LTR', 'LTR', 1),
        (v_fld_mr_unit, 'PC',  'PC',  2),
        (v_fld_mr_unit, 'BOX', 'BOX', 3),
        (v_fld_mr_unit, 'BTL', 'BTL', 4)
    ON CONFLICT DO NOTHING;

    -- Material Request: urgency
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_mr_urgency, 'Low',      'low',      0),
        (v_fld_mr_urgency, 'Medium',   'medium',   1),
        (v_fld_mr_urgency, 'High',     'high',     2),
        (v_fld_mr_urgency, 'Critical', 'critical', 3)
    ON CONFLICT DO NOTHING;

    -- Customer Feedback: rating
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_cf_rating, '1 - Terrible',  '1', 0),
        (v_fld_cf_rating, '2 - Poor',      '2', 1),
        (v_fld_cf_rating, '3 - Average',   '3', 2),
        (v_fld_cf_rating, '4 - Good',      '4', 3),
        (v_fld_cf_rating, '5 - Excellent', '5', 4)
    ON CONFLICT DO NOTHING;

    -- Customer Feedback: category
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_cf_category, 'Food Quality', 'food_quality', 0),
        (v_fld_cf_category, 'Service',      'service',      1),
        (v_fld_cf_category, 'Delivery',     'delivery',     2),
        (v_fld_cf_category, 'Packaging',    'packaging',    3),
        (v_fld_cf_category, 'Price',        'price',        4)
    ON CONFLICT DO NOTHING;

    -- Equipment Maintenance: location
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_eq_location, 'Kitchen A',           'kitchen_a',      0),
        (v_fld_eq_location, 'Kitchen B',           'kitchen_b',      1),
        (v_fld_eq_location, 'Production Line 1',   'prod_line_1',    2),
        (v_fld_eq_location, 'Production Line 2',   'prod_line_2',    3),
        (v_fld_eq_location, 'Warehouse',           'warehouse',      4)
    ON CONFLICT DO NOTHING;

    -- Equipment Maintenance: maintenance_type
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_eq_type, 'Preventive',  'preventive',  0),
        (v_fld_eq_type, 'Corrective',  'corrective',  1),
        (v_fld_eq_type, 'Emergency',   'emergency',   2),
        (v_fld_eq_type, 'Inspection',  'inspection',  3)
    ON CONFLICT DO NOTHING;

    -- Equipment Maintenance: status
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_eq_status, 'Completed',     'completed',     0),
        (v_fld_eq_status, 'In Progress',   'in_progress',   1),
        (v_fld_eq_status, 'Pending Parts', 'pending_parts', 2)
    ON CONFLICT DO NOTHING;

    -- Food Safety: shift
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_fs_shift, 'Morning',   'morning',   0),
        (v_fld_fs_shift, 'Afternoon', 'afternoon', 1),
        (v_fld_fs_shift, 'Night',     'night',     2)
    ON CONFLICT DO NOTHING;

    -- Food Safety: area
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_fs_area, 'Kitchen',   'kitchen',   0),
        (v_fld_fs_area, 'Storage',   'storage',   1),
        (v_fld_fs_area, 'Receiving', 'receiving', 2),
        (v_fld_fs_area, 'Shipping',  'shipping',  3)
    ON CONFLICT DO NOTHING;

    -- Food Safety: overall_result
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_fs_result, 'Pass',             'pass',             0),
        (v_fld_fs_result, 'Fail',             'fail',             1),
        (v_fld_fs_result, 'Conditional Pass', 'conditional_pass', 2)
    ON CONFLICT DO NOTHING;

    -- Supplier Evaluation: delivery_score, quality_score, price_score, communication_score (all 1-5)
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_se_delivery, '1', '1', 0), (v_fld_se_delivery, '2', '2', 1), (v_fld_se_delivery, '3', '3', 2),
        (v_fld_se_delivery, '4', '4', 3), (v_fld_se_delivery, '5', '5', 4),
        (v_fld_se_quality,  '1', '1', 0), (v_fld_se_quality,  '2', '2', 1), (v_fld_se_quality,  '3', '3', 2),
        (v_fld_se_quality,  '4', '4', 3), (v_fld_se_quality,  '5', '5', 4),
        (v_fld_se_price,    '1', '1', 0), (v_fld_se_price,    '2', '2', 1), (v_fld_se_price,    '3', '3', 2),
        (v_fld_se_price,    '4', '4', 3), (v_fld_se_price,    '5', '5', 4),
        (v_fld_se_comm,     '1', '1', 0), (v_fld_se_comm,     '2', '2', 1), (v_fld_se_comm,     '3', '3', 2),
        (v_fld_se_comm,     '4', '4', 3), (v_fld_se_comm,     '5', '5', 4)
    ON CONFLICT DO NOTHING;

    -- Supplier Evaluation: overall_rating
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_se_overall, 'A - Excellent',      'A', 0),
        (v_fld_se_overall, 'B - Good',           'B', 1),
        (v_fld_se_overall, 'C - Average',        'C', 2),
        (v_fld_se_overall, 'D - Below Average',  'D', 3),
        (v_fld_se_overall, 'F - Unacceptable',   'F', 4)
    ON CONFLICT DO NOTHING;

    -- Supplier Evaluation: recommendation
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_se_recommend, 'Continue',  'continue',  0),
        (v_fld_se_recommend, 'Probation', 'probation', 1),
        (v_fld_se_recommend, 'Terminate', 'terminate', 2)
    ON CONFLICT DO NOTHING;

    -- Employee Training: training_type
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_tr_type, 'Food Safety',          'food_safety',      0),
        (v_fld_tr_type, 'Equipment Operation',  'equipment_op',     1),
        (v_fld_tr_type, 'Quality Standards',    'quality_standards', 2),
        (v_fld_tr_type, 'Health & Safety',      'health_safety',    3),
        (v_fld_tr_type, 'Customer Service',     'customer_service', 4)
    ON CONFLICT DO NOTHING;

    -- Employee Training: department
    INSERT INTO lc_field_options (field_id, option_label, option_value, sort_order) VALUES
        (v_fld_tr_dept, 'Production', 'production', 0),
        (v_fld_tr_dept, 'Kitchen',    'kitchen',    1),
        (v_fld_tr_dept, 'Sales',      'sales',      2),
        (v_fld_tr_dept, 'Warehouse',  'warehouse',  3),
        (v_fld_tr_dept, 'Quality',    'quality',    4),
        (v_fld_tr_dept, 'Admin',      'admin',      5)
    ON CONFLICT DO NOTHING;

    -- ============================================================
    -- 7. SAMPLE DATA (lc_operation_data)
    -- ============================================================

    -- Material Request: 4 sample records
    INSERT INTO lc_operation_data (operation_id, data, created_by, created_at) VALUES
        (v_op_mat_req_id, '{"request_date":"2026-03-10","requester_name":"Chen Wei-Lin","department":"kitchen","material_name":"Extra Virgin Olive Oil","material_type":"raw_material","quantity_needed":50,"unit":"LTR","urgency":"high","reason":"Current stock running low; needed for new Mediterranean menu launch next week.","approved_by":"Manager Wang"}', v_admin_id, '2026-03-10 09:15:00+08'),
        (v_op_mat_req_id, '{"request_date":"2026-03-12","requester_name":"Liu Mei-Hua","department":"production","material_name":"Biodegradable Food Containers (750ml)","material_type":"packaging","quantity_needed":2000,"unit":"PC","urgency":"medium","reason":"Switching from plastic to eco-friendly packaging per company sustainability policy.","approved_by":""}', v_admin_id, '2026-03-12 14:30:00+08'),
        (v_op_mat_req_id, '{"request_date":"2026-03-14","requester_name":"Zhang Yi","department":"warehouse","material_name":"Japanese Wagyu Beef A5","material_type":"raw_material","quantity_needed":20,"unit":"KG","urgency":"critical","reason":"VIP catering event on March 18. Must arrive by March 16.","approved_by":"Director Lee"}', v_admin_id, '2026-03-14 08:00:00+08'),
        (v_op_mat_req_id, '{"request_date":"2026-03-15","requester_name":"Huang Jia-Wen","department":"admin","material_name":"Thermal Receipt Paper Rolls","material_type":"packaging","quantity_needed":100,"unit":"BOX","urgency":"low","reason":"Routine restocking for POS terminals across all locations.","approved_by":""}', v_admin_id, '2026-03-15 11:20:00+08')
    ON CONFLICT DO NOTHING;

    -- Customer Feedback: 4 sample records
    INSERT INTO lc_operation_data (operation_id, data, created_by, created_at) VALUES
        (v_op_cust_fb_id, '{"feedback_date":"2026-03-08","customer_name":"Grand Hyatt Taipei","order_number":"SO-00000012","rating":"5","category":"food_quality","comments":"The wagyu bento boxes were outstanding. Our guests at the corporate luncheon were very impressed. Will order again for next quarter event.","follow_up_required":false,"contact_email":"events@grandhyatt.tw"}', v_admin_id, '2026-03-08 16:45:00+08'),
        (v_op_cust_fb_id, '{"feedback_date":"2026-03-11","customer_name":"FamilyMart Central Kitchen","order_number":"SO-00000015","rating":"3","category":"delivery","comments":"Delivery was 2 hours late on March 10. Temperature of chilled items was 8C on arrival instead of required 4C. Need to investigate cold chain process.","follow_up_required":true,"contact_email":"procurement@familymart.tw"}', v_admin_id, '2026-03-11 10:00:00+08'),
        (v_op_cust_fb_id, '{"feedback_date":"2026-03-13","customer_name":"Shin Kong Mitsukoshi Food Court","order_number":"SO-00000016","rating":"4","category":"packaging","comments":"Food quality is great but the new biodegradable containers are leaking slightly with soup items. Please consider a better seal.","follow_up_required":true,"contact_email":"food@skm.tw"}', v_admin_id, '2026-03-13 14:20:00+08'),
        (v_op_cust_fb_id, '{"feedback_date":"2026-03-15","customer_name":"Taiwan University Cafeteria","order_number":"SO-00000017","rating":"4","category":"price","comments":"Good value for the quality. Students appreciate the healthy options. Would like to discuss volume discount for next semester contract.","follow_up_required":false,"contact_email":"cafeteria@ntu.edu.tw"}', v_admin_id, '2026-03-15 09:30:00+08')
    ON CONFLICT DO NOTHING;

    -- Equipment Maintenance Log: 4 sample records
    INSERT INTO lc_operation_data (operation_id, data, created_by, created_at) VALUES
        (v_op_equip_id, '{"maintenance_date":"2026-03-05","equipment_name":"Commercial Blast Chiller","equipment_id":"EQ-BC-001","location":"kitchen_a","maintenance_type":"preventive","technician":"Technician Tsai","description":"Quarterly preventive maintenance. Cleaned condenser coils, checked refrigerant levels, calibrated temperature sensors. All readings within specification.","parts_replaced":"Air filter x1","downtime_hours":3,"next_scheduled":"2026-06-05","status":"completed"}', v_admin_id, '2026-03-05 07:00:00+08'),
        (v_op_equip_id, '{"maintenance_date":"2026-03-09","equipment_name":"Vacuum Sealing Machine VS-200","equipment_id":"EQ-VS-003","location":"prod_line_1","maintenance_type":"corrective","technician":"Technician Lin","description":"Machine was producing inconsistent seals. Replaced heating element and adjusted pressure settings. Ran 50 test seals, all passed.","parts_replaced":"Heating element, Teflon tape","downtime_hours":5,"next_scheduled":"2026-04-09","status":"completed"}', v_admin_id, '2026-03-09 13:00:00+08'),
        (v_op_equip_id, '{"maintenance_date":"2026-03-14","equipment_name":"Industrial Rice Cooker RC-50","equipment_id":"EQ-RC-002","location":"kitchen_b","maintenance_type":"emergency","technician":"Technician Wang","description":"Unit tripped breaker during morning prep. Found burnt wiring in control panel. Awaiting replacement control board from manufacturer (ETA 3 days).","parts_replaced":"","downtime_hours":0,"next_scheduled":"2026-03-17","status":"pending_parts"}', v_admin_id, '2026-03-14 06:30:00+08'),
        (v_op_equip_id, '{"maintenance_date":"2026-03-16","equipment_name":"Walk-in Freezer Unit A","equipment_id":"EQ-WF-001","location":"warehouse","maintenance_type":"inspection","technician":"Technician Tsai","description":"Monthly inspection. Temperature stable at -18C. Door seal integrity good. Defrost cycle operating normally. Minor ice buildup on rear panel noted - will monitor.","parts_replaced":"","downtime_hours":1,"next_scheduled":"2026-04-16","status":"completed"}', v_admin_id, '2026-03-16 08:00:00+08')
    ON CONFLICT DO NOTHING;

    -- Food Safety Checklist: 5 sample records
    INSERT INTO lc_operation_data (operation_id, data, created_by, created_at) VALUES
        (v_op_food_safe_id, '{"inspection_date":"2026-03-14","inspector_name":"Inspector Chen","shift":"morning","area":"kitchen","fridge_temp":3.2,"freezer_temp":-18.5,"hot_holding_temp":65,"hand_washing_ok":true,"surfaces_clean":true,"pest_control_ok":true,"food_labeling_ok":true,"overall_result":"pass","corrective_actions":""}', v_admin_id, '2026-03-14 06:30:00+08'),
        (v_op_food_safe_id, '{"inspection_date":"2026-03-14","inspector_name":"Inspector Wu","shift":"afternoon","area":"storage","fridge_temp":4.8,"freezer_temp":-17.2,"hot_holding_temp":null,"hand_washing_ok":true,"surfaces_clean":false,"pest_control_ok":true,"food_labeling_ok":false,"overall_result":"conditional_pass","corrective_actions":"Storage room B shelf 3 had dust accumulation. Two items in dry storage missing expiry labels. Assigned cleaning crew for immediate action. Re-labeling completed by 15:00."}', v_admin_id, '2026-03-14 13:00:00+08'),
        (v_op_food_safe_id, '{"inspection_date":"2026-03-15","inspector_name":"Inspector Chen","shift":"morning","area":"receiving","fridge_temp":3.5,"freezer_temp":-19.0,"hot_holding_temp":null,"hand_washing_ok":true,"surfaces_clean":true,"pest_control_ok":true,"food_labeling_ok":true,"overall_result":"pass","corrective_actions":""}', v_admin_id, '2026-03-15 07:00:00+08'),
        (v_op_food_safe_id, '{"inspection_date":"2026-03-15","inspector_name":"Inspector Wu","shift":"afternoon","area":"kitchen","fridge_temp":5.1,"freezer_temp":-16.8,"hot_holding_temp":62,"hand_washing_ok":true,"surfaces_clean":true,"pest_control_ok":false,"food_labeling_ok":true,"overall_result":"fail","corrective_actions":"Fridge #3 temperature above 5C threshold - adjusted thermostat. Freezer slightly warm - checked door seal. Found evidence of rodent activity near loading dock. Called pest control for emergency treatment scheduled March 16 morning."}', v_admin_id, '2026-03-15 14:00:00+08'),
        (v_op_food_safe_id, '{"inspection_date":"2026-03-16","inspector_name":"Inspector Chen","shift":"morning","area":"kitchen","fridge_temp":2.8,"freezer_temp":-18.9,"hot_holding_temp":67,"hand_washing_ok":true,"surfaces_clean":true,"pest_control_ok":true,"food_labeling_ok":true,"overall_result":"pass","corrective_actions":"Post pest-control follow-up: no further evidence found. Fridge #3 now reading correctly after thermostat adjustment."}', v_admin_id, '2026-03-16 06:45:00+08')
    ON CONFLICT DO NOTHING;

    -- Supplier Evaluation: 3 sample records
    INSERT INTO lc_operation_data (operation_id, data, created_by, created_at) VALUES
        (v_op_supplier_id, '{"evaluation_date":"2026-03-01","supplier_name":"Fresh Harvest Farms Co.","evaluator":"Manager Wang","delivery_score":"5","quality_score":"5","price_score":"4","communication_score":"4","overall_rating":"A","recommendation":"continue","notes":"Outstanding organic vegetable supplier. Consistently delivers on time with excellent freshness. Slightly above market price but justified by quality. Recommend extending contract for another year."}', v_admin_id, '2026-03-01 10:00:00+08'),
        (v_op_supplier_id, '{"evaluation_date":"2026-03-05","supplier_name":"Pacific Seafood Trading","evaluator":"Manager Wang","delivery_score":"3","quality_score":"4","price_score":"3","communication_score":"2","overall_rating":"C","recommendation":"probation","notes":"Two late deliveries in February. Quality is acceptable but communication is poor - often unreachable by phone. Placed on 90-day probation. Must improve delivery reliability and responsiveness."}', v_admin_id, '2026-03-05 15:00:00+08'),
        (v_op_supplier_id, '{"evaluation_date":"2026-03-12","supplier_name":"Taiwan Premium Meats","evaluator":"Director Lee","delivery_score":"4","quality_score":"5","price_score":"3","communication_score":"5","overall_rating":"B","recommendation":"continue","notes":"Excellent meat quality and very responsive team. Pricing is premium but consistent with market for A5 grade products. Key supplier for our high-end catering line."}', v_admin_id, '2026-03-12 11:00:00+08')
    ON CONFLICT DO NOTHING;

    -- Employee Training Record: 5 sample records
    INSERT INTO lc_operation_data (operation_id, data, created_by, created_at) VALUES
        (v_op_training_id, '{"training_date":"2026-03-03","training_title":"HACCP Level 2 Certification","trainer":"Dr. Lin Mei-Ling","training_type":"food_safety","duration_hours":8,"employee_name":"Chen Wei-Lin","department":"kitchen","score":92,"passed":true,"certificate_number":"HACCP-2026-0341","expiry_date":"2027-03-03"}', v_admin_id, '2026-03-03 09:00:00+08'),
        (v_op_training_id, '{"training_date":"2026-03-03","training_title":"HACCP Level 2 Certification","trainer":"Dr. Lin Mei-Ling","training_type":"food_safety","duration_hours":8,"employee_name":"Liu Mei-Hua","department":"production","score":88,"passed":true,"certificate_number":"HACCP-2026-0342","expiry_date":"2027-03-03"}', v_admin_id, '2026-03-03 09:00:00+08'),
        (v_op_training_id, '{"training_date":"2026-03-07","training_title":"Blast Chiller & Vacuum Sealer Operation","trainer":"Technician Tsai","training_type":"equipment_op","duration_hours":4,"employee_name":"Zhang Yi","department":"warehouse","score":78,"passed":true,"certificate_number":"","expiry_date":""}', v_admin_id, '2026-03-07 13:00:00+08'),
        (v_op_training_id, '{"training_date":"2026-03-10","training_title":"Fire Safety & Emergency Evacuation Drill","trainer":"Fire Captain Hsu","training_type":"health_safety","duration_hours":3,"employee_name":"Huang Jia-Wen","department":"admin","score":95,"passed":true,"certificate_number":"FS-2026-087","expiry_date":"2027-03-10"}', v_admin_id, '2026-03-10 14:00:00+08'),
        (v_op_training_id, '{"training_date":"2026-03-14","training_title":"Customer Complaint Handling Workshop","trainer":"Consultant Yang","training_type":"customer_service","duration_hours":6,"employee_name":"Lin Shu-Fen","department":"sales","score":85,"passed":true,"certificate_number":"","expiry_date":""}', v_admin_id, '2026-03-14 09:00:00+08')
    ON CONFLICT DO NOTHING;

    -- ============================================================
    -- 8. RELEASES
    -- ============================================================
    INSERT INTO lc_releases (id, release_number, operation_id, version, title, description, status, submitted_by, reviewed_by, review_notes, form_snapshot, submitted_at, reviewed_at, released_at)
    VALUES
        (v_rel_1, 'LCR00000001', v_op_mat_req_id, 1,
         'Material Request Form v1.0',
         'Initial release of the Material Request Form with basic fields for requesting inventory additions.',
         'RELEASED', v_admin_id, v_admin_id, 'Approved. Form covers all required fields for MM integration.',
         '{"operation":"Material Request Form","version":1,"sections":2,"fields":10}',
         '2026-02-20 10:00:00+08', '2026-02-21 09:00:00+08', '2026-02-21 09:30:00+08'),
        (v_rel_2, 'LCR00000002', v_op_food_safe_id, 1,
         'Food Safety Checklist v1.0',
         'Initial release of the daily Food Safety Checklist covering temperature checks and hygiene compliance.',
         'RELEASED', v_admin_id, v_admin_id, 'Good coverage of food safety requirements. Approved for production use.',
         '{"operation":"Food Safety Checklist","version":1,"sections":3,"fields":13}',
         '2026-02-25 14:00:00+08', '2026-02-26 10:00:00+08', '2026-02-26 10:30:00+08'),
        (v_rel_3, 'LCR00000003', v_op_mat_req_id, 2,
         'Material Request Form v2.0',
         'Added urgency field and improved validation. Reason field now spans full width for better usability.',
         'SUBMITTED', v_admin_id, NULL, NULL,
         '{"operation":"Material Request Form","version":2,"sections":2,"fields":10,"changes":["Added urgency dropdown","Reason field column_span=2"]}',
         '2026-03-15 16:00:00+08', NULL, NULL)
    ON CONFLICT (id) DO NOTHING;

    -- ============================================================
    -- 9. FEEDBACK TICKETS
    -- ============================================================
    INSERT INTO lc_feedback (id, ticket_number, operation_id, feedback_type, title, description, priority, status, assigned_to, submitted_by)
    VALUES
        (v_fb_1, 'TKT00000001', v_op_food_safe_id, 'BUG',
         'Temperature field accepts negative values beyond -50',
         'The freezer_temp field allows entering values like -999 which is physically impossible. Should have a minimum value of -40C for standard commercial freezers.',
         'MEDIUM', 'RESOLVED', v_admin_id, v_admin_id),
        (v_fb_2, 'TKT00000002', v_op_mat_req_id, 'FEATURE_REQUEST',
         'Add file attachment for material specification sheet',
         'When requesting new materials, it would be helpful to attach a PDF specification sheet or product datasheet from the supplier. This helps the procurement team evaluate the request faster.',
         'HIGH', 'OPEN', v_admin_id, v_admin_id),
        (v_fb_3, 'TKT00000003', v_op_cust_fb_id, 'IMPROVEMENT',
         'Auto-fill customer name from order number lookup',
         'When the user enters an order number (SO-XXXXXXXX), the customer_name field should auto-populate by looking up the sales order. This reduces data entry errors and saves time.',
         'MEDIUM', 'IN_PROGRESS', v_admin_id, v_admin_id),
        (v_fb_4, 'TKT00000004', v_op_equip_id, 'BUG',
         'Next Scheduled date allows past dates',
         'The next_scheduled field in the Equipment Maintenance Log should not allow dates in the past. Currently you can set it to any date. Need a validation rule to enforce future dates only.',
         'LOW', 'OPEN', NULL, v_admin_id)
    ON CONFLICT (id) DO NOTHING;

    -- Link resolved feedback to release
    INSERT INTO lc_release_feedback_links (release_id, feedback_id)
    VALUES (v_rel_2, v_fb_1)
    ON CONFLICT (release_id, feedback_id) DO NOTHING;

    -- ============================================================
    -- 10. FEEDBACK COMMENTS
    -- ============================================================
    INSERT INTO lc_feedback_comments (feedback_id, user_id, content, created_at) VALUES
        (v_fb_1, v_admin_id, 'Confirmed the issue. Will add min_value=-40 and max_value=100 constraints to all temperature fields in the Food Safety Checklist.', '2026-03-02 11:00:00+08'),
        (v_fb_1, v_admin_id, 'Fixed in Food Safety Checklist v1.0 release. Temperature fields now have proper range validation.', '2026-03-03 09:00:00+08'),
        (v_fb_2, v_admin_id, 'Good suggestion. We will add a FILE_UPLOAD field to the Material Request Form in the next release. Need to confirm max file size with IT.', '2026-03-14 10:00:00+08'),
        (v_fb_3, v_admin_id, 'This requires a LOOKUP_WINDOW field type integration with the SD module. Starting analysis on how to query sd_sales_orders from the low-code form.', '2026-03-12 15:30:00+08'),
        (v_fb_3, v_admin_id, 'Prototype working. Using data_source_sql to query sd_sales_orders JOIN sd_customers. Will add to next Customer Feedback Form release.', '2026-03-16 11:00:00+08')
    ON CONFLICT DO NOTHING;

    -- ============================================================
    -- 11. DEV JOURNAL
    -- ============================================================
    INSERT INTO lc_dev_journal (operation_id, changed_by, change_type, entity_type, diff_summary, version, created_at) VALUES
        (v_op_mat_req_id, v_admin_id, 'FORM_CREATED', 'operation',
         'Created Material Request Form with 2 sections and 10 fields for MM module.', 1, '2026-02-18 10:00:00+08'),
        (v_op_food_safe_id, v_admin_id, 'FORM_CREATED', 'operation',
         'Created Food Safety Checklist with 3 sections (Inspection, Temperature, Hygiene) and 13 fields.', 1, '2026-02-22 14:00:00+08'),
        (v_op_mat_req_id, v_admin_id, 'PUBLISHED', 'operation',
         'Published Material Request Form v1 after release LCR00000001 approval.', 1, '2026-02-21 09:30:00+08'),
        (v_op_food_safe_id, v_admin_id, 'PUBLISHED', 'operation',
         'Published Food Safety Checklist v1 after release LCR00000002 approval.', 1, '2026-02-26 10:30:00+08'),
        (v_op_mat_req_id, v_admin_id, 'FIELD_UPDATED', 'field',
         'Updated reason field: column_span changed from 1 to 2 for better layout.', 2, '2026-03-14 16:00:00+08'),
        (v_op_cust_fb_id, v_admin_id, 'FORM_CREATED', 'operation',
         'Created Customer Feedback Form with 2 sections and 8 fields linked to SD module.', 1, '2026-03-01 11:00:00+08')
    ON CONFLICT DO NOTHING;

    -- ============================================================
    -- 12. UPDATE NUMBER RANGES
    -- ============================================================
    UPDATE number_ranges SET current_number = GREATEST(current_number, 2) WHERE object_type = 'LCP';
    UPDATE number_ranges SET current_number = GREATEST(current_number, 8) WHERE object_type = 'LCO';
    UPDATE number_ranges SET current_number = GREATEST(current_number, 3) WHERE object_type = 'LCR';
    UPDATE number_ranges SET current_number = GREATEST(current_number, 4) WHERE object_type = 'TKT';

END;
$$;

COMMIT;
