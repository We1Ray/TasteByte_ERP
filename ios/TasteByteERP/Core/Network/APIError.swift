import Foundation

enum APIError: LocalizedError {
    case invalidURL
    case encodingFailed
    case decodingFailed(Error)
    case networkError(Error)
    case unauthorized
    case forbidden
    case notFound
    case serverError(Int, String?)
    case unknown(Int)

    var errorDescription: String? {
        switch self {
        case .invalidURL:
            return "Invalid URL"
        case .encodingFailed:
            return "Failed to encode request data"
        case .decodingFailed(let error):
            return "Failed to decode response: \(error.localizedDescription)"
        case .networkError(let error):
            return "Network error: \(error.localizedDescription)"
        case .unauthorized:
            return "Session expired. Please log in again."
        case .forbidden:
            return "You do not have permission to perform this action."
        case .notFound:
            return "The requested resource was not found."
        case .serverError(_, let message):
            return message ?? "An internal server error occurred."
        case .unknown(let code):
            return "Unexpected error (HTTP \(code))"
        }
    }
}
