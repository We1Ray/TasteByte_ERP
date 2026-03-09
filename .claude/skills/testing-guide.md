# 測試最佳實踐

## 目錄
1. [測試金字塔](#測試金字塔)
2. [iOS 測試 (XCTest)](#ios-測試-xctest)
3. [Android 測試 (JUnit)](#android-測試-junit)
4. [UI 測試](#ui-測試)
5. [Mock 策略](#mock-策略)

## 測試金字塔

```
        /\
       /  \      E2E Tests (少量)
      /----\     iOS UI Tests / Android Espresso
     /      \
    /--------\   Integration Tests (中量)
   /          \  ViewModel + Repository 整合
  /------------\ Unit Tests (大量)
 /              \ ViewModel、Repository、UseCase
```

**覆蓋率目標**
- 核心業務邏輯：90%+
- 資料層：80%+
- ViewModel：85%+
- UI 元件：70%+

## iOS 測試 (XCTest)

### ViewModel 測試
```swift
@MainActor
final class AuthViewModelTests: XCTestCase {
    var sut: AuthViewModel!
    var mockLoginUseCase: MockLoginUseCase!
    var mockLogoutUseCase: MockLogoutUseCase!

    override func setUp() {
        super.setUp()
        mockLoginUseCase = MockLoginUseCase()
        mockLogoutUseCase = MockLogoutUseCase()
        sut = AuthViewModel(loginUseCase: mockLoginUseCase, logoutUseCase: mockLogoutUseCase)
    }

    override func tearDown() {
        sut = nil
        super.tearDown()
    }

    func testInitialStateIsInitial() {
        XCTAssertEqual(sut.state, .initial)
    }

    func testLoginSuccess() async {
        // Arrange
        let expectedUser = User(id: "1", name: "Test", email: "test@example.com")
        mockLoginUseCase.result = .success(expectedUser)

        // Act
        await sut.login(email: "test@example.com", password: "password123")

        // Assert
        if case .authenticated(let user) = sut.state {
            XCTAssertEqual(user.id, expectedUser.id)
            XCTAssertEqual(user.name, expectedUser.name)
        } else {
            XCTFail("Expected authenticated state")
        }
    }

    func testLoginFailure() async {
        // Arrange
        mockLoginUseCase.result = .failure(APIError.unauthorized)

        // Act
        await sut.login(email: "test@example.com", password: "wrong")

        // Assert
        if case .error(let message) = sut.state {
            XCTAssertFalse(message.isEmpty)
        } else {
            XCTFail("Expected error state")
        }
    }
}
```

### Repository 測試
```swift
final class UserRepositoryTests: XCTestCase {
    var sut: UserRepositoryImpl!
    var mockAPIClient: MockAPIClient!
    var mockLocalStore: MockUserLocalStore!
    var mockNetworkMonitor: MockNetworkMonitor!

    override func setUp() {
        super.setUp()
        mockAPIClient = MockAPIClient()
        mockLocalStore = MockUserLocalStore()
        mockNetworkMonitor = MockNetworkMonitor()
        sut = UserRepositoryImpl(
            apiClient: mockAPIClient,
            localStore: mockLocalStore,
            networkMonitor: mockNetworkMonitor
        )
    }

    func testGetUserOnline_fetchesFromAPIAndCaches() async throws {
        // Arrange
        mockNetworkMonitor.isConnected = true
        let expectedUser = UserResponse(id: "123", name: "Test", email: "test@example.com", createdAt: Date())
        mockAPIClient.stubbedResponse = expectedUser

        // Act
        let user = try await sut.getUser(id: "123")

        // Assert
        XCTAssertEqual(user.id, "123")
        XCTAssertEqual(user.name, "Test")
        XCTAssertTrue(mockLocalStore.cacheUserCalled)
    }

    func testGetUserOffline_fetchesFromCache() async throws {
        // Arrange
        mockNetworkMonitor.isConnected = false
        let cachedUser = User(id: "123", name: "Cached", email: "cached@example.com")
        mockLocalStore.cachedUser = cachedUser

        // Act
        let user = try await sut.getUser(id: "123")

        // Assert
        XCTAssertEqual(user.name, "Cached")
        XCTAssertFalse(mockAPIClient.getCalled)
    }
}
```

## Android 測試 (JUnit)

### ViewModel 測試
```kotlin
@OptIn(ExperimentalCoroutinesApi::class)
class AuthViewModelTest {
    @get:Rule
    val mainDispatcherRule = MainDispatcherRule()

    private lateinit var viewModel: AuthViewModel
    private lateinit var mockLoginUseCase: FakeLoginUseCase
    private lateinit var mockLogoutUseCase: FakeLogoutUseCase

    @Before
    fun setUp() {
        mockLoginUseCase = FakeLoginUseCase()
        mockLogoutUseCase = FakeLogoutUseCase()
        viewModel = AuthViewModel(mockLoginUseCase, mockLogoutUseCase)
    }

    @Test
    fun `initial state is Initial`() {
        assertEquals(AuthUiState.Initial, viewModel.uiState.value)
    }

    @Test
    fun `login success emits Loading then Authenticated`() = runTest {
        // Arrange
        val expectedUser = User("1", "Test", "test@example.com")
        mockLoginUseCase.result = Result.success(expectedUser)

        // Act & Assert
        viewModel.uiState.test {
            assertEquals(AuthUiState.Initial, awaitItem())

            viewModel.login("test@example.com", "password123")

            assertEquals(AuthUiState.Loading, awaitItem())

            val authenticated = awaitItem()
            assertIs<AuthUiState.Authenticated>(authenticated)
            assertEquals(expectedUser, (authenticated as AuthUiState.Authenticated).user)
        }
    }

    @Test
    fun `login failure emits Loading then Error`() = runTest {
        // Arrange
        mockLoginUseCase.result = Result.failure(AppException.Unauthorized())

        // Act & Assert
        viewModel.uiState.test {
            assertEquals(AuthUiState.Initial, awaitItem())

            viewModel.login("test@example.com", "wrong")

            assertEquals(AuthUiState.Loading, awaitItem())

            val error = awaitItem()
            assertIs<AuthUiState.Error>(error)
        }
    }
}
```

### Repository 測試
```kotlin
@OptIn(ExperimentalCoroutinesApi::class)
class UserRepositoryTest {
    private lateinit var repository: UserRepositoryImpl
    private lateinit var mockApi: FakeUserApi
    private lateinit var mockLocalStore: FakeUserLocalStore
    private lateinit var mockNetworkMonitor: FakeNetworkMonitor

    @Before
    fun setUp() {
        mockApi = FakeUserApi()
        mockLocalStore = FakeUserLocalStore()
        mockNetworkMonitor = FakeNetworkMonitor()
        repository = UserRepositoryImpl(mockApi, mockLocalStore, mockNetworkMonitor)
    }

    @Test
    fun `getUser online fetches from API and caches`() = runTest {
        // Arrange
        mockNetworkMonitor.isConnected = true
        val expectedUser = UserResponse("123", "Test", "test@example.com", "2026-01-01")
        mockApi.stubbedUser = expectedUser

        // Act
        val user = repository.getUser("123")

        // Assert
        assertEquals("123", user.id)
        assertEquals("Test", user.name)
        assertTrue(mockLocalStore.cacheUserCalled)
    }

    @Test
    fun `getUser offline fetches from cache`() = runTest {
        // Arrange
        mockNetworkMonitor.isConnected = false
        val cachedUser = UserResponse("123", "Cached", "cached@example.com", "2026-01-01")
        mockLocalStore.cachedUser = cachedUser

        // Act
        val user = repository.getUser("123")

        // Assert
        assertEquals("Cached", user.name)
        assertFalse(mockApi.getUserCalled)
    }
}
```

## UI 測試

### iOS - SwiftUI View 測試
```swift
@MainActor
final class LoginScreenTests: XCTestCase {
    func testLoginButtonTriggersLogin() async throws {
        // Using ViewInspector or snapshot testing
        let mockViewModel = AuthViewModel(
            loginUseCase: MockLoginUseCase(),
            logoutUseCase: MockLogoutUseCase()
        )

        let sut = LoginScreen(viewModel: mockViewModel)

        // Snapshot test
        assertSnapshot(matching: sut, as: .image(layout: .device(config: .iPhone13)))
    }
}

// iOS UI Test (XCUITest)
final class LoginUITests: XCTestCase {
    let app = XCUIApplication()

    override func setUp() {
        continueAfterFailure = false
        app.launch()
    }

    func testSuccessfulLogin() {
        let emailField = app.textFields["email_field"]
        emailField.tap()
        emailField.typeText("test@example.com")

        let passwordField = app.secureTextFields["password_field"]
        passwordField.tap()
        passwordField.typeText("password123")

        app.buttons["login_button"].tap()

        // Verify navigation to home
        XCTAssertTrue(app.staticTexts["Welcome"].waitForExistence(timeout: 5))
    }
}
```

### Android - Compose UI 測試
```kotlin
class LoginScreenTest {
    @get:Rule
    val composeTestRule = createComposeRule()

    @Test
    fun loginButton_triggersLogin() {
        val mockViewModel = FakeAuthViewModel()

        composeTestRule.setContent {
            LoginScreen(viewModel = mockViewModel)
        }

        composeTestRule
            .onNodeWithTag("email_field")
            .performTextInput("test@example.com")

        composeTestRule
            .onNodeWithTag("password_field")
            .performTextInput("password123")

        composeTestRule
            .onNodeWithTag("login_button")
            .performClick()

        assertTrue(mockViewModel.loginCalled)
    }

    @Test
    fun errorState_showsErrorMessage() {
        val mockViewModel = FakeAuthViewModel(
            initialState = AuthUiState.Error("登入失敗")
        )

        composeTestRule.setContent {
            LoginScreen(viewModel = mockViewModel)
        }

        composeTestRule
            .onNodeWithText("登入失敗")
            .assertIsDisplayed()
    }
}
```

### Android - Espresso E2E 測試
```kotlin
@HiltAndroidTest
class LoginE2ETest {
    @get:Rule(order = 0)
    val hiltRule = HiltAndroidRule(this)

    @get:Rule(order = 1)
    val activityRule = ActivityScenarioRule(MainActivity::class.java)

    @Test
    fun userCanLoginSuccessfully() {
        onView(withId(R.id.email_field))
            .perform(typeText("test@example.com"))

        onView(withId(R.id.password_field))
            .perform(typeText("password123"))

        onView(withId(R.id.login_button))
            .perform(click())

        onView(withText("Welcome"))
            .check(matches(isDisplayed()))
    }
}
```

## Mock 策略

### iOS - Protocol-Based Mocking
```swift
// Define protocol
protocol UserRepositoryProtocol {
    func getUser(id: String) async throws -> User
    func deleteUser(id: String) async throws
}

// Mock implementation
class MockUserRepository: UserRepositoryProtocol {
    var getUserResult: Result<User, Error>?
    var getUserCallCount = 0
    var deleteUserCalled = false

    func getUser(id: String) async throws -> User {
        getUserCallCount += 1
        switch getUserResult {
        case .success(let user): return user
        case .failure(let error): throw error
        case .none: fatalError("getUserResult not set")
        }
    }

    func deleteUser(id: String) async throws {
        deleteUserCalled = true
    }
}

// Usage in test
func testGetUser() async throws {
    let mockRepo = MockUserRepository()
    mockRepo.getUserResult = .success(User(id: "123", name: "Test", email: "test@example.com"))

    let user = try await mockRepo.getUser(id: "123")
    XCTAssertEqual(user.id, "123")
    XCTAssertEqual(mockRepo.getUserCallCount, 1)
}
```

### Android - Fake Implementation
```kotlin
// Fake implementation
class FakeUserRepository : UserRepository {
    var stubbedUser: User? = null
    var getUserCallCount = 0
    var deleteUserCalled = false
    var shouldThrow: Exception? = null

    override suspend fun getUser(id: String): User {
        getUserCallCount++
        shouldThrow?.let { throw it }
        return stubbedUser ?: throw AppException.Server("User not found")
    }

    override suspend fun deleteUser(id: String) {
        deleteUserCalled = true
        shouldThrow?.let { throw it }
    }
}

// Usage with MockK (alternative)
val mockRepo = mockk<UserRepository>()
coEvery { mockRepo.getUser("123") } returns testUser
coVerify(exactly = 1) { mockRepo.getUser("123") }
```

### 測試資料工廠
```swift
// iOS
enum TestDataFactory {
    static func createUser(
        id: String = UUID().uuidString,
        name: String = "Test User",
        email: String = "test@example.com"
    ) -> User {
        User(id: id, name: name, email: email)
    }

    static func createUsers(count: Int) -> [User] {
        (0..<count).map { i in
            createUser(id: "user-\(i)", name: "User \(i)", email: "user\(i)@example.com")
        }
    }
}
```

```kotlin
// Android
object TestDataFactory {
    fun createUser(
        id: String = UUID.randomUUID().toString(),
        name: String = "Test User",
        email: String = "test@example.com",
    ) = User(id = id, name = name, email = email)

    fun createUsers(count: Int) = (0 until count).map { i ->
        createUser(id = "user-$i", name = "User $i", email = "user$i@example.com")
    }
}
```

### 常用測試指令
```bash
# iOS - 執行所有測試
xcodebuild test -scheme TasteByte -destination 'platform=iOS Simulator,name=iPhone 15'

# iOS - 執行特定測試
xcodebuild test -scheme TasteByte -only-testing:TasteByteTests/AuthViewModelTests

# iOS - 產生覆蓋率報告
xcodebuild test -scheme TasteByte -enableCodeCoverage YES

# Android - 執行所有單元測試
./gradlew test

# Android - 執行特定測試
./gradlew test --tests "com.tastebyte.auth.AuthViewModelTest"

# Android - 產生覆蓋率報告
./gradlew jacocoTestReport

# Android - 執行 instrumented 測試
./gradlew connectedAndroidTest
```
