import Foundation

@MainActor
final class HRViewModel: ObservableObject {
    @Published var employees: [Employee] = []
    @Published var todayAttendance: Attendance?
    @Published var isClockedIn = false
    @Published var isLoading = false
    @Published var isClocking = false
    @Published var errorMessage: String?
    @Published var successMessage: String?
    @Published var searchText = ""
    @Published var currentTime = Date()

    private var timer: Timer?

    var filteredEmployees: [Employee] {
        guard !searchText.isEmpty else { return employees }
        let query = searchText.lowercased()
        return employees.filter {
            $0.fullName.lowercased().contains(query) ||
            $0.employeeNumber.lowercased().contains(query) ||
            ($0.department?.lowercased().contains(query) ?? false)
        }
    }

    func startClock() {
        timer = Timer.scheduledTimer(withTimeInterval: 1, repeats: true) { [weak self] _ in
            Task { @MainActor in
                self?.currentTime = Date()
            }
        }
    }

    func stopClock() {
        timer?.invalidate()
        timer = nil
    }

    func loadTodayAttendance() async {
        do {
            let response: APIResponse<Attendance> = try await APIClient.shared.get(
                APIEndpoints.attendanceToday
            )
            if response.success, let data = response.data {
                todayAttendance = data
                isClockedIn = data.clockIn != nil && data.clockOut == nil
            } else {
                todayAttendance = nil
                isClockedIn = false
            }
        } catch {
            todayAttendance = nil
            isClockedIn = false
        }
    }

    func clockIn() async {
        isClocking = true
        errorMessage = nil
        successMessage = nil

        do {
            let response: APIResponse<ClockInResponse> = try await APIClient.shared.postNoBody(
                APIEndpoints.clockIn
            )
            if response.success {
                isClockedIn = true
                successMessage = "Successfully clocked in!"
                await loadTodayAttendance()
            } else {
                errorMessage = response.error ?? "Failed to clock in"
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to clock in"
        }

        isClocking = false
    }

    func clockOut() async {
        isClocking = true
        errorMessage = nil
        successMessage = nil

        do {
            let response: APIResponse<ClockOutResponse> = try await APIClient.shared.postNoBody(
                APIEndpoints.clockOut
            )
            if response.success {
                isClockedIn = false
                successMessage = "Successfully clocked out!"
                await loadTodayAttendance()
            } else {
                errorMessage = response.error ?? "Failed to clock out"
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to clock out"
        }

        isClocking = false
    }

    func loadEmployees() async {
        isLoading = true
        errorMessage = nil

        do {
            let response: APIResponse<PaginatedResponse<Employee>> = try await APIClient.shared.getPaginated(
                APIEndpoints.employees,
                perPage: 100
            )
            if response.success, let paginated = response.data {
                employees = paginated.items
            } else {
                errorMessage = response.error ?? "Failed to load employees"
            }
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "Failed to load employees"
        }

        isLoading = false
    }
}
