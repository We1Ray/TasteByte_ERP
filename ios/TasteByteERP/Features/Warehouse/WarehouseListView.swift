import SwiftUI

struct WarehouseListView: View {
    @StateObject private var viewModel = WarehouseViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.warehouses.isEmpty {
                LoadingView(message: "Loading warehouses...")
            } else if viewModel.warehouses.isEmpty {
                EmptyStateView(
                    icon: "building.2",
                    title: "No Warehouses",
                    message: "Warehouse locations will appear here once configured.",
                    action: { Task { await viewModel.loadWarehouses() } },
                    actionLabel: "Refresh"
                )
            } else {
                warehouseList
            }
        }
        .navigationTitle("Warehouses")
        .navigationBarTitleDisplayMode(.inline)
        .refreshable {
            await viewModel.loadWarehouses()
        }
        .task {
            await viewModel.loadWarehouses()
        }
    }

    private var warehouseList: some View {
        List {
            ForEach(viewModel.warehouses) { warehouse in
                HStack(spacing: 12) {
                    Image(systemName: "building.2.fill")
                        .font(.title3)
                        .foregroundStyle(.erpPrimary)
                        .frame(width: 40, height: 40)
                        .background(Color.erpPrimary.opacity(0.1), in: RoundedRectangle(cornerRadius: 8))

                    VStack(alignment: .leading, spacing: 4) {
                        HStack {
                            Text(warehouse.name)
                                .font(.subheadline)
                                .fontWeight(.medium)
                            StatusBadge(status: warehouse.isActive ? "Active" : "Inactive", size: .small)
                        }

                        Text(warehouse.warehouseNumber)
                            .font(.caption)
                            .foregroundStyle(.secondary)

                        if let address = warehouse.address {
                            Text(address)
                                .font(.caption2)
                                .foregroundStyle(.secondary)
                                .lineLimit(1)
                        }
                    }

                    Spacer()
                }
                .padding(.vertical, 4)
            }
        }
        .listStyle(.insetGrouped)
    }
}
