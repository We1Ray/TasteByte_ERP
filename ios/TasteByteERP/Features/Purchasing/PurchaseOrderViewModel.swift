import Foundation

@MainActor
final class PurchaseOrderViewModel: ObservableObject {
    @Published var purchaseOrders: [PurchaseOrder] = []
    @Published var vendors: [Vendor] = []
    @Published var selectedOrderItems: [PurchaseOrderItem] = []
    @Published var isLoading = false
    @Published var errorMessage: String?
    @Published var searchText = ""
    @Published var currentPage = 1
    @Published var totalPages = 1

    var filteredOrders: [PurchaseOrder] {
        guard !searchText.isEmpty else { return purchaseOrders }
        let query = searchText.lowercased()
        return purchaseOrders.filter {
            $0.orderNumber.lowercased().contains(query) ||
            $0.status.lowercased().contains(query) ||
            vendorName(for: $0.vendorId).lowercased().contains(query)
        }
    }

    func loadPurchaseOrders() async {
        isLoading = true
        errorMessage = nil

        do {
            let response: APIResponse<PaginatedResponse<PurchaseOrder>> = try await APIClient.shared.getPaginated(
                APIEndpoints.purchaseOrders,
                page: currentPage,
                perPage: 50
            )
            if response.success, let paginated = response.data {
                purchaseOrders = paginated.items
                totalPages = paginated.totalPages
            } else {
                errorMessage = response.error ?? "Failed to load purchase orders"
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load purchase orders"
        }

        isLoading = false
    }

    func loadVendors() async {
        do {
            let response: APIResponse<PaginatedResponse<Vendor>> = try await APIClient.shared.getPaginated(
                APIEndpoints.vendors,
                perPage: 100
            )
            if response.success, let paginated = response.data {
                vendors = paginated.items
            }
        } catch {
            // Vendors are supplementary data
        }
    }

    func loadOrderItems(orderId: String) async {
        do {
            let endpoint = APIEndpoints.purchaseOrder(orderId) + "/items"
            let response: APIResponse<[PurchaseOrderItem]> = try await APIClient.shared.get(endpoint)
            if response.success, let data = response.data {
                selectedOrderItems = data
            }
        } catch {
            // Items will show empty
        }
    }

    func receiveOrder(orderId: String) async {
        isLoading = true
        errorMessage = nil
        do {
            let _: APIResponse<PurchaseOrder> = try await APIClient.shared.postNoBody(
                APIEndpoints.purchaseOrderReceive(orderId)
            )
            await loadPurchaseOrders()
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to receive purchase order"
        }
        isLoading = false
    }

    func vendorName(for vendorId: String) -> String {
        vendors.first { $0.id == vendorId }?.name ?? "Vendor"
    }
}
