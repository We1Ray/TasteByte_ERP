import Foundation

struct APIResponse<T: Decodable>: Decodable {
    let success: Bool
    let data: T?
    let error: String?
    let message: String?
}

extension APIResponse: Encodable where T: Encodable {}

struct PaginatedResponse<T: Decodable>: Decodable {
    let items: [T]
    let total: Int
    let page: Int
    let pageSize: Int
    let totalPages: Int
}

extension PaginatedResponse: Encodable where T: Encodable {}

struct TokenResponse: Decodable {
    let accessToken: String
    let tokenType: String
    let expiresIn: Int
    let refreshToken: String
}
