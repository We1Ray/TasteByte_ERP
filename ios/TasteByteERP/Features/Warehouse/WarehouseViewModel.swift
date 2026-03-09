import Foundation

@MainActor
final class WarehouseViewModel: ObservableObject {
    @Published var warehouses: [Warehouse] = []
    @Published var stockCounts: [StockCount] = []
    @Published var materials: [Material] = []
    @Published var isLoading = false
    @Published var isSubmitting = false
    @Published var errorMessage: String?
    @Published var successMessage: String?
    @Published var selectedWarehouseId: String?

    func loadWarehouses() async {
        isLoading = true
        errorMessage = nil

        do {
            let response: APIResponse<PaginatedResponse<Warehouse>> = try await APIClient.shared.getPaginated(
                APIEndpoints.warehouses,
                perPage: 50
            )
            if response.success, let paginated = response.data {
                warehouses = paginated.items
                if selectedWarehouseId == nil, let first = warehouses.first {
                    selectedWarehouseId = first.id
                }
            } else {
                errorMessage = response.error ?? "Failed to load warehouses"
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load warehouses"
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
            // Supplementary data
        }
    }

    func loadStockCounts(warehouseId: String) async {
        do {
            let params = [URLQueryItem(name: "warehouse_id", value: warehouseId)]
            let response: APIResponse<PaginatedResponse<StockCount>> = try await APIClient.shared.getPaginated(
                APIEndpoints.stockCounts,
                perPage: 100,
                extraParams: params
            )
            if response.success, let paginated = response.data {
                stockCounts = paginated.items
            }
        } catch {
            stockCounts = []
        }
    }

    func submitStockCount(warehouseId: String, materialId: String, countedQuantity: Double) async {
        isSubmitting = true
        errorMessage = nil
        successMessage = nil

        do {
            let body = CreateStockCount(
                warehouseId: warehouseId,
                materialId: materialId,
                countedQuantity: countedQuantity
            )
            let response: APIResponse<StockCount> = try await APIClient.shared.post(
                APIEndpoints.stockCounts,
                body: body
            )
            if response.success {
                successMessage = "Stock count submitted successfully!"
                if let id = selectedWarehouseId {
                    await loadStockCounts(warehouseId: id)
                }
            } else {
                errorMessage = response.error ?? "Failed to submit stock count"
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to submit stock count"
        }

        isSubmitting = false
    }

    func materialName(for materialId: String) -> String {
        materials.first { $0.id == materialId }?.name ?? "Unknown Material"
    }

    func warehouseName(for warehouseId: String) -> String {
        warehouses.first { $0.id == warehouseId }?.name ?? "Unknown Warehouse"
    }
}
