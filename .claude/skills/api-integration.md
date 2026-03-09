# API 整合指南

## 目錄
1. [iOS - URLSession 配置](#ios---urlsession-配置)
2. [Android - Retrofit/Ktor 配置](#android---retrofitktor-配置)
3. [認證處理](#認證處理)
4. [錯誤處理](#錯誤處理)
5. [Model 與序列化](#model-與序列化)
6. [Repository 實作範本](#repository-實作範本)

## iOS - URLSession 配置

### 基礎 API Client
```swift
class APIClient {
    private let session: URLSession
    private let baseURL: URL
    private let tokenStorage: TokenStorage

    init(baseURL: URL, tokenStorage: TokenStorage) {
        self.baseURL = baseURL
        self.tokenStorage = tokenStorage
        let config = URLSessionConfiguration.default
        config.timeoutIntervalForRequest = 30
        config.timeoutIntervalForResource = 60
        self.session = URLSession(configuration: config)
    }

    func get<T: Decodable>(_ path: String) async throws -> T {
        let request = try buildRequest(path: path, method: "GET")
        return try await execute(request)
    }

    func post<T: Decodable, B: Encodable>(_ path: String, body: B) async throws -> T {
        var request = try buildRequest(path: path, method: "POST")
        request.httpBody = try JSONEncoder.api.encode(body)
        return try await execute(request)
    }

    func put<T: Decodable, B: Encodable>(_ path: String, body: B) async throws -> T {
        var request = try buildRequest(path: path, method: "PUT")
        request.httpBody = try JSONEncoder.api.encode(body)
        return try await execute(request)
    }

    func delete(_ path: String) async throws {
        let request = try buildRequest(path: path, method: "DELETE")
        let (_, response) = try await session.data(for: request)
        try validateResponse(response)
    }

    private func buildRequest(path: String, method: String) throws -> URLRequest {
        guard let url = URL(string: path, relativeTo: baseURL) else {
            throw APIError.invalidURL
        }
        var request = URLRequest(url: url)
        request.httpMethod = method
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.setValue("application/json", forHTTPHeaderField: "Accept")
        if let token = tokenStorage.accessToken {
            request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        }
        return request
    }

    private func execute<T: Decodable>(_ request: URLRequest) async throws -> T {
        let (data, response) = try await session.data(for: request)
        try validateResponse(response)
        return try JSONDecoder.api.decode(T.self, from: data)
    }

    private func validateResponse(_ response: URLResponse) throws {
        guard let httpResponse = response as? HTTPURLResponse else {
            throw APIError.invalidResponse
        }
        guard (200...299).contains(httpResponse.statusCode) else {
            throw APIError.httpError(statusCode: httpResponse.statusCode)
        }
    }
}
```

### 環境配置
```swift
enum Environment {
    case dev, staging, prod

    var baseURL: URL {
        switch self {
        case .dev: return URL(string: "http://localhost:8000/api/v1")!
        case .staging: return URL(string: "https://staging-api.example.com/api/v1")!
        case .prod: return URL(string: "https://api.example.com/api/v1")!
        }
    }

    var enableLogging: Bool {
        self != .prod
    }
}
```

## Android - Retrofit/Ktor 配置

### Retrofit 配置
```kotlin
@Module
@InstallIn(SingletonComponent::class)
object NetworkModule {
    @Provides
    @Singleton
    fun provideOkHttpClient(
        tokenStorage: TokenStorage
    ): OkHttpClient {
        return OkHttpClient.Builder()
            .connectTimeout(30, TimeUnit.SECONDS)
            .readTimeout(30, TimeUnit.SECONDS)
            .addInterceptor(AuthInterceptor(tokenStorage))
            .addInterceptor(HttpLoggingInterceptor().apply {
                level = if (BuildConfig.DEBUG)
                    HttpLoggingInterceptor.Level.BODY
                else
                    HttpLoggingInterceptor.Level.NONE
            })
            .build()
    }

    @Provides
    @Singleton
    fun provideRetrofit(okHttpClient: OkHttpClient): Retrofit {
        return Retrofit.Builder()
            .baseUrl(BuildConfig.API_BASE_URL)
            .client(okHttpClient)
            .addConverterFactory(MoshiConverterFactory.create())
            .build()
    }
}
```

### Ktor 配置 (替代方案)
```kotlin
val httpClient = HttpClient(Android) {
    install(ContentNegotiation) {
        json(Json {
            ignoreUnknownKeys = true
            prettyPrint = false
        })
    }
    install(Auth) {
        bearer {
            loadTokens { BearerTokens(tokenStorage.accessToken, tokenStorage.refreshToken) }
            refreshTokens {
                val response = client.post("/auth/refresh") {
                    setBody(mapOf("refresh_token" to tokenStorage.refreshToken))
                }
                val tokens = response.body<TokenResponse>()
                tokenStorage.saveTokens(tokens.accessToken, tokens.refreshToken)
                BearerTokens(tokens.accessToken, tokens.refreshToken)
            }
        }
    }
    defaultRequest {
        url(BuildConfig.API_BASE_URL)
        contentType(ContentType.Application.Json)
    }
}
```

## 認證處理

### iOS - Token Refresh
```swift
actor TokenManager {
    private let tokenStorage: TokenStorage
    private var refreshTask: Task<String, Error>?

    init(tokenStorage: TokenStorage) {
        self.tokenStorage = tokenStorage
    }

    func validAccessToken() async throws -> String {
        if let token = tokenStorage.accessToken, !token.isExpired {
            return token
        }
        // Coalesce concurrent refresh requests
        if let existingTask = refreshTask {
            return try await existingTask.value
        }
        let task = Task { () -> String in
            defer { refreshTask = nil }
            guard let refreshToken = tokenStorage.refreshToken else {
                throw APIError.unauthorized
            }
            let response = try await refreshTokenRequest(refreshToken)
            tokenStorage.save(accessToken: response.accessToken, refreshToken: response.refreshToken)
            return response.accessToken
        }
        refreshTask = task
        return try await task.value
    }
}
```

### Android - Auth Interceptor
```kotlin
class AuthInterceptor(private val tokenStorage: TokenStorage) : Interceptor {
    override fun intercept(chain: Interceptor.Chain): Response {
        val token = tokenStorage.accessToken
        val request = chain.request().newBuilder().apply {
            token?.let { addHeader("Authorization", "Bearer $it") }
        }.build()

        val response = chain.proceed(request)

        if (response.code == 401) {
            synchronized(this) {
                val newToken = refreshToken()
                if (newToken != null) {
                    val retryRequest = request.newBuilder()
                        .header("Authorization", "Bearer $newToken")
                        .build()
                    response.close()
                    return chain.proceed(retryRequest)
                }
            }
        }
        return response
    }

    private fun refreshToken(): String? {
        return try {
            val refreshToken = tokenStorage.refreshToken ?: return null
            // Synchronous refresh call
            val response = refreshApi.refreshToken(RefreshRequest(refreshToken)).execute()
            response.body()?.let { tokens ->
                tokenStorage.saveTokens(tokens.accessToken, tokens.refreshToken)
                tokens.accessToken
            }
        } catch (_: Exception) {
            tokenStorage.clearTokens()
            null
        }
    }
}
```

## 錯誤處理

### iOS - 自訂錯誤
```swift
enum APIError: LocalizedError {
    case invalidURL
    case invalidResponse
    case httpError(statusCode: Int)
    case decodingError(Error)
    case networkError(Error)
    case unauthorized
    case forbidden(String)
    case validation(message: String, errors: [String: [String]]?)

    var errorDescription: String? {
        switch self {
        case .invalidURL: return "Invalid URL"
        case .invalidResponse: return "Invalid response"
        case .httpError(let code): return "HTTP error: \(code)"
        case .decodingError(let error): return "Decoding error: \(error.localizedDescription)"
        case .networkError(let error): return error.localizedDescription
        case .unauthorized: return "認證失敗，請重新登入"
        case .forbidden(let msg): return "無權限：\(msg)"
        case .validation(let msg, _): return "驗證失敗：\(msg)"
        }
    }
}
```

### Android - 自訂例外
```kotlin
sealed class AppException(override val message: String) : Exception(message) {
    class Server(message: String, val statusCode: Int? = null) : AppException(message)
    class Network(message: String = "網路連線失敗") : AppException(message)
    class Validation(message: String, val errors: Map<String, List<String>>? = null) : AppException(message)
    class Unauthorized(message: String = "認證失敗，請重新登入") : AppException(message)
}

fun handleHttpException(code: Int, body: ErrorBody?): AppException {
    return when (code) {
        400 -> AppException.Validation(body?.message ?: "請求格式錯誤", body?.errors)
        401 -> AppException.Unauthorized()
        403 -> AppException.Server("無權限執行此操作", statusCode = 403)
        404 -> AppException.Server("找不到請求的資源", statusCode = 404)
        422 -> AppException.Validation(body?.message ?: "資料驗證失敗", body?.errors)
        else -> AppException.Server(body?.message ?: "伺服器錯誤", statusCode = code)
    }
}
```

## Model 與序列化

### iOS - Codable Models
```swift
struct UserResponse: Codable {
    let id: String
    let email: String
    let name: String
    let avatarUrl: String?
    let createdAt: Date
    let isVerified: Bool

    enum CodingKeys: String, CodingKey {
        case id, email, name
        case avatarUrl = "avatar_url"
        case createdAt = "created_at"
        case isVerified = "is_verified"
    }

    func toEntity() -> User {
        User(id: id, email: email, name: name, avatarUrl: avatarUrl, createdAt: createdAt, isVerified: isVerified)
    }
}

// JSON Encoder/Decoder with snake_case
extension JSONDecoder {
    static let api: JSONDecoder = {
        let decoder = JSONDecoder()
        decoder.keyDecodingStrategy = .convertFromSnakeCase
        decoder.dateDecodingStrategy = .iso8601
        return decoder
    }()
}

extension JSONEncoder {
    static let api: JSONEncoder = {
        let encoder = JSONEncoder()
        encoder.keyEncodingStrategy = .convertToSnakeCase
        encoder.dateEncodingStrategy = .iso8601
        return encoder
    }()
}
```

### Android - Kotlinx Serialization / Moshi Models
```kotlin
@Serializable
data class UserResponse(
    val id: String,
    val email: String,
    val name: String,
    @SerialName("avatar_url") val avatarUrl: String? = null,
    @SerialName("created_at") val createdAt: String,
    @SerialName("is_verified") val isVerified: Boolean = false,
) {
    fun toEntity() = User(
        id = id, email = email, name = name,
        avatarUrl = avatarUrl, createdAt = createdAt, isVerified = isVerified
    )
}
```

### 分頁回應模型

**iOS:**
```swift
struct PaginatedResponse<T: Decodable>: Decodable {
    let data: [T]
    let currentPage: Int
    let lastPage: Int
    let perPage: Int
    let total: Int

    enum CodingKeys: String, CodingKey {
        case data
        case currentPage = "current_page"
        case lastPage = "last_page"
        case perPage = "per_page"
        case total
    }
}
```

**Android:**
```kotlin
@Serializable
data class PaginatedResponse<T>(
    val data: List<T>,
    @SerialName("current_page") val currentPage: Int,
    @SerialName("last_page") val lastPage: Int,
    @SerialName("per_page") val perPage: Int,
    val total: Int,
)
```

## Repository 實作範本

### iOS 完整範例
```swift
protocol ProductRepository {
    func getProducts(page: Int, perPage: Int, category: String?) async throws -> [Product]
    func getProduct(id: String) async throws -> Product
    func createProduct(_ dto: CreateProductDTO) async throws -> Product
    func updateProduct(id: String, _ dto: UpdateProductDTO) async throws -> Product
    func deleteProduct(id: String) async throws
}

class ProductRepositoryImpl: ProductRepository {
    private let apiClient: APIClient
    private let localStore: ProductLocalStore
    private let networkMonitor: NetworkMonitor

    init(apiClient: APIClient, localStore: ProductLocalStore, networkMonitor: NetworkMonitor) {
        self.apiClient = apiClient
        self.localStore = localStore
        self.networkMonitor = networkMonitor
    }

    func getProducts(page: Int = 1, perPage: Int = 20, category: String? = nil) async throws -> [Product] {
        if networkMonitor.isConnected {
            do {
                let response: PaginatedResponse<ProductResponse> = try await apiClient.get(
                    "/mm/products?page=\(page)&per_page=\(perPage)" + (category.map { "&category=\($0)" } ?? "")
                )
                let products = response.data.map { $0.toEntity() }
                if page == 1 { try await localStore.cacheProducts(products) }
                return products
            } catch {
                throw error
            }
        } else {
            return try await localStore.getCachedProducts()
        }
    }

    func createProduct(_ dto: CreateProductDTO) async throws -> Product {
        let response: ApiResponse<ProductResponse> = try await apiClient.post("/mm/products", body: dto)
        return response.data.toEntity()
    }
}
```

### Android 完整範例
```kotlin
interface ProductRepository {
    suspend fun getProducts(page: Int = 1, perPage: Int = 20, category: String? = null): List<Product>
    suspend fun getProduct(id: String): Product
    suspend fun createProduct(dto: CreateProductDto): Product
    suspend fun updateProduct(id: String, dto: UpdateProductDto): Product
    suspend fun deleteProduct(id: String)
}

class ProductRepositoryImpl @Inject constructor(
    private val api: ProductApi,
    private val localStore: ProductLocalStore,
    private val networkMonitor: NetworkMonitor,
) : ProductRepository {

    override suspend fun getProducts(page: Int, perPage: Int, category: String?): List<Product> {
        return if (networkMonitor.isConnected) {
            try {
                val response = api.getProducts(page, perPage, category)
                val products = response.data.map { it.toEntity() }
                if (page == 1) localStore.cacheProducts(products)
                products
            } catch (e: Exception) {
                throw AppException.Server(e.message ?: "Failed to load products")
            }
        } else {
            localStore.getCachedProducts().map { it.toEntity() }
        }
    }

    override suspend fun createProduct(dto: CreateProductDto): Product {
        return try {
            val response = api.createProduct(dto)
            response.data.toEntity()
        } catch (e: HttpException) {
            throw handleHttpException(e.code(), e.parseErrorBody())
        }
    }
}
```
