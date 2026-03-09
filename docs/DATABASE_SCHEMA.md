# TasteByte ERP - Database Schema Reference

> PostgreSQL 17 | Database: TastyByte | Port: 5432
> 74 Tables | 18 Migrations
> Last Updated: 2026-02-21

---

## Table of Contents

1. [Foundation & Core](#1-foundation--core-8-tables)
2. [FI - Financial Accounting](#2-fi---financial-accounting-9-tables)
3. [MM - Materials Management](#3-mm---materials-management-6-tables)
4. [SD - Sales & Distribution](#4-sd---sales--distribution-5-tables)
5. [PP - Production Planning](#5-pp---production-planning-5-tables)
6. [HR - Human Resources](#6-hr---human-resources-4-tables)
7. [WM - Warehouse Management](#7-wm---warehouse-management-6-tables)
8. [QM - Quality Management](#8-qm---quality-management-3-tables)
9. [CO - Controlling](#9-co---controlling-4-tables)
10. [Low-Code Platform](#10-low-code-platform-24-tables)
11. [Workflow Infrastructure](#11-workflow-infrastructure-1-table)

---

## 1. Foundation & Core (8 tables)

### `_schema_versions`
Migration tracking table.

| Column | Type | Constraints |
|--------|------|------------|
| version | INT | PRIMARY KEY |
| name | VARCHAR(255) | |
| checksum | VARCHAR(64) | |
| applied_at | TIMESTAMPTZ | DEFAULT NOW() |

### `users`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY, DEFAULT gen_random_uuid() |
| username | VARCHAR(100) | UNIQUE, NOT NULL |
| email | VARCHAR(255) | UNIQUE, NOT NULL |
| password_hash | VARCHAR(255) | NOT NULL |
| display_name | VARCHAR(200) | |
| is_active | BOOLEAN | DEFAULT true |
| created_at | TIMESTAMPTZ | DEFAULT NOW() |
| updated_at | TIMESTAMPTZ | DEFAULT NOW() |

### `roles`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| name | VARCHAR(100) | UNIQUE, NOT NULL |
| description | TEXT | |
| is_system | BOOLEAN | DEFAULT false |
| created_at | TIMESTAMPTZ | DEFAULT NOW() |

### `permissions`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| module | VARCHAR(50) | NOT NULL |
| action | VARCHAR(50) | NOT NULL |
| description | TEXT | |

UNIQUE: (module, action)

### `role_permissions`

| Column | Type | Constraints |
|--------|------|------------|
| role_id | UUID | FK -> roles(id) CASCADE |
| permission_id | UUID | FK -> permissions(id) CASCADE |

PK: (role_id, permission_id)

### `user_roles`

| Column | Type | Constraints |
|--------|------|------------|
| user_id | UUID | FK -> users(id) CASCADE |
| role_id | UUID | FK -> roles(id) CASCADE |

PK: (user_id, role_id)

### `audit_log`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| table_name | VARCHAR(100) | NOT NULL |
| record_id | UUID | |
| action | VARCHAR(20) | NOT NULL |
| old_values | JSONB | |
| new_values | JSONB | |
| changed_by | UUID | FK -> users(id) |
| changed_at | TIMESTAMPTZ | DEFAULT NOW() |

Indexes: `idx_audit_log_entity`, `idx_audit_log_user`, `idx_audit_log_time`

### `number_ranges`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| object_type | VARCHAR(50) | UNIQUE, NOT NULL |
| prefix | VARCHAR(10) | NOT NULL |
| current_number | BIGINT | DEFAULT 0 |
| pad_length | INT | DEFAULT 10 |

---

## 2. FI - Financial Accounting (9 tables)

### `fi_company_codes`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| code | VARCHAR(10) | UNIQUE, NOT NULL |
| name | VARCHAR(200) | NOT NULL |
| currency | VARCHAR(3) | DEFAULT 'TWD' |
| country | VARCHAR(2) | |

### `fi_fiscal_years`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| company_code_id | UUID | FK -> fi_company_codes(id) |
| year | INT | NOT NULL |
| start_date | DATE | NOT NULL |
| end_date | DATE | NOT NULL |
| is_closed | BOOLEAN | DEFAULT false |

### `fi_fiscal_periods`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| fiscal_year_id | UUID | FK -> fi_fiscal_years(id) |
| period | INT | NOT NULL |
| start_date | DATE | NOT NULL |
| end_date | DATE | NOT NULL |
| is_closed | BOOLEAN | DEFAULT false |

### `fi_account_groups`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| code | VARCHAR(20) | NOT NULL |
| name | VARCHAR(200) | NOT NULL |
| account_type | VARCHAR(20) | CHECK: ASSET, LIABILITY, EQUITY, REVENUE, EXPENSE |

### `fi_accounts`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| account_number | VARCHAR(20) | UNIQUE, NOT NULL |
| name | VARCHAR(200) | NOT NULL |
| account_group_id | UUID | FK -> fi_account_groups(id) |
| account_type | VARCHAR(20) | CHECK: ASSET, LIABILITY, EQUITY, REVENUE, EXPENSE |
| is_reconciliation | BOOLEAN | DEFAULT false |
| is_active | BOOLEAN | DEFAULT true |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

### `fi_journal_entries`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| document_number | VARCHAR(30) | UNIQUE, NOT NULL |
| company_code_id | UUID | FK -> fi_company_codes(id) |
| fiscal_year | INT | |
| fiscal_period | INT | |
| posting_date | DATE | NOT NULL |
| document_date | DATE | NOT NULL |
| reference | VARCHAR(100) | |
| description | TEXT | |
| status | VARCHAR(20) | CHECK: DRAFT, POSTED |
| created_by | UUID | FK -> users(id) |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

### `fi_journal_items`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| journal_entry_id | UUID | FK -> fi_journal_entries(id) CASCADE |
| line_number | INT | |
| account_id | UUID | FK -> fi_accounts(id) |
| debit_amount | DECIMAL(18,4) | DEFAULT 0 |
| credit_amount | DECIMAL(18,4) | DEFAULT 0 |
| cost_center_id | UUID | |
| description | TEXT | |

### `fi_ar_invoices`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| document_number | VARCHAR(30) | UNIQUE |
| customer_id | UUID | |
| invoice_date | DATE | |
| due_date | DATE | |
| total_amount | DECIMAL(18,4) | |
| paid_amount | DECIMAL(18,4) | DEFAULT 0 |
| status | VARCHAR(20) | CHECK: OPEN, PAID, CANCELLED |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

### `fi_ap_invoices`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| document_number | VARCHAR(30) | UNIQUE |
| vendor_id | UUID | |
| invoice_date | DATE | |
| due_date | DATE | |
| total_amount | DECIMAL(18,4) | |
| paid_amount | DECIMAL(18,4) | DEFAULT 0 |
| status | VARCHAR(20) | CHECK: OPEN, PAID, CANCELLED |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

---

## 3. MM - Materials Management (6 tables)

### `mm_uom`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| code | VARCHAR(10) | UNIQUE, NOT NULL |
| name | VARCHAR(50) | NOT NULL |
| is_base | BOOLEAN | DEFAULT false |

### `mm_material_groups`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| code | VARCHAR(20) | UNIQUE, NOT NULL |
| name | VARCHAR(200) | NOT NULL |
| description | TEXT | |

### `mm_materials`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| material_number | VARCHAR(30) | UNIQUE, NOT NULL |
| name | VARCHAR(200) | NOT NULL |
| description | TEXT | |
| material_group_id | UUID | FK -> mm_material_groups(id) |
| base_uom_id | UUID | FK -> mm_uom(id) |
| material_type | VARCHAR(20) | |
| weight | DECIMAL(12,4) | |
| weight_uom | VARCHAR(10) | |
| is_active | BOOLEAN | DEFAULT true |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

### `mm_vendors`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| vendor_number | VARCHAR(30) | UNIQUE, NOT NULL |
| name | VARCHAR(200) | NOT NULL |
| contact_person | VARCHAR(200) | |
| email | VARCHAR(255) | |
| phone | VARCHAR(50) | |
| address | TEXT | |
| payment_terms | INT | |
| is_active | BOOLEAN | DEFAULT true |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

### `mm_plant_stock`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| material_id | UUID | FK -> mm_materials(id) |
| warehouse_id | UUID | |
| quantity | DECIMAL(18,4) | DEFAULT 0 |
| reserved_quantity | DECIMAL(18,4) | DEFAULT 0 |
| uom_id | UUID | FK -> mm_uom(id) |
| last_count_date | DATE | |
| updated_at | TIMESTAMPTZ | |

UNIQUE: (material_id, warehouse_id)

### `mm_material_movements`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| document_number | VARCHAR(30) | UNIQUE, NOT NULL |
| movement_type | VARCHAR(20) | CHECK: GOODS_RECEIPT, GOODS_ISSUE, TRANSFER, ADJUSTMENT |
| material_id | UUID | FK -> mm_materials(id) |
| warehouse_id | UUID | |
| quantity | DECIMAL(18,4) | NOT NULL |
| uom_id | UUID | FK -> mm_uom(id) |
| reference_type | VARCHAR(50) | |
| reference_id | UUID | |
| posted_by | UUID | FK -> users(id) |
| posted_at | TIMESTAMPTZ | DEFAULT NOW() |

### `mm_purchase_orders`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| po_number | VARCHAR(30) | UNIQUE, NOT NULL |
| vendor_id | UUID | FK -> mm_vendors(id) |
| order_date | DATE | NOT NULL |
| delivery_date | DATE | |
| status | VARCHAR(20) | CHECK: DRAFT, RELEASED, PARTIALLY_RECEIVED, RECEIVED, CLOSED, CANCELLED |
| total_amount | DECIMAL(18,4) | DEFAULT 0 |
| currency | VARCHAR(3) | DEFAULT 'TWD' |
| notes | TEXT | |
| created_by | UUID | FK -> users(id) |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

### `mm_purchase_order_items`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| purchase_order_id | UUID | FK -> mm_purchase_orders(id) CASCADE |
| line_number | INT | |
| material_id | UUID | FK -> mm_materials(id) |
| quantity | DECIMAL(18,4) | |
| unit_price | DECIMAL(18,4) | |
| total_price | DECIMAL(18,4) | |
| uom_id | UUID | FK -> mm_uom(id) |
| delivery_date | DATE | |
| received_quantity | DECIMAL(18,4) | DEFAULT 0 |

---

## 4. SD - Sales & Distribution (5 tables)

### `sd_customers`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| customer_number | VARCHAR(30) | UNIQUE, NOT NULL |
| name | VARCHAR(200) | NOT NULL |
| contact_person | VARCHAR(200) | |
| email | VARCHAR(255) | |
| phone | VARCHAR(50) | |
| address | TEXT | |
| payment_terms | INT | |
| credit_limit | DECIMAL(18,4) | |
| is_active | BOOLEAN | DEFAULT true |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

### `sd_sales_orders`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| order_number | VARCHAR(30) | UNIQUE, NOT NULL |
| customer_id | UUID | FK -> sd_customers(id) |
| order_date | DATE | NOT NULL |
| requested_delivery_date | DATE | |
| status | VARCHAR(20) | CHECK: DRAFT, CONFIRMED, PARTIALLY_DELIVERED, DELIVERED, CLOSED, CANCELLED |
| total_amount | DECIMAL(18,4) | DEFAULT 0 |
| currency | VARCHAR(3) | DEFAULT 'TWD' |
| notes | TEXT | |
| created_by | UUID | FK -> users(id) |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

### `sd_sales_order_items`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| sales_order_id | UUID | FK -> sd_sales_orders(id) CASCADE |
| line_number | INT | |
| material_id | UUID | FK -> mm_materials(id) |
| quantity | DECIMAL(18,4) | |
| unit_price | DECIMAL(18,4) | |
| total_price | DECIMAL(18,4) | |
| uom_id | UUID | FK -> mm_uom(id) |
| delivered_quantity | DECIMAL(18,4) | DEFAULT 0 |

### `sd_deliveries`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| delivery_number | VARCHAR(30) | UNIQUE, NOT NULL |
| sales_order_id | UUID | FK -> sd_sales_orders(id) |
| delivery_date | DATE | |
| status | VARCHAR(20) | CHECK: CREATED, SHIPPED, DELIVERED, CANCELLED |
| shipped_by | UUID | FK -> users(id) |
| shipped_at | TIMESTAMPTZ | |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

### `sd_delivery_items`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| delivery_id | UUID | FK -> sd_deliveries(id) CASCADE |
| sales_order_item_id | UUID | FK -> sd_sales_order_items(id) |
| quantity | DECIMAL(18,4) | |

### `sd_invoices`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| invoice_number | VARCHAR(30) | UNIQUE, NOT NULL |
| sales_order_id | UUID | FK -> sd_sales_orders(id) |
| delivery_id | UUID | FK -> sd_deliveries(id) |
| customer_id | UUID | FK -> sd_customers(id) |
| invoice_date | DATE | |
| due_date | DATE | |
| total_amount | DECIMAL(18,4) | |
| status | VARCHAR(20) | CHECK: CREATED, POSTED, PAID, CANCELLED |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

---

## 5. PP - Production Planning (5 tables)

### `pp_boms`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| bom_number | VARCHAR(30) | UNIQUE, NOT NULL |
| material_id | UUID | FK -> mm_materials(id) |
| name | VARCHAR(200) | |
| version | INT | DEFAULT 1 |
| status | VARCHAR(20) | |
| valid_from | DATE | |
| valid_to | DATE | |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

### `pp_bom_items`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| bom_id | UUID | FK -> pp_boms(id) CASCADE |
| line_number | INT | |
| component_material_id | UUID | FK -> mm_materials(id) |
| quantity | DECIMAL(18,4) | |
| uom_id | UUID | FK -> mm_uom(id) |
| scrap_percentage | DECIMAL(5,2) | DEFAULT 0 |

### `pp_routings`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| routing_number | VARCHAR(30) | UNIQUE, NOT NULL |
| material_id | UUID | FK -> mm_materials(id) |
| name | VARCHAR(200) | |
| version | INT | DEFAULT 1 |
| created_at | TIMESTAMPTZ | |

### `pp_routing_operations`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| routing_id | UUID | FK -> pp_routings(id) CASCADE |
| operation_number | INT | |
| work_center | VARCHAR(100) | |
| description | TEXT | |
| setup_time_minutes | INT | |
| run_time_minutes | INT | |

### `pp_production_orders`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| order_number | VARCHAR(30) | UNIQUE, NOT NULL |
| material_id | UUID | FK -> mm_materials(id) |
| bom_id | UUID | FK -> pp_boms(id) |
| routing_id | UUID | FK -> pp_routings(id) |
| planned_quantity | DECIMAL(18,4) | |
| actual_quantity | DECIMAL(18,4) | DEFAULT 0 |
| uom_id | UUID | FK -> mm_uom(id) |
| planned_start | DATE | |
| planned_end | DATE | |
| actual_start | DATE | |
| actual_end | DATE | |
| status | VARCHAR(20) | CHECK: CREATED, RELEASED, IN_PROGRESS, COMPLETED, CLOSED, CANCELLED |
| created_by | UUID | FK -> users(id) |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

---

## 6. HR - Human Resources (4 tables)

### `hr_departments`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| code | VARCHAR(20) | UNIQUE, NOT NULL |
| name | VARCHAR(200) | NOT NULL |
| parent_department_id | UUID | FK -> hr_departments(id) [Self-ref] |
| manager_id | UUID | |
| is_active | BOOLEAN | DEFAULT true |
| created_at | TIMESTAMPTZ | |

### `hr_positions`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| code | VARCHAR(20) | UNIQUE, NOT NULL |
| title | VARCHAR(200) | NOT NULL |
| department_id | UUID | FK -> hr_departments(id) |
| grade | VARCHAR(10) | |
| is_active | BOOLEAN | DEFAULT true |

### `hr_employees`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| employee_number | VARCHAR(30) | UNIQUE, NOT NULL |
| user_id | UUID | FK -> users(id) |
| first_name | VARCHAR(100) | NOT NULL |
| last_name | VARCHAR(100) | NOT NULL |
| email | VARCHAR(255) | |
| phone | VARCHAR(50) | |
| department_id | UUID | FK -> hr_departments(id) |
| position_id | UUID | FK -> hr_positions(id) |
| hire_date | DATE | |
| termination_date | DATE | |
| status | VARCHAR(20) | |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

### `hr_attendance`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| employee_id | UUID | FK -> hr_employees(id) |
| date | DATE | NOT NULL |
| clock_in | TIMESTAMPTZ | |
| clock_out | TIMESTAMPTZ | |
| status | VARCHAR(20) | |
| notes | TEXT | |

UNIQUE: (employee_id, date)

---

## 7. WM - Warehouse Management (6 tables)

### `wm_warehouses`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| code | VARCHAR(20) | UNIQUE, NOT NULL |
| name | VARCHAR(200) | NOT NULL |
| address | TEXT | |
| warehouse_type | VARCHAR(20) | |
| is_active | BOOLEAN | DEFAULT true |
| created_at | TIMESTAMPTZ | |

### `wm_storage_bins`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| warehouse_id | UUID | FK -> wm_warehouses(id) |
| bin_code | VARCHAR(30) | |
| zone | VARCHAR(20) | |
| aisle | VARCHAR(20) | |
| rack | VARCHAR(20) | |
| level | VARCHAR(20) | |
| max_weight | DECIMAL(12,4) | |
| is_active | BOOLEAN | DEFAULT true |

UNIQUE: (warehouse_id, bin_code)

### `wm_stock_transfers`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| transfer_number | VARCHAR(30) | UNIQUE, NOT NULL |
| from_warehouse_id | UUID | FK -> wm_warehouses(id) |
| to_warehouse_id | UUID | FK -> wm_warehouses(id) |
| material_id | UUID | FK -> mm_materials(id) |
| quantity | DECIMAL(18,4) | |
| uom_id | UUID | FK -> mm_uom(id) |
| status | VARCHAR(20) | |
| requested_by | UUID | FK -> users(id) |
| completed_at | TIMESTAMPTZ | |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

### `wm_stock_counts`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| count_number | VARCHAR(30) | UNIQUE, NOT NULL |
| warehouse_id | UUID | FK -> wm_warehouses(id) |
| count_date | DATE | |
| status | VARCHAR(20) | |
| counted_by | UUID | FK -> users(id) |
| completed_at | TIMESTAMPTZ | |
| created_at | TIMESTAMPTZ | |

### `wm_stock_count_items`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| stock_count_id | UUID | FK -> wm_stock_counts(id) CASCADE |
| material_id | UUID | FK -> mm_materials(id) |
| storage_bin_id | UUID | FK -> wm_storage_bins(id) |
| book_quantity | DECIMAL(18,4) | |
| counted_quantity | DECIMAL(18,4) | |
| difference | DECIMAL(18,4) | |

---

## 8. QM - Quality Management (3 tables)

### `qm_inspection_lots`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| lot_number | VARCHAR(30) | UNIQUE, NOT NULL |
| material_id | UUID | FK -> mm_materials(id) |
| reference_type | VARCHAR(50) | |
| reference_id | UUID | |
| inspection_type | VARCHAR(20) | |
| planned_quantity | DECIMAL(18,4) | |
| inspected_quantity | DECIMAL(18,4) | DEFAULT 0 |
| status | VARCHAR(20) | |
| created_by | UUID | FK -> users(id) |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

### `qm_inspection_results`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| inspection_lot_id | UUID | FK -> qm_inspection_lots(id) CASCADE |
| characteristic | VARCHAR(200) | |
| target_value | VARCHAR(200) | |
| actual_value | VARCHAR(200) | |
| is_conforming | BOOLEAN | |
| inspected_by | UUID | FK -> users(id) |
| inspected_at | TIMESTAMPTZ | |

### `qm_quality_notifications`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| notification_number | VARCHAR(30) | UNIQUE, NOT NULL |
| notification_type | VARCHAR(50) | |
| material_id | UUID | FK -> mm_materials(id) |
| description | TEXT | |
| priority | VARCHAR(10) | |
| status | VARCHAR(20) | |
| reported_by | UUID | FK -> users(id) |
| assigned_to | UUID | FK -> users(id) |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

---

## 9. CO - Controlling (4 tables)

### `co_cost_centers`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| code | VARCHAR(20) | UNIQUE, NOT NULL |
| name | VARCHAR(200) | NOT NULL |
| description | TEXT | |
| responsible_person | UUID | FK -> users(id) |
| is_active | BOOLEAN | DEFAULT true |
| valid_from | DATE | |
| valid_to | DATE | |
| created_at | TIMESTAMPTZ | |

### `co_profit_centers`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| code | VARCHAR(20) | UNIQUE, NOT NULL |
| name | VARCHAR(200) | NOT NULL |
| description | TEXT | |
| responsible_person | UUID | FK -> users(id) |
| is_active | BOOLEAN | DEFAULT true |
| created_at | TIMESTAMPTZ | |

### `co_internal_orders`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| order_number | VARCHAR(30) | UNIQUE, NOT NULL |
| name | VARCHAR(200) | NOT NULL |
| order_type | VARCHAR(50) | |
| cost_center_id | UUID | FK -> co_cost_centers(id) |
| status | VARCHAR(20) | |
| budget | DECIMAL(18,4) | |
| actual_cost | DECIMAL(18,4) | DEFAULT 0 |
| created_at | TIMESTAMPTZ | |
| updated_at | TIMESTAMPTZ | |

### `co_cost_allocations`

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY |
| from_cost_center_id | UUID | FK -> co_cost_centers(id) |
| to_cost_center_id | UUID | FK -> co_cost_centers(id) |
| allocation_date | DATE | |
| amount | DECIMAL(18,4) | |
| description | TEXT | |
| created_at | TIMESTAMPTZ | |

---

## 10. Low-Code Platform (24 tables)

### Core Tables

| Table | Purpose | Key Columns |
|-------|---------|-------------|
| `lc_projects` | Project container | project_number, name, is_active |
| `lc_operations` | Form/List/Dashboard/Report definition | operation_code, operation_type (CHECK: FORM/LIST/DASHBOARD/REPORT) |
| `lc_form_definitions` | Form layout metadata | operation_id (UNIQUE FK), layout_config (JSONB) |
| `lc_form_sections` | Form sections | form_id, title, columns (CHECK: 1-4), sort_order |
| `lc_field_definitions` | Field configuration | section_id, field_type (CHECK: 14 types), validation rules |
| `lc_field_options` | Dropdown options | field_id, option_label, option_value |
| `lc_operation_data` | User data (JSONB) | operation_id, data (JSONB), created_by |
| `lc_file_uploads` | File attachments | file_name, storage_path, file_size |

### Permission Tables

| Table | Purpose | Key Columns |
|-------|---------|-------------|
| `lc_platform_roles` | PLATFORM_ADMIN / DEVELOPER / USER | role_name (CHECK) |
| `lc_user_platform_roles` | User-role assignments | user_id, role_id (UNIQUE pair) |
| `lc_project_developers` | Project member assignments | project_id, user_id, role (LEAD/DEVELOPER/VIEWER) |
| `lc_operation_permissions` | CRUD permissions per operation | can_create, can_read, can_update, can_delete |
| `lc_field_permissions` | Field visibility rules | visibility (VISIBLE/HIDDEN/MASKED), is_editable |
| `lc_record_policies` | Row-level security (SQL filters) | filter_sql, is_active |

### Workflow Tables

| Table | Purpose | Key Columns |
|-------|---------|-------------|
| `lc_dev_journal` | Git-like change tracking | change_type (12 types), old_values, new_values, form_snapshot |
| `lc_releases` | Version release management | status (DRAFT/SUBMITTED/APPROVED/RELEASED/REJECTED) |
| `lc_release_feedback_links` | Release-feedback junction | release_id, feedback_id |
| `lc_feedback` | Bug reports / feature requests | feedback_type, priority, status |
| `lc_feedback_comments` | Discussion threads | feedback_id, content |
| `lc_notifications` | In-app notifications | notification_type, is_read |
| `lc_navigation_items` | Dynamic sidebar menu | parent_id (self-ref), sort_order, required_role |

---

## 11. Workflow Infrastructure (1 table)

### `document_status_history`

Universal audit trail for all ERP document status transitions.

| Column | Type | Constraints |
|--------|------|------------|
| id | UUID | PRIMARY KEY, DEFAULT gen_random_uuid() |
| document_type | VARCHAR(50) | NOT NULL |
| document_id | UUID | NOT NULL |
| from_status | VARCHAR(30) | Nullable (NULL for initial) |
| to_status | VARCHAR(30) | NOT NULL |
| changed_by | UUID | FK -> users(id), NOT NULL |
| change_reason | TEXT | |
| created_at | TIMESTAMPTZ | DEFAULT NOW() |

Indexes:
- `idx_status_history_doc` ON (document_type, document_id)
- `idx_status_history_time` ON (created_at)

**document_type values**: `SALES_ORDER`, `PURCHASE_ORDER`, `PRODUCTION_ORDER`, `JOURNAL_ENTRY`, `DELIVERY`, `INVOICE`

---

## Summary Statistics

| Category | Count |
|----------|-------|
| Total Tables | 74 |
| Foundation/Core | 8 |
| ERP Module Tables | 41 |
| Low-Code Tables | 24 |
| Workflow Infrastructure | 1 |
| CHECK Constraints | 20+ |
| Foreign Key Relations | 60+ |
| Auto-number Ranges | 9 |
| JSONB Columns | 12 |
