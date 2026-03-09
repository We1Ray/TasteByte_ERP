import SwiftUI

// MARK: - Sales Summary

struct SalesSummaryView: View {
    @StateObject private var viewModel = ReportsViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.salesSummary.isEmpty {
                LoadingView(message: "Loading sales summary...")
            } else if viewModel.salesSummary.isEmpty {
                EmptyStateView(
                    icon: "chart.bar",
                    title: "No Data",
                    message: "Sales summary data is not available.",
                    action: { Task { await viewModel.loadSalesSummary() } },
                    actionLabel: "Refresh"
                )
            } else {
                List {
                    ForEach(viewModel.salesSummary) { entry in
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text(entry.period)
                                    .font(.subheadline)
                                    .fontWeight(.medium)
                                Text("\(entry.orderCount) orders")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }
                            Spacer()
                            Text(entry.totalAmount.currencyFormatted)
                                .font(.subheadline)
                                .fontWeight(.bold)
                                .foregroundStyle(.erpPrimary)
                        }
                    }
                }
                .listStyle(.insetGrouped)
            }
        }
        .navigationTitle("Sales Summary")
        .navigationBarTitleDisplayMode(.inline)
        .refreshable { await viewModel.loadSalesSummary() }
        .task { await viewModel.loadSalesSummary() }
    }
}

// MARK: - Order Fulfillment

struct OrderFulfillmentView: View {
    @StateObject private var viewModel = ReportsViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.orderFulfillment.isEmpty {
                LoadingView(message: "Loading order fulfillment...")
            } else if viewModel.orderFulfillment.isEmpty {
                EmptyStateView(
                    icon: "checkmark.circle",
                    title: "No Data",
                    message: "Order fulfillment data is not available.",
                    action: { Task { await viewModel.loadOrderFulfillment() } },
                    actionLabel: "Refresh"
                )
            } else {
                List {
                    ForEach(viewModel.orderFulfillment) { entry in
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                HStack {
                                    Text(entry.status.replacingOccurrences(of: "_", with: " ").capitalized)
                                        .font(.subheadline)
                                        .fontWeight(.medium)
                                    StatusBadge(status: entry.status, size: .small)
                                }
                                Text("\(entry.count) orders")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }
                            Spacer()
                            Text(String(format: "%.1f%%", entry.percentage))
                                .font(.subheadline)
                                .fontWeight(.bold)
                        }
                    }
                }
                .listStyle(.insetGrouped)
            }
        }
        .navigationTitle("Order Fulfillment")
        .navigationBarTitleDisplayMode(.inline)
        .refreshable { await viewModel.loadOrderFulfillment() }
        .task { await viewModel.loadOrderFulfillment() }
    }
}

// MARK: - Top Customers

struct TopCustomersView: View {
    @StateObject private var viewModel = ReportsViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.topCustomers.isEmpty {
                LoadingView(message: "Loading top customers...")
            } else if viewModel.topCustomers.isEmpty {
                EmptyStateView(
                    icon: "person.3",
                    title: "No Data",
                    message: "Top customers data is not available.",
                    action: { Task { await viewModel.loadTopCustomers() } },
                    actionLabel: "Refresh"
                )
            } else {
                List {
                    ForEach(Array(viewModel.topCustomers.enumerated()), id: \.element.id) { index, entry in
                        HStack(spacing: 12) {
                            Text("#\(index + 1)")
                                .font(.caption)
                                .fontWeight(.bold)
                                .foregroundStyle(.secondary)
                                .frame(width: 28)

                            VStack(alignment: .leading, spacing: 4) {
                                Text(entry.customerName)
                                    .font(.subheadline)
                                    .fontWeight(.medium)
                                Text("\(entry.orderCount) orders")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }
                            Spacer()
                            Text(entry.totalAmount.currencyFormatted)
                                .font(.subheadline)
                                .fontWeight(.bold)
                                .foregroundStyle(.erpSuccess)
                        }
                    }
                }
                .listStyle(.insetGrouped)
            }
        }
        .navigationTitle("Top Customers")
        .navigationBarTitleDisplayMode(.inline)
        .refreshable { await viewModel.loadTopCustomers() }
        .task { await viewModel.loadTopCustomers() }
    }
}
