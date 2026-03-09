import Foundation

final class APIClient {
    static let shared = APIClient()

    private let session: URLSession
    private let decoder: JSONDecoder
    private let encoder: JSONEncoder
    private let cache = CacheManager.shared
    private var isRefreshing = false

    private init() {
        let config = URLSessionConfiguration.default
        config.timeoutIntervalForRequest = 30
        config.timeoutIntervalForResource = 60
        self.session = URLSession(configuration: config)

        self.decoder = JSONDecoder()
        self.decoder.keyDecodingStrategy = .convertFromSnakeCase
        self.decoder.dateDecodingStrategy = .custom { decoder in
            let container = try decoder.singleValueContainer()
            let dateString = try container.decode(String.self)

            let formatters: [ISO8601DateFormatter] = {
                let full = ISO8601DateFormatter()
                full.formatOptions = [.withInternetDateTime, .withFractionalSeconds]
                let standard = ISO8601DateFormatter()
                standard.formatOptions = [.withInternetDateTime]
                return [full, standard]
            }()

            for formatter in formatters {
                if let date = formatter.date(from: dateString) {
                    return date
                }
            }

            let dateOnly = DateFormatter()
            dateOnly.dateFormat = "yyyy-MM-dd"
            dateOnly.locale = Locale(identifier: "en_US_POSIX")
            if let date = dateOnly.date(from: dateString) {
                return date
            }

            throw DecodingError.dataCorruptedError(
                in: container,
                debugDescription: "Cannot decode date: \(dateString)"
            )
        }

        self.encoder = JSONEncoder()
        self.encoder.keyEncodingStrategy = .convertToSnakeCase
        self.encoder.dateEncodingStrategy = .iso8601
    }

    private func buildURL(endpoint: String, queryItems: [URLQueryItem]? = nil) throws -> URL {
        var components = URLComponents(string: APIEndpoints.baseURL + endpoint)
        components?.queryItems = queryItems
        guard let url = components?.url else {
            throw APIError.invalidURL
        }
        return url
    }

    private func buildRequest(url: URL, method: String, body: Data? = nil) -> URLRequest {
        var request = URLRequest(url: url)
        request.httpMethod = method
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.setValue("application/json", forHTTPHeaderField: "Accept")

        if let token = KeychainHelper.shared.readString(forKey: "com.tastebyte.erp.accessToken") {
            request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        }

        request.httpBody = body
        return request
    }

    private func cacheKey(for url: URL) -> String {
        url.absoluteString
    }

    private func execute<T: Decodable>(_ request: URLRequest, allowRetry: Bool = true) async throws -> T {
        let data: Data
        let response: URLResponse
        do {
            (data, response) = try await session.data(for: request)
        } catch {
            throw APIError.networkError(error)
        }

        guard let httpResponse = response as? HTTPURLResponse else {
            throw APIError.unknown(0)
        }

        switch httpResponse.statusCode {
        case 200...299:
            break
        case 401:
            if allowRetry && !isRefreshing && !isAuthEndpoint(request.url) {
                return try await handleUnauthorizedAndRetry(request)
            }
            throw APIError.unauthorized
        case 403:
            throw APIError.forbidden
        case 404:
            throw APIError.notFound
        case 500...599:
            let errorMessage = try? decoder.decode(APIResponse<EmptyData>.self, from: data)
            throw APIError.serverError(httpResponse.statusCode, errorMessage?.error)
        default:
            throw APIError.unknown(httpResponse.statusCode)
        }

        do {
            return try decoder.decode(T.self, from: data)
        } catch {
            throw APIError.decodingFailed(error)
        }
    }

    private func isAuthEndpoint(_ url: URL?) -> Bool {
        guard let path = url?.path else { return false }
        return path.contains("/auth/login") || path.contains("/auth/refresh") || path.contains("/auth/logout")
    }

    private func handleUnauthorizedAndRetry<T: Decodable>(_ originalRequest: URLRequest) async throws -> T {
        isRefreshing = true
        defer { isRefreshing = false }

        do {
            try await AuthManager.shared.refreshToken()
        } catch {
            await AuthManager.shared.logout()
            throw APIError.unauthorized
        }

        // Rebuild the request with the new token
        guard let url = originalRequest.url else {
            throw APIError.invalidURL
        }
        var retryRequest = buildRequest(url: url, method: originalRequest.httpMethod ?? "GET", body: originalRequest.httpBody)
        retryRequest.allHTTPHeaderFields = originalRequest.allHTTPHeaderFields
        if let token = KeychainHelper.shared.readString(forKey: "com.tastebyte.erp.accessToken") {
            retryRequest.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
        }

        return try await execute(retryRequest, allowRetry: false)
    }

    // MARK: - Cache-aware GET execution

    private func executeWithCache<T: Decodable & Encodable>(
        _ request: URLRequest,
        cacheKey key: String,
        ttl: TimeInterval = 3600
    ) async throws -> T {
        do {
            let result: T = try await execute(request)
            cache.save(result, forKey: key, ttl: ttl)
            return result
        } catch let error as APIError {
            if case .networkError = error, let cached: T = cache.get(forKey: key) {
                return cached
            }
            throw error
        }
    }

    // MARK: - Public Methods

    func get<T: Decodable & Encodable>(
        _ endpoint: String,
        queryItems: [URLQueryItem]? = nil
    ) async throws -> APIResponse<T> {
        let url = try buildURL(endpoint: endpoint, queryItems: queryItems)
        let request = buildRequest(url: url, method: "GET")
        return try await executeWithCache(request, cacheKey: cacheKey(for: url))
    }

    func get<T: Decodable>(
        _ endpoint: String,
        queryItems: [URLQueryItem]? = nil
    ) async throws -> APIResponse<T> {
        let url = try buildURL(endpoint: endpoint, queryItems: queryItems)
        let request = buildRequest(url: url, method: "GET")
        return try await execute(request)
    }

    func getPaginated<T: Decodable & Encodable>(
        _ endpoint: String,
        page: Int = 1,
        perPage: Int = 20,
        extraParams: [URLQueryItem]? = nil
    ) async throws -> APIResponse<PaginatedResponse<T>> {
        var queryItems: [URLQueryItem] = [
            URLQueryItem(name: "page", value: String(page)),
            URLQueryItem(name: "per_page", value: String(perPage)),
        ]
        if let extra = extraParams {
            queryItems.append(contentsOf: extra)
        }
        let url = try buildURL(endpoint: endpoint, queryItems: queryItems)
        let request = buildRequest(url: url, method: "GET")
        return try await executeWithCache(request, cacheKey: cacheKey(for: url))
    }

    func getPaginated<T: Decodable>(
        _ endpoint: String,
        page: Int = 1,
        perPage: Int = 20,
        extraParams: [URLQueryItem]? = nil
    ) async throws -> APIResponse<PaginatedResponse<T>> {
        var queryItems: [URLQueryItem] = [
            URLQueryItem(name: "page", value: String(page)),
            URLQueryItem(name: "per_page", value: String(perPage)),
        ]
        if let extra = extraParams {
            queryItems.append(contentsOf: extra)
        }
        let url = try buildURL(endpoint: endpoint, queryItems: queryItems)
        let request = buildRequest(url: url, method: "GET")
        return try await execute(request)
    }

    func post<T: Decodable, B: Encodable>(
        _ endpoint: String,
        body: B
    ) async throws -> APIResponse<T> {
        let url = try buildURL(endpoint: endpoint)
        let bodyData = try encoder.encode(body)
        let request = buildRequest(url: url, method: "POST", body: bodyData)
        do {
            return try await execute(request)
        } catch let error as APIError {
            if case .networkError = error {
                OfflineSyncManager.shared.enqueue(method: "POST", endpoint: endpoint, body: bodyData)
            }
            throw error
        }
    }

    func put<T: Decodable, B: Encodable>(
        _ endpoint: String,
        body: B
    ) async throws -> APIResponse<T> {
        let url = try buildURL(endpoint: endpoint)
        let bodyData = try encoder.encode(body)
        let request = buildRequest(url: url, method: "PUT", body: bodyData)
        do {
            return try await execute(request)
        } catch let error as APIError {
            if case .networkError = error {
                OfflineSyncManager.shared.enqueue(method: "PUT", endpoint: endpoint, body: bodyData)
            }
            throw error
        }
    }

    func delete<T: Decodable>(_ endpoint: String) async throws -> APIResponse<T> {
        let url = try buildURL(endpoint: endpoint)
        let request = buildRequest(url: url, method: "DELETE")
        return try await execute(request)
    }

    func postNoBody<T: Decodable>(_ endpoint: String) async throws -> APIResponse<T> {
        let url = try buildURL(endpoint: endpoint)
        let request = buildRequest(url: url, method: "POST")
        do {
            return try await execute(request)
        } catch let error as APIError {
            if case .networkError = error {
                OfflineSyncManager.shared.enqueue(method: "POST", endpoint: endpoint, body: nil)
            }
            throw error
        }
    }
}

struct EmptyData: Codable {}
