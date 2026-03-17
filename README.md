<p align="center">
  <img src="https://img.shields.io/badge/Rust-Axum-orange?style=for-the-badge&logo=rust" />
  <img src="https://img.shields.io/badge/Next.js-15-black?style=for-the-badge&logo=next.js" />
  <img src="https://img.shields.io/badge/PostgreSQL-17-336791?style=for-the-badge&logo=postgresql" />
  <img src="https://img.shields.io/badge/Flutter-3.10-02569B?style=for-the-badge&logo=flutter" />
  <img src="https://img.shields.io/badge/Swift-5.9-FA7343?style=for-the-badge&logo=swift" />
  <img src="https://img.shields.io/badge/Kotlin-1.9-7F52FF?style=for-the-badge&logo=kotlin" />
</p>

# TasteByte ERP

> A production-grade, full-stack Enterprise Resource Planning system built for the food & beverage industry — featuring 8 SAP-style modules, a low-code platform, and multi-platform clients.

---

## Architecture Overview

```
                        ┌──────────────────────────────────────┐
                        │           Nginx Reverse Proxy         │
                        │            (Port 80 / 443)            │
                        └────┬──────────┬──────────┬───────────┘
                             │          │          │
                    ┌────────▼──┐  ┌────▼────┐  ┌──▼──────────┐
                    │  Next.js  │  │   iOS   │  │  Android /  │
                    │  15 Web   │  │  Swift  │  │  Flutter    │
                    │  :3000    │  │  UI     │  │  Mobile     │
                    └────────┬──┘  └────┬────┘  └──┬──────────┘
                             │          │          │
                             └──────────┼──────────┘
                                        │ REST API (JSON)
                              ┌─────────▼─────────┐
                              │   Rust / Axum      │
                              │   Backend :8000    │
                              │   160+ Endpoints   │
                              └─────────┬──────────┘
                                        │ SQLx (compile-time checked)
                              ┌─────────▼──────────┐
                              │   PostgreSQL 17     │
                              │   74 Tables         │
                              │   31 Migrations     │
                              └────────────────────┘
```

---

## Test Status

| Layer | Suite | Status | Passed | Total |
|-------|-------|--------|--------|-------|
| Backend | Unit Tests (`cargo test --lib`) | ✅ PASS | 160 | 160 |
| Backend | Integration Tests (10 modules) | ✅ COMPILES | — | 10 files |
| Web | Vitest Unit Tests | ✅ PASS | 96 | 96 |
| Web | Playwright E2E | ⏳ Configured | — | 5 specs |
| Mobile | Flutter | ⏳ Configured | — | 1 file |

---

## Modules & Features

### Dashboard — Real-time KPIs & Analytics

```
┌─────────────────────────────────────────────────────────────────────────┐
│  TasteByte ERP                                        🔔  Admin  ▾  EN │
├──────────┬──────────────────────────────────────────────────────────────┤
│          │                                                              │
│ 📊 Dashboard │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐   │
│ 💰 FI    │  │ Revenue │  │ Orders  │  │ Items   │  │ Employees│   │
│ 📈 CO    │  │ $1.2M   │  │  342    │  │ 1,205   │  │   89     │   │
│ 📦 MM    │  │ ▲ 12.5% │  │ ▲ 8.3%  │  │ ▼ 2.1%  │  │ ▲ 3.4%  │   │
│ 🛒 SD    │  └─────────┘  └─────────┘  └─────────┘  └─────────┘   │
│ 🏭 PP    │                                                              │
│ 👥 HR    │  ┌───────────────────────────────────────────────────────┐   │
│ 🏪 WM    │  │          Monthly Revenue Trend (Bar Chart)            │   │
│ ✅ QM    │  │  ████                                                 │   │
│          │  │  ████ ██████                                          │   │
│ ─────── │  │  ████ ██████ ████████                                 │   │
│ 🧩 Lowcode │  │  ████ ██████ ████████ ██████████                    │   │
│ 💻 Developer│  │  Jan   Feb    Mar      Apr                          │   │
│ ⚙️ Admin │  └───────────────────────────────────────────────────────┘   │
│          │                                                              │
│ v1.0     │  ┌─────────────────────┐  ┌─────────────────────────────┐   │
│          │  │ Sales by Category   │  │ Recent Activities           │   │
│          │  │     (Pie Chart)     │  │ • SO-000342 confirmed       │   │
│          │  │    🔴 Food  45%     │  │ • PO-000128 received        │   │
│          │  │    🔵 Drink 35%     │  │ • JE-000089 posted          │   │
│          │  │    🟢 Other 20%     │  │ • INV-000256 paid           │   │
│          │  └─────────────────────┘  └─────────────────────────────┘   │
└──────────┴──────────────────────────────────────────────────────────────┘
```

**Features:** KPI cards with trend indicators, bar/line/pie charts (Recharts), real-time data refresh

---

### FI — Financial Accounting

```
┌─ FI > Chart of Accounts ──────────────────────────────────────────────┐
│                                                                        │
│  [+ New Account]  [Export CSV]         🔍 Search accounts...           │
│                                                                        │
│  ┌──────────┬────────────────────┬──────────┬──────────┬────────┐     │
│  │ Account# │ Description        │ Type     │ Group    │ Balance│     │
│  ├──────────┼────────────────────┼──────────┼──────────┼────────┤     │
│  │ 1000     │ Cash & Equivalents │ Asset    │ Current  │ $45.2K │     │
│  │ 1100     │ Accounts Receivable│ Asset    │ Current  │ $128K  │     │
│  │ 2000     │ Accounts Payable   │ Liability│ Current  │ $67.8K │     │
│  │ 3000     │ Retained Earnings  │ Equity   │ Capital  │ $340K  │     │
│  │ 4000     │ Sales Revenue      │ Revenue  │ Operating│ $1.2M  │     │
│  │ 5000     │ Cost of Goods Sold │ Expense  │ Operating│ $780K  │     │
│  └──────────┴────────────────────┴──────────┴──────────┴────────┘     │
│                                                                        │
│  « 1  2  3 ... 8 »    Showing 1-20 of 156           Page size: [20▾] │
└────────────────────────────────────────────────────────────────────────┘
```

```
┌─ FI > Journal Entry #JE-000089 ────────────────────────────────────────┐
│                                                                         │
│  Status: ● DRAFT ──→ ○ POSTED                    [Post] [Cancel]       │
│                                                                         │
│  Date: 2026-03-15    Reference: INV-2026-0315    Company: TasteByte    │
│  Description: Monthly rent payment                                      │
│                                                                         │
│  ┌───────────┬───────────────────────┬──────────┬──────────┐           │
│  │ Account   │ Description           │  Debit   │ Credit   │           │
│  ├───────────┼───────────────────────┼──────────┼──────────┤           │
│  │ 6100      │ Rent Expense          │ $5,000   │          │           │
│  │ 1000      │ Cash                  │          │ $5,000   │           │
│  ├───────────┼───────────────────────┼──────────┼──────────┤           │
│  │           │ Total                 │ $5,000   │ $5,000   │ ✅ Balanced│
│  └───────────┴───────────────────────┴──────────┴──────────┘           │
│                                                                         │
│  ┌─ Workflow Timeline ─────────────────────────────────────────┐       │
│  │  ● Created (Admin, 2026-03-15 09:30)                       │       │
│  │  ○ Pending Post                                              │       │
│  └──────────────────────────────────────────────────────────────┘       │
└─────────────────────────────────────────────────────────────────────────┘
```

**Features:** Chart of Accounts, Account Groups, Journal Entries (DRAFT → POSTED workflow), AR/AP Invoices, Payment recording, Trial Balance, Income Statement, Balance Sheet, AR/AP Aging Reports

---

### MM — Materials Management

```
┌─ MM > Materials ───────────────────────────────────────────────────────┐
│                                                                         │
│  [+ New Material]  [Bulk Import]  [Export]     🔍 Search...            │
│                                                                         │
│  ┌────────┬──────────────────┬──────────┬───────┬────────┬──────────┐ │
│  │ ID     │ Description      │ Group    │ UoM   │ Stock  │ Status   │ │
│  ├────────┼──────────────────┼──────────┼───────┼────────┼──────────┤ │
│  │ M-0001 │ Premium Flour    │ Raw Mat. │ KG    │ 5,200  │ 🟢 Active│ │
│  │ M-0002 │ Olive Oil        │ Raw Mat. │ LTR   │ 320    │ 🟢 Active│ │
│  │ M-0003 │ Packaging Box L  │ Packaging│ PC    │ 12,400 │ 🟢 Active│ │
│  │ M-0004 │ Vanilla Extract  │ Flavoring│ ML    │ 85     │ 🟡 Low   │ │
│  └────────┴──────────────────┴──────────┴───────┴────────┴──────────┘ │
│                                                                         │
│  ☑ 2 selected                               [Delete] [Change Group]   │
└─────────────────────────────────────────────────────────────────────────┘
```

```
┌─ MM > Purchase Order #PO-000128 ───────────────────────────────────────┐
│                                                                         │
│  Status Flow:                                                           │
│  ● CREATED ──→ ● RELEASED ──→ ● RECEIVED ──→ ○ CLOSED                 │
│                                                                         │
│  Vendor: Fresh Farms Inc.        Order Date: 2026-03-10                │
│  Payment Terms: Net 30           Expected: 2026-03-20                  │
│                                                                         │
│  ┌──────────┬──────────────────┬───────┬────────┬──────────┐          │
│  │ Material │ Description      │ Qty   │ Price  │ Total    │          │
│  ├──────────┼──────────────────┼───────┼────────┼──────────┤          │
│  │ M-0001   │ Premium Flour    │ 2,000 │ $2.50  │ $5,000   │          │
│  │ M-0002   │ Olive Oil        │ 100   │ $12.00 │ $1,200   │          │
│  ├──────────┼──────────────────┼───────┼────────┼──────────┤          │
│  │          │ Grand Total      │       │        │ $6,200   │          │
│  └──────────┴──────────────────┴───────┴────────┴──────────┘          │
│                                                                         │
│  [Release]  [Receive Goods]  [Print]                                   │
└─────────────────────────────────────────────────────────────────────────┘
```

**Features:** Materials CRUD with soft delete, UOM management, Vendor management, Purchase Orders (CREATED → RELEASED → RECEIVED → CLOSED), Plant Stock tracking, Material Movements, Stock Valuation Reports

---

### SD — Sales & Distribution

```
┌─ SD > Sales Orders ────────────────────────────────────────────────────┐
│                                                                         │
│  [+ New Order]  [Export]              🔍 Filter by status: [All ▾]     │
│                                                                         │
│  ┌────────────┬──────────────┬────────────┬──────────┬────────────┐   │
│  │ Order #    │ Customer     │ Date       │ Total    │ Status     │   │
│  ├────────────┼──────────────┼────────────┼──────────┼────────────┤   │
│  │ SO00000342 │ Cafe Deluxe  │ 2026-03-15 │ $8,450   │ 🟢 Confirm │   │
│  │ SO00000341 │ Restaurant A │ 2026-03-14 │ $3,200   │ 🔵 Deliver │   │
│  │ SO00000340 │ Hotel Grand  │ 2026-03-13 │ $15,600  │ 🟣 Invoice │   │
│  │ SO00000339 │ Bakery Plus  │ 2026-03-12 │ $2,100   │ ⚪ Draft   │   │
│  └────────────┴──────────────┴────────────┴──────────┴────────────┘   │
│                                                                         │
│  Document Flow:                                                         │
│  Sales Order ──→ Delivery ──→ Invoice ──→ Payment                      │
└─────────────────────────────────────────────────────────────────────────┘
```

**Features:** Customer management, Sales Orders (auto-numbering SO00000001+), Deliveries, Invoices, Order confirmation workflow, Document flow tracking, Sales summary & top customer reports

---

### PP — Production Planning

```
┌─ PP > Production Order #PRD-000045 ────────────────────────────────────┐
│                                                                         │
│  Product: Artisan Bread Mix (M-0050)         Quantity: 500 KG          │
│                                                                         │
│  Status: ● CREATED ──→ ● RELEASED ──→ ○ IN_PROGRESS ──→ ○ COMPLETED  │
│                                                                         │
│  ┌─ Bill of Materials ──────────────────────────────────────────┐      │
│  │ Component        │ Required  │ Available │ Status            │      │
│  │ Premium Flour    │ 350 KG    │ 5,200 KG  │ ✅ Sufficient     │      │
│  │ Olive Oil        │ 25 LTR    │ 320 LTR   │ ✅ Sufficient     │      │
│  │ Yeast            │ 10 KG     │ 8 KG      │ ❌ Shortage       │      │
│  └──────────────────────────────────────────────────────────────┘      │
│                                                                         │
│  ┌─ Routing Steps ──────────────────────────────────────────────┐      │
│  │ Step │ Operation      │ Work Center │ Time    │ Status       │      │
│  │ 10   │ Mixing         │ WC-001      │ 2.0 hr  │ ○ Pending    │      │
│  │ 20   │ Proofing       │ WC-002      │ 4.0 hr  │ ○ Pending    │      │
│  │ 30   │ Baking         │ WC-003      │ 1.5 hr  │ ○ Pending    │      │
│  │ 40   │ Packaging      │ WC-004      │ 1.0 hr  │ ○ Pending    │      │
│  └──────────────────────────────────────────────────────────────┘      │
│                                                                         │
│  [Release]  [Confirm]  [Print]                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

**Features:** Bill of Materials (BOMs), Routings (manufacturing sequences), Production Orders with status machine, Component availability check, Production analysis & lead time reports

---

### HR — Human Resources

```
┌─ HR > Employees ───────────────────────────────────────────────────────┐
│                                                                         │
│  [+ New Employee]  [Export CSV]           🔍 Search by name/dept...     │
│                                                                         │
│  ┌────────┬──────────────┬──────────────┬──────────┬─────────────┐    │
│  │ ID     │ Name         │ Department   │ Position │ Status      │    │
│  ├────────┼──────────────┼──────────────┼──────────┼─────────────┤    │
│  │ E-001  │ John Chen    │ Production   │ Manager  │ 🟢 Active   │    │
│  │ E-002  │ Sarah Lin    │ Sales        │ Rep      │ 🟢 Active   │    │
│  │ E-003  │ Mike Wang    │ Warehouse    │ Staff    │ 🟡 On Leave │    │
│  └────────┴──────────────┴──────────────┴──────────┴─────────────┘    │
│                                                                         │
│  ┌─ Attendance Today ──────────────────────────────────────────┐       │
│  │  Total: 89  │  Present: 82  │  On Leave: 5  │  Absent: 2   │       │
│  └─────────────────────────────────────────────────────────────┘       │
└─────────────────────────────────────────────────────────────────────────┘
```

**Features:** Department & Position management, Employee CRUD & export, Attendance (clock in/out), Salary Structures, Payroll Runs, Headcount & attendance summary reports

---

### WM — Warehouse Management

```
┌─ WM > Warehouse Overview ─────────────────────────────────────────────┐
│                                                                         │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐       │
│  │ Main Warehouse  │  │ Cold Storage     │  │ Packaging Area  │       │
│  │ Bins: 120/150   │  │ Bins: 45/50      │  │ Bins: 30/40     │       │
│  │ ████████████░░  │  │ █████████████░   │  │ █████████░░░░   │       │
│  │ 80% utilized    │  │ 90% utilized     │  │ 75% utilized    │       │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘       │
│                                                                         │
│  Recent Transfers:                                                      │
│  • TR-001: M-0001 (500 KG) Main → Cold Storage    ✅ Completed        │
│  • TR-002: M-0003 (200 PC) Packaging → Main       🔄 In Progress     │
└─────────────────────────────────────────────────────────────────────────┘
```

**Features:** Warehouse & Storage Bin management, Stock Transfers, Stock Counts (inventory adjustment), Warehouse utilization & transfer summary reports

---

### QM — Quality Management

```
┌─ QM > Inspection Lots ────────────────────────────────────────────────┐
│                                                                         │
│  ┌────────┬──────────────┬────────────┬──────────┬──────────────┐     │
│  │ Lot #  │ Material     │ Quantity   │ Result   │ Date         │     │
│  ├────────┼──────────────┼────────────┼──────────┼──────────────┤     │
│  │ IL-042 │ Premium Flour│ 2,000 KG   │ ✅ Pass  │ 2026-03-15   │     │
│  │ IL-041 │ Olive Oil    │ 100 LTR    │ ✅ Pass  │ 2026-03-14   │     │
│  │ IL-040 │ Vanilla Ext. │ 50 ML      │ ❌ Fail  │ 2026-03-13   │     │
│  └────────┴──────────────┴────────────┴──────────┴──────────────┘     │
│                                                                         │
│  Pass Rate This Month: 94.2%  ▲ 2.1%                                  │
└─────────────────────────────────────────────────────────────────────────┘
```

**Features:** Inspection Lots, Inspection Results (pass/fail), Quality Notifications, Inspection pass rate & notification summary reports

---

### CO — Controlling

```
┌─ CO > Cost Center Summary ────────────────────────────────────────────┐
│                                                                         │
│  Period: March 2026                    [Budget vs Actual Report]       │
│                                                                         │
│  ┌──────────────┬──────────┬──────────┬──────────┬─────────────┐      │
│  │ Cost Center  │ Budget   │ Actual   │ Variance │ Status      │      │
│  ├──────────────┼──────────┼──────────┼──────────┼─────────────┤      │
│  │ Production   │ $150,000 │ $142,300 │ -$7,700  │ 🟢 Under    │      │
│  │ Sales & Mktg │ $80,000  │ $83,500  │ +$3,500  │ 🔴 Over     │      │
│  │ Admin & HR   │ $45,000  │ $44,200  │ -$800    │ 🟢 Under    │      │
│  │ Warehouse    │ $35,000  │ $34,100  │ -$900    │ 🟢 Under    │      │
│  └──────────────┴──────────┴──────────┴──────────┴─────────────┘      │
└─────────────────────────────────────────────────────────────────────────┘
```

**Features:** Cost Centers, Profit Centers, Internal Orders (project accounting), Cost Allocations, Auto-posting, Budget vs Actual reports

---

### Low-Code Platform

```
┌─ Developer > Form Builder ─────────────────────────────────────────────┐
│                                                                         │
│  Operation: Customer Feedback Form          [Save] [Preview] [Release] │
│                                                                         │
│  ┌─ Field Palette ──┐  ┌─ Canvas ──────────────────────────────────┐  │
│  │                   │  │                                            │  │
│  │  📝 Text          │  │  ┌─ Customer Name ──────────────────┐     │  │
│  │  🔢 Number        │  │  │ [________________________]       │     │  │
│  │  📅 Date          │  │  └──────────────────────────────────┘     │  │
│  │  📋 Select        │  │                                            │  │
│  │  ☑️ Checkbox      │  │  ┌─ Rating ─────────────────────────┐     │  │
│  │  📎 File Upload   │  │  │ [★ ★ ★ ★ ☆] (1-5)               │     │  │
│  │                   │  │  └──────────────────────────────────┘     │  │
│  │  ─────────────── │  │                                            │  │
│  │  🔗 Data Source   │  │  ┌─ Comments ───────────────────────┐     │  │
│  │  🔒 Permissions   │  │  │ [                                ]│     │  │
│  │  ⚡ Actions       │  │  │ [                                ]│     │  │
│  │  📊 List View     │  │  └──────────────────────────────────┘     │  │
│  │  📈 Dashboard     │  │                                            │  │
│  └───────────────────┘  │  [Submit]  [Reset]                         │  │
│                          └──────────────────────────────────────────┘  │
│                                                                         │
│  ┌─ Field Settings ────────────────────────────────────────────────┐   │
│  │ Label: Customer Name    Type: text    Required: ☑               │   │
│  │ Placeholder: Enter customer name...                              │   │
│  │ Validation: Min 2 chars, Max 100 chars                          │   │
│  └──────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────┘
```

**Features:**
- **Form Builder** — drag & drop WYSIWYG field editor
- **List Builder** — custom list views with columns, sorting, filtering
- **Dashboard Builder** — KPI visualization designer
- **Data Sources** — query builder with SQL validation & table browser
- **Workflow Engine** — configurable status transitions
- **Document Flow** — automatic document linking across operations
- **Release Management** — development → approval → production lifecycle
- **Version Control** — journal history with rollback capability
- **AI Chat Assistant** — integrated LLM (OpenAI/Claude) for developer help
- **Bulk Import/Export** — CSV upload/download support
- **Record & Field-level Security** — granular row policies & permission control

---

### Admin Panel

```
┌─ Admin > User Management ──────────────────────────────────────────────┐
│                                                                         │
│  ┌─ Users ──┐  ┌─ Roles ──┐  ┌─ Permissions ──┐  ┌─ Navigation ──┐  │
│                                                                         │
│  ┌──────────┬──────────────┬──────────────────┬────────────────────┐   │
│  │ Username │ Display Name │ Roles            │ Last Login         │   │
│  ├──────────┼──────────────┼──────────────────┼────────────────────┤   │
│  │ admin    │ System Admin │ ADMIN, PLATFORM  │ 2026-03-17 09:00   │   │
│  │ jchen    │ John Chen    │ MANAGER, FI, MM  │ 2026-03-17 08:45   │   │
│  │ slin     │ Sarah Lin    │ USER, SD         │ 2026-03-16 17:30   │   │
│  └──────────┴──────────────┴──────────────────┴────────────────────┘   │
│                                                                         │
│  RBAC: Operation-level → Field-level → Record-level permissions        │
└─────────────────────────────────────────────────────────────────────────┘
```

**Features:** User management, Role-based access control (RBAC), Hierarchical permissions, Account lockout protection, Navigation builder, Project & release management

---

### Login & Authentication

```
┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│                                                                         │
│                         ┌─────────────────────┐                        │
│                         │                     │                        │
│                         │    🍴 TasteByte     │                        │
│                         │       ERP           │                        │
│                         │                     │                        │
│                         │  ┌───────────────┐  │                        │
│                         │  │ Username      │  │                        │
│                         │  └───────────────┘  │                        │
│                         │  ┌───────────────┐  │                        │
│                         │  │ Password  👁  │  │                        │
│                         │  └───────────────┘  │                        │
│                         │                     │                        │
│                         │  [    Login     ]   │                        │
│                         │                     │                        │
│                         │  Don't have an      │                        │
│                         │  account? Register  │                        │
│                         └─────────────────────┘                        │
│                                                                         │
│                           [EN] | [繁體中文]                              │
└─────────────────────────────────────────────────────────────────────────┘
```

**Features:** JWT authentication with refresh tokens, Rate limiting (10 req/min login, 5 req/min register), Argon2 password hashing, Account lockout after failed attempts, i18n (English / 繁體中文)

---

## Document Workflow System

All business documents follow a state machine pattern with full audit trail:

```
Purchase Order:     CREATED ──→ RELEASED ──→ RECEIVED ──→ CLOSED
                         └──→ CANCELLED

Sales Order:        DRAFT ──→ CONFIRMED ──→ DELIVERED ──→ INVOICED ──→ CLOSED
                        └──→ CANCELLED      └──→ PARTIALLY_DELIVERED

Journal Entry:      DRAFT ──→ POSTED
                        └──→ CANCELLED

Production Order:   CREATED ──→ RELEASED ──→ IN_PROGRESS ──→ COMPLETED
                         └──→ CANCELLED

Invoice:            DRAFT ──→ POSTED ──→ PAID
                        └──→ CANCELLED    └──→ PARTIALLY_PAID
```

Every transition is validated, timestamped, and recorded with the user who performed it.

---

## Cross-Module Integration

```
   ┌─────────┐         ┌─────────┐         ┌─────────┐
   │   SD    │────────→│   MM    │────────→│   FI    │
   │ Sales   │ Delivery │ Stock   │ Posting  │ Journal │
   │ Order   │ triggers │ Movement│ triggers │ Entry   │
   └─────────┘         └─────────┘         └─────────┘
       │                    │                    │
       │                    ▼                    ▼
       │               ┌─────────┐         ┌─────────┐
       │               │   WM    │         │   CO    │
       │               │ Transfer│         │ Cost    │
       │               │ Order   │         │ Posting │
       │               └─────────┘         └─────────┘
       │
       ▼
   ┌─────────┐         ┌─────────┐
   │   PP    │────────→│   QM    │
   │ Prod.   │ Creates  │ Inspect │
   │ Order   │ lot for  │ Lot     │
   └─────────┘         └─────────┘
```

---

## Mobile Apps

The system ships with **three mobile clients** for field operations:

| Platform | Technology | Features |
|----------|-----------|----------|
| **Flutter** | Dart + Dio | Cross-platform, all modules, offline-capable |
| **iOS Native** | Swift + SwiftUI | Inventory scan, attendance, quality checks |
| **Android Native** | Kotlin + Jetpack Compose | Warehouse ops, HR attendance, sales orders |

```
┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐
│  📱 Flutter       │  │  📱 iOS           │  │  📱 Android       │
│                    │  │                    │  │                    │
│  ┌──────────────┐ │  │  ┌──────────────┐ │  │  ┌──────────────┐ │
│  │  Dashboard   │ │  │  │  Materials   │ │  │  │  Warehouse   │ │
│  │  ┌────┐┌───┐│ │  │  │  Scan QR to  │ │  │  │  Stock Count │ │
│  │  │ KPI││KPI││ │  │  │  lookup      │ │  │  │  ┌────────┐  │ │
│  │  └────┘└───┘│ │  │  │  material    │ │  │  │  │ Bin A1 │  │ │
│  │  ┌────┐┌───┐│ │  │  │  details     │ │  │  │  │ 120 KG │  │ │
│  │  │ KPI││KPI││ │  │  │              │ │  │  │  └────────┘  │ │
│  │  └────┘└───┘│ │  │  │  [📷 Scan]   │ │  │  │  ┌────────┐  │ │
│  └──────────────┘ │  │  └──────────────┘ │  │  │  │ Bin A2 │  │ │
│                    │  │                    │  │  │  │  85 KG │  │ │
│  ━━━━━━━━━━━━━━━━ │  │  ━━━━━━━━━━━━━━━━ │  │  │  └────────┘  │ │
│  🏠 📦 🛒 👤      │  │  🏠 📦 🛒 👤      │  │  └──────────────┘ │
└──────────────────┘  └──────────────────┘  │  ━━━━━━━━━━━━━━━━ │
                                             │  🏠 📦 🏪 👤      │
                                             └──────────────────┘
```

---

## Tech Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Backend** | Rust + Axum + Tokio | High-performance async API server |
| **Database** | PostgreSQL 17 + SQLx | Compile-time checked SQL queries |
| **Web Frontend** | Next.js 15 + React 19 + Tailwind CSS 4 | Modern SSR/CSR hybrid |
| **State Management** | Zustand + TanStack Query | Client state + server cache |
| **Tables** | TanStack Table | Headless table with sorting/filtering |
| **Forms** | React Hook Form + Zod | Type-safe form validation |
| **Charts** | Recharts | Data visualization |
| **Mobile** | Flutter / SwiftUI / Jetpack Compose | Native multi-platform |
| **Auth** | JWT + Argon2 + Refresh Tokens | Secure authentication |
| **i18n** | next-intl | English + 繁體中文 |
| **Proxy** | Nginx | Reverse proxy + SSL termination |
| **Monitoring** | Prometheus + Sentry | Metrics + error tracking |
| **Container** | Docker Compose | Full stack orchestration |

---

## Quick Start

### Prerequisites

- Rust (2021 edition)
- Node.js 20+ & pnpm
- PostgreSQL 17
- Docker & Docker Compose (optional)

### Docker (Recommended)

```bash
# Clone and start all services
cp .env.docker.example .env
docker compose up -d

# Access the application
# Web:     http://localhost:3000
# API:     http://localhost:8000
# Swagger: http://localhost:8000/api/v1
```

### Local Development

```bash
# Backend
cd backend
cp .env.example .env  # configure DATABASE_URL
cargo run

# Frontend
cd web
pnpm install
pnpm dev

# Run tests
make test
# or individually:
cd backend && cargo test --lib
cd web && pnpm test
```

### Makefile Commands

```bash
make dev          # Start development servers
make build        # Build all services
make test         # Run all tests
make lint         # Run linters
make docker-up    # Start Docker stack
make docker-down  # Stop Docker stack
make db-backup    # Backup database
make db-restore   # Restore database
```

---

## Project Structure

```
TasteByte_ERP/
├── backend/                # Rust/Axum API (160+ endpoints)
│   ├── src/
│   │   ├── auth/           # Authentication & RBAC
│   │   ├── fi/             # Financial Accounting (23 endpoints)
│   │   ├── co/             # Controlling (12 endpoints)
│   │   ├── mm/             # Materials Management (23 endpoints)
│   │   ├── sd/             # Sales & Distribution (15 endpoints)
│   │   ├── pp/             # Production Planning (10 endpoints)
│   │   ├── hr/             # Human Resources (11 endpoints)
│   │   ├── wm/             # Warehouse Management (10 endpoints)
│   │   ├── qm/             # Quality Management (9 endpoints)
│   │   ├── lowcode/        # Low-Code Platform (67+ endpoints)
│   │   ├── notifications/  # Notification system
│   │   ├── middleware/      # CORS, logging, metrics, rate limiting
│   │   └── shared/         # Common utilities, error handling, pagination
│   ├── migrations/         # 31 PostgreSQL migrations (74 tables)
│   └── tests/              # Integration tests (10 modules)
├── web/                    # Next.js 15 Frontend
│   ├── src/
│   │   ├── app/            # App Router pages (14 module routes)
│   │   ├── components/     # Reusable UI (40+ components)
│   │   ├── lib/            # API clients, hooks, stores, types
│   │   └── __tests__/      # Vitest unit tests (96 tests)
│   ├── e2e/                # Playwright E2E tests (5 specs)
│   └── messages/           # i18n (en.json, zh-TW.json)
├── mobile/                 # Flutter cross-platform app
├── ios/                    # Native iOS (SwiftUI)
├── android/                # Native Android (Jetpack Compose)
├── docs/                   # Technical documentation
│   ├── API_REFERENCE.md    # All API endpoints
│   ├── DATABASE_SCHEMA.md  # Complete schema reference
│   ├── TECHNICAL_MANUAL.md # Architecture & deployment guide
│   └── WORKFLOW_GUIDE.md   # Business process workflows
├── nginx/                  # Reverse proxy config
├── scripts/                # Backup/restore utilities
├── docker-compose.yml      # Full stack orchestration
└── Makefile                # Development commands
```

---

## API Highlights

Base URL: `http://localhost:8000/api/v1`

| Module | Endpoints | Example |
|--------|-----------|---------|
| Auth | 5 | `POST /auth/login`, `POST /auth/refresh` |
| FI | 23 | `GET /fi/accounts`, `POST /fi/journal-entries` |
| CO | 12 | `GET /co/cost-centers`, `POST /co/allocations` |
| MM | 23 | `GET /mm/materials`, `POST /mm/purchase-orders` |
| SD | 15 | `GET /sd/sales-orders`, `POST /sd/deliveries` |
| PP | 10 | `GET /pp/boms`, `POST /pp/production-orders` |
| HR | 11 | `GET /hr/employees`, `POST /hr/payroll-runs` |
| WM | 10 | `GET /wm/warehouses`, `POST /wm/transfers` |
| QM | 9 | `GET /qm/inspection-lots`, `POST /qm/results` |
| Lowcode | 67+ | `GET /lowcode/projects`, `POST /lowcode/forms/execute` |
| **Total** | **160+** | |

See [docs/API_REFERENCE.md](docs/API_REFERENCE.md) for complete documentation.

---

## Security

- **Authentication**: JWT with Argon2 password hashing & refresh tokens
- **Authorization**: Hierarchical RBAC (Operation → Field → Record level)
- **Rate Limiting**: Login (10/min), Register (5/min) per IP
- **Account Lockout**: Auto-lock after repeated failed attempts
- **Audit Trail**: All changes logged with timestamp, user, and operation
- **SQL Injection Prevention**: Compile-time checked queries via SQLx
- **CORS**: Configurable cross-origin policy
- **Request Tracing**: Unique request IDs for debugging

---

## Documentation

| Document | Description |
|----------|-------------|
| [API Reference](docs/API_REFERENCE.md) | Complete REST API documentation (160+ endpoints) |
| [Database Schema](docs/DATABASE_SCHEMA.md) | All 74 tables with relationships & indexes |
| [Technical Manual](docs/TECHNICAL_MANUAL.md) | Architecture, deployment & development guide |
| [Workflow Guide](docs/WORKFLOW_GUIDE.md) | Business process flows & state machines |

---

## License

Proprietary - TasteByte Inc. All rights reserved.
