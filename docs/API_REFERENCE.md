# TasteByte ERP - API Reference

> Base URL: `http://localhost:8000/api/v1`
> Auth: Bearer Token (JWT) required for all endpoints except `/auth/login` and `/auth/register`
> Last Updated: 2026-02-21

---

## Authentication (AUTH)

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| POST | `/auth/login` | Login with credentials | No |
| POST | `/auth/register` | Register new user | No |
| POST | `/auth/refresh` | Refresh access token | No |

### POST `/auth/login`

**Request:**
```json
{
  "username": "admin",
  "password": "admin123"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "access_token": "eyJ...",
    "refresh_token": "eyJ...",
    "token_type": "Bearer",
    "user": {
      "id": "uuid",
      "username": "admin",
      "email": "admin@tastebyte.com",
      "full_name": "System Administrator",
      "is_active": true,
      "role": "ADMIN"
    }
  }
}
```

---

## Dashboard

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/dashboard/kpis` | Main dashboard KPIs | Any |

**Response fields:** `total_revenue`, `total_order_count`, `total_inventory_quantity`, `pending_production_orders`, `open_ar_amount`, `open_ap_amount`

---

## FI - Financial Accounting (23 endpoints)

### Accounts

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/fi/accounts` | List chart of accounts | FiRead |
| POST | `/fi/accounts` | Create account | FiWrite |
| GET | `/fi/accounts/{id}` | Get account detail | FiRead |
| PUT | `/fi/accounts/{id}` | Update account | FiWrite |

### Account Groups

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/fi/account-groups` | List account groups | FiRead |
| POST | `/fi/account-groups` | Create account group | FiWrite |

### Company Codes

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/fi/company-codes` | List company codes | FiRead |
| POST | `/fi/company-codes` | Create company code | FiWrite |

### Fiscal Years

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/fi/company-codes/{id}/fiscal-years` | List fiscal years | FiRead |
| POST | `/fi/fiscal-years` | Create fiscal year | FiWrite |

### Journal Entries

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/fi/journal-entries` | List journal entries | FiRead |
| POST | `/fi/journal-entries` | Create journal entry | FiWrite |
| GET | `/fi/journal-entries/{id}` | Get journal entry detail | FiRead |
| POST | `/fi/journal-entries/{id}/post` | Post journal entry (DRAFT -> POSTED) | FiWrite |

**Create Journal Entry Request:**
```json
{
  "company_code_id": "uuid",
  "posting_date": "2026-02-21",
  "document_date": "2026-02-21",
  "reference": "REF-001",
  "description": "Monthly entry",
  "items": [
    {
      "account_id": "uuid",
      "debit_amount": 1000.00,
      "credit_amount": 0,
      "cost_center_id": "uuid (optional)",
      "description": "Debit side"
    },
    {
      "account_id": "uuid",
      "debit_amount": 0,
      "credit_amount": 1000.00,
      "description": "Credit side"
    }
  ]
}
```

### AR/AP Invoices

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/fi/ar-invoices` | List AR invoices | FiRead |
| POST | `/fi/ar-invoices` | Create AR invoice | FiWrite |
| GET | `/fi/ap-invoices` | List AP invoices | FiRead |
| POST | `/fi/ap-invoices` | Create AP invoice | FiWrite |

### Financial Reports

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/fi/reports/trial-balance` | Trial balance report | FiRead |
| GET | `/fi/reports/income-statement` | Income statement | FiRead |
| GET | `/fi/reports/balance-sheet` | Balance sheet | FiRead |
| GET | `/fi/reports/ar-aging` | AR aging analysis | FiRead |
| GET | `/fi/reports/ap-aging` | AP aging analysis | FiRead |

---

## CO - Controlling (12 endpoints)

### Cost Centers

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/co/cost-centers` | List cost centers | CoRead |
| POST | `/co/cost-centers` | Create cost center | CoWrite |
| GET | `/co/cost-centers/{id}` | Get cost center | CoRead |

### Profit Centers

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/co/profit-centers` | List profit centers | CoRead |
| POST | `/co/profit-centers` | Create profit center | CoWrite |
| GET | `/co/profit-centers/{id}` | Get profit center | CoRead |

### Internal Orders

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/co/internal-orders` | List internal orders | CoRead |
| POST | `/co/internal-orders` | Create internal order | CoWrite |
| GET | `/co/internal-orders/{id}` | Get internal order | CoRead |
| PUT | `/co/internal-orders/{id}` | Update internal order | CoWrite |

### Cost Allocations

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/co/cost-allocations` | List allocations | CoRead |
| POST | `/co/cost-allocations` | Create allocation | CoWrite |

---

## MM - Materials Management (23 endpoints)

### Units of Measure

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/mm/uoms` | List UoMs | MmRead |
| POST | `/mm/uoms` | Create UoM | MmWrite |

### Material Groups

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/mm/material-groups` | List groups | MmRead |
| POST | `/mm/material-groups` | Create group | MmWrite |

### Materials

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/mm/materials` | List materials (paginated) | MmRead |
| POST | `/mm/materials` | Create material (auto MAT number) | MmWrite |
| GET | `/mm/materials/{id}` | Get material detail | MmRead |
| PUT | `/mm/materials/{id}` | Update material | MmWrite |

### Vendors

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/mm/vendors` | List vendors (paginated) | MmRead |
| POST | `/mm/vendors` | Create vendor (auto VND number) | MmWrite |
| GET | `/mm/vendors/{id}` | Get vendor detail | MmRead |
| PUT | `/mm/vendors/{id}` | Update vendor | MmWrite |

### Inventory

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/mm/plant-stock` | List plant stock | MmRead |
| GET | `/mm/material-movements` | List movements | MmRead |
| POST | `/mm/material-movements` | Create movement | MmWrite |

### Purchase Orders

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/mm/purchase-orders` | List POs (paginated) | MmRead |
| POST | `/mm/purchase-orders` | Create PO (auto PO number) | MmWrite |
| GET | `/mm/purchase-orders/{id}` | Get PO with items | MmRead |
| POST | `/mm/purchase-orders/{id}/release` | Release PO (DRAFT -> RELEASED) | MmWrite |
| POST | `/mm/purchase-orders/{id}/receive` | Goods receipt (MM->FI integration) | MmWrite |

**Goods Receipt Request:**
```json
{
  "items": [
    {
      "po_item_id": "uuid",
      "quantity": 100,
      "warehouse_id": "uuid"
    }
  ]
}
```

### MM Reports

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/mm/reports/stock-valuation` | Stock valuation | MmRead |
| GET | `/mm/reports/movement-summary` | Movement summary | MmRead |
| GET | `/mm/reports/slow-moving` | Slow-moving items | MmRead |

---

## SD - Sales & Distribution (15 endpoints)

### Customers

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/sd/customers` | List customers | SdRead |
| POST | `/sd/customers` | Create customer | SdWrite |
| GET | `/sd/customers/{id}` | Get customer | SdRead |
| PUT | `/sd/customers/{id}` | Update customer | SdWrite |

### Sales Orders

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/sd/sales-orders` | List sales orders | SdRead |
| POST | `/sd/sales-orders` | Create sales order | SdWrite |
| GET | `/sd/sales-orders/{id}` | Get SO with items | SdRead |
| POST | `/sd/sales-orders/{id}/confirm` | Confirm SO (DRAFT -> CONFIRMED) | SdWrite |

### Deliveries & Invoices

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/sd/deliveries` | List deliveries | SdRead |
| POST | `/sd/deliveries` | Create delivery from SO | SdWrite |
| GET | `/sd/invoices` | List SD invoices | SdRead |
| POST | `/sd/invoices` | Create invoice (SD->FI integration) | SdWrite |

### SD Reports

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/sd/reports/sales-summary` | Sales summary | SdRead |
| GET | `/sd/reports/order-fulfillment` | Order fulfillment rate | SdRead |
| GET | `/sd/reports/top-customers` | Top customers by revenue | SdRead |

---

## PP - Production Planning (10 endpoints)

### BOMs (Bill of Materials)

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/pp/boms` | List BOMs | PpRead |
| POST | `/pp/boms` | Create BOM | PpWrite |
| GET | `/pp/boms/{id}` | Get BOM with items | PpRead |

### Routings

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/pp/routings` | List routings | PpRead |
| POST | `/pp/routings` | Create routing | PpWrite |
| GET | `/pp/routings/{id}` | Get routing with operations | PpRead |

### Production Orders

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/pp/production-orders` | List production orders | PpRead |
| POST | `/pp/production-orders` | Create production order | PpWrite |
| GET | `/pp/production-orders/{id}` | Get production order | PpRead |
| PUT | `/pp/production-orders/{id}/status` | Update status (CREATED->RELEASED->IN_PROGRESS->COMPLETED) | PpWrite |

---

## HR - Human Resources (11 endpoints)

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/hr/departments` | List departments | HrRead |
| POST | `/hr/departments` | Create department | HrWrite |
| GET | `/hr/departments/{id}` | Get department | HrRead |
| GET | `/hr/positions` | List positions | HrRead |
| POST | `/hr/positions` | Create position | HrWrite |
| GET | `/hr/employees` | List employees | HrRead |
| POST | `/hr/employees` | Create employee | HrWrite |
| GET | `/hr/employees/{id}` | Get employee | HrRead |
| PUT | `/hr/employees/{id}` | Update employee | HrWrite |
| GET | `/hr/employees/{id}/attendance` | List attendance records | HrRead |
| POST | `/hr/attendance` | Create attendance record | HrWrite |

---

## WM - Warehouse Management (10 endpoints)

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/wm/warehouses` | List warehouses | WmRead |
| POST | `/wm/warehouses` | Create warehouse | WmWrite |
| GET | `/wm/warehouses/{id}` | Get warehouse | WmRead |
| GET | `/wm/warehouses/{id}/storage-bins` | List storage bins | WmRead |
| POST | `/wm/storage-bins` | Create storage bin | WmWrite |
| GET | `/wm/stock-transfers` | List transfers | WmRead |
| POST | `/wm/stock-transfers` | Create transfer | WmWrite |
| GET | `/wm/stock-counts` | List stock counts | WmRead |
| POST | `/wm/stock-counts` | Create stock count | WmWrite |
| GET | `/wm/stock-counts/{id}` | Get stock count detail | WmRead |

---

## QM - Quality Management (9 endpoints)

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/qm/inspection-lots` | List inspection lots | QmRead |
| POST | `/qm/inspection-lots` | Create inspection lot | QmWrite |
| GET | `/qm/inspection-lots/{id}` | Get lot detail | QmRead |
| GET | `/qm/inspection-lots/{id}/results` | List inspection results | QmRead |
| POST | `/qm/inspection-results` | Record inspection result | QmWrite |
| GET | `/qm/notifications` | List quality notifications | QmRead |
| POST | `/qm/notifications` | Create quality notification | QmWrite |
| GET | `/qm/notifications/{id}` | Get notification detail | QmRead |
| PUT | `/qm/notifications/{id}` | Update notification | QmWrite |

---

## Workflow - Status History (1 endpoint)

| Method | Endpoint | Description | Role |
|--------|----------|-------------|------|
| GET | `/workflow/history/{doc_type}/{doc_id}` | Get status transition history | Any |

**doc_type values:** `SALES_ORDER`, `PURCHASE_ORDER`, `PRODUCTION_ORDER`, `JOURNAL_ENTRY`, `DELIVERY`, `INVOICE`

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "document_type": "PURCHASE_ORDER",
      "document_id": "uuid",
      "from_status": "DRAFT",
      "to_status": "RELEASED",
      "changed_by": "uuid",
      "change_reason": null,
      "created_at": "2026-02-21T10:30:00Z"
    }
  ]
}
```

---

## Low-Code Platform (67 endpoints)

### Projects (Admin)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/lowcode/projects` | List projects |
| POST | `/lowcode/projects` | Create project |
| GET | `/lowcode/projects/{id}` | Get project |
| PUT | `/lowcode/projects/{id}` | Update project |
| DELETE | `/lowcode/projects/{id}` | Delete project |

### Operations (Developer)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/lowcode/operations` | List operations |
| POST | `/lowcode/operations` | Create operation |
| GET | `/lowcode/operations/{id}` | Get operation |
| PUT | `/lowcode/operations/{id}` | Update operation |

### Form Builder

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/lowcode/operations/{id}/form` | Get form definition |
| PUT | `/lowcode/operations/{id}/form` | Save form definition |

### Form Execution (User)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/lowcode/exec/{code}` | Get form by operation code |
| GET | `/lowcode/exec/{code}/data` | List form records |
| POST | `/lowcode/exec/{code}/data` | Create form record |
| GET | `/lowcode/exec/{code}/data/{id}` | Get single record |
| PUT | `/lowcode/exec/{code}/data/{id}` | Update record |
| DELETE | `/lowcode/exec/{code}/data/{id}` | Delete record |

### Datasource (SQL Engine)

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/lowcode/datasource/query` | Execute read-only SQL |
| GET | `/lowcode/datasource/tables` | List available tables |
| POST | `/lowcode/datasource/validate-sql` | Validate SQL syntax |
| GET | `/lowcode/datasource/tables/{name}/columns` | Get table columns |

### Permissions

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/lowcode/permissions/operations/{id}` | Get operation permissions |
| POST | `/lowcode/permissions/operations/{id}` | Set operation permission |
| PUT | `/lowcode/permissions/operations/{id}/{perm_id}` | Update permission |
| DELETE | `/lowcode/permissions/operations/{id}/{perm_id}` | Delete permission |
| GET | `/lowcode/permissions/fields/{id}` | Get field permissions |
| POST | `/lowcode/permissions/fields/{id}` | Set field permission |
| DELETE | `/lowcode/permissions/fields/{id}/{perm_id}` | Delete field permission |
| GET | `/lowcode/permissions/records/{id}` | Get record policies |
| POST | `/lowcode/permissions/records/{id}` | Create record policy |
| DELETE | `/lowcode/permissions/records/{id}/{policy_id}` | Delete record policy |

### Releases

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/lowcode/releases` | List releases |
| POST | `/lowcode/releases` | Create release |
| PUT | `/lowcode/releases/{id}/submit` | Submit for review |
| PUT | `/lowcode/releases/{id}/approve` | Approve release |
| PUT | `/lowcode/releases/{id}/reject` | Reject release |
| PUT | `/lowcode/releases/{id}/publish` | Publish release |

### Feedback

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/lowcode/feedback` | List feedback tickets |
| POST | `/lowcode/feedback` | Create feedback |
| PUT | `/lowcode/feedback/{id}` | Update feedback |
| GET | `/lowcode/feedback/{id}/comments` | List comments |
| POST | `/lowcode/feedback/{id}/comments` | Add comment |

### Journal (Change Tracking)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/lowcode/operations/{id}/journal` | Get change journal |
| POST | `/lowcode/operations/{id}/journal/rollback/{version}` | Rollback to version |

### Files

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/lowcode/files/upload` | Upload file (50MB max) |
| GET | `/lowcode/files/{id}` | Download file |

### Navigation

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/lowcode/navigation` | Get navigation tree |
| POST | `/lowcode/navigation` | Create nav item |
| PUT | `/lowcode/navigation/{id}` | Update nav item |
| DELETE | `/lowcode/navigation/{id}` | Delete nav item |
| PUT | `/lowcode/navigation/reorder` | Reorder nav items |

### Notifications

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/lowcode/notifications` | List notifications |
| PUT | `/lowcode/notifications/{id}/read` | Mark as read |
| PUT | `/lowcode/notifications/read-all` | Mark all as read |

### User Profile

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/lowcode/user/me` | Get my profile & roles |

### Role Management (Admin)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/lowcode/users` | List users with roles |
| POST | `/lowcode/users/{id}/roles` | Assign platform role |
| DELETE | `/lowcode/users/{id}/roles/{role}` | Revoke role |
| GET | `/lowcode/projects/{id}/developers` | List project devs |
| POST | `/lowcode/projects/{id}/developers` | Assign developer |
| DELETE | `/lowcode/projects/{id}/developers/{user_id}` | Remove developer |

---

## Pagination

Most list endpoints support pagination via query parameters:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page` | integer | 1 | Page number |
| `page_size` | integer | 20 | Items per page |
| `search` | string | — | Search filter |
| `status` | string | — | Status filter |

**Paginated Response Format:**
```json
{
  "success": true,
  "data": {
    "data": [ ... ],
    "total": 150,
    "page": 1,
    "per_page": 20
  }
}
```

---

## Error Codes

| HTTP Status | Error Code | Description |
|-------------|-----------|-------------|
| 400 | VALIDATION_ERROR | Invalid input data |
| 401 | UNAUTHORIZED | Missing or invalid token |
| 403 | FORBIDDEN | Insufficient permissions |
| 404 | NOT_FOUND | Resource not found |
| 422 | BUSINESS_RULE | Business rule violation |
| 500 | DB_ERROR | Internal database error |
