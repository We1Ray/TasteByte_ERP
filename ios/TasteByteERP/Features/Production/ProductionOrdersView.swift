import SwiftUI

struct ProductionOrdersView: View {
    @StateObject private var viewModel = ProductionViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.productionOrders.isEmpty {
                LoadingView(message: "Loading production orders...")
            } else if viewModel.productionOrders.isEmpty {
                EmptyStateView(
                    icon: "gearshape.2",
                    title: "No Production Orders",
                    message: "Production orders will appear here once they are created.",
                    action: { Task { await viewModel.loadProductionOrders() } },
                    actionLabel: "Refresh"
                )
            } else {
                ordersList
            }
        }
        .navigationTitle("Production Orders")
        .refreshable {
            await viewModel.loadProductionOrders()
            await viewModel.loadMaterials()
        }
        .task {
            await viewModel.loadProductionOrders()
            await viewModel.loadMaterials()
        }
        .alert("Error", isPresented: .constant(viewModel.errorMessage != nil)) {
            Button("OK") { viewModel.errorMessage = nil }
        } message: {
            Text(viewModel.errorMessage ?? "")
        }
    }

    private var ordersList: some View {
        List {
            Section {
                SearchField(placeholder: "Search production orders...", text: $viewModel.searchText)
                    .listRowInsets(EdgeInsets(top: 4, leading: 16, bottom: 4, trailing: 16))
                    .listRowSeparator(.hidden)
            }

            Section {
                ForEach(viewModel.filteredOrders) { order in
                    NavigationLink {
                        ProductionOrderDetailView(order: order, viewModel: viewModel)
                    } label: {
                        ProductionOrderRow(
                            order: order,
                            materialName: viewModel.materialName(for: order.materialId)
                        )
                    }
                }
            } header: {
                Text("\(viewModel.filteredOrders.count) Orders")
            }
        }
        .listStyle(.insetGrouped)
    }
}

struct ProductionOrderRow: View {
    let order: ProductionOrder
    let materialName: String

    var body: some View {
        HStack(spacing: 12) {
            VStack(alignment: .leading, spacing: 6) {
                HStack {
                    Text(order.orderNumber)
                        .font(.subheadline)
                        .fontWeight(.semibold)
                    StatusBadge(status: order.status, size: .small)
                }

                Text(materialName)
                    .font(.caption)
                    .foregroundStyle(.secondary)
                    .lineLimit(1)

                if let startDate = order.plannedStartDate {
                    Text("Start: \(startDate.formatted(as: .medium))")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 4) {
                Text(order.quantity.quantityFormatted)
                    .font(.subheadline)
                    .fontWeight(.bold)
                Text("qty")
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }
        }
        .padding(.vertical, 4)
    }
}
