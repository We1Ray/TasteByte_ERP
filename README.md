<p align="center">
  <img src="https://img.shields.io/badge/Rust-Axum_0.8-orange?style=for-the-badge&logo=rust" />
  <img src="https://img.shields.io/badge/Next.js-16-black?style=for-the-badge&logo=next.js" />
  <img src="https://img.shields.io/badge/PostgreSQL-17-336791?style=for-the-badge&logo=postgresql" />
  <img src="https://img.shields.io/badge/Tests-256_pass-brightgreen?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Flutter-3.x-02569B?style=for-the-badge&logo=flutter" />
  <img src="https://img.shields.io/badge/Swift-6.0-FA7343?style=for-the-badge&logo=swift" />
  <img src="https://img.shields.io/badge/Kotlin-1.9-7F52FF?style=for-the-badge&logo=kotlin" />
</p>

# TasteByte ERP

> A production-grade, full-stack Enterprise Resource Planning system built for the food & beverage industry — featuring 8 SAP-style modules, a YAML-driven low-code platform, 260+ API endpoints, 123 database tables, and multi-platform clients (Web + iOS + Android + Flutter).

---

## Demo Animations

### Login Flow
<p align="center">
  <img src="docs/videos/login-flow.gif" alt="Login Flow" width="720" />
</p>

### Module Navigation Tour
<p align="center">
  <img src="docs/videos/module-tour.gif" alt="Module Tour" width="720" />
</p>

### Create Journal Entry
<p align="center">
  <img src="docs/videos/create-journal.gif" alt="Create Journal Entry" width="720" />
</p>

### Sales Order Workflow
<p align="center">
  <img src="docs/videos/sales-order-detail.gif" alt="Sales Order Detail" width="720" />
</p>

### Purchase Order Flow
<p align="center">
  <img src="docs/videos/purchase-order-flow.gif" alt="Purchase Order Flow" width="720" />
</p>

### Developer & Low-Code Platform Workflow
<p align="center">
  <img src="docs/videos/developer-workflow.gif" alt="Developer Workflow" width="720" />
</p>

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
                    │  16 Web   │  │  Swift  │  │  Flutter    │
                    │  :3000    │  │  UI     │  │  Mobile     │
                    └────────┬──┘  └────┬────┘  └──┬──────────┘
                             │          │          │
                             └──────────┼──────────┘
                                        │ REST API (JSON)
                              ┌─────────▼─────────┐
                              │   Rust / Axum      │
                              │   Backend :8000    │
                              │   260+ Endpoints   │
                              └─────────┬──────────┘
                                        │ SQLx (compile-time checked)
                              ┌─────────▼──────────┐
                              │   PostgreSQL 17     │
                              │   123 Tables        │
                              │   44 Migrations     │
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

## Screenshots & Features

### Login & Authentication

<p align="center">
  <img src="docs/screenshots/01-login.png" alt="Login Page" width="720" />
</p>

- JWT authentication with refresh tokens
- Rate limiting (10 req/min login, 5 req/min register)
- Argon2 password hashing
- Account lockout after failed attempts
- i18n support (English / 繁體中文)

---

### Dashboard — Real-time KPIs & Analytics

<p align="center">
  <img src="docs/screenshots/02-dashboard.png" alt="Dashboard" width="720" />
</p>

- KPI cards: total revenue, sales orders, inventory, production orders
- AR/AP summary
- Revenue & cost trend chart (Recharts bar chart)
- Order trend chart (line chart)
- Recent sales orders & quick actions

---

### FI — Financial Accounting

<p align="center">
  <img src="docs/screenshots/03-fi-accounts.png" alt="FI Chart of Accounts" width="720" />
</p>

<p align="center">
  <img src="docs/screenshots/04-fi-journal.png" alt="FI Journal Entries" width="720" />
</p>

<p align="center">
  <img src="docs/screenshots/05-fi-reports.png" alt="FI Financial Reports" width="720" />
</p>

- Chart of Accounts with account groups
- Journal Entries (DRAFT → POSTED workflow)
- AR/AP Invoices & Payment recording
- Reports: Trial Balance, Income Statement, Balance Sheet, AR/AP Aging

---

### MM — Materials Management

<p align="center">
  <img src="docs/screenshots/06-mm-materials.png" alt="MM Materials" width="720" />
</p>

<p align="center">
  <img src="docs/screenshots/07-mm-purchase-orders.png" alt="MM Purchase Orders" width="720" />
</p>

<p align="center">
  <img src="docs/screenshots/08-mm-stock.png" alt="MM Stock Overview" width="720" />
</p>

<p align="center">
  <img src="docs/screenshots/09-mm-reports.png" alt="MM Reports" width="720" />
</p>

- Materials CRUD with soft delete, Print & Export
- UOM & Vendor management
- Purchase Orders (CREATED → RELEASED → RECEIVED → CLOSED)
- Plant Stock tracking & Material Movements
- Reports: Stock Valuation, Movement Summary, Slow-moving Items

---

### SD — Sales & Distribution

<p align="center">
  <img src="docs/screenshots/10-sd-customers.png" alt="SD Customers" width="720" />
</p>

<p align="center">
  <img src="docs/screenshots/11-sd-sales-orders.png" alt="SD Sales Orders" width="720" />
</p>

<p align="center">
  <img src="docs/screenshots/12-sd-invoices.png" alt="SD Invoices" width="720" />
</p>

<p align="center">
  <img src="docs/screenshots/13-sd-reports.png" alt="SD Reports" width="720" />
</p>

- Customer management
- Sales Orders with auto-numbering (SO00000001+)
- Deliveries & Invoices
- Document flow tracking (Sales Order → Delivery → Invoice → Payment)
- Reports: Sales Summary, Order Fulfillment, Top Customers

---

### PP — Production Planning

<p align="center">
  <img src="docs/screenshots/14-pp-boms.png" alt="PP Bill of Materials" width="720" />
</p>

<p align="center">
  <img src="docs/screenshots/15-pp-production-orders.png" alt="PP Production Orders" width="720" />
</p>

- Bill of Materials (BOMs) with component hierarchy
- Routings (manufacturing operation sequences)
- Production Orders with status machine
- Production analysis & lead time reports

---

### HR — Human Resources

<p align="center">
  <img src="docs/screenshots/16-hr-employees.png" alt="HR Employees" width="720" />
</p>

<p align="center">
  <img src="docs/screenshots/17-hr-attendance.png" alt="HR Attendance" width="720" />
</p>

- Department & Position management
- Employee CRUD with export
- Attendance tracking (clock in/out)
- Salary Structures & Payroll Runs
- Reports: Headcount by Department, Attendance Summary

---

### WM — Warehouse Management

<p align="center">
  <img src="docs/screenshots/18-wm-warehouse.png" alt="WM Warehouse" width="720" />
</p>

- Warehouse & Storage Bin management
- Stock Transfers between locations
- Stock Counts (inventory adjustment)
- Reports: Warehouse Utilization, Transfer Summary

---

### QM — Quality Management

<p align="center">
  <img src="docs/screenshots/19-qm-quality.png" alt="QM Quality" width="720" />
</p>

- Inspection Lots with pass/fail results
- Quality Notifications
- Reports: Inspection Pass Rate, Notification Summary

---

### CO — Controlling

<p align="center">
  <img src="docs/screenshots/20-co-controlling.png" alt="CO Controlling" width="720" />
</p>

- Cost Centers & Profit Centers
- Internal Orders (project accounting)
- Cost Allocations & Auto-posting
- Reports: Cost Center Summary, Budget vs Actual

---

### Admin Panel

<p align="center">
  <img src="docs/screenshots/21-admin-users.png" alt="Admin Users" width="720" />
</p>

<p align="center">
  <img src="docs/screenshots/22-admin-roles.png" alt="Admin Roles" width="720" />
</p>

- User management with ERP & Platform roles
- Hierarchical RBAC (Operation → Field → Record level)
- Account lockout protection
- Navigation builder
- Project & release management

---

### Low-Code Developer Platform

<p align="center">
  <img src="docs/screenshots/23-developer-hub.png" alt="Developer Hub" width="720" />
</p>

> Developer dashboard showing 6 operations across 2 projects, 3 releases, and recent activity

<p align="center">
  <img src="docs/screenshots/24-developer-operations.png" alt="Developer Operations" width="720" />
</p>

> Operations management with cross-project forms: Material Request, Customer Feedback, Equipment Maintenance, Food Safety Checklist, Supplier Evaluation, Employee Training

<p align="center">
  <img src="docs/screenshots/28-lowcode-platform.png" alt="Low-Code Platform" width="720" />
</p>

> Published operations directory — all 6 forms available as cards for end users

<p align="center">
  <img src="docs/screenshots/29-lowcode-form-data.png" alt="Module-linked Operation" width="720" />
</p>

> Cross-module integration: Material Request Form linked to MM module sidebar, with dynamic navigation

<p align="center">
  <img src="docs/screenshots/26-developer-feedback.png" alt="Developer Feedback" width="720" />
</p>

<p align="center">
  <img src="docs/screenshots/31-admin-projects.png" alt="Admin Projects" width="720" />
</p>

<p align="center">
  <img src="docs/screenshots/32-admin-releases.png" alt="Admin Releases" width="720" />
</p>

**Cross-module operations (6 forms linked to ERP modules):**
| Operation | Module | Description |
|-----------|--------|-------------|
| Material Request Form | MM | Request new materials for inventory |
| Customer Feedback Form | SD | Collect customer satisfaction feedback |
| Equipment Maintenance Log | PP | Log production equipment maintenance |
| Food Safety Checklist | QM | Daily food safety inspection checklist |
| Supplier Evaluation Form | MM | Evaluate supplier performance |
| Employee Training Record | HR | Track employee training & certifications |

**Platform features:**
- **Form Builder** — WYSIWYG field editor with sections, dropdowns, checkboxes
- **List Builder** — custom list views with columns, sorting, filtering
- **Dashboard Builder** — KPI visualization designer
- **Cross-module Linking** — operations appear in ERP module sidebars (MM, SD, PP, QM, HR)
- **Release Management** — development → approval → production lifecycle
- **Version Control** — journal history with rollback capability
- **Feedback System** — bug reports, feature requests with comment threads
- **Record & Field-level Security** — granular row policies & permission control
- **AI Chat Assistant** — integrated LLM (OpenAI/Claude) for developer help

---

### Notification Center

<p align="center">
  <img src="docs/screenshots/33-notifications.png" alt="Notifications" width="720" />
</p>

- In-app notification system
- Read/unread tracking
- Mark all as read

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

---

## Tech Stack

| Layer | Technology | Purpose |
|-------|-----------|---------|
| **Backend** | Rust + Axum + Tokio | High-performance async API server |
| **Database** | PostgreSQL 17 + SQLx | Compile-time checked SQL queries |
| **Web Frontend** | Next.js 16 + React 19 + Tailwind CSS 4 | Modern SSR/CSR hybrid |
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

- Rust 1.75+ (2021 edition)
- Node.js 20+ & pnpm 9+
- PostgreSQL 17
- Docker & Docker Compose (optional, recommended)
- Flutter SDK 3.x+ (for cross-platform mobile)

### Docker (Recommended)

```bash
# Clone and start all services
cp .env.docker.example .env
docker compose up -d

# Access the application
# Web:     http://localhost:3000
# API:     http://localhost:8000
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
├── backend/                # Rust/Axum API (260+ endpoints)
│   ├── src/
│   │   ├── auth/           # Authentication & RBAC (14 endpoints)
│   │   ├── fi/             # Financial Accounting (19 endpoints)
│   │   ├── co/             # Controlling (9 endpoints)
│   │   ├── mm/             # Materials Management (24 endpoints)
│   │   ├── sd/             # Sales & Distribution (13 endpoints)
│   │   ├── pp/             # Production Planning (11 endpoints)
│   │   ├── hr/             # Human Resources (18 endpoints)
│   │   ├── wm/             # Warehouse Management (10 endpoints)
│   │   ├── qm/             # Quality Management (9 endpoints)
│   │   ├── lowcode/        # Low-Code Platform (71 endpoints)
│   │   ├── notifications/  # Notification system (5 endpoints)
│   │   ├── middleware/      # CORS, logging, metrics, rate limiting
│   │   └── shared/         # Common utilities, admin API (55 system endpoints)
│   ├── operations/         # YAML operation definitions (7 files)
│   ├── migrations/         # 44 PostgreSQL migrations (123 tables)
│   └── tests/              # Integration tests (10 modules)
├── web/                    # Next.js 16 Frontend
│   ├── src/
│   │   ├── app/            # App Router pages (14 module routes)
│   │   ├── components/     # Reusable UI (40+ components)
│   │   ├── lib/            # API clients (16), hooks (8), stores (7), types
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
| Auth | 14 | `POST /auth/login`, `POST /auth/refresh` |
| FI | 19 | `GET /fi/accounts`, `POST /fi/journal-entries` |
| CO | 9 | `GET /co/cost-centers`, `POST /co/allocations` |
| MM | 24 | `GET /mm/materials`, `POST /mm/purchase-orders` |
| SD | 13 | `GET /sd/sales-orders`, `POST /sd/deliveries` |
| PP | 11 | `GET /pp/boms`, `POST /pp/production-orders` |
| HR | 18 | `GET /hr/employees`, `POST /hr/payroll-runs` |
| WM | 10 | `GET /wm/warehouses`, `POST /wm/transfers` |
| QM | 9 | `GET /qm/inspection-lots`, `POST /qm/results` |
| Lowcode | 71 | `GET /lowcode/projects`, `POST /lowcode/forms/execute` |
| Notifications | 5 | `GET /notifications`, `PUT /notifications/read` |
| System | 55 | `GET /system/approvals/pending`, `POST /system/workflows` |
| **Total** | **260+** | |

See [docs/API_REFERENCE.md](docs/API_REFERENCE.md) for complete documentation.

> Endpoint counts are based on HTTP handler registrations in route files (get/post/put/delete methods).

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
| [API Reference](docs/API_REFERENCE.md) | Complete REST API documentation (260+ endpoints) |
| [Database Schema](docs/DATABASE_SCHEMA.md) | All 123 tables with relationships & indexes |
| [Technical Manual](docs/TECHNICAL_MANUAL.md) | Architecture, deployment & development guide |
| [Workflow Guide](docs/WORKFLOW_GUIDE.md) | Business process flows & state machines |

---

## License

Proprietary - TasteByte Inc. All rights reserved.
