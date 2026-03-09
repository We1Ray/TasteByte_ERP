import Foundation
import SwiftUI

@MainActor
final class LoginViewModel: ObservableObject {
    @Published var username = ""
    @Published var password = ""
    @Published var errorMessage: String?
    @Published var isLoading = false

    var isFormValid: Bool {
        !username.trimmingCharacters(in: .whitespaces).isEmpty &&
        !password.isEmpty
    }

    func login(authManager: AuthManager) async {
        guard isFormValid else {
            errorMessage = "Please enter both username and password."
            return
        }

        isLoading = true
        errorMessage = nil

        do {
            try await authManager.login(
                username: username.trimmingCharacters(in: .whitespaces),
                password: password
            )
        } catch let error as APIError {
            errorMessage = error.errorDescription
        } catch {
            errorMessage = "An unexpected error occurred. Please try again."
        }

        isLoading = false
    }
}
