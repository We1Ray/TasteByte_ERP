import Foundation

@MainActor
final class MaterialsViewModel: ObservableObject {
    @Published var materials: [Material] = []
    @Published var plantStocks: [PlantStock] = []
    @Published var isLoading = false
    @Published var errorMessage: String?
    @Published var searchText = ""
    @Published var currentPage = 1
    @Published var totalPages = 1
    @Published var selectedMaterial: Material?
    @Published var materialDetail: Material?

    var filteredMaterials: [Material] {
        guard !searchText.isEmpty else { return materials }
        let query = searchText.lowercased()
        return materials.filter {
            $0.name.lowercased().contains(query) ||
            $0.materialNumber.lowercased().contains(query) ||
            $0.materialType.lowercased().contains(query)
        }
    }

    func loadMaterials() async {
        isLoading = true
        errorMessage = nil

        do {
            let response: APIResponse<PaginatedResponse<Material>> = try await APIClient.shared.getPaginated(
                APIEndpoints.materials,
                page: currentPage,
                perPage: 50
            )
            if response.success, let paginated = response.data {
                materials = paginated.items
                totalPages = paginated.totalPages
            } else {
                errorMessage = response.error ?? "Failed to load materials"
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load materials"
        }

        isLoading = false
    }

    func loadMaterialDetail(id: String) async {
        do {
            let response: APIResponse<Material> = try await APIClient.shared.get(
                APIEndpoints.material(id)
            )
            if response.success, let data = response.data {
                materialDetail = data
            } else {
                errorMessage = response.error ?? "Failed to load material details"
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load material details"
        }
    }

    func loadPlantStock() async {
        do {
            let response: APIResponse<PaginatedResponse<PlantStock>> = try await APIClient.shared.getPaginated(
                APIEndpoints.plantStock,
                perPage: 100
            )
            if response.success, let paginated = response.data {
                plantStocks = paginated.items
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load stock data"
        }
    }

    func stockForMaterial(_ materialId: String) -> PlantStock? {
        plantStocks.first { $0.materialId == materialId }
    }
}
