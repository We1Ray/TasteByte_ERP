import Foundation

@MainActor
final class ProductionViewModel: ObservableObject {
    @Published var productionOrders: [ProductionOrder] = []
    @Published var materials: [Material] = []
    @Published var isLoading = false
    @Published var errorMessage: String?
    @Published var searchText = ""
    @Published var currentPage = 1
    @Published var totalPages = 1

    var filteredOrders: [ProductionOrder] {
        guard !searchText.isEmpty else { return productionOrders }
        let query = searchText.lowercased()
        return productionOrders.filter {
            $0.orderNumber.lowercased().contains(query) ||
            $0.status.lowercased().contains(query) ||
            materialName(for: $0.materialId).lowercased().contains(query)
        }
    }

    func loadProductionOrders() async {
        isLoading = true
        errorMessage = nil

        do {
            let response: APIResponse<PaginatedResponse<ProductionOrder>> = try await APIClient.shared.getPaginated(
                APIEndpoints.productionOrders,
                page: currentPage,
                perPage: 50
            )
            if response.success, let paginated = response.data {
                productionOrders = paginated.items
                totalPages = paginated.totalPages
            } else {
                errorMessage = response.error ?? "Failed to load production orders"
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load production orders"
        }

        isLoading = false
    }

    func loadMaterials() async {
        do {
            let response: APIResponse<PaginatedResponse<Material>> = try await APIClient.shared.getPaginated(
                APIEndpoints.materials,
                perPage: 100
            )
            if response.success, let paginated = response.data {
                materials = paginated.items
            }
        } catch {
            // Materials are supplementary data
        }
    }

    func updateStatus(orderId: String, newStatus: String) async {
        isLoading = true
        errorMessage = nil
        do {
            let body = ProductionOrderStatusUpdate(status: newStatus)
            let _: APIResponse<ProductionOrder> = try await APIClient.shared.put(
                APIEndpoints.productionOrderStatus(orderId),
                body: body
            )
            await loadProductionOrders()
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to update production order status"
        }
        isLoading = false
    }

    func materialName(for materialId: String) -> String {
        materials.first { $0.id == materialId }?.name ?? "Material"
    }
}
