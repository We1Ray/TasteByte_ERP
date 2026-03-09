import SwiftUI

struct KPICardView: View {
    let title: String
    let value: String
    let icon: String
    let color: Color
    var trend: String?

    var body: some View {
        VStack(alignment: .leading, spacing: 10) {
            HStack {
                Image(systemName: icon)
                    .font(.title3)
                    .foregroundStyle(color)
                Spacer()
                if let trend = trend {
                    Text(trend)
                        .font(.caption2)
                        .fontWeight(.semibold)
                        .foregroundStyle(trend.hasPrefix("+") ? Color.erpSuccess : .secondary)
                }
            }

            Text(value)
                .font(.title2)
                .fontWeight(.bold)
                .lineLimit(1)
                .minimumScaleFactor(0.7)

            Text(title)
                .font(.caption)
                .foregroundStyle(.secondary)
                .lineLimit(1)
        }
        .padding(14)
        .frame(maxWidth: .infinity, alignment: .leading)
        .background(Color(uiColor: .secondarySystemGroupedBackground))
        .clipShape(RoundedRectangle(cornerRadius: 12))
        .shadow(color: .black.opacity(0.04), radius: 2, y: 1)
    }
}
