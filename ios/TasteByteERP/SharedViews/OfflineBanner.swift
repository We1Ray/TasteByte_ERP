import SwiftUI

struct OfflineBanner: View {
    @ObservedObject var networkMonitor = NetworkMonitor.shared
    @ObservedObject var syncManager = OfflineSyncManager.shared

    var body: some View {
        if !networkMonitor.isConnected {
            HStack(spacing: 8) {
                Image(systemName: "wifi.slash")
                    .font(.subheadline)
                Text("Offline Mode")
                    .font(.subheadline)
                    .fontWeight(.medium)
                if syncManager.pendingCount > 0 {
                    Spacer()
                    Text("\(syncManager.pendingCount) pending")
                        .font(.caption)
                        .padding(.horizontal, 8)
                        .padding(.vertical, 2)
                        .background(Color.white.opacity(0.25))
                        .clipShape(Capsule())
                }
            }
            .frame(maxWidth: .infinity, alignment: .leading)
            .padding(.horizontal, 16)
            .padding(.vertical, 8)
            .background(Color.erpWarning)
            .foregroundStyle(.white)
        }
    }
}
