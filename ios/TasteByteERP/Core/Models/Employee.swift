import Foundation

struct Employee: Codable, Identifiable {
    let id: String
    let employeeNumber: String
    let firstName: String
    let lastName: String
    let email: String?
    let phone: String?
    let department: String?
    let position: String?
    let hireDate: Date?
    let isActive: Bool
    let createdAt: Date
    let updatedAt: Date

    var fullName: String {
        "\(firstName) \(lastName)"
    }
}
