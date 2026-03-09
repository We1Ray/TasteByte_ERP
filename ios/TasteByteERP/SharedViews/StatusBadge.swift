import SwiftUI

struct StatusBadge: View {
    let status: String
    var size: BadgeSize = .regular

    var body: some View {
        Text(displayText)
            .font(size == .small ? .caption2 : .caption)
            .fontWeight(.semibold)
            .foregroundStyle(Color.statusColor(for: status))
            .padding(.horizontal, size == .small ? 6 : 8)
            .padding(.vertical, size == .small ? 2 : 4)
            .background(
                Color.statusColor(for: status).opacity(0.12),
                in: Capsule()
            )
    }

    private var displayText: String {
        status
            .replacingOccurrences(of: "_", with: " ")
            .capitalized
    }

    enum BadgeSize {
        case small, regular
    }
}
