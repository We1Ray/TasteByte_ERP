import SwiftUI

@main
struct TasteByteERPApp: App {
    @StateObject private var authManager = AuthManager()
    @StateObject private var networkMonitor = NetworkMonitor.shared

    var body: some Scene {
        WindowGroup {
            Group {
                if authManager.isAuthenticated {
                    ContentView()
                } else {
                    LoginView()
                }
            }
            .environmentObject(authManager)
            .environmentObject(networkMonitor)
        }
    }
}
