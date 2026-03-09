# Native Mobile Feature Template

> Use this template when creating a new feature in the iOS or Android mobile apps.

---

## iOS Directory Structure (Swift/SwiftUI)

```
ios/TasteByte/Features/{FeatureName}/
├── Domain/
│   ├── Entities/
│   │   └── {Feature}.swift
│   ├── Repositories/
│   │   └── {Feature}RepositoryProtocol.swift
│   └── UseCases/
│       └── Get{Feature}UseCase.swift
├── Data/
│   ├── Models/
│   │   └── {Feature}Response.swift
│   ├── DataSources/
│   │   ├── {Feature}RemoteDataSource.swift
│   │   └── {Feature}LocalDataSource.swift
│   └── Repositories/
│       └── {Feature}RepositoryImpl.swift
└── Presentation/
    ├── ViewModels/
    │   └── {Feature}ViewModel.swift
    ├── Views/
    │   └── {Feature}Screen.swift
    └── Components/
        └── {Feature}Card.swift
```

## Android Directory Structure (Kotlin/Jetpack Compose)

```
android/app/src/main/java/com/tastebyte/features/{feature}/
├── domain/
│   ├── model/
│   │   └── {Feature}.kt
│   ├── repository/
│   │   └── {Feature}Repository.kt
│   └── usecase/
│       └── Get{Feature}UseCase.kt
├── data/
│   ├── model/
│   │   └── {Feature}Response.kt
│   ├── remote/
│   │   └── {Feature}Api.kt
│   ├── local/
│   │   └── {Feature}LocalDataSource.kt
│   └── repository/
│       └── {Feature}RepositoryImpl.kt
└── presentation/
    ├── viewmodel/
    │   └── {Feature}ViewModel.kt
    ├── screen/
    │   └── {Feature}Screen.kt
    └── component/
        └── {Feature}Card.kt
```

---

## 1. Entity (Domain Layer)

### iOS
```swift
// Features/{Feature}/Domain/Entities/{Feature}.swift
struct {Feature}: Identifiable, Equatable {
    let id: String
    let name: String
    let createdAt: Date
    let updatedAt: Date?
}
```

### Android
```kotlin
// features/{feature}/domain/model/{Feature}.kt
data class {Feature}(
    val id: String,
    val name: String,
    val createdAt: String,
    val updatedAt: String? = null,
)
```

---

## 2. API Response Model (Data Layer)

### iOS
```swift
// Features/{Feature}/Data/Models/{Feature}Response.swift
struct {Feature}Response: Codable {
    let id: String
    let name: String
    let createdAt: Date
    let updatedAt: Date?

    enum CodingKeys: String, CodingKey {
        case id, name
        case createdAt = "created_at"
        case updatedAt = "updated_at"
    }

    func toEntity() -> {Feature} {
        {Feature}(id: id, name: name, createdAt: createdAt, updatedAt: updatedAt)
    }
}
```

### Android
```kotlin
// features/{feature}/data/model/{Feature}Response.kt
@Serializable
data class {Feature}Response(
    val id: String,
    val name: String,
    @SerialName("created_at") val createdAt: String,
    @SerialName("updated_at") val updatedAt: String? = null,
) {
    fun toEntity() = {Feature}(id = id, name = name, createdAt = createdAt, updatedAt = updatedAt)
}
```

---

## 3. Repository Interface (Domain Layer)

### iOS
```swift
// Features/{Feature}/Domain/Repositories/{Feature}RepositoryProtocol.swift
protocol {Feature}RepositoryProtocol {
    func getAll() async throws -> [{Feature}]
    func getById(id: String) async throws -> {Feature}
    func create(_ entity: {Feature}) async throws -> {Feature}
    func update(_ entity: {Feature}) async throws -> {Feature}
    func delete(id: String) async throws
}
```

### Android
```kotlin
// features/{feature}/domain/repository/{Feature}Repository.kt
interface {Feature}Repository {
    suspend fun getAll(): List<{Feature}>
    suspend fun getById(id: String): {Feature}
    suspend fun create(entity: {Feature}): {Feature}
    suspend fun update(entity: {Feature}): {Feature}
    suspend fun delete(id: String)
}
```

---

## 4. Repository Implementation (Data Layer)

### iOS
```swift
// Features/{Feature}/Data/Repositories/{Feature}RepositoryImpl.swift
class {Feature}RepositoryImpl: {Feature}RepositoryProtocol {
    private let apiClient: APIClient
    private let localStore: {Feature}LocalDataSource
    private let networkMonitor: NetworkMonitor

    init(apiClient: APIClient, localStore: {Feature}LocalDataSource, networkMonitor: NetworkMonitor) {
        self.apiClient = apiClient
        self.localStore = localStore
        self.networkMonitor = networkMonitor
    }

    func getAll() async throws -> [{Feature}] {
        if networkMonitor.isConnected {
            let response: ApiResponse<[{Feature}Response]> = try await apiClient.get("/{module}/{feature}s")
            let entities = response.data.map { $0.toEntity() }
            try await localStore.cacheAll(entities)
            return entities
        } else {
            return try await localStore.getCached()
        }
    }
}
```

### Android
```kotlin
// features/{feature}/data/repository/{Feature}RepositoryImpl.kt
class {Feature}RepositoryImpl @Inject constructor(
    private val api: {Feature}Api,
    private val localStore: {Feature}LocalDataSource,
    private val networkMonitor: NetworkMonitor,
) : {Feature}Repository {

    override suspend fun getAll(): List<{Feature}> {
        return if (networkMonitor.isConnected) {
            val response = api.getAll()
            val entities = response.data.map { it.toEntity() }
            localStore.cacheAll(entities)
            entities
        } else {
            localStore.getCached().map { it.toEntity() }
        }
    }
}
```

---

## 5. ViewModel (Presentation Layer)

### iOS
```swift
// Features/{Feature}/Presentation/ViewModels/{Feature}ViewModel.swift
@MainActor
class {Feature}ViewModel: ObservableObject {
    @Published var items: [{Feature}] = []
    @Published var isLoading = false
    @Published var error: String?

    private let repository: {Feature}RepositoryProtocol

    init(repository: {Feature}RepositoryProtocol) {
        self.repository = repository
    }

    func loadItems() async {
        isLoading = true
        error = nil
        do {
            items = try await repository.getAll()
        } catch {
            self.error = error.localizedDescription
        }
        isLoading = false
    }

    func refresh() async {
        await loadItems()
    }

    func addItem(_ item: {Feature}) async {
        do {
            let created = try await repository.create(item)
            items.append(created)
        } catch {
            self.error = error.localizedDescription
        }
    }
}
```

### Android
```kotlin
// features/{feature}/presentation/viewmodel/{Feature}ViewModel.kt
@HiltViewModel
class {Feature}ViewModel @Inject constructor(
    private val repository: {Feature}Repository,
) : ViewModel() {
    private val _uiState = MutableStateFlow<{Feature}UiState>({Feature}UiState.Loading)
    val uiState: StateFlow<{Feature}UiState> = _uiState.asStateFlow()

    init { loadItems() }

    fun loadItems() {
        viewModelScope.launch {
            _uiState.value = {Feature}UiState.Loading
            try {
                val items = repository.getAll()
                _uiState.value = {Feature}UiState.Success(items)
            } catch (e: Exception) {
                _uiState.value = {Feature}UiState.Error(e.message ?: "Unknown error")
            }
        }
    }

    fun addItem(item: {Feature}) {
        viewModelScope.launch {
            try {
                repository.create(item)
                loadItems()
            } catch (e: Exception) {
                _uiState.value = {Feature}UiState.Error(e.message ?: "Failed to create")
            }
        }
    }
}

sealed interface {Feature}UiState {
    data object Loading : {Feature}UiState
    data class Success(val items: List<{Feature}>) : {Feature}UiState
    data class Error(val message: String) : {Feature}UiState
}
```

---

## 6. Screen (Presentation Layer)

### iOS
```swift
// Features/{Feature}/Presentation/Views/{Feature}Screen.swift
struct {Feature}Screen: View {
    @StateObject private var viewModel: {Feature}ViewModel

    init(repository: {Feature}RepositoryProtocol) {
        _viewModel = StateObject(wrappedValue: {Feature}ViewModel(repository: repository))
    }

    var body: some View {
        NavigationStack {
            Group {
                if viewModel.isLoading {
                    ProgressView()
                } else if let error = viewModel.error {
                    VStack(spacing: 16) {
                        Text("Error: \(error)")
                        Button("Retry") { Task { await viewModel.loadItems() } }
                    }
                } else {
                    List(viewModel.items) { item in
                        {Feature}Card(item: item)
                    }
                    .refreshable { await viewModel.refresh() }
                }
            }
            .navigationTitle("{Feature}")
            .toolbar {
                ToolbarItem(placement: .primaryAction) {
                    Button(action: showAddDialog) {
                        Image(systemName: "plus")
                    }
                }
            }
            .task { await viewModel.loadItems() }
        }
    }

    private func showAddDialog() {
        // TODO: Implement add dialog
    }
}
```

### Android
```kotlin
// features/{feature}/presentation/screen/{Feature}Screen.kt
@Composable
fun {Feature}Screen(
    viewModel: {Feature}ViewModel = hiltViewModel(),
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()

    Scaffold(
        topBar = { TopAppBar(title = { Text("{Feature}") }) },
        floatingActionButton = {
            FloatingActionButton(onClick = { /* TODO: show add dialog */ }) {
                Icon(Icons.Default.Add, contentDescription = "Add")
            }
        }
    ) { padding ->
        Box(modifier = Modifier.padding(padding)) {
            when (val state = uiState) {
                is {Feature}UiState.Loading -> {
                    CircularProgressIndicator(modifier = Modifier.align(Alignment.Center))
                }
                is {Feature}UiState.Error -> {
                    Column(
                        modifier = Modifier.align(Alignment.Center),
                        horizontalAlignment = Alignment.CenterHorizontally,
                    ) {
                        Text("Error: ${state.message}")
                        Spacer(modifier = Modifier.height(16.dp))
                        Button(onClick = { viewModel.loadItems() }) { Text("Retry") }
                    }
                }
                is {Feature}UiState.Success -> {
                    LazyColumn {
                        items(state.items, key = { it.id }) { item ->
                            {Feature}Card(item = item)
                        }
                    }
                }
            }
        }
    }
}
```

---

## Checklist

### iOS
- [ ] Created directory structure under `ios/TasteByte/Features/`
- [ ] Implemented entity as Swift struct
- [ ] Implemented Codable response model with snake_case mapping
- [ ] Created repository protocol
- [ ] Implemented repository with offline support
- [ ] Created @MainActor ViewModel with @Published properties
- [ ] Built SwiftUI screen with loading/error/success states
- [ ] Added to navigation/router configuration
- [ ] Written XCTest unit tests for ViewModel

### Android
- [ ] Created directory structure under `android/app/.../features/`
- [ ] Implemented entity as Kotlin data class
- [ ] Implemented @Serializable response model with @SerialName mapping
- [ ] Created repository interface
- [ ] Implemented repository with offline support
- [ ] Created @HiltViewModel with StateFlow
- [ ] Built Compose screen with loading/error/success states
- [ ] Added to navigation graph
- [ ] Written JUnit tests for ViewModel
