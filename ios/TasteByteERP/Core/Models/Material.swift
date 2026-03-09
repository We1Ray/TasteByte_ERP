import Foundation

struct Material: Codable, Identifiable {
    let id: String
    let materialNumber: String
    let name: String
    let description: String?
    let materialGroupId: String?
    let baseUomId: String?
    let materialType: String
    let weight: Double?
    let weightUom: String?
    let isActive: Bool
    let createdAt: Date
    let updatedAt: Date
}

struct MaterialGroup: Codable, Identifiable {
    let id: String
    let code: String
    let name: String
    let description: String?
}

struct Uom: Codable, Identifiable {
    let id: String
    let code: String
    let name: String
    let isBase: Bool
}

struct PlantStock: Codable, Identifiable {
    let id: String
    let materialId: String
    let warehouseId: String?
    let quantity: Double
    let reservedQuantity: Double
    let uomId: String?
    let lastCountDate: Date?
    let updatedAt: Date
}

struct MaterialMovement: Codable, Identifiable {
    let id: String
    let documentNumber: String
    let movementType: String
    let materialId: String
    let warehouseId: String?
    let quantity: Double
    let uomId: String?
    let referenceType: String?
    let referenceId: String?
    let postedBy: String?
    let postedAt: Date
}

struct CreateMaterialMovement: Encodable {
    let movementType: String
    let materialId: String
    let warehouseId: String?
    let quantity: Double
    let uomId: String?
}
