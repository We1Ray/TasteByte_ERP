import Foundation

struct ProductionOrder: Codable, Identifiable {
    let id: String
    let orderNumber: String
    let materialId: String
    let quantity: Double
    let uomId: String?
    let status: String
    let plannedStartDate: Date?
    let plannedEndDate: Date?
    let actualStartDate: Date?
    let actualEndDate: Date?
    let notes: String?
    let createdBy: String?
    let createdAt: Date
    let updatedAt: Date
}

struct ProductionOrderStatusUpdate: Encodable {
    let status: String
}
