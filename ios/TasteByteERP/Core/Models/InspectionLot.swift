import Foundation

struct InspectionLot: Codable, Identifiable {
    let id: String
    let lotNumber: String
    let materialId: String
    let materialName: String?
    let inspectionType: String
    let status: String
    let quantity: Double
    let uomId: String?
    let createdAt: Date
    let updatedAt: Date
}

struct InspectionCharacteristic: Codable, Identifiable {
    let id: String
    let inspectionLotId: String
    let name: String
    let targetValue: String?
    let lowerLimit: Double?
    let upperLimit: Double?
    let uom: String?
}

struct InspectionResult: Codable, Identifiable {
    let id: String
    let characteristicId: String
    let actualValue: String
    let isConforming: Bool
    let inspectedBy: String?
    let inspectedAt: Date?
}

struct CreateInspectionResult: Encodable {
    let characteristicId: String
    let actualValue: String
    let isConforming: Bool
}

struct SubmitInspectionResults: Encodable {
    let results: [CreateInspectionResult]
}
