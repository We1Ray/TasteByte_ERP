# TasteByte ERP - Workflow & Status Management Guide

> Last Updated: 2026-02-21

## Table of Contents

1. [Status Machine Overview](#1-status-machine-overview)
2. [Document Workflows](#2-document-workflows)
3. [Status History & Audit Trail](#3-status-history--audit-trail)
4. [Cross-Module Workflow Integrations](#4-cross-module-workflow-integrations)
5. [Frontend Workflow Components](#5-frontend-workflow-components)
6. [DB Constraints & Data Integrity](#6-db-constraints--data-integrity)
7. [Extending Workflows](#7-extending-workflows)

---

## 1. Status Machine Overview

The TasteByte ERP uses a centralized status machine (`backend/src/shared/status.rs`) that validates all status transitions. Six document types are supported, each with explicitly defined valid transitions.

### Core Implementation

```rust
// backend/src/shared/status.rs
pub enum DocumentType {
    SalesOrder,
    PurchaseOrder,
    ProductionOrder,
    JournalEntry,
    Delivery,
    Invoice,
}

pub fn validate_transition(
    doc_type: DocumentType,
    from: &str,
    to: &str
) -> Result<(), AppError>
```

Every status change passes through `validate_transition()`. Invalid transitions return `AppError::Validation`.

---

## 2. Document Workflows

### 2.1 Sales Order (SD)

```
                    ┌───────────┐
                    │   DRAFT   │
                    └─────┬─────┘
                          │
              ┌───────────┼───────────┐
              ▼                       ▼
      ┌───────────────┐       ┌─────────────┐
      │   CONFIRMED   │       │  CANCELLED  │
      └───────┬───────┘       └─────────────┘
              │
    ┌─────────┼──────────┐
    ▼                    ▼
┌─────────────────┐ ┌───────────┐
│ PARTIALLY_      │ │ DELIVERED │
│ DELIVERED       │ │           │
└────────┬────────┘ └─────┬─────┘
         │                │
         └───────┬────────┘
                 ▼
         ┌───────────┐
         │  CLOSED   │
         └───────────┘
```

| Transition | Trigger | Service Function | Side Effects |
|-----------|---------|-----------------|-------------|
| DRAFT -> CONFIRMED | User confirms order | `sd::confirm_sales_order()` | Records status history |
| CONFIRMED -> PARTIALLY_DELIVERED | Partial delivery created | `sd::create_delivery()` | Creates delivery document |
| CONFIRMED -> DELIVERED | Full delivery | `sd::create_delivery()` | Updates all item delivered_qty |
| PARTIALLY_DELIVERED -> DELIVERED | Remaining delivery | `sd::create_delivery()` | — |
| DELIVERED -> CLOSED | All invoiced | — | — |
| DRAFT -> CANCELLED | User cancels | — | — |

**CHECK constraint** (`sd_sales_orders`):
```sql
status IN ('DRAFT', 'CONFIRMED', 'PARTIALLY_DELIVERED', 'DELIVERED', 'CLOSED', 'CANCELLED')
```

---

### 2.2 Purchase Order (MM)

```
         ┌───────────┐
         │   DRAFT   │
         └─────┬─────┘
               │
     ┌─────────┼──────────┐
     ▼                     ▼
┌──────────┐        ┌─────────────┐
│ RELEASED │        │  CANCELLED  │
└────┬─────┘        └─────────────┘
     │                     ▲
     ├─────────────────────┘
     │
     ├──────────────────────┐
     ▼                      ▼
┌─────────────────┐   ┌──────────┐
│ PARTIALLY_      │   │ RECEIVED │
│ RECEIVED        │   │          │
└────────┬────────┘   └────┬─────┘
         │                 │
         └────────┬────────┘
                  ▼
          ┌───────────┐
          │  CLOSED   │
          └───────────┘
```

| Transition | Trigger | Service Function | Side Effects |
|-----------|---------|-----------------|-------------|
| DRAFT -> RELEASED | Release for ordering | `mm::release_purchase_order()` | Records status history |
| RELEASED -> PARTIALLY_RECEIVED | Partial goods receipt | `mm::receive_purchase_order()` | Creates GOODS_RECEIPT movements, FI journal + AP invoice |
| RELEASED -> RECEIVED | Full goods receipt | `mm::receive_purchase_order()` | Same as above |
| PARTIALLY_RECEIVED -> RECEIVED | Remaining receipt | `mm::receive_purchase_order()` | — |
| RELEASED -> CANCELLED | User cancels | — | — |
| DRAFT -> CANCELLED | User cancels | — | — |
| RECEIVED -> CLOSED | Manual close | — | — |

**MM -> FI Auto-Integration on Goods Receipt:**
```
1. GOODS_RECEIPT movement -> Updates mm_plant_stock
2. PO item received_quantity += received amount
3. FI Journal Entry: DR Inventory(1300) / CR AP(2100)
4. FI Journal Entry auto-posted
5. FI AP Invoice created (due: 30 days)
```

**CHECK constraint** (`mm_purchase_orders`):
```sql
status IN ('DRAFT', 'RELEASED', 'PARTIALLY_RECEIVED', 'RECEIVED', 'CLOSED', 'CANCELLED')
```

---

### 2.3 Production Order (PP)

```
    ┌───────────┐
    │  CREATED  │
    └─────┬─────┘
          │
    ┌─────┼──────────┐
    ▼                 ▼
┌──────────┐   ┌─────────────┐
│ RELEASED │   │  CANCELLED  │
└────┬─────┘   └─────────────┘
     │               ▲
     ├───────────────┘
     ▼
┌─────────────┐
│ IN_PROGRESS │
└──────┬──────┘
       ▼
┌─────────────┐
│  COMPLETED  │
└──────┬──────┘
       ▼
┌─────────────┐
│   CLOSED    │
└─────────────┘
```

| Transition | Trigger | Service Function | Side Effects |
|-----------|---------|-----------------|-------------|
| CREATED -> RELEASED | Release for production | `pp::update_production_order_status("RELEASED")` | Records status history |
| RELEASED -> IN_PROGRESS | Start production | `pp::update_production_order_status("IN_PROGRESS")` | Sets actual_start date |
| IN_PROGRESS -> COMPLETED | Complete production | `pp::update_production_order_status("COMPLETED")` | Sets actual_end date |
| COMPLETED -> CLOSED | Manual close | — | — |
| CREATED -> CANCELLED | User cancels | — | — |
| RELEASED -> CANCELLED | User cancels | — | — |

**CHECK constraint** (`pp_production_orders`):
```sql
status IN ('CREATED', 'RELEASED', 'IN_PROGRESS', 'COMPLETED', 'CLOSED', 'CANCELLED')
```

---

### 2.4 Journal Entry (FI)

```
    ┌───────────┐
    │   DRAFT   │
    └─────┬─────┘
          ▼
    ┌───────────┐
    │  POSTED   │
    └───────────┘
```

| Transition | Trigger | Service Function | Side Effects |
|-----------|---------|-----------------|-------------|
| DRAFT -> POSTED | User or auto-post | `fi::post_journal_entry()` | Records status history, updates account balances |

**CHECK constraint** (`fi_journal_entries`):
```sql
status IN ('DRAFT', 'POSTED')
```

---

### 2.5 Delivery (SD)

```
    ┌───────────┐
    │  CREATED  │
    └─────┬─────┘
          │
    ┌─────┼──────────┐
    ▼                 ▼
┌──────────┐   ┌─────────────┐
│ SHIPPED  │   │  CANCELLED  │
└────┬─────┘   └─────────────┘
     ▼
┌──────────┐
│DELIVERED │
└──────────┘
```

**CHECK constraint** (`sd_deliveries`):
```sql
status IN ('CREATED', 'SHIPPED', 'DELIVERED', 'CANCELLED')
```

---

### 2.6 Invoice (SD)

```
    ┌───────────┐
    │  CREATED  │
    └─────┬─────┘
          │
    ┌─────┼──────────┐
    ▼                 ▼
┌──────────┐   ┌─────────────┐
│  POSTED  │   │  CANCELLED  │
└────┬─────┘   └─────────────┘
     ▼
┌──────────┐
│   PAID   │
└──────────┘
```

**CHECK constraint** (`sd_invoices`):
```sql
status IN ('CREATED', 'POSTED', 'PAID', 'CANCELLED')
```

---

### 2.7 Low-Code Release

```
    ┌───────────┐
    │   DRAFT   │
    └─────┬─────┘
          ▼
    ┌───────────┐
    │ SUBMITTED │
    └─────┬─────┘
          │
    ┌─────┼──────────┐
    ▼                 ▼
┌──────────┐   ┌─────────────┐
│ APPROVED │   │  REJECTED   │
└────┬─────┘   └─────────────┘
     ▼
┌──────────┐
│ RELEASED │
└──────────┘
```

---

## 3. Status History & Audit Trail

### 3.1 Database Table

```sql
-- document_status_history (Migration 018)
CREATE TABLE document_status_history (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_type VARCHAR(50) NOT NULL,    -- 'SALES_ORDER', 'PURCHASE_ORDER', etc.
    document_id   UUID NOT NULL,
    from_status   VARCHAR(30),             -- NULL for initial creation
    to_status     VARCHAR(30) NOT NULL,
    changed_by    UUID NOT NULL REFERENCES users(id),
    change_reason TEXT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_status_history_doc ON document_status_history(document_type, document_id);
CREATE INDEX idx_status_history_time ON document_status_history(created_at);
```

### 3.2 Recording Transitions (Backend)

```rust
// backend/src/shared/status_history.rs
pub async fn record_transition(
    pool: &PgPool,
    doc_type: &DocumentType,    // e.g., DocumentType::PurchaseOrder
    doc_id: Uuid,               // Document UUID
    from: Option<&str>,         // Previous status (None for creation)
    to: &str,                   // New status
    user_id: Uuid,              // Who made the change
    reason: Option<&str>,       // Optional reason
) -> Result<(), AppError>
```

**Document Type Keys:**

| DocumentType | DB Key |
|-------------|--------|
| `SalesOrder` | `SALES_ORDER` |
| `PurchaseOrder` | `PURCHASE_ORDER` |
| `ProductionOrder` | `PRODUCTION_ORDER` |
| `JournalEntry` | `JOURNAL_ENTRY` |
| `Delivery` | `DELIVERY` |
| `Invoice` | `INVOICE` |

### 3.3 Wired Transition Points

| Module | Function | From -> To |
|--------|---------|-----------|
| SD | `confirm_sales_order()` | DRAFT -> CONFIRMED |
| SD | `create_delivery()` | CONFIRMED -> PARTIALLY_DELIVERED/DELIVERED |
| SD | `create_delivery()` | Delivery: (init) -> CREATED |
| SD | `create_sd_invoice()` | Invoice: (init) -> CREATED |
| MM | `release_purchase_order()` | DRAFT -> RELEASED |
| MM | `receive_purchase_order()` | RELEASED -> PARTIALLY_RECEIVED/RECEIVED |
| PP | `update_production_order_status()` | Any valid transition |
| FI | `post_journal_entry()` | DRAFT -> POSTED |

### 3.4 Query API

```
GET /api/v1/workflow/history/{doc_type}/{doc_id}
```

Returns chronologically ordered list of all status changes for a document.

---

## 4. Cross-Module Workflow Integrations

### 4.1 Order-to-Cash Flow (SD -> FI)

```
┌──────────────────────────────────────────────────────┐
│                 Order-to-Cash Flow                    │
├──────────────────────────────────────────────────────┤
│                                                      │
│  Sales Order     Delivery       Invoice    Payment   │
│  ┌────────┐    ┌──────────┐   ┌────────┐  ┌──────┐ │
│  │ DRAFT  │───►│ CREATED  │──►│CREATED │─►│ PAID │ │
│  │CONFIRMED│   │ SHIPPED  │   │ POSTED │  │      │ │
│  │DELIVERED│   │DELIVERED │   │        │  │      │ │
│  └────────┘    └──────────┘   └────────┘  └──────┘ │
│       │                            │                 │
│       │          Auto-created      │                 │
│       └────────────────────────────┘                 │
│                                                      │
│  FI Integration:                                     │
│  Invoice POST → Journal Entry                        │
│    DR: AR (1200)                                     │
│    CR: Revenue (4000)                                │
└──────────────────────────────────────────────────────┘
```

### 4.2 Procure-to-Pay Flow (MM -> FI)

```
┌──────────────────────────────────────────────────────┐
│                 Procure-to-Pay Flow                   │
├──────────────────────────────────────────────────────┤
│                                                      │
│  Purchase Order     Goods Receipt      AP Invoice    │
│  ┌──────────┐      ┌────────────┐    ┌───────────┐  │
│  │  DRAFT   │─────►│ GOODS_RECV │───►│   OPEN    │  │
│  │ RELEASED │      │ Movement   │    │           │  │
│  │ RECEIVED │      │            │    │           │  │
│  └──────────┘      └────────────┘    └───────────┘  │
│                          │                           │
│                          ▼                           │
│                    ┌────────────┐                    │
│                    │ FI Journal │                    │
│                    │ DR: Inv    │                    │
│                    │ CR: AP     │                    │
│                    └────────────┘                    │
└──────────────────────────────────────────────────────┘
```

### 4.3 Plan-to-Produce Flow (PP)

```
┌──────────────────────────────────────────────────────┐
│                Plan-to-Produce Flow                   │
├──────────────────────────────────────────────────────┤
│                                                      │
│  BOM          Production Order                       │
│  ┌───────┐   ┌──────────────────┐                   │
│  │ BOM   │──►│ CREATED          │                   │
│  │ Items │   │ RELEASED         │                   │
│  └───────┘   │ IN_PROGRESS      │                   │
│              │ COMPLETED        │                   │
│  Routing     └──────────────────┘                   │
│  ┌───────┐          │                               │
│  │ Steps │──────────┘                               │
│  └───────┘                                          │
│                                                      │
│  Future: PP → MM (auto material issue/receipt)       │
└──────────────────────────────────────────────────────┘
```

---

## 5. Frontend Workflow Components

### 5.1 WorkflowTimeline Component

**Location:** `web/src/components/shared/workflow-timeline.tsx`

Renders a vertical timeline showing all status transitions for a document.

**Usage:**
```tsx
<WorkflowTimeline
  documentType="PURCHASE_ORDER"   // matches DocumentType.db_key()
  documentId={id}                  // UUID string
/>
```

**Features:**
- Auto-fetches from `GET /api/v1/workflow/history/{type}/{id}`
- Color-coded status badges (green for completed, blue for in-progress, gray for initial, red for cancelled)
- Shows from -> to transition arrows
- Timestamps in human-readable format
- Loading state with animated icon

**Integrated Pages:**
- `web/src/app/(erp)/sd/sales-orders/[id]/page.tsx`
- `web/src/app/(erp)/mm/purchase-orders/[id]/page.tsx`
- `web/src/app/(erp)/pp/production-orders/[id]/page.tsx`

### 5.2 StatusBadge Component

**Location:** `web/src/components/ui/badge.tsx`

Renders a colored badge for status values.

```tsx
<StatusBadge status={order.status} />
```

### 5.3 Toast Notifications

**Location:** `web/src/lib/hooks/use-toast-mutation.ts`

Wraps mutations with automatic success/error toasts.

```tsx
const mutation = useToastMutation(
  (data) => api.updateStatus(id, data),
  {
    successMessage: "Status updated successfully",
    invalidateKeys: ["module", "resource"],
  }
);

mutation.mutate({ status: "RELEASED" });
```

---

## 6. DB Constraints & Data Integrity

### 6.1 CHECK Constraints (Added in Migration 018)

All constraints use safe `DO $$ ... EXCEPTION ... END $$` blocks for idempotent execution:

| Table | Constraint Name | Valid Values |
|-------|----------------|-------------|
| `sd_sales_orders` | `chk_sd_so_status` | DRAFT, CONFIRMED, PARTIALLY_DELIVERED, DELIVERED, CLOSED, CANCELLED |
| `mm_purchase_orders` | `chk_mm_po_status` | DRAFT, RELEASED, PARTIALLY_RECEIVED, RECEIVED, CLOSED, CANCELLED |
| `pp_production_orders` | `chk_pp_prod_status` | CREATED, RELEASED, IN_PROGRESS, COMPLETED, CLOSED, CANCELLED |
| `fi_journal_entries` | `chk_fi_je_status` | DRAFT, POSTED |
| `sd_deliveries` | `chk_sd_del_status` | CREATED, SHIPPED, DELIVERED, CANCELLED |
| `sd_invoices` | `chk_sd_inv_status` | CREATED, POSTED, PAID, CANCELLED |
| `mm_material_movements` | `chk_mm_mvt_type` | GOODS_RECEIPT, GOODS_ISSUE, TRANSFER, ADJUSTMENT |
| `fi_ar_invoices` | `chk_fi_ar_status` | OPEN, PAID, CANCELLED |

### 6.2 Foreign Key Relationships

Status history links back to users:
```sql
changed_by UUID NOT NULL REFERENCES users(id)
```

All document tables link to `users` via `created_by`.

### 6.3 updated_at Columns (Added in Migration 018)

Six core tables gained `updated_at TIMESTAMPTZ` columns:
- `sd_sales_orders`
- `mm_purchase_orders`
- `pp_production_orders`
- `fi_journal_entries`
- `sd_invoices`
- `sd_deliveries`

---

## 7. Extending Workflows

### 7.1 Adding a New Status to an Existing Document Type

1. **Migration**: Add the new status to the CHECK constraint
   ```sql
   ALTER TABLE your_table DROP CONSTRAINT chk_your_status;
   ALTER TABLE your_table ADD CONSTRAINT chk_your_status
     CHECK (status IN ('EXISTING_1', 'EXISTING_2', 'NEW_STATUS'));
   ```

2. **Status Machine**: Update `validate_transition()` in `backend/src/shared/status.rs`
   ```rust
   DocumentType::YourType => matches!(
       (from, to),
       ("EXISTING_1", "EXISTING_2")
           | ("EXISTING_2", "NEW_STATUS")  // Add new transition
   ),
   ```

3. **Service**: Add `record_transition()` call at the transition point
   ```rust
   status_history::record_transition(
       pool,
       &DocumentType::YourType,
       doc_id,
       Some(&old_status),
       "NEW_STATUS",
       user_id,
       None,
   ).await?;
   ```

4. **Frontend**: Update StatusBadge color mapping if needed

### 7.2 Adding a New Document Type

1. **Add to enum** in `backend/src/shared/status.rs`:
   ```rust
   pub enum DocumentType {
       // ... existing
       NewDocument,
   }
   ```

2. **Define transitions** in `validate_transition()`:
   ```rust
   DocumentType::NewDocument => matches!(
       (from, to),
       ("DRAFT", "ACTIVE") | ("ACTIVE", "CLOSED")
   ),
   ```

3. **Add db_key()** in `backend/src/shared/status_history.rs`:
   ```rust
   DocumentType::NewDocument => "NEW_DOCUMENT",
   ```

4. **Add CHECK constraint** via new migration:
   ```sql
   ALTER TABLE new_documents ADD CONSTRAINT chk_new_doc_status
     CHECK (status IN ('DRAFT', 'ACTIVE', 'CLOSED'));
   ```

5. **Wire record_transition()** in the service functions that change status

6. **Frontend**: Add `<WorkflowTimeline documentType="NEW_DOCUMENT" documentId={id} />` to the detail page

### 7.3 Adding a Transition Reason

Pass a reason when recording transitions for audit purposes:

```rust
status_history::record_transition(
    pool,
    &DocumentType::PurchaseOrder,
    po_id,
    Some("RELEASED"),
    "CANCELLED",
    user_id,
    Some("Vendor unable to fulfill order"),  // Reason stored in DB
).await?;
```

The reason appears in the WorkflowTimeline component as italic text below the transition.
