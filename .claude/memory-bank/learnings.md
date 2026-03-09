# Project Learnings & Gotchas

> **Usage**: Record issues encountered and their solutions. Helps avoid repeating mistakes.

---

## General

### Git Workflow
- Always work on feature branches, never commit directly to `main`
- Branch naming: `feature/xxx`, `fix/xxx`, `refactor/xxx`
- Commit messages in Chinese are acceptable

### Environment Variables
- Never commit `.env` files
- Each service has its own `.env`: `backend/.env`, `web/.env.local`
- Use `.env.example` files as templates

---

## Rust Backend

### Issue: Axum State Extraction Order
**Problem**: Multiple extractors fail.
**Solution**: Body-consuming extractors (Json) must come last.

```rust
// Correct order
pub async fn handler(
    State(state): State<Arc<AppState>>,  // First
    Path(id): Path<Uuid>,                // Second
    Json(body): Json<Request>,           // Last (consumes body)
) -> Result<Json<Response>, AppError> { ... }
```

### Issue: sqlx Compile-Time Checking
**Problem**: `DATABASE_URL` must be set during compilation for query_as! macros.
**Solution**: Use `.env` file or `sqlx prepare` for offline mode.

### Issue: sqlx Query Fails with Unknown Column
**Problem**: Query references column that exists in migration but not in actual DB.
**Solution**: Check actual schema before writing queries. Always run migrations before building.

### Issue: Decimal for Monetary Fields
**Problem**: Float precision errors in financial calculations.
**Solution**: Always use `rust_decimal::Decimal` for amounts, prices, quantities. Never use f32/f64.

```rust
use rust_decimal::Decimal;

pub struct SalesOrderItem {
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub amount: Decimal,  // = quantity * unit_price
}
```

---

## Next.js Web

### Issue: Server vs Client Component Confusion
**Problem**: Using hooks (useState, useEffect) in Server Components causes errors.
**Solution**: Add `'use client'` directive to any component that uses React hooks, browser APIs, or event handlers.

### Issue: Tanstack Query SSR Hydration
**Problem**: Data fetched on server not matching client hydration.
**Solution**: Use QueryClientProvider at root level, configure staleTime properly.

---

## Native Mobile (iOS + Android)

### Issue: iOS - @StateObject vs @ObservedObject Confusion
**Problem**: View recreates ViewModel on every re-render when using `@ObservedObject`.
**Solution**: Use `@StateObject` for the view that owns the ViewModel, `@ObservedObject` for views that receive it from a parent.

```swift
// Owner view - creates the ViewModel
struct InventoryScreen: View {
    @StateObject private var viewModel = InventoryViewModel()  // Persists across re-renders
    var body: some View { ChildView(viewModel: viewModel) }
}

// Child view - receives the ViewModel
struct ChildView: View {
    @ObservedObject var viewModel: InventoryViewModel  // Observes but doesn't own
}
```

### Issue: Android - StateFlow Not Collecting in Compose
**Problem**: UI not updating when StateFlow value changes.
**Solution**: Use `collectAsStateWithLifecycle()` instead of `collectAsState()` for lifecycle-aware collection.

```kotlin
// Correct - lifecycle-aware
val uiState by viewModel.uiState.collectAsStateWithLifecycle()

// Incorrect - not lifecycle-aware, may leak
val uiState by viewModel.uiState.collectAsState()
```

### Issue: iOS - Async/Await in SwiftUI Task
**Problem**: Network calls on background thread trying to update `@Published` properties causes warnings.
**Solution**: Mark ViewModel with `@MainActor` to ensure UI updates happen on main thread.

```swift
@MainActor
class InventoryViewModel: ObservableObject {
    @Published var items: [InventoryItem] = []
    func loadItems() async { /* Safe to update @Published here */ }
}
```

### Issue: Android - Hilt Injection in Compose
**Problem**: ViewModel dependencies not injected properly in Composable functions.
**Solution**: Use `@HiltViewModel` annotation and `hiltViewModel()` in Composable.

```kotlin
@HiltViewModel
class InventoryViewModel @Inject constructor(
    private val repository: InventoryRepository
) : ViewModel() { /* ... */ }

// In Composable
@Composable
fun InventoryScreen(viewModel: InventoryViewModel = hiltViewModel()) { /* ... */ }
```

### Issue: Dynamic Type from API (Both Platforms)
**Problem**: Backend returns inconsistent types (`"1"` vs `1` vs `1.0`) for numeric fields.
**Solution (iOS)**: Use custom Codable decoder:

```swift
extension KeyedDecodingContainer {
    func decodeFlexibleInt(forKey key: Key) throws -> Int {
        if let intVal = try? decode(Int.self, forKey: key) { return intVal }
        if let doubleVal = try? decode(Double.self, forKey: key) { return Int(doubleVal) }
        if let strVal = try? decode(String.self, forKey: key), let parsed = Int(strVal) { return parsed }
        throw DecodingError.typeMismatch(Int.self, .init(codingPath: [key], debugDescription: "Cannot decode Int"))
    }
}
```

**Solution (Android)**: Use custom Gson/Moshi adapter or Kotlinx Serialization:

```kotlin
@Serializable
data class InventoryItem(
    val id: String,
    @Serializable(with = FlexibleIntSerializer::class)
    val count: Int
)
```

---

## Cross-Layer Issues

### Issue: Cross-Layer Field Name Mismatch
**Problem**: API returns `field_a` but frontend model expects `field_b`, causing errors or null values.
**Solution**: Always verify field names match across all layers before deployment.

```
Check order:
1. DB schema (migration SQL)
2. Rust model + repository SQL
3. TypeScript types (web)
4. iOS Codable model / Android @Serializable data class (mobile)
```

### Issue: Migration Not Executed Before Deploy
**Problem**: Code references columns that haven't been created yet.
**Solution**: Always run migrations before deploying new code. Include migration check in deployment workflow.

---

## Security

### Issue: API Keys in Source Code
**Problem**: .env files or hardcoded keys in source control.
**Solution**:
1. Never commit .env files (ensure .gitignore)
2. Use `.env.example` with placeholder values
3. Run secret scanning (Gitleaks) in CI

### DDoS Protection (Middleware Stack)
**Learning**: Rate Limiting should be Per-IP, not Global. Middleware order matters:
1. Compression
2. Tracing
3. Security Headers (X-Content-Type-Options, X-Frame-Options, X-XSS-Protection)
4. Request timeout (30s, prevents Slowloris)
5. Request body size limit (10MB)
6. Per-IP rate limiting
7. CORS

---

## Workflow

### Issue: Status Values Differ Between Plan and Reality
**Problem**: Plan assumed generic status names (APPROVED, PLANNED) but actual `status.rs` had different values (RELEASED, CREATED).
**Solution**: Always read `backend/src/shared/status.rs` before planning status-related changes. The source of truth is the `validate_transition()` match patterns.

### Issue: Table Names Are Module-Prefixed
**Problem**: Assumed table names like `sales_orders` but actual names are `sd_sales_orders`, `mm_purchase_orders`, etc.
**Solution**: Always read migration files to verify actual table names. Naming convention is `{module_code}_{entity_name}`.

### Issue: users.id is UUID, not INTEGER
**Problem**: Plan referenced `changed_by INTEGER` but users.id is UUID.
**Solution**: Always verify referenced table column types before creating foreign keys.

### Issue: ApiResponse Double Wrapping
**Problem**: Frontend WorkflowTimeline used `.then(res => res.data)` but `res.data` is the Axios-unwrapped response which is `ApiResponse<T>`, meaning `res.data.data` holds the actual payload.
**Solution**: When using Axios with ApiResponse backend, the pattern is:
```typescript
// Axios: res.data = { success: true, data: [...], message: null }
// To get the actual data: res.data.data (or res.data?.data ?? [])
```

### Issue: Cross-Module Function Signature Changes
**Problem**: When `post_journal_entry()` gained a `user_id` parameter, it broke callers in SD and MM modules.
**Solution**: Use Grep to find ALL callers of a function before changing its signature:
```bash
grep -r "post_journal_entry" backend/src/
```

### Issue: Agent Coordination Gaps
**Problem**: Cross-layer changes require multiple agents but coordination is manual.
**Solution**: Use workflow commands with explicit agent specification and follow the communication protocol.

### Code Quality Gates
| Gate | Tool | Threshold |
|------|------|-----------|
| Format Check | cargo fmt / eslint / swiftformat / ktlint | 100% compliance |
| Linting | clippy / eslint / swiftlint / detekt | Zero warnings |
| Unit Tests | cargo test / vitest / XCTest / JUnit | 85% coverage |
| Security Scan | cargo audit / pnpm audit | Zero high-severity |

### Documentation Sync Workflow
1. API change -> Update TypeScript types + iOS Codable models + Android data classes
2. Schema change -> Update migration + Rust models + TS types + iOS models + Android models
3. ERP module change -> Verify cross-module integration

---

## Template for New Learnings

```markdown
### Issue: [Brief Title]
**Problem**: [What went wrong]
**Solution**: [How to fix it]

[Optional code example]
```
