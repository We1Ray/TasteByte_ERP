import Foundation

@MainActor
final class QualityViewModel: ObservableObject {
    @Published var inspectionLots: [InspectionLot] = []
    @Published var characteristics: [InspectionCharacteristic] = []
    @Published var isLoading = false
    @Published var isSubmitting = false
    @Published var errorMessage: String?
    @Published var successMessage: String?
    @Published var searchText = ""

    var filteredLots: [InspectionLot] {
        guard !searchText.isEmpty else { return inspectionLots }
        let query = searchText.lowercased()
        return inspectionLots.filter {
            $0.lotNumber.lowercased().contains(query) ||
            ($0.materialName?.lowercased().contains(query) ?? false) ||
            $0.status.lowercased().contains(query) ||
            $0.inspectionType.lowercased().contains(query)
        }
    }

    func loadInspectionLots() async {
        isLoading = true
        errorMessage = nil

        do {
            let response: APIResponse<PaginatedResponse<InspectionLot>> = try await APIClient.shared.getPaginated(
                APIEndpoints.inspectionLots,
                perPage: 50
            )
            if response.success, let paginated = response.data {
                inspectionLots = paginated.items
            } else {
                errorMessage = response.error ?? "Failed to load inspection lots"
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load inspection lots"
        }

        isLoading = false
    }

    func loadCharacteristics(lotId: String) async {
        do {
            let endpoint = APIEndpoints.inspectionLot(lotId) + "/characteristics"
            let response: APIResponse<[InspectionCharacteristic]> = try await APIClient.shared.get(endpoint)
            if response.success, let data = response.data {
                characteristics = data
            }
        } catch {
            characteristics = []
        }
    }

    func submitResults(lotId: String, results: [CreateInspectionResult]) async {
        isSubmitting = true
        errorMessage = nil
        successMessage = nil

        do {
            let body = SubmitInspectionResults(results: results)
            let response: APIResponse<[InspectionResult]> = try await APIClient.shared.post(
                APIEndpoints.inspectionResults(lotId),
                body: body
            )
            if response.success {
                successMessage = "Inspection results submitted successfully!"
                await loadInspectionLots()
            } else {
                errorMessage = response.error ?? "Failed to submit results"
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to submit inspection results"
        }

        isSubmitting = false
    }
}
