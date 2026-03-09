# Architecture Decisions Record (ADR)

> **Usage**: Record important technical decisions here. Agents should consult this before making architecture changes.

---

## ADR-001: Rust/Axum as Backend Framework

**Date**: 2026-02-01
**Status**: Accepted

### Context
Need a high-performance, type-safe backend framework for ERP system handling complex business logic and high concurrent connections.

### Decision
Use Rust with Axum 0.7 as the sole backend framework. No Python, no additional microservices.

### Rationale
- High performance and memory safety
- Strong type system prevents runtime errors
- Excellent for concurrent request handling
- Single backend simplifies deployment and maintenance
- SQLx provides compile-time checked queries

### Consequences
- Steeper learning curve for Rust
- All business logic in one language
- Simpler deployment (single binary)

---

## ADR-002: Local PostgreSQL (No Cloud/SaaS DB)

**Date**: 2026-02-01
**Status**: Accepted

### Context
ERP data is sensitive (financial, HR, inventory). Need full data sovereignty.

### Decision
Use local PostgreSQL 17 on port 5432. No Supabase, no cloud database services.

### Rationale
- Full data sovereignty and control
- No vendor lock-in
- Low latency (local connection)
- No recurring SaaS costs
- Custom migration engine for full control

### Consequences
- Must manage backups manually
- No built-in RLS (implement RBAC in application layer)
- No real-time subscriptions (use polling or SSE if needed)

---

## ADR-003: Next.js 15 App Router for Web Frontend

**Date**: 2026-02-01
**Status**: Accepted

### Context
Need a modern web framework for ERP management interface with complex tables, forms, and reports.

### Decision
Use Next.js 15 with App Router, Tailwind CSS, Tanstack Query/Table, Zustand, and Zod.

### Rationale
- Server Components reduce client bundle size
- App Router provides excellent route organization for ERP modules
- Tanstack Table handles complex data grids efficiently
- Zod provides type-safe form validation matching backend schemas

### Consequences
- Need to carefully manage Server vs Client Component boundaries
- Tanstack Query replaces server-side data fetching for interactive pages

---

## ADR-004: Native iOS + Android for Mobile (Field Operations Only)

**Date**: 2026-02-01
**Status**: Accepted

### Context
Mobile app needed for field operations: inventory scanning, attendance tracking, quality inspection.

### Decision
Use native iOS (Swift/SwiftUI) and Android (Kotlin/Jetpack Compose) for mobile apps. Focus on field operations, not full ERP.

### Rationale
- Native performance and full platform API access
- SwiftUI and Jetpack Compose provide modern declarative UI frameworks
- Better integration with device hardware (camera, NFC for scanning)
- Offline-first patterns important for field use
- Native apps provide superior UX on each platform

### Consequences
- Mobile app scope limited to field operations
- Full ERP management done via web interface
- Two codebases to maintain (iOS + Android)
- Shared business logic via API contracts

---

## ADR-005: SAP-like ERP Module Design

**Date**: 2026-02-01
**Status**: Accepted

### Context
Building an ERP system that follows industry-standard patterns.

### Decision
Adopt SAP-like module structure: FI, CO, MM, SD, PP, HR, WM, QM with document flow, number ranges, and status management.

### Rationale
- Proven ERP design patterns from SAP
- Clear module boundaries and responsibilities
- Document flow enables full traceability
- Status management provides lifecycle control

### Consequences
- More complex initial design
- Need ERP domain expertise
- Cross-module integration requires careful planning

---

## ADR-006: RBAC with SAP-like Authorization Objects

**Date**: 2026-02-01
**Status**: Accepted

### Context
ERP systems require fine-grained access control per module and operation.

### Decision
Implement RBAC using SAP-like authorization objects (e.g., S_SD_ORDER with CREATE/READ/UPDATE/DELETE actions).

### Rationale
- Fine-grained control at module + action level
- Scalable to new modules without code changes
- Audit-friendly (clear permission model)
- Industry standard for ERP systems

### Consequences
- More complex permission management
- Need role/auth-object administration UI
- JWT tokens carry authorization data

---

---

## ADR-007: Status Machine with Audit Trail

**Date**: 2026-02-21
**Status**: Accepted

### Context
ERP workflows need validated status transitions and audit history. Need to prevent invalid status changes (e.g., DRAFT -> CLOSED) and track who changed what and when.

### Decision
1. Centralized `validate_transition()` in `backend/src/shared/status.rs` with explicit match patterns for 6 document types
2. `document_status_history` table recording every transition with from/to status, user, reason, timestamp
3. `record_transition()` function called at every status change point across SD, MM, PP, FI modules
4. CHECK constraints on 8 core tables to enforce valid status values at DB level
5. `WorkflowTimeline` frontend component showing visual status history on detail pages

### Rationale
- Defense in depth: application-level validation + DB constraints
- Full audit trail for regulatory compliance
- Visual timeline helps users understand document lifecycle
- Centralized status machine prevents inconsistencies across modules

### Consequences
- Every status-changing service function must call `record_transition()`
- New document types require additions to both `DocumentType` enum and `validate_transition()` match
- CHECK constraints must be updated when adding new valid statuses

---

## ADR-008: Toast Notifications via Sonner

**Date**: 2026-02-21
**Status**: Accepted

### Context
Frontend mutations (create order, confirm, goods receipt, etc.) executed silently with no user feedback on success or failure.

### Decision
Use `sonner` library with custom `useToastMutation` hook wrapping TanStack Query's `useMutation`. Auto-shows success/error toasts and invalidates related query caches.

### Rationale
- Lightweight (< 5KB) with rich color support
- Generic hook pattern allows easy adoption across all pages
- Built-in query invalidation reduces boilerplate

### Consequences
- All new mutations should use `useToastMutation` instead of raw `useMutation`
- Toast position: top-right (configured in providers.tsx)

---

## Template for New Decisions

```markdown
## ADR-XXX: [Title]

**Date**: YYYY-MM-DD
**Status**: Proposed | Accepted | Deprecated | Superseded

### Context
[What is the issue?]

### Decision
[What was decided?]

### Rationale
[Why this decision?]

### Consequences
[What are the implications?]
```
