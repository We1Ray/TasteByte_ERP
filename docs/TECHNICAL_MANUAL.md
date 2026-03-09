# TasteByte ERP - Technical Manual

> Version 1.0 | Last Updated: 2026-02-21

## Table of Contents

1. [System Architecture](#1-system-architecture)
2. [Tech Stack](#2-tech-stack)
3. [Project Structure](#3-project-structure)
4. [Authentication & Authorization](#4-authentication--authorization)
5. [ERP Module Overview](#5-erp-module-overview)
6. [Cross-Module Integrations](#6-cross-module-integrations)
7. [Low-Code Platform](#7-low-code-platform)
8. [Development Guide](#8-development-guide)
9. [Deployment](#9-deployment)

---

## 1. System Architecture

```
                         ┌──────────────────────────────────┐
                         │          Load Balancer           │
                         └──────────────┬───────────────────┘
                                        │
              ┌─────────────────────────┼─────────────────────────┐
              │                         │                         │
    ┌─────────▼─────────┐   ┌──────────▼──────────┐   ┌─────────▼─────────┐
    │   Next.js 15 Web  │   │   iOS App (Native)  │   │ Android App(Native│
    │   Port: 3000      │   │   Swift / SwiftUI   │   │ Kotlin / Compose  │
    │   App Router       │   │   MVVM Pattern      │   │ MVVM + Retrofit   │
    └─────────┬─────────┘   └──────────┬──────────┘   └─────────┬─────────┘
              │                         │                         │
              └─────────────────────────┼─────────────────────────┘
                                        │
                              REST API (JSON)
                                        │
                         ┌──────────────▼───────────────────┐
                         │     Rust / Axum 0.8 Backend     │
                         │         Port: 8000               │
                         │   JWT Auth + RBAC Middleware      │
                         │   18 SQL Migrations (auto-run)   │
                         └──────────────┬───────────────────┘
                                        │
                              SQLx 0.8 (async)
                                        │
                         ┌──────────────▼───────────────────┐
                         │    PostgreSQL 17 (Local)         │
                         │    Port: 5432 | DB: TastyByte    │
                         │    74 Tables | 18 Migrations     │
                         └──────────────────────────────────┘
```

### Design Principles

- **SAP-like Module Design**: 8 standard ERP modules (FI, CO, MM, SD, PP, HR, WM, QM)
- **Document Flow**: Auto-generated document numbers (`SO00000001`, `PO00000001`)
- **Status Machine**: Validated state transitions with audit trail
- **Single Backend**: All business logic in one Rust binary
- **Native Mobile**: iOS (Swift) + Android (Kotlin), NOT cross-platform

---

## 2. Tech Stack

| Layer | Technology | Version | Purpose |
|-------|-----------|---------|---------|
| Backend | Rust + Axum | 0.8 | API server, business logic |
| ORM | SQLx | 0.8 | Async PostgreSQL driver |
| Database | PostgreSQL | 17 | Primary data store |
| Web Frontend | Next.js | 15 | ERP management UI |
| CSS | Tailwind CSS | 4.x | Utility-first styling |
| State (Server) | TanStack Query | 5.x | API cache & mutations |
| State (Client) | Zustand | 5.x | Auth, UI, form builder state |
| Tables | TanStack Table | 8.x | Data grid rendering |
| Validation | Zod | 3.x | Form validation |
| Notifications | Sonner | 2.x | Toast notifications |
| iOS | Swift / SwiftUI | 6.0 | Native iOS app |
| Android | Kotlin / Jetpack Compose | 1.9 | Native Android app |
| Auth | JWT + Argon2 | — | Token auth + password hashing |
| HTTP Client | Axios | 1.x | Frontend API calls |

---

## 3. Project Structure

```
TasteByte_ERP/
├── backend/                          # Rust/Axum Backend
│   ├── Cargo.toml                    # Dependencies
│   ├── .env                          # DATABASE_URL config
│   ├── migrations/                   # 18 SQL migration files
│   │   ├── 001_foundation.sql        # Users, roles, audit_log
│   │   ├── 002_fi_accounts.sql       # FI chart of accounts
│   │   ├── 003_fi_journal.sql        # FI journal entries
│   │   ├── 004_mm_materials.sql      # MM materials, vendors
│   │   ├── 005_mm_inventory.sql      # MM POs, stock, movements
│   │   ├── 006_sd_sales.sql          # SD orders, deliveries, invoices
│   │   ├── 007_pp_production.sql     # PP BOMs, routings, orders
│   │   ├── 008_hr_employees.sql      # HR departments, employees
│   │   ├── 009_wm_warehouse.sql      # WM warehouses, transfers
│   │   ├── 010_qm_quality.sql        # QM inspections
│   │   ├── 011_co_controlling.sql    # CO cost centers
│   │   ├── 012_seed_data.sql         # Initial data
│   │   ├── 013_reporting_views.sql   # FI/SD/MM report views
│   │   ├── 014_lowcode_core.sql      # LC projects, operations, forms
│   │   ├── 015_lowcode_permissions.sql # LC RBAC
│   │   ├── 016_lowcode_workflow.sql  # LC releases, feedback, journal
│   │   ├── 017_lowcode_navigation.sql # LC navigation
│   │   └── 018_workflow_infrastructure.sql # Status history, CHECK constraints
│   └── src/
│       ├── main.rs                   # Entry point, server startup
│       ├── routes.rs                 # Top-level router
│       ├── auth/                     # Authentication module
│       │   ├── handlers.rs           # Login, register, refresh
│       │   ├── middleware.rs         # JWT extraction middleware
│       │   ├── models.rs             # Claims, LoginRequest
│       │   ├── rbac.rs               # RequireRole<R> extractor
│       │   ├── routes.rs             # /auth routes
│       │   └── services.rs           # Password hashing, token generation
│       ├── shared/                   # Shared infrastructure
│       │   ├── mod.rs                # Module exports
│       │   ├── audit.rs              # log_change() for audit trail
│       │   ├── error.rs              # AppError enum
│       │   ├── handlers.rs           # StatusHistory query handler
│       │   ├── pagination.rs         # PaginatedResponse, PaginationParams
│       │   ├── response.rs           # ApiResponse<T> wrapper
│       │   ├── status.rs             # DocumentType, validate_transition()
│       │   ├── status_history.rs     # record_transition()
│       │   └── types.rs              # AppState
│       ├── fi/                       # Financial Accounting (23 endpoints)
│       ├── co/                       # Controlling (12 endpoints)
│       ├── mm/                       # Materials Management (23 endpoints)
│       ├── sd/                       # Sales & Distribution (15 endpoints)
│       ├── pp/                       # Production Planning (10 endpoints)
│       ├── hr/                       # Human Resources (11 endpoints)
│       ├── wm/                       # Warehouse Management (10 endpoints)
│       ├── qm/                       # Quality Management (9 endpoints)
│       ├── lowcode/                  # Low-Code Platform (67 endpoints)
│       └── schema/                   # Migration engine
│           └── migrator.rs           # Auto-migration on startup
│
├── web/                              # Next.js 15 Frontend
│   ├── package.json                  # Dependencies
│   ├── next.config.ts                # Next.js config
│   ├── tailwind.config.ts            # Tailwind config
│   └── src/
│       ├── app/                      # App Router pages
│       │   ├── layout.tsx            # Root layout
│       │   ├── login/page.tsx        # Login page
│       │   ├── (erp)/               # ERP module pages
│       │   │   ├── layout.tsx        # ERP shell (sidebar + header)
│       │   │   ├── dashboard/        # Main dashboard
│       │   │   ├── fi/               # FI pages (7 routes)
│       │   │   ├── co/               # CO pages (5 routes)
│       │   │   ├── mm/               # MM pages (8 routes)
│       │   │   ├── sd/               # SD pages (6 routes)
│       │   │   ├── pp/               # PP pages (4 routes)
│       │   │   ├── hr/               # HR pages (3 routes)
│       │   │   ├── wm/               # WM pages (4 routes)
│       │   │   └── qm/               # QM pages (3 routes)
│       │   ├── admin/                # LC Admin pages
│       │   ├── developer/            # LC Developer pages
│       │   └── lowcode/              # LC User runtime pages
│       ├── components/
│       │   ├── layout/               # Sidebar, Header, PageHeader
│       │   ├── ui/                   # Button, Card, Input, Badge, Modal, Table
│       │   ├── shared/               # StatusBadge, WorkflowTimeline
│       │   └── lowcode/              # 27 form builder components
│       └── lib/
│           ├── api/                  # 12 API client modules
│           │   ├── client.ts         # Axios instance + interceptors
│           │   ├── auth.ts           # Auth API
│           │   ├── fi.ts             # FI API
│           │   ├── co.ts             # CO API
│           │   ├── mm.ts             # MM API
│           │   ├── sd.ts             # SD API
│           │   ├── pp.ts             # PP API
│           │   ├── hr.ts             # HR API
│           │   ├── wm.ts             # WM API
│           │   ├── qm.ts             # QM API
│           │   ├── lowcode.ts        # LC API (14 sub-modules)
│           │   └── dashboard.ts      # Dashboard KPI API
│           ├── hooks/                # 7 custom hooks
│           │   ├── use-auth.ts       # Auth flow
│           │   ├── use-api-query.ts  # React Query wrappers
│           │   ├── use-pagination.ts # Pagination state
│           │   ├── use-toast-mutation.ts # Toast-enabled mutations
│           │   ├── use-dynamic-form.ts   # Form builder hooks
│           │   ├── use-lookup.ts     # Dropdown/lookup hooks
│           │   └── use-platform-role.ts  # Platform role checks
│           ├── stores/               # 3 Zustand stores
│           │   ├── auth-store.ts     # Auth state
│           │   ├── ui-store.ts       # Layout state
│           │   └── builder-store.ts  # Form builder state
│           └── utils.ts              # formatCurrency, formatDate, etc.
│
├── ios/                              # iOS Native App (57 files)
│   └── TasteByte/
│       ├── Models/                   # API response models
│       ├── Services/                 # APIClient, AuthService
│       ├── ViewModels/               # MVVM view models
│       └── Views/                    # SwiftUI screens (8 modules)
│
├── android/                          # Android Native App (56 files)
│   └── app/src/main/
│       ├── data/                     # Repository, API service
│       ├── ui/                       # Compose screens (8 modules)
│       └── viewmodel/                # MVVM view models
│
└── docs/                             # Documentation
    ├── TECHNICAL_MANUAL.md           # This file
    ├── API_REFERENCE.md              # Complete API endpoint reference
    ├── DATABASE_SCHEMA.md            # All 74 tables
    └── WORKFLOW_GUIDE.md             # Status machines & workflow
```

---

## 4. Authentication & Authorization

### 4.1 Auth Flow

```
┌──────────┐     POST /auth/login      ┌──────────────┐
│  Client   │ ──────────────────────►   │   Backend    │
│           │     {username, password}   │              │
│           │                            │  1. Verify   │
│           │     {access_token,         │     Argon2   │
│           │ ◄──────────────────────    │  2. Generate │
│           │      refresh_token}        │     JWT      │
└──────────┘                            └──────────────┘
     │
     │  Authorization: Bearer <token>
     │  (all subsequent requests)
     ▼
┌──────────────────────────────────────────────────────┐
│                  JWT Claims                          │
│  {                                                   │
│    "sub": "uuid",          // user ID                │
│    "username": "admin",                              │
│    "roles": ["ADMIN"],     // ERP roles              │
│    "exp": 1740000000       // expiry                 │
│  }                                                   │
└──────────────────────────────────────────────────────┘
```

### 4.2 RBAC System

**Module-level Role Markers** (Axum extractors):

| Module | Read Marker | Write Marker |
|--------|------------|--------------|
| FI | `FiRead` | `FiWrite` |
| CO | `CoRead` | `CoWrite` |
| MM | `MmRead` | `MmWrite` |
| SD | `SdRead` | `SdWrite` |
| PP | `PpRead` | `PpWrite` |
| HR | `HrRead` | `HrWrite` |
| WM | `WmRead` | `WmWrite` |
| QM | `QmRead` | `QmWrite` |

**Usage in handlers:**
```rust
pub async fn list_materials(
    State(state): State<AppState>,
    _role: RequireRole<MmRead>,    // Enforces MM read permission
    Query(params): Query<PaginationParams>,
) -> Result<Json<ApiResponse<PaginatedResponse<Material>>>, AppError> { ... }
```

**ADMIN bypass**: Users with `ADMIN` role in JWT claims bypass all permission checks.

### 4.3 Low-Code Platform Roles

Three-tier platform roles (separate from ERP roles):

| Role | Access | Guard |
|------|--------|-------|
| `PLATFORM_ADMIN` | Full LC admin | `RequirePlatformRole<PlatformAdmin>` |
| `DEVELOPER` | Build forms/operations | `RequirePlatformRole<PlatformDeveloper>` |
| `USER` | Execute published forms | `RequirePlatformRole<PlatformUser>` |

---

## 5. ERP Module Overview

### 5.1 Module Summary

| Module | Code | Tables | Endpoints | Description |
|--------|------|--------|-----------|-------------|
| Financial Accounting | FI | 9 | 23 | Chart of accounts, journals, AR/AP, reports |
| Controlling | CO | 4 | 12 | Cost centers, profit centers, internal orders |
| Materials Management | MM | 6 | 23 | Materials, vendors, POs, goods receipt |
| Sales & Distribution | SD | 5 | 15 | Customers, sales orders, deliveries, invoices |
| Production Planning | PP | 5 | 10 | BOMs, routings, production orders |
| Human Resources | HR | 4 | 11 | Departments, employees, attendance |
| Warehouse Management | WM | 6 | 10 | Warehouses, storage bins, transfers, counts |
| Quality Management | QM | 3 | 9 | Inspection lots, results, notifications |
| Low-Code Platform | LC | 24 | 67 | Projects, forms, permissions, releases |
| **Total** | | **74** | **183** | |

### 5.2 Document Number Ranges

| Object Type | Prefix | Example | Table |
|-------------|--------|---------|-------|
| Material | MAT | MAT00000001 | `number_ranges` |
| Vendor | VND | VND00000001 | `number_ranges` |
| Purchase Order | PO | PO00000001 | `number_ranges` |
| Sales Order | SO | SO00000001 | `number_ranges` |
| Delivery | DLV | DLV00000001 | `number_ranges` |
| Invoice (SD) | INV | INV00000001 | `number_ranges` |
| Material Movement | MVT | MVT00000001 | `number_ranges` |
| Journal Entry | JE | JE00000001 | `number_ranges` |
| Production Order | PRD | PRD00000001 | `number_ranges` |

---

## 6. Cross-Module Integrations

### 6.1 SD -> FI Integration (Sales Invoice)

```
SD: Create Invoice
    │
    ├──► FI: Create Journal Entry
    │       DR: Accounts Receivable (1200)
    │       CR: Revenue (4000)
    │
    └──► FI: Post Journal Entry (auto)
```

**Code path**: `sd/services.rs::create_sd_invoice()` -> `fi/services.rs::create_journal_entry()` + `post_journal_entry()`

### 6.2 MM -> FI Integration (Goods Receipt)

```
MM: Receive Purchase Order
    │
    ├──► MM: Create GOODS_RECEIPT movements
    ├──► MM: Update PO item received quantities
    ├──► MM: Update PO status (PARTIALLY_RECEIVED / RECEIVED)
    │
    ├──► FI: Create Journal Entry
    │       DR: Inventory (1300)
    │       CR: Accounts Payable (2100)
    │
    ├──► FI: Post Journal Entry (auto)
    └──► FI: Create AP Invoice
```

**Code path**: `mm/services.rs::receive_purchase_order()` -> `fi/services.rs`

### 6.3 Integration Matrix

```
     ┌────┬────┬────┬────┬────┬────┬────┬────┐
     │ FI │ CO │ MM │ SD │ PP │ HR │ WM │ QM │
┌────┼────┼────┼────┼────┼────┼────┼────┼────┤
│ FI │ -- │    │ ←  │ ←  │    │    │    │    │
│ CO │    │ -- │    │    │    │    │    │    │
│ MM │ →  │    │ -- │    │ ←  │    │ ←  │ ←  │
│ SD │ →  │    │    │ -- │    │    │    │    │
│ PP │    │    │ →  │    │ -- │    │    │    │
│ HR │    │    │    │    │    │ -- │    │    │
│ WM │    │    │ →  │    │    │    │ -- │    │
│ QM │    │    │ →  │    │    │    │    │ -- │
└────┴────┴────┴────┴────┴────┴────┴────┴────┘

→ = calls into    ← = called from
```

---

## 7. Low-Code Platform

### 7.1 Architecture

```
┌─────────────────────────────────────────────┐
│              Platform Admin                  │
│  Projects | Releases | Navigation | Roles   │
└──────────────────┬──────────────────────────┘
                   │
┌──────────────────▼──────────────────────────┐
│              Developer                       │
│  Operations | Form Builder | Journal         │
│  Datasource | Permissions | Feedback         │
└──────────────────┬──────────────────────────┘
                   │
┌──────────────────▼──────────────────────────┐
│              End User                        │
│  Form Execution | Data CRUD | File Upload   │
└─────────────────────────────────────────────┘
```

### 7.2 Operation Types

| Type | Purpose | DB Storage |
|------|---------|------------|
| `FORM` | Data entry form | `lc_operation_data` (JSONB) |
| `LIST` | Data listing/grid | SQL query against any table |
| `DASHBOARD` | KPI dashboard | SQL datasource queries |
| `REPORT` | Report generation | SQL datasource queries |

### 7.3 Permission Layers

1. **Platform Role** - Who can access the platform (Admin/Developer/User)
2. **Operation Permission** - CRUD permissions per operation per role/user
3. **Field Permission** - Field visibility (VISIBLE/HIDDEN/MASKED) + editability
4. **Record Policy** - Row-level filtering via SQL WHERE clauses

### 7.4 Release Workflow

```
DRAFT ──► SUBMITTED ──► APPROVED ──► RELEASED
                    └──► REJECTED
```

---

## 8. Development Guide

### 8.1 Prerequisites

- Rust 1.75+ (rustup)
- Node.js 20+ (for Next.js)
- pnpm 9+ (package manager)
- PostgreSQL 17 (Homebrew)
- Xcode 16+ (iOS development)
- Android Studio (Android development)

### 8.2 Local Setup

```bash
# 1. Database
PGPASSWORD=postgres psql -h localhost -p 5432 -U postgres -c "CREATE DATABASE \"TastyByte\";"

# 2. Backend
cd backend
cp .env.example .env  # DATABASE_URL=postgres://postgres:postgres@localhost:5432/TastyByte
cargo build
cargo run             # Auto-runs all 18 migrations on startup

# 3. Web Frontend
cd web
pnpm install
pnpm dev              # http://localhost:3000

# 4. Login
# Username: admin
# Password: admin123
```

### 8.3 Adding a New API Endpoint

1. **Model** (`module/models.rs`): Define request/response structs
2. **Repository** (`module/repositories.rs`): Write SQL queries
3. **Service** (`module/services.rs`): Business logic
4. **Handler** (`module/handlers.rs`): HTTP handler with extractors
5. **Route** (`module/routes.rs`): Register in router
6. **Frontend API** (`web/src/lib/api/module.ts`): Add API function
7. **Frontend Page**: Create/update page component

### 8.4 Adding a New Migration

```bash
# Create file: backend/migrations/019_your_feature.sql
# Backend auto-runs on startup
# Use sqlx::raw_sql() for multi-statement migrations
```

### 8.5 Key Commands

| Command | Purpose |
|---------|---------|
| `cd backend && cargo build` | Compile backend |
| `cd backend && cargo test` | Run 41 tests |
| `cd backend && cargo clippy` | Lint Rust code |
| `cd backend && cargo fmt` | Format Rust code |
| `pnpm --prefix web build` | Build web frontend |
| `pnpm --prefix web dev` | Dev server |
| `pnpm --prefix web lint` | ESLint check |

### 8.6 API Response Format

All endpoints return:
```json
{
  "success": true,
  "data": { ... },
  "message": null
}
```

Error response:
```json
{
  "success": false,
  "data": null,
  "message": "Error description"
}
```

Paginated response:
```json
{
  "success": true,
  "data": {
    "data": [...],
    "total": 100,
    "page": 1,
    "per_page": 20
  }
}
```

---

## 9. Deployment

### 9.1 Backend

```bash
cd backend
cargo build --release
# Binary: target/release/tastebyte-erp
# Requires: DATABASE_URL, JWT_SECRET environment variables
```

### 9.2 Web Frontend

```bash
cd web
pnpm build
pnpm start
# Or deploy to Vercel/Netlify
# Requires: NEXT_PUBLIC_API_URL environment variable
```

### 9.3 Environment Variables

**Backend** (`backend/.env`):
| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | Yes | — | PostgreSQL connection string |
| `JWT_SECRET` | Yes | — | Secret key for JWT signing |
| `JWT_EXPIRATION` | No | 3600 | Token TTL in seconds |
| `RUST_LOG` | No | info | Log level |
| `CORS_ORIGIN` | No | http://localhost:3000 | Allowed CORS origin |

**Web** (`web/.env.local`):
| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `NEXT_PUBLIC_API_URL` | No | http://localhost:8000/api/v1 | Backend API URL |
