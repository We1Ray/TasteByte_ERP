import SwiftUI

struct PurchaseOrdersView: View {
    @StateObject private var viewModel = PurchaseOrderViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.purchaseOrders.isEmpty {
                LoadingView(message: "Loading purchase orders...")
            } else if viewModel.purchaseOrders.isEmpty {
                EmptyStateView(
                    icon: "doc.plaintext",
                    title: "No Purchase Orders",
                    message: "Purchase orders will appear here once they are created.",
                    action: { Task { await viewModel.loadPurchaseOrders() } },
                    actionLabel: "Refresh"
                )
            } else {
                ordersList
            }
        }
        .navigationTitle("Purchase Orders")
        .refreshable {
            await viewModel.loadPurchaseOrders()
            await viewModel.loadVendors()
        }
        .task {
            await viewModel.loadPurchaseOrders()
            await viewModel.loadVendors()
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
                SearchField(placeholder: "Search purchase orders...", text: $viewModel.searchText)
                    .listRowInsets(EdgeInsets(top: 4, leading: 16, bottom: 4, trailing: 16))
                    .listRowSeparator(.hidden)
            }

            Section {
                ForEach(viewModel.filteredOrders) { order in
                    NavigationLink {
                        PurchaseOrderDetailView(order: order, viewModel: viewModel)
                    } label: {
                        PurchaseOrderRow(
                            order: order,
                            vendorName: viewModel.vendorName(for: order.vendorId)
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

struct PurchaseOrderRow: View {
    let order: PurchaseOrder
    let vendorName: String

    var body: some View {
        HStack(spacing: 12) {
            VStack(alignment: .leading, spacing: 6) {
                HStack {
                    Text(order.orderNumber)
                        .font(.subheadline)
                        .fontWeight(.semibold)
                    StatusBadge(status: order.status, size: .small)
                }

                Text(vendorName)
                    .font(.caption)
                    .foregroundStyle(.secondary)
                    .lineLimit(1)

                Text(order.orderDate.formatted(as: .medium))
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 4) {
                Text(order.totalAmount.currencyFormatted)
                    .font(.subheadline)
                    .fontWeight(.bold)
                Text(order.currency)
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }
        }
        .padding(.vertical, 4)
    }
}
