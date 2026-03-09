import Foundation

struct Warehouse: Codable, Identifiable {
    let id: String
    let warehouseNumber: String
    let name: String
    let description: String?
    let address: String?
    let isActive: Bool
    let createdAt: Date
    let updatedAt: Date
}

struct StockCount: Codable, Identifiable {
    let id: String
    let warehouseId: String
    let materialId: String
    let bookQuantity: Double
    let countedQuantity: Double?
    let difference: Double?
    let countedBy: String?
    let countedAt: Date?
    let status: String
}

struct CreateStockCount: Encodable {
    let warehouseId: String
    let materialId: String
    let countedQuantity: Double
}
