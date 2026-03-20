# TasteByte ERP - Technical Manual

> Version 2.0 | Last Updated: 2026-03-21

## Table of Contents

1. [System Architecture](#1-system-architecture)
2. [Tech Stack](#2-tech-stack)
3. [Project Structure](#3-project-structure)
4. [Authentication & Authorization](#4-authentication--authorization)
5. [ERP Module Overview](#5-erp-module-overview)
6. [Cross-Module Integrations](#6-cross-module-integrations)
7. [Low-Code Platform](#7-low-code-platform)
8. [YAML Operations System](#8-yaml-operations-system)
9. [Internationalization (i18n)](#9-internationalization-i18n)
10. [Development Guide](#10-development-guide)
11. [Deployment](#11-deployment)

---

## 1. System Architecture

```
                         ┌──────────────────────────────────┐
                         │      Nginx Reverse Proxy         │
                         │        (Port 80 / 443)           │
                         └──────────────┬───────────────────┘
                                        │
         ┌──────────────────┬───────────┼───────────┬──────────────────┐
         │                  │           │           │                  │
┌────────▼────────┐ ┌──────▼───────┐ ┌─▼──────────┐ ┌────────▼───────┐
│  Next.js 16 Web │ │  iOS Native  │ │Android Natv│ │ Flutter Mobile │
│  Port: 3000     │ │  SwiftUI     │ │ Compose    │ │ Cross-platform │
│  App Router     │ │  MVVM        │ │ MVVM       │ │ Dart + Dio     │
└────────┬────────┘ └──────┬───────┘ └─┬──────────┘ └────────┬───────┘
         │                  │           │                      │
         └──────────────────┼───────────┼──────────────────────┘
                            │           │
                   REST API (JSON) + JWT Auth
                            │
              ┌─────────────▼────────────────────┐
              │     Rust / Axum 0.8 Backend      │
              │         Port: 8000                │
              │   RBAC + Rate Limiting + Metrics  │
              │   44 SQL Migrations (auto-run)    │
              │   YAML Operation Sync on Startup  │
              └─────────────┬────────────────────┘
                            │
                  SQLx 0.8 (compile-time checked)
                            │
              ┌─────────────▼────────────────────┐
              │    PostgreSQL 17 (Local)          │
              │    Port: 5432 | DB: TastyByte     │
              │    123 Tables | 44 Migrations     │
              └──────────────────────────────────┘
```

### Design Principles

- **SAP-like Module Design**: 8 standard ERP modules (FI, CO, MM, SD, PP, HR, WM, QM)
- **Document Flow**: Auto-generated document numbers (`SO00000001`, `PO00000001`)
- **Status Machine**: Validated state transitions with audit trail
- **Single Backend**: All business logic in one Rust binary
- **Multi-Platform Mobile**: Native iOS (Swift) + Native Android (Kotlin) + Flutter cross-platform
- **YAML-Driven Low-Code**: File-based operation definitions synced on startup
- **i18n**: English + Traditional Chinese (next-intl)

---

## 2. Tech Stack

| Layer | Technology | Version | Purpose |
|-------|-----------|---------|---------|
| Backend | Rust + Axum | 0.8 | API server, business logic |
| ORM | SQLx | 0.8 | Compile-time checked async PostgreSQL driver |
| Database | PostgreSQL | 17 | Primary data store (123 tables) |
| Web Frontend | Next.js | 16.x | ERP management UI (App Router) |
| React | React | 19.x | UI framework |
| CSS | Tailwind CSS | 4.x | Utility-first styling |
| State (Server) | TanStack Query | 5.x | API cache & mutations |
| State (Client) | Zustand | 5.x | Auth, UI, form builder, locale state |
| Tables | TanStack Table | 8.x | Data grid rendering |
| Validation | Zod | 3.x | Form validation |
| i18n | next-intl | 4.x | English + Traditional Chinese |
| Notifications | Sonner | 2.x | Toast notifications |
| Charts | Recharts | — | Data visualization |
| iOS | Swift / SwiftUI | 6.0 | Native iOS app |
| Android | Kotlin / Jetpack Compose | 1.9 | Native Android app |
| Mobile | Flutter / Dart | 3.x | Cross-platform mobile app |
| Auth | JWT + Argon2 | — | Token auth + password hashing + refresh tokens |
| HTTP Client | Axios | 1.x | Frontend API calls |
| Reverse Proxy | Nginx | 1.27 | Load balancing, SSL termination |
| Container | Docker Compose | — | Full stack orchestration (4 services) |

---

## 3. Project Structure

```
TasteByte_ERP/
├── backend/                          # Rust/Axum Backend (260+ endpoints)
│   ├── Cargo.toml                    # Dependencies (Rust 2021 edition)
│   ├── .env                          # DATABASE_URL config
│   ├── migrations/                   # 44 SQL migration files
│   │   ├── 001_foundation.sql        # Users, roles, audit_log
│   │   ├── 002_fi_chart_of_accounts.sql # FI chart of accounts
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
│   │   ├── 013_rbac_permissions.sql  # RBAC permissions
│   │   ├── 014_lowcode_platform.sql  # LC projects, operations, forms
│   │   ├── 015_lowcode_permissions.sql # LC RBAC
│   │   ├── 016_lowcode_workflow.sql  # LC releases, feedback, journal
│   │   ├── 017_lowcode_seed.sql      # LC seed data
│   │   ├── 018_workflow_infrastructure.sql # Status history, CHECK constraints
│   │   ├── 019-028                   # Phase 1 extensions, hierarchical RBAC,
│   │   │                             #   account lockout, refresh tokens, indexes
│   │   ├── 029_notifications.sql     # In-app notification system
│   │   ├── 030_cross_module_integration.sql # Cross-module linking
│   │   ├── 031_module_linked_operations.sql # LC-ERP module linking
│   │   ├── 032-034                   # Demo/seed data, backend fixes
│   │   ├── 035_infrastructure.sql    # Email, webhooks, scheduled jobs
│   │   ├── 036_q1q2_features.sql     # Approvals, BPM workflows, reports
│   │   ├── 037_q3q4_features.sql     # Print, analytics, exchange rates
│   │   ├── 038_business_operations_via_lowcode.sql # LC business ops
│   │   ├── 039_lowcode_improvements.sql # Form variants, cross-field rules
│   │   ├── 040_supply_chain_core.sql # Supply chain extensions
│   │   ├── 041_test_seed_data.sql    # Test seed data
│   │   ├── 042_yaml_sync_support.sql # YAML sync metadata columns
│   │   ├── 043_100_operations_bulk.sql # Bulk operation seed data
│   │   └── 044_form_improvements.sql # Form UI improvements
│   ├── operations/                   # YAML operation definitions (file-driven)
│   │   ├── hr/hr-leave.yaml          # Leave request form
│   │   ├── mm/mm-eval.yaml           # Supplier evaluation form
│   │   ├── mm/mm-grn.yaml            # Goods receipt note form
│   │   ├── pp/pp-consume.yaml        # Material consumption form
│   │   ├── qm/qm-insp.yaml          # Quality inspection form
│   │   ├── sd/sd-delivery.yaml       # Delivery note form
│   │   └── wm/wm-move.yaml          # Warehouse movement form
│   ├── tests/                        # Integration tests (10 modules)
│   └── src/
│       ├── main.rs                   # Entry point, server startup + YAML sync
│       ├── routes.rs                 # Top-level router (build_router)
│       ├── config/                   # Settings, create_pool()
│       ├── auth/                     # Authentication module (14 endpoints)
│       │   ├── handlers.rs           # Login, register, refresh, lockout
│       │   ├── middleware.rs          # JWT extraction middleware
│       │   ├── models.rs             # Claims, LoginRequest
│       │   ├── rbac.rs               # RequireRole<R> extractor
│       │   ├── routes.rs             # /auth routes
│       │   └── services.rs           # Password hashing, token generation
│       ├── shared/                   # Shared infrastructure
│       │   ├── mod.rs                # Module exports
│       │   ├── admin_api.rs          # System admin endpoints (55 handlers)
│       │   ├── audit.rs              # log_change() for audit trail
│       │   ├── error.rs              # AppError enum
│       │   ├── handlers.rs           # StatusHistory query handler
│       │   ├── pagination.rs         # PaginatedResponse, PaginationParams
│       │   ├── response.rs           # ApiResponse<T> wrapper
│       │   ├── status.rs             # DocumentType, validate_transition()
│       │   ├── status_history.rs     # record_transition()
│       │   └── types.rs              # AppState (pool + metrics_handle)
│       ├── fi/                       # Financial Accounting (19 endpoints)
│       ├── co/                       # Controlling (9 endpoints)
│       ├── mm/                       # Materials Management (24 endpoints)
│       ├── sd/                       # Sales & Distribution (13 endpoints)
│       ├── pp/                       # Production Planning (11 endpoints)
│       ├── hr/                       # Human Resources (18 endpoints)
│       ├── wm/                       # Warehouse Management (10 endpoints)
│       ├── qm/                       # Quality Management (9 endpoints)
│       ├── lowcode/                  # Low-Code Platform (71 endpoints)
│       │   ├── handlers/             # HTTP handlers
│       │   ├── services/             # Business logic
│       │   ├── models.rs             # Data models
│       │   ├── routes.rs             # Route definitions
│       │   └── yaml_sync/            # YAML operation sync engine
│       │       ├── loader.rs         # Load YAML files from disk
│       │       ├── syncer.rs         # Upsert into database
│       │       ├── exporter.rs       # Export operations to YAML
│       │       └── schema.rs         # YAML schema definitions
│       ├── notifications/            # Notification system (5 endpoints)
│       │   ├── handlers.rs           # CRUD + mark-read
│       │   ├── models.rs             # Notification model
│       │   ├── routes.rs             # /notifications routes
│       │   └── services.rs           # Notification logic
│       ├── middleware/                # CORS, logging, metrics, rate limiting
│       └── schema/                   # Migration engine
│           └── migrator.rs           # Auto-migration on startup
│
├── web/                              # Next.js 16 Frontend
│   ├── package.json                  # Dependencies
│   ├── next.config.ts                # Next.js config
│   ├── messages/                     # i18n translation files
│   │   ├── en.json                   # English
│   │   └── zh-TW.json               # Traditional Chinese
│   └── src/
│       ├── app/                      # App Router pages
│       │   ├── layout.tsx            # Root layout (i18n provider)
│       │   ├── login/page.tsx        # Login page
│       │   └── (erp)/               # ERP module pages
│       │       ├── layout.tsx        # ERP shell (sidebar + header)
│       │       ├── dashboard/        # Main dashboard
│       │       ├── fi/               # FI pages
│       │       ├── co/               # CO pages
│       │       ├── mm/               # MM pages
│       │       ├── sd/               # SD pages
│       │       ├── pp/               # PP pages
│       │       ├── hr/               # HR pages
│       │       ├── wm/               # WM pages
│       │       ├── qm/               # QM pages
│       │       ├── notifications/    # Notification center
│       │       ├── admin/            # LC Admin pages (users, roles, projects...)
│       │       ├── developer/        # LC Developer pages (operations, feedback)
│       │       └── lowcode/          # LC User runtime pages
│       ├── components/
│       │   ├── layout/               # Sidebar, Header, PageHeader
│       │   ├── ui/                   # Button, Card, Input, Badge, Modal, Table
│       │   ├── shared/               # StatusBadge, WorkflowTimeline
│       │   ├── charts/               # Recharts wrappers
│       │   ├── forms/                # Form components
│       │   └── lowcode/              # Form builder components
│       └── lib/
│           ├── api/                  # 16 API client modules
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
│           │   ├── lowcode.ts        # LC API
│           │   ├── notifications.ts  # Notification API
│           │   ├── rbac.ts           # RBAC API
│           │   ├── system.ts         # System/admin API
│           │   ├── ai-chat.ts        # AI chat API
│           │   └── dashboard.ts      # Dashboard KPI API
│           ├── hooks/                # 8 custom hooks
│           │   ├── use-auth.ts       # Auth flow
│           │   ├── use-api-query.ts  # React Query wrappers
│           │   ├── use-pagination.ts # Pagination state
│           │   ├── use-toast-mutation.ts # Toast-enabled mutations
│           │   ├── use-dynamic-form.ts   # Form builder hooks
│           │   ├── use-form-calculations.ts # Formula evaluation
│           │   ├── use-lookup.ts     # Dropdown/lookup hooks
│           │   └── use-platform-role.ts  # Platform role checks
│           ├── stores/               # 7 Zustand stores
│           │   ├── auth-store.ts     # Auth state
│           │   ├── ui-store.ts       # Layout state
│           │   ├── builder-store.ts  # Form builder state
│           │   ├── list-builder-store.ts  # List builder state
│           │   ├── dashboard-builder-store.ts # Dashboard builder state
│           │   ├── locale-store.ts   # i18n locale state
│           │   └── ai-chat-store.ts  # AI chat state
│           ├── types/                # TypeScript type definitions
│           └── utils/                # formatCurrency, formatDate, etc.
│
├── mobile/                           # Flutter Cross-Platform App
│   ├── pubspec.yaml                  # Dart dependencies
│   ├── lib/                          # Dart source code
│   ├── ios/                          # iOS platform config
│   ├── android/                      # Android platform config
│   └── test/                         # Flutter tests
│
├── ios/                              # Native iOS App (Swift/SwiftUI)
│   └── TasteByte/
│       ├── Models/                   # API response models
│       ├── Services/                 # APIClient, AuthService
│       ├── ViewModels/               # MVVM view models
│       └── Views/                    # SwiftUI screens (8 modules)
│
├── android/                          # Native Android App (Kotlin/Compose)
│   └── app/src/main/
│       ├── data/                     # Repository, API service
│       ├── ui/                       # Compose screens (8 modules)
│       └── viewmodel/                # MVVM view models
│
├── nginx/                            # Reverse proxy configuration
├── scripts/                          # Backup/restore utilities
├── docker-compose.yml                # Full stack orchestration (4 services)
├── Makefile                          # Development commands
│
└── docs/                             # Documentation
    ├── TECHNICAL_MANUAL.md           # This file
    ├── API_REFERENCE.md              # Complete API endpoint reference
    ├── DATABASE_SCHEMA.md            # All 123 tables
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
| Financial Accounting | FI | 10 | 19 | Chart of accounts, journals, AR/AP, payments, reports |
| Controlling | CO | 4 | 9 | Cost centers, profit centers, internal orders, allocations |
| Materials Management | MM | 12 | 24 | Materials, vendors, POs, goods receipt, stock, movements |
| Sales & Distribution | SD | 5 | 13 | Customers, sales orders, deliveries, invoices |
| Production Planning | PP | 5 | 11 | BOMs, routings, production orders |
| Human Resources | HR | 6 | 18 | Departments, employees, attendance, payroll, salary |
| Warehouse Management | WM | 5 | 10 | Warehouses, storage bins, transfers, stock counts |
| Quality Management | QM | 3 | 9 | Inspection lots, results, quality notifications |
| Auth | — | 5 | 14 | Login, register, refresh, lockout, roles |
| Low-Code Platform | LC | 24+ | 71 | Projects, forms, permissions, releases, YAML sync |
| Notifications | — | 2 | 5 | In-app notifications, read/unread tracking |
| System/Admin | — | 20+ | 55 | Approvals, BPM, email, webhooks, jobs, analytics |
| **Total** | | **123** | **260+** | |

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

### 7.5 Cross-Field Rules & Formulas

Operations support declarative validation and calculation rules:

- **Cross-Field Validation**: Rules that validate one field against another (e.g., received_qty <= ordered_qty * 1.1)
- **Calculation Formulas**: Auto-computed fields based on other field values
- **Output Determination**: Automatic notifications or actions triggered by field values on create/update

### 7.6 Form Variants

Operations can have multiple form variants (different field layouts/defaults) selected at runtime based on user role or context.

---

## 8. YAML Operations System

### 8.1 Overview

The YAML operations system provides a file-driven approach to defining low-code operations. YAML files stored in `backend/operations/` are automatically synced to the database on server startup, enabling version-controlled operation definitions.

### 8.2 Directory Structure

```
backend/operations/
├── hr/
│   └── hr-leave.yaml          # Leave request form
├── mm/
│   ├── mm-eval.yaml           # Supplier evaluation form
│   └── mm-grn.yaml            # Goods receipt note form
├── pp/
│   └── pp-consume.yaml        # Material consumption form
├── qm/
│   └── qm-insp.yaml          # Quality inspection form
├── sd/
│   └── sd-delivery.yaml       # Delivery note form
└── wm/
    └── wm-move.yaml           # Warehouse movement form
```

### 8.3 YAML Schema

Each YAML operation file defines a complete operation including form layout, fields, validation rules, and output actions:

```yaml
operation_code: MM-GRN
name: "Goods Receipt Note (GRN)"
description: "Purchase receipt confirmation with auto inventory update"
module: MM
operation_type: FORM
project_code: LCP-100
is_published: true
version: 1

sidebar:
  icon: PackageCheck
  sort_order: 30

form:
  layout: {}
  settings: {}
  sections:
    - title: "GRN Information"
      columns: 2
      fields:
        - field_name: grn_number
          label: "GRN Number"
          type: TEXT
          required: true
          searchable: true
          config:
            is_readonly: true
          default_value_sql: "SELECT ..."  # Auto-generated number

        - field_name: status
          label: "Status"
          type: DROPDOWN
          options:
            - label: "Draft"
              value: DRAFT
              is_default: true

cross_field_rules:
  - name: "Received quantity limit"
    rule_type: VALIDATION
    source_field: received_qty
    operator: lte
    error_message: "Received quantity exceeds order limit"

output_rules:
  - name: "Notify finance on confirmation"
    trigger_event: ON_CREATE
    condition_field: status
    condition_operator: equals
    condition_value: CONFIRMED
    output_type: NOTIFICATION
```

### 8.4 Sync Engine

The YAML sync engine (`backend/src/lowcode/yaml_sync/`) handles:

| Component | File | Purpose |
|-----------|------|---------|
| Loader | `loader.rs` | Reads and parses YAML files from `backend/operations/` |
| Syncer | `syncer.rs` | Upserts operations, fields, rules, and outputs into the database |
| Exporter | `exporter.rs` | Exports database operations back to YAML format |
| Schema | `schema.rs` | Rust structs matching the YAML schema |

**Sync behavior:**
- Runs automatically on server startup
- Uses `operation_code` as the unique key for upsert
- Preserves user-created operations (only syncs YAML-sourced ones)
- Tracks sync metadata via `042_yaml_sync_support.sql` migration columns

### 8.5 Adding a New YAML Operation

1. Create a YAML file in `backend/operations/{module}/`:
   ```bash
   backend/operations/mm/mm-new-form.yaml
   ```

2. Define the operation following the schema in section 8.3

3. Restart the backend server -- the sync engine picks up the file automatically

4. The operation appears in the low-code platform and (if `sidebar` is defined) in the ERP module sidebar

---

## 9. Internationalization (i18n)

### 9.1 Overview

The web frontend supports English and Traditional Chinese using `next-intl`.

### 9.2 Translation Files

```
web/messages/
├── en.json      # English (default)
└── zh-TW.json   # Traditional Chinese
```

### 9.3 Locale State

Locale selection is managed by the `locale-store.ts` Zustand store and persisted across sessions.

### 9.4 Usage in Components

```tsx
import { useTranslations } from 'next-intl';

function MyComponent() {
  const t = useTranslations('module.section');
  return <h1>{t('title')}</h1>;
}
```

---

## 10. Development Guide

### 10.1 Prerequisites

- Rust 1.75+ (rustup, 2021 edition)
- Node.js 20+ (for Next.js 16)
- pnpm 9+ (package manager)
- PostgreSQL 17 (Homebrew or Docker)
- Xcode 16+ (iOS development)
- Android Studio (Android development)
- Flutter SDK 3.x+ (cross-platform mobile development)
- Docker & Docker Compose (optional, for containerized setup)

### 10.2 Local Setup

```bash
# 1. Database
PGPASSWORD=postgres psql -h localhost -p 5432 -U postgres -c "CREATE DATABASE \"TastyByte\";"

# 2. Backend
cd backend
cp .env.example .env  # DATABASE_URL=postgres://postgres:postgres@localhost:5432/TastyByte
cargo build
cargo run             # Auto-runs all 44 migrations + YAML sync on startup

# 3. Web Frontend
cd web
pnpm install
pnpm dev              # http://localhost:3000

# 4. Login
# Username: admin
# Password: admin123
```

### 10.3 Adding a New API Endpoint

1. **Model** (`module/models.rs`): Define request/response structs
2. **Repository** (`module/repositories.rs`): Write SQL queries
3. **Service** (`module/services.rs`): Business logic
4. **Handler** (`module/handlers.rs`): HTTP handler with extractors
5. **Route** (`module/routes.rs`): Register in router
6. **Frontend API** (`web/src/lib/api/module.ts`): Add API function
7. **Frontend Page**: Create/update page component

### 10.4 Adding a New Migration

```bash
# Create file: backend/migrations/045_your_feature.sql
# Backend auto-runs on startup
# Use sqlx::raw_sql() for multi-statement migrations
# Current migration count: 44 (001-044)
```

### 10.5 Key Commands

| Command | Purpose |
|---------|---------|
| `make dev` | Start all services (backend + web) |
| `make build` | Build all (backend + web) |
| `make test` | Run all tests (backend + web) |
| `make lint` | Run linters (clippy + eslint) |
| `make docker-up` | Build and start Docker containers |
| `make docker-down` | Stop Docker containers |
| `make db-backup` | Backup database |
| `make db-restore` | Restore database from backup |
| `cd backend && cargo build` | Compile backend |
| `cd backend && cargo test --lib` | Run 160 unit tests |
| `cd backend && cargo clippy` | Lint Rust code |
| `cd backend && cargo fmt` | Format Rust code |
| `pnpm --prefix web build` | Build web frontend |
| `pnpm --prefix web dev` | Dev server (http://localhost:3000) |
| `pnpm --prefix web test` | Run 96 Vitest unit tests |
| `pnpm --prefix web lint` | ESLint check |

### 10.6 API Response Format

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

## 11. Deployment

### 11.1 Docker Compose (Recommended)

```bash
# Copy environment file
cp .env.docker.example .env

# Start all 4 services (postgres, backend, web, nginx)
docker compose up -d

# Access:
# Web:  http://localhost (via nginx)
# API:  http://localhost:8000 (direct)
# DB:   localhost:5432
```

**Services:**
| Service | Image | Port | Purpose |
|---------|-------|------|---------|
| postgres | postgres:17-alpine | 5432 | Database |
| backend | Custom (Rust) | 8000 | API server |
| web | Custom (Next.js) | 3000 | Web frontend |
| nginx | nginx:1.27-alpine | 80/443 | Reverse proxy |

Optional monitoring (uncomment in docker-compose.yml): Prometheus + Grafana

### 11.2 Backend

```bash
cd backend
cargo build --release
# Binary: target/release/tastebyte-erp
# Requires: DATABASE_URL, JWT_SECRET environment variables
```

### 11.3 Web Frontend

```bash
cd web
pnpm build
pnpm start
# Or deploy to Vercel/Netlify
# Requires: NEXT_PUBLIC_API_URL environment variable
```

### 11.4 Environment Variables

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
