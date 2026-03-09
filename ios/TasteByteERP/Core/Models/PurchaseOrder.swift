import Foundation

struct PurchaseOrder: Codable, Identifiable {
    let id: String
    let orderNumber: String
    let vendorId: String
    let orderDate: Date
    let expectedDeliveryDate: Date?
    let status: String
    let totalAmount: Double
    let currency: String
    let notes: String?
    let createdBy: String?
    let createdAt: Date
    let updatedAt: Date
}

struct PurchaseOrderItem: Codable, Identifiable {
    let id: String
    let purchaseOrderId: String
    let lineNumber: Int
    let materialId: String
    let quantity: Double
    let unitPrice: Double
    let totalPrice: Double
    let uomId: String?
    let receivedQuantity: Double
}

struct Vendor: Codable, Identifiable {
    let id: String
    let vendorNumber: String
    let name: String
    let contactPerson: String?
    let email: String?
    let phone: String?
    let address: String?
    let paymentTerms: Int
    let isActive: Bool
    let createdAt: Date
    let updatedAt: Date
}
