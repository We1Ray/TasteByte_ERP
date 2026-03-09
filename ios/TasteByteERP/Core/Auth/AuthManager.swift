import Foundation
import SwiftUI

@MainActor
final class AuthManager: ObservableObject {
    static let shared = AuthManager()

    @Published var isAuthenticated = false
    @Published var currentUser: User?
    @Published var isLoading = false

    private let tokenKey = "com.tastebyte.erp.accessToken"
    private let refreshTokenKey = "com.tastebyte.erp.refreshToken"

    nonisolated var accessToken: String? {
        KeychainHelper.shared.readString(forKey: tokenKey)
    }

    init() {
        if let token = KeychainHelper.shared.readString(forKey: tokenKey), !token.isEmpty {
            isAuthenticated = true
            loadUserFromToken(token)
        }
    }

    func login(username: String, password: String) async throws {
        isLoading = true
        defer { isLoading = false }

        let body = LoginRequest(username: username, password: password)
        let response: APIResponse<TokenResponse> = try await APIClient.shared.post(
            APIEndpoints.login,
            body: body
        )

        guard response.success, let tokenData = response.data else {
            throw APIError.serverError(401, response.error ?? "Login failed")
        }

        KeychainHelper.shared.saveString(tokenData.accessToken, forKey: tokenKey)
        KeychainHelper.shared.saveString(tokenData.refreshToken, forKey: refreshTokenKey)
        loadUserFromToken(tokenData.accessToken)
        isAuthenticated = true
    }

    func logout() async {
        if let refreshToken = KeychainHelper.shared.readString(forKey: refreshTokenKey) {
            let body = LogoutRequest(refreshToken: refreshToken)
            let _: APIResponse<EmptyData>? = try? await APIClient.shared.post(
                APIEndpoints.logout,
                body: body
            )
        }

        KeychainHelper.shared.delete(forKey: tokenKey)
        KeychainHelper.shared.delete(forKey: refreshTokenKey)
        currentUser = nil
        isAuthenticated = false
    }

    func refreshToken() async throws {
        guard let storedRefreshToken = KeychainHelper.shared.readString(forKey: refreshTokenKey) else {
            await logout()
            return
        }

        let body = RefreshRequest(refreshToken: storedRefreshToken)
        let response: APIResponse<TokenResponse> = try await APIClient.shared.post(
            APIEndpoints.refresh,
            body: body
        )

        guard response.success, let tokenData = response.data else {
            await logout()
            return
        }

        KeychainHelper.shared.saveString(tokenData.accessToken, forKey: tokenKey)
        KeychainHelper.shared.saveString(tokenData.refreshToken, forKey: refreshTokenKey)
        loadUserFromToken(tokenData.accessToken)
    }

    private func loadUserFromToken(_ token: String) {
        guard let payload = decodeJWTPayload(token) else { return }

        currentUser = User(
            id: payload["sub"] as? String ?? "",
            username: payload["username"] as? String ?? "",
            email: payload["email"] as? String ?? "",
            displayName: payload["display_name"] as? String,
            isActive: true
        )
    }

    private func decodeJWTPayload(_ token: String) -> [String: Any]? {
        let segments = token.split(separator: ".")
        guard segments.count >= 2 else { return nil }

        var base64 = String(segments[1])
        let remainder = base64.count % 4
        if remainder > 0 {
            base64 += String(repeating: "=", count: 4 - remainder)
        }

        guard let data = Data(base64Encoded: base64) else { return nil }
        return try? JSONSerialization.jsonObject(with: data) as? [String: Any]
    }
}

struct LoginRequest: Encodable {
    let username: String
    let password: String
}

struct RefreshRequest: Encodable {
    let refreshToken: String
}

struct LogoutRequest: Encodable {
    let refreshToken: String
}
