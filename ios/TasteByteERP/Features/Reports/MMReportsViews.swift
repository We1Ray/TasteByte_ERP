import SwiftUI

// MARK: - Stock Valuation

struct StockValuationView: View {
    @StateObject private var viewModel = ReportsViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.stockValuation.isEmpty {
                LoadingView(message: "Loading stock valuation...")
            } else if viewModel.stockValuation.isEmpty {
                EmptyStateView(
                    icon: "cube.box",
                    title: "No Data",
                    message: "Stock valuation data is not available.",
                    action: { Task { await viewModel.loadStockValuation() } },
                    actionLabel: "Refresh"
                )
            } else {
                List {
                    Section {
                        ForEach(viewModel.stockValuation) { entry in
                            VStack(alignment: .leading, spacing: 6) {
                                HStack {
                                    Text(entry.materialNumber)
                                        .font(.caption)
                                        .fontWeight(.medium)
                                        .foregroundStyle(.secondary)
                                    Spacer()
                                    Text(entry.totalValue.currencyFormatted)
                                        .font(.subheadline)
                                        .fontWeight(.bold)
                                }
                                Text(entry.materialName)
                                    .font(.subheadline)
                                HStack {
                                    Text("Qty: \(entry.quantity.quantityFormatted)")
                                        .font(.caption)
                                        .foregroundStyle(.secondary)
                                    Spacer()
                                    Text("Unit Cost: \(entry.unitCost.currencyFormatted)")
                                        .font(.caption)
                                        .foregroundStyle(.secondary)
                                }
                            }
                            .padding(.vertical, 2)
                        }
                    } header: {
                        Text("\(viewModel.stockValuation.count) Materials")
                    }
                }
                .listStyle(.insetGrouped)
            }
        }
        .navigationTitle("Stock Valuation")
        .navigationBarTitleDisplayMode(.inline)
        .refreshable { await viewModel.loadStockValuation() }
        .task { await viewModel.loadStockValuation() }
    }
}

// MARK: - Movement Summary

struct MovementSummaryView: View {
    @StateObject private var viewModel = ReportsViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.movementSummary.isEmpty {
                LoadingView(message: "Loading movement summary...")
            } else if viewModel.movementSummary.isEmpty {
                EmptyStateView(
                    icon: "arrow.left.arrow.right",
                    title: "No Data",
                    message: "Movement summary data is not available.",
                    action: { Task { await viewModel.loadMovementSummary() } },
                    actionLabel: "Refresh"
                )
            } else {
                List {
                    ForEach(viewModel.movementSummary) { entry in
                        HStack {
                            VStack(alignment: .leading, spacing: 4) {
                                Text(entry.movementType.replacingOccurrences(of: "_", with: " ").capitalized)
                                    .font(.subheadline)
                                    .fontWeight(.medium)
                                Text("\(entry.count) movements")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }
                            Spacer()
                            Text(entry.totalQuantity.quantityFormatted)
                                .font(.subheadline)
                                .fontWeight(.bold)
                        }
                    }
                }
                .listStyle(.insetGrouped)
            }
        }
        .navigationTitle("Movement Summary")
        .navigationBarTitleDisplayMode(.inline)
        .refreshable { await viewModel.loadMovementSummary() }
        .task { await viewModel.loadMovementSummary() }
    }
}

// MARK: - Slow-Moving Items

struct SlowMovingView: View {
    @StateObject private var viewModel = ReportsViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.slowMoving.isEmpty {
                LoadingView(message: "Loading slow-moving items...")
            } else if viewModel.slowMoving.isEmpty {
                EmptyStateView(
                    icon: "tortoise",
                    title: "No Data",
                    message: "Slow-moving items data is not available.",
                    action: { Task { await viewModel.loadSlowMoving() } },
                    actionLabel: "Refresh"
                )
            } else {
                List {
                    Section {
                        ForEach(viewModel.slowMoving) { entry in
                            VStack(alignment: .leading, spacing: 6) {
                                HStack {
                                    Text(entry.materialNumber)
                                        .font(.caption)
                                        .fontWeight(.medium)
                                        .foregroundStyle(.secondary)
                                    Spacer()
                                    Text("\(entry.daysSinceLastMovement) days")
                                        .font(.caption)
                                        .fontWeight(.semibold)
                                        .foregroundStyle(entry.daysSinceLastMovement > 90 ? .erpError : .erpWarning)
                                }
                                Text(entry.materialName)
                                    .font(.subheadline)
                                HStack {
                                    Text("Qty: \(entry.quantity.quantityFormatted)")
                                        .font(.caption)
                                        .foregroundStyle(.secondary)
                                    Spacer()
                                    if let lastDate = entry.lastMovementDate {
                                        Text("Last: \(lastDate.formatted(as: .medium))")
                                            .font(.caption)
                                            .foregroundStyle(.secondary)
                                    }
                                }
                            }
                            .padding(.vertical, 2)
                        }
                    } header: {
                        Text("\(viewModel.slowMoving.count) Items")
                    }
                }
                .listStyle(.insetGrouped)
            }
        }
        .navigationTitle("Slow-Moving Items")
        .navigationBarTitleDisplayMode(.inline)
        .refreshable { await viewModel.loadSlowMoving() }
        .task { await viewModel.loadSlowMoving() }
    }
}
