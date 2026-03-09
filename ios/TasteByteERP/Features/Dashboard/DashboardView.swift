import SwiftUI

struct DashboardView: View {
    @EnvironmentObject var authManager: AuthManager
    @StateObject private var viewModel = DashboardViewModel()

    private let columns = [
        GridItem(.flexible(), spacing: 12),
        GridItem(.flexible(), spacing: 12),
    ]

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 20) {
                    // Welcome header
                    HStack {
                        VStack(alignment: .leading, spacing: 4) {
                            Text("Welcome back,")
                                .font(.subheadline)
                                .foregroundStyle(.secondary)
                            Text(authManager.currentUser?.displayName ?? authManager.currentUser?.username ?? "User")
                                .font(.title2)
                                .fontWeight(.bold)
                        }
                        Spacer()
                        Text(Date().formatted(as: .medium))
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                    .padding(.horizontal, 16)

                    // KPI Grid
                    if let kpis = viewModel.kpis {
                        LazyVGrid(columns: columns, spacing: 12) {
                            KPICardView(
                                title: "Total Revenue",
                                value: kpis.totalRevenue.currencyFormatted,
                                icon: "dollarsign.circle.fill",
                                color: .erpSuccess
                            )
                            KPICardView(
                                title: "Sales Orders",
                                value: "\(kpis.totalOrderCount)",
                                icon: "cart.fill",
                                color: .erpPrimary
                            )
                            KPICardView(
                                title: "Inventory Qty",
                                value: kpis.totalInventoryQuantity.quantityFormatted,
                                icon: "cube.box.fill",
                                color: .erpAccent
                            )
                            KPICardView(
                                title: "Pending Production",
                                value: "\(kpis.pendingProductionOrders)",
                                icon: "gearshape.2.fill",
                                color: .erpWarning
                            )
                            KPICardView(
                                title: "Open AR",
                                value: kpis.openArAmount.currencyFormatted,
                                icon: "arrow.down.circle.fill",
                                color: .erpPrimary
                            )
                            KPICardView(
                                title: "Open AP",
                                value: kpis.openApAmount.currencyFormatted,
                                icon: "arrow.up.circle.fill",
                                color: .erpError
                            )
                        }
                        .padding(.horizontal, 16)
                    }

                    // Quick Actions
                    ERPSectionHeader(title: "Quick Actions")
                    ScrollView(.horizontal, showsIndicators: false) {
                        HStack(spacing: 12) {
                            QuickActionButton(
                                title: "Clock In",
                                icon: "clock.badge.checkmark.fill",
                                color: .erpSuccess
                            )
                            QuickActionButton(
                                title: "Stock Count",
                                icon: "checklist",
                                color: .erpPrimary
                            )
                            QuickActionButton(
                                title: "New Order",
                                icon: "plus.circle.fill",
                                color: .erpAccent
                            )
                            QuickActionButton(
                                title: "Inspection",
                                icon: "checkmark.shield.fill",
                                color: .erpWarning
                            )
                        }
                        .padding(.horizontal, 16)
                    }

                    // Recent Activity
                    if !viewModel.recentActivities.isEmpty {
                        ERPSectionHeader(title: "Recent Activity")

                        VStack(spacing: 0) {
                            ForEach(viewModel.recentActivities) { activity in
                                ActivityRow(activity: activity)
                                if activity.id != viewModel.recentActivities.last?.id {
                                    Divider()
                                        .padding(.leading, 52)
                                }
                            }
                        }
                        .background(Color(uiColor: .secondarySystemGroupedBackground))
                        .clipShape(RoundedRectangle(cornerRadius: 12))
                        .padding(.horizontal, 16)
                    }

                    Spacer(minLength: 20)
                }
                .padding(.top, 12)
            }
            .background(Color.erpBackground)
            .navigationTitle("Dashboard")
            .refreshable {
                await viewModel.loadDashboard()
            }
            .task {
                await viewModel.loadDashboard()
            }
        }
    }
}

struct QuickActionButton: View {
    let title: String
    let icon: String
    let color: Color

    var body: some View {
        VStack(spacing: 8) {
            Image(systemName: icon)
                .font(.title2)
                .foregroundStyle(color)
                .frame(width: 52, height: 52)
                .background(color.opacity(0.12), in: RoundedRectangle(cornerRadius: 14))

            Text(title)
                .font(.caption)
                .foregroundStyle(.primary)
        }
        .frame(width: 80)
    }
}

struct ActivityRow: View {
    let activity: RecentActivity

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: activityIcon)
                .font(.body)
                .foregroundStyle(activityColor)
                .frame(width: 36, height: 36)
                .background(activityColor.opacity(0.12), in: Circle())

            VStack(alignment: .leading, spacing: 2) {
                Text(activity.description)
                    .font(.subheadline)
                    .lineLimit(2)
                HStack(spacing: 6) {
                    if let ref = activity.referenceNumber {
                        Text(ref)
                            .font(.caption)
                            .fontWeight(.medium)
                            .foregroundStyle(.erpPrimary)
                    }
                    Text(activity.timestamp.relativeDescription())
                        .font(.caption)
                        .foregroundStyle(.secondary)
                }
            }
            Spacer()
        }
        .padding(.horizontal, 16)
        .padding(.vertical, 10)
    }

    private var activityIcon: String {
        switch activity.type.lowercased() {
        case "sales_order": return "cart.fill"
        case "material": return "cube.box.fill"
        case "delivery": return "shippingbox.fill"
        case "attendance": return "clock.fill"
        case "inspection": return "checkmark.shield.fill"
        default: return "doc.fill"
        }
    }

    private var activityColor: Color {
        switch activity.type.lowercased() {
        case "sales_order": return .erpPrimary
        case "material": return .erpAccent
        case "delivery": return .erpSuccess
        case "attendance": return .purple
        case "inspection": return .erpWarning
        default: return .secondary
        }
    }
}
