import Foundation

struct User: Codable, Identifiable {
    let id: String
    let username: String
    let email: String
    let displayName: String?
    let isActive: Bool
}
