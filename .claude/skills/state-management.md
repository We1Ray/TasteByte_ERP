# 狀態管理指南

## 目錄
1. [選擇策略](#選擇策略)
2. [iOS - SwiftUI 狀態管理](#ios---swiftui-狀態管理)
3. [Android - Jetpack Compose 狀態管理](#android---jetpack-compose-狀態管理)
4. [狀態設計原則](#狀態設計原則)

## 選擇策略

### iOS (Swift/SwiftUI)
| 使用場景 | 推薦方案 | 理由 |
|----------|----------|------|
| 簡單 View 狀態 | @State | 本地狀態，SwiftUI 管理生命週期 |
| ViewModel 持有 | @StateObject | 擁有者建立，跨 re-render 存活 |
| 父 View 傳入 | @ObservedObject | 觀察但不擁有 |
| 全域共享 | @EnvironmentObject | 依賴注入，跨 View 層級共享 |
| 持久化 | @AppStorage | UserDefaults 自動同步 |

### Android (Kotlin/Jetpack Compose)
| 使用場景 | 推薦方案 | 理由 |
|----------|----------|------|
| 簡單 UI 狀態 | remember + mutableStateOf | 本地 Composable 狀態 |
| 業務邏輯狀態 | ViewModel + StateFlow | 生命週期感知，可測試 |
| 跨層級共享 | CompositionLocal | 隱式依賴注入 |
| 導航參數 | SavedStateHandle | 進程恢復安全 |
| 分頁資料 | Paging 3 | 內建分頁支援 |

## iOS - SwiftUI 狀態管理

### @State - 本地 View 狀態
```swift
struct CounterView: View {
    @State private var count = 0

    var body: some View {
        VStack {
            Text("Count: \(count)")
            Button("Increment") { count += 1 }
        }
    }
}
```

### @StateObject + ObservableObject - ViewModel 模式
```swift
@MainActor
class AuthViewModel: ObservableObject {
    @Published var state: AuthState = .initial

    enum AuthState {
        case initial
        case loading
        case authenticated(User)
        case unauthenticated
        case error(String)
    }

    private let loginUseCase: LoginUseCase
    private let logoutUseCase: LogoutUseCase

    init(loginUseCase: LoginUseCase, logoutUseCase: LogoutUseCase) {
        self.loginUseCase = loginUseCase
        self.logoutUseCase = logoutUseCase
    }

    func login(email: String, password: String) async {
        state = .loading
        do {
            let user = try await loginUseCase.execute(email: email, password: password)
            state = .authenticated(user)
        } catch {
            state = .error(error.localizedDescription)
        }
    }

    func logout() async {
        do {
            try await logoutUseCase.execute()
            state = .unauthenticated
        } catch {
            state = .error(error.localizedDescription)
        }
    }
}
```

### View 使用 ViewModel
```swift
// Owner view - uses @StateObject
struct LoginScreen: View {
    @StateObject private var viewModel = AuthViewModel(
        loginUseCase: DIContainer.shared.loginUseCase,
        logoutUseCase: DIContainer.shared.logoutUseCase
    )

    var body: some View {
        Group {
            switch viewModel.state {
            case .initial, .unauthenticated:
                LoginForm(onSubmit: { email, password in
                    Task { await viewModel.login(email: email, password: password) }
                })
            case .loading:
                ProgressView()
            case .authenticated(let user):
                UserProfile(user: user)
            case .error(let message):
                ErrorView(message: message, onRetry: {})
            }
        }
    }
}
```

### @EnvironmentObject - 全域狀態共享
```swift
// Provide at app root
@main
struct TasteByteApp: App {
    @StateObject private var authViewModel = AuthViewModel(...)

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(authViewModel)
        }
    }
}

// Access from any child view
struct ProfileButton: View {
    @EnvironmentObject var authViewModel: AuthViewModel

    var body: some View {
        if case .authenticated(let user) = authViewModel.state {
            Text(user.name)
        }
    }
}
```

### 依賴注入 (DI Container)
```swift
class DIContainer {
    static let shared = DIContainer()

    lazy var apiClient = APIClient(baseURL: Environment.current.baseURL, tokenStorage: tokenStorage)
    lazy var tokenStorage = KeychainTokenStorage()

    // Repositories
    lazy var userRepository: UserRepository = UserRepositoryImpl(apiClient: apiClient)
    lazy var productRepository: ProductRepository = ProductRepositoryImpl(apiClient: apiClient)

    // Use Cases
    lazy var loginUseCase = LoginUseCase(repository: userRepository)
    lazy var logoutUseCase = LogoutUseCase(repository: userRepository)
}
```

## Android - Jetpack Compose 狀態管理

### remember + mutableStateOf - 本地 UI 狀態
```kotlin
@Composable
fun CounterView() {
    var count by remember { mutableIntStateOf(0) }

    Column {
        Text("Count: $count")
        Button(onClick = { count++ }) { Text("Increment") }
    }
}
```

### ViewModel + StateFlow - 業務邏輯狀態
```kotlin
sealed interface AuthUiState {
    data object Initial : AuthUiState
    data object Loading : AuthUiState
    data class Authenticated(val user: User) : AuthUiState
    data object Unauthenticated : AuthUiState
    data class Error(val message: String) : AuthUiState
}

@HiltViewModel
class AuthViewModel @Inject constructor(
    private val loginUseCase: LoginUseCase,
    private val logoutUseCase: LogoutUseCase,
) : ViewModel() {
    private val _uiState = MutableStateFlow<AuthUiState>(AuthUiState.Initial)
    val uiState: StateFlow<AuthUiState> = _uiState.asStateFlow()

    fun login(email: String, password: String) {
        viewModelScope.launch {
            _uiState.value = AuthUiState.Loading
            try {
                val user = loginUseCase(LoginParams(email, password))
                _uiState.value = AuthUiState.Authenticated(user)
            } catch (e: Exception) {
                _uiState.value = AuthUiState.Error(e.message ?: "Login failed")
            }
        }
    }

    fun logout() {
        viewModelScope.launch {
            try {
                logoutUseCase()
                _uiState.value = AuthUiState.Unauthenticated
            } catch (e: Exception) {
                _uiState.value = AuthUiState.Error(e.message ?: "Logout failed")
            }
        }
    }
}
```

### Composable 使用 ViewModel
```kotlin
@Composable
fun LoginScreen(
    viewModel: AuthViewModel = hiltViewModel(),
    onLoginSuccess: () -> Unit = {}
) {
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()

    LaunchedEffect(uiState) {
        if (uiState is AuthUiState.Authenticated) onLoginSuccess()
    }

    when (val state = uiState) {
        is AuthUiState.Initial,
        is AuthUiState.Unauthenticated -> LoginForm(
            onSubmit = { email, password -> viewModel.login(email, password) }
        )
        is AuthUiState.Loading -> Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
            CircularProgressIndicator()
        }
        is AuthUiState.Authenticated -> UserProfile(user = state.user)
        is AuthUiState.Error -> ErrorView(
            message = state.message,
            onRetry = {}
        )
    }
}
```

### 副作用處理
```kotlin
// One-time event (navigation, snackbar)
@Composable
fun LoginScreen(viewModel: AuthViewModel = hiltViewModel()) {
    val snackbarHostState = remember { SnackbarHostState() }
    val uiState by viewModel.uiState.collectAsStateWithLifecycle()

    // Show error as snackbar
    LaunchedEffect(uiState) {
        if (uiState is AuthUiState.Error) {
            snackbarHostState.showSnackbar((uiState as AuthUiState.Error).message)
        }
    }

    Scaffold(snackbarHost = { SnackbarHost(snackbarHostState) }) { /* ... */ }
}
```

### Hilt 依賴注入
```kotlin
@Module
@InstallIn(SingletonComponent::class)
object RepositoryModule {
    @Provides
    @Singleton
    fun provideUserRepository(api: UserApi, localStore: UserLocalStore): UserRepository {
        return UserRepositoryImpl(api, localStore)
    }
}

@Module
@InstallIn(ViewModelComponent::class)
object UseCaseModule {
    @Provides
    fun provideLoginUseCase(repository: UserRepository): LoginUseCase {
        return LoginUseCase(repository)
    }
}
```

## 狀態設計原則

### 1. 單一資料來源
- 每個狀態只有一個權威來源
- 避免在多處維護相同資料

### 2. 不可變狀態
- iOS: 使用 `struct` 或 `enum` 表示狀態
- Android: 使用 `data class` 或 `sealed interface`
- 透過複製產生新狀態，不修改原狀態

### 3. 狀態正規化
```swift
// iOS - Avoid nested structures
// Avoid
struct AppState {
    let users: [User]  // Each User contains [Post]
}

// Prefer - normalized
struct AppState {
    let users: [String: User]
    let posts: [String: Post]
    let userPostIds: [String: [String]]
}
```

```kotlin
// Android - Same principle
// Avoid
data class AppState(val users: List<User>)  // Each User contains List<Post>

// Prefer - normalized
data class AppState(
    val users: Map<String, User>,
    val posts: Map<String, Post>,
    val userPostIds: Map<String, List<String>>,
)
```

### 4. 衍生狀態
```swift
// iOS - computed properties
extension AuthViewModel {
    var isLoggedIn: Bool {
        if case .authenticated = state { return true }
        return false
    }

    var userName: String? {
        if case .authenticated(let user) = state { return user.name }
        return nil
    }
}
```

```kotlin
// Android - derivedStateOf or StateFlow.map
val isLoggedIn: StateFlow<Boolean> = uiState.map { it is AuthUiState.Authenticated }
    .stateIn(viewModelScope, SharingStarted.WhileSubscribed(5000), false)
```
