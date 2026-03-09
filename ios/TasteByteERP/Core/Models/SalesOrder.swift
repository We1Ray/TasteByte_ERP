import Foundation

struct SalesOrder: Codable, Identifiable {
    let id: String
    let orderNumber: String
    let customerId: String
    let orderDate: Date
    let requestedDeliveryDate: Date?
    let status: String
    let totalAmount: Double
    let currency: String
    let notes: String?
    let createdBy: String?
    let createdAt: Date
    let updatedAt: Date
}

struct SalesOrderItem: Codable, Identifiable {
    let id: String
    let salesOrderId: String
    let lineNumber: Int
    let materialId: String
    let quantity: Double
    let unitPrice: Double
    let totalPrice: Double
    let uomId: String?
    let deliveredQuantity: Double
}

struct Customer: Codable, Identifiable {
    let id: String
    let customerNumber: String
    let name: String
    let contactPerson: String?
    let email: String?
    let phone: String?
    let address: String?
    let paymentTerms: Int
    let creditLimit: Double
    let isActive: Bool
    let createdAt: Date
    let updatedAt: Date
}

struct Delivery: Codable, Identifiable {
    let id: String
    let deliveryNumber: String
    let salesOrderId: String
    let deliveryDate: Date
    let status: String
    let shippedBy: String?
    let shippedAt: Date?
    let createdAt: Date
}
