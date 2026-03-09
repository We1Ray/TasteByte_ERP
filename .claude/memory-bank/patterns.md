# Code Patterns & Conventions

> **Usage**: Reference these patterns when implementing new features. Maintain consistency across the codebase.

---

## Rust Patterns

### Axum Handler Pattern
```rust
use axum::{Json, Extension, extract::{Path, Query, State}};
use crate::{AppState, Result, ApiResponse};

pub async fn get_sales_order(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<CurrentUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<SalesOrder>>, AppError> {
    user.check_auth_object("S_SD_ORDER", "READ")?;

    let order = state.sd_service
        .get_by_id(id)
        .await?
        .ok_or(AppError::NotFound("Sales order not found".into()))?;

    Ok(Json(ApiResponse::success(order)))
}

pub async fn create_sales_order(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<CurrentUser>,
    Json(payload): Json<CreateSalesOrderRequest>,
) -> Result<Json<ApiResponse<SalesOrder>>, AppError> {
    user.check_auth_object("S_SD_ORDER", "CREATE")?;
    payload.validate()?;

    let order = state.sd_service
        .create(user.id, payload)
        .await?;

    Ok(Json(ApiResponse::success(order)))
}
```

### Error Handling Pattern
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Business rule: {0}")]
    BusinessRule(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            AppError::Forbidden(_) => (StatusCode::FORBIDDEN, "FORBIDDEN"),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR"),
            AppError::BusinessRule(_) => (StatusCode::UNPROCESSABLE_ENTITY, "BUSINESS_RULE"),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DB_ERROR"),
        };

        let body = Json(ApiResponse::<()>::error(code, &self.to_string()));
        (status, body).into_response()
    }
}
```

### Repository Pattern (SQLx)
```rust
pub struct SdRepository {
    pool: PgPool,
}

impl SdRepository {
    pub async fn find_sales_order_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<SalesOrder>, sqlx::Error> {
        sqlx::query_as!(
            SalesOrder,
            r#"
            SELECT id, document_number, customer_id, status,
                   total_amount, currency, posting_date, fiscal_year,
                   created_at, updated_at
            FROM sd_sales_orders
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }
}
```

---

## Next.js Patterns

### Tanstack Query Pattern
```typescript
// lib/hooks/use-sales-orders.ts
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { apiClient } from '@/lib/api/client';

export const queryKeys = {
  salesOrders: {
    all: ['sd', 'sales-orders'] as const,
    detail: (id: string) => ['sd', 'sales-orders', id] as const,
  },
};

export function useSalesOrders(params?: SalesOrderQuery) {
  return useQuery({
    queryKey: [...queryKeys.salesOrders.all, params],
    queryFn: () => apiClient<PaginatedResponse<SalesOrder>>('/sd/sales-orders', { params }),
  });
}

export function useCreateSalesOrder() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateSalesOrderInput) =>
      apiClient<SalesOrder>('/sd/sales-orders', {
        method: 'POST',
        body: JSON.stringify(data),
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: queryKeys.salesOrders.all });
    },
  });
}
```

### Zustand Store Pattern
```typescript
// lib/stores/auth-store.ts
import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface AuthState {
  token: string | null;
  user: User | null;
  login: (email: string, password: string) => Promise<void>;
  logout: () => void;
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      token: null,
      user: null,
      login: async (email, password) => { /* ... */ },
      logout: () => set({ token: null, user: null }),
    }),
    { name: 'auth-storage' },
  ),
);
```

### Zod Validation Pattern
```typescript
import { z } from 'zod';

export const salesOrderSchema = z.object({
  customer_id: z.string().uuid(),
  order_date: z.string().date(),
  items: z.array(z.object({
    material_id: z.string().uuid(),
    quantity: z.number().positive(),
    unit_price: z.number().nonnegative(),
  })).min(1),
});

export type CreateSalesOrderInput = z.infer<typeof salesOrderSchema>;
```

---

## Native Mobile Patterns

### iOS - SwiftUI State Management (@StateObject / @ObservedObject)
```swift
// InventoryViewModel.swift
@MainActor
class InventoryViewModel: ObservableObject {
    @Published var items: [InventoryItem] = []
    @Published var isLoading = false
    @Published var error: String?

    private let repository: InventoryRepository

    init(repository: InventoryRepository) {
        self.repository = repository
    }

    func loadItems() async {
        isLoading = true
        error = nil
        do {
            items = try await repository.getItems()
        } catch {
            self.error = error.localizedDescription
        }
        isLoading = false
    }

    func updateCount(itemId: String, count: Int) async {
        do {
            try await repository.updateCount(itemId: itemId, count: count)
            await loadItems()
        } catch {
            self.error = error.localizedDescription
        }
    }
}

// InventoryScreen.swift
struct InventoryScreen: View {
    @StateObject private var viewModel: InventoryViewModel

    var body: some View {
        Group {
            if viewModel.isLoading {
                ProgressView()
            } else if let error = viewModel.error {
                ErrorView(message: error)
            } else {
                List(viewModel.items) { item in
                    InventoryRow(item: item)
                }
            }
        }
        .task { await viewModel.loadItems() }
    }
}
```

### Android - Kotlin Jetpack Compose (ViewModel / StateFlow)
```kotlin
// InventoryViewModel.kt
class InventoryViewModel(
    private val repository: InventoryRepository
) : ViewModel() {
    private val _uiState = MutableStateFlow<InventoryUiState>(InventoryUiState.Loading)
    val uiState: StateFlow<InventoryUiState> = _uiState.asStateFlow()

    init { loadItems() }

    fun loadItems() {
        viewModelScope.launch {
            _uiState.value = InventoryUiState.Loading
            try {
                val items = repository.getItems()
                _uiState.value = InventoryUiState.Success(items)
            } catch (e: Exception) {
                _uiState.value = InventoryUiState.Error(e.message ?: "Unknown error")
            }
        }
    }

    fun updateCount(itemId: String, count: Int) {
        viewModelScope.launch {
            try {
                repository.updateCount(itemId, count)
                loadItems()
            } catch (e: Exception) {
                _uiState.value = InventoryUiState.Error(e.message ?: "Unknown error")
            }
        }
    }
}

// InventoryScreen.kt
@Composable
fun InventoryScreen(viewModel: InventoryViewModel = hiltViewModel()) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()

    when (val state = uiState) {
        is InventoryUiState.Loading -> CircularProgressIndicator()
        is InventoryUiState.Error -> ErrorView(message = state.message)
        is InventoryUiState.Success -> {
            LazyColumn {
                items(state.items) { item ->
                    InventoryRow(item = item)
                }
            }
        }
    }
}
```

### iOS - Repository Pattern (URLSession)
```swift
class InventoryRepository {
    private let apiClient: APIClient

    init(apiClient: APIClient) {
        self.apiClient = apiClient
    }

    func getItems() async throws -> [InventoryItem] {
        let response: ApiResponse<[InventoryItem]> = try await apiClient.get("/wm/inventory")
        return response.data
    }

    func updateCount(itemId: String, count: Int) async throws {
        let _: ApiResponse<EmptyResponse> = try await apiClient.put(
            "/wm/inventory/\(itemId)",
            body: ["count": count]
        )
    }
}
```

### Android - Repository Pattern (Retrofit/Ktor)
```kotlin
class InventoryRepository(private val api: InventoryApi) {
    suspend fun getItems(): List<InventoryItem> {
        val response = api.getInventory()
        return response.data
    }

    suspend fun updateCount(itemId: String, count: Int) {
        api.updateInventory(itemId, UpdateCountRequest(count = count))
    }
}
```

---

## SQL Patterns

### ERP Transaction Table
```sql
CREATE TABLE sd_sales_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_number VARCHAR(20) NOT NULL UNIQUE,
    customer_id UUID NOT NULL REFERENCES sd_customers(id),
    status VARCHAR(20) NOT NULL DEFAULT 'DRAFT',
    total_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    currency VARCHAR(3) NOT NULL DEFAULT 'TWD',
    fiscal_year INTEGER NOT NULL,
    posting_date DATE NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id)
);

-- Indexes
CREATE INDEX idx_sd_sales_orders_customer_id ON sd_sales_orders(customer_id);
CREATE INDEX idx_sd_sales_orders_status ON sd_sales_orders(status);
CREATE INDEX idx_sd_sales_orders_posting_date ON sd_sales_orders(posting_date);

-- Trigger
CREATE TRIGGER update_sd_sales_orders_updated_at
    BEFORE UPDATE ON sd_sales_orders
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

### Paginated Query Pattern
```sql
SELECT *, COUNT(*) OVER() as total_count
FROM sd_sales_orders
WHERE status = $1
  AND ($2::DATE IS NULL OR posting_date >= $2)
ORDER BY created_at DESC
LIMIT $3 OFFSET $4;
```

---

## ERP Integration Patterns

### Document Number Generation
```rust
pub async fn generate_document_number(
    pool: &PgPool,
    prefix: &str,  // "SO", "PO", "JE"
    fiscal_year: i32,
) -> Result<String, sqlx::Error> {
    let result = sqlx::query_scalar!(
        r#"
        SELECT COALESCE(MAX(CAST(SPLIT_PART(document_number, '-', 3) AS INTEGER)), 0) + 1
        FROM sd_sales_orders
        WHERE document_number LIKE $1
        "#,
        format!("{}-{}-%%", prefix, fiscal_year),
    )
    .fetch_one(pool)
    .await?;

    Ok(format!("{}-{}-{:05}", prefix, fiscal_year, result.unwrap_or(1)))
}
// Result: "SO-2026-00001"
```

### Status Transition Validation
```rust
pub fn validate_status_transition(
    current: &str,
    target: &str,
) -> Result<(), AppError> {
    let valid_transitions = match current {
        "DRAFT" => vec!["CONFIRMED", "CANCELLED"],
        "CONFIRMED" => vec!["IN_PROGRESS", "CANCELLED"],
        "IN_PROGRESS" => vec!["COMPLETED", "CANCELLED"],
        "COMPLETED" => vec!["CLOSED"],
        _ => vec![],
    };

    if valid_transitions.contains(&target) {
        Ok(())
    } else {
        Err(AppError::BusinessRule(format!(
            "Invalid transition: {} -> {}", current, target
        )))
    }
}
```

---

## Testing Patterns

### Rust Test Pattern
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    async fn setup_test_server() -> TestServer {
        let app = create_app(test_config()).await;
        TestServer::new(app).unwrap()
    }

    #[tokio::test]
    async fn test_create_sales_order_success() {
        let server = setup_test_server().await;
        let response = server
            .post("/api/v1/sd/sales-orders")
            .json(&json!({
                "customer_id": "...",
                "order_date": "2026-02-19",
                "items": [{"material_id": "...", "quantity": 10, "unit_price": 100}]
            }))
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);
    }
}
```

### Next.js Test Pattern (vitest)
```typescript
import { render, screen } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { SalesOrderTable } from './sales-order-table';

describe('SalesOrderTable', () => {
  it('renders loading state', () => {
    const queryClient = new QueryClient();
    render(
      <QueryClientProvider client={queryClient}>
        <SalesOrderTable />
      </QueryClientProvider>
    );
    expect(screen.getByRole('progressbar')).toBeInTheDocument();
  });
});
```

### iOS Test Pattern (XCTest)
```swift
final class InventoryViewModelTests: XCTestCase {
    var sut: InventoryViewModel!
    var mockRepository: MockInventoryRepository!

    override func setUp() {
        super.setUp()
        mockRepository = MockInventoryRepository()
        sut = InventoryViewModel(repository: mockRepository)
    }

    func testLoadItemsSuccess() async {
        mockRepository.stubbedItems = [InventoryItem(id: "1", name: "Item A", count: 10)]

        await sut.loadItems()

        XCTAssertEqual(sut.items.count, 1)
        XCTAssertEqual(sut.items.first?.name, "Item A")
        XCTAssertFalse(sut.isLoading)
        XCTAssertNil(sut.error)
    }
}
```

### Android Test Pattern (JUnit + Turbine)
```kotlin
@OptIn(ExperimentalCoroutinesApi::class)
class InventoryViewModelTest {
    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()

    private lateinit var viewModel: InventoryViewModel
    private lateinit var mockRepository: FakeInventoryRepository

    @Before
    fun setUp() {
        mockRepository = FakeInventoryRepository()
        viewModel = InventoryViewModel(mockRepository)
    }

    @Test
    fun `loadItems emits success state`() = runTest {
        mockRepository.setItems(listOf(InventoryItem("1", "Item A", 10)))

        viewModel.uiState.test {
            viewModel.loadItems()
            assertIs<InventoryUiState.Loading>(awaitItem())
            val success = awaitItem() as InventoryUiState.Success
            assertEquals(1, success.items.size)
            assertEquals("Item A", success.items.first().name)
        }
    }
}
```

---

## Pre-commit Hook Pattern

```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: cargo-fmt
        name: Rust Format
        entry: cargo fmt --manifest-path backend/Cargo.toml --
        language: system
        types: [rust]
        pass_filenames: false

      - id: cargo-clippy
        name: Rust Lint
        entry: cargo clippy --manifest-path backend/Cargo.toml -- -D warnings
        language: system
        types: [rust]
        pass_filenames: false

  - repo: local
    hooks:
      - id: eslint
        name: ESLint
        entry: pnpm --dir web run lint
        language: system
        types: [typescript, tsx]
        pass_filenames: false

  - repo: https://github.com/gitleaks/gitleaks
    rev: v8.18.0
    hooks:
      - id: gitleaks
```
