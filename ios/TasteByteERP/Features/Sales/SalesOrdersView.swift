import SwiftUI

struct SalesOrdersView: View {
    @StateObject private var viewModel = SalesViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.salesOrders.isEmpty {
                LoadingView(message: "Loading sales orders...")
            } else if viewModel.salesOrders.isEmpty {
                EmptyStateView(
                    icon: "cart",
                    title: "No Sales Orders",
                    message: "Sales orders will appear here once they are created.",
                    action: { Task { await viewModel.loadSalesOrders() } },
                    actionLabel: "Refresh"
                )
            } else {
                ordersList
            }
        }
        .navigationTitle("Sales Orders")
        .refreshable {
            await viewModel.loadSalesOrders()
            await viewModel.loadCustomers()
        }
        .task {
            await viewModel.loadSalesOrders()
            await viewModel.loadCustomers()
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
                SearchField(placeholder: "Search orders...", text: $viewModel.searchText)
                    .listRowInsets(EdgeInsets(top: 4, leading: 16, bottom: 4, trailing: 16))
                    .listRowSeparator(.hidden)
            }

            Section {
                ForEach(viewModel.filteredOrders) { order in
                    NavigationLink {
                        SalesOrderDetailView(order: order, viewModel: viewModel)
                    } label: {
                        SalesOrderRow(
                            order: order,
                            customerName: viewModel.customerName(for: order.customerId)
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

struct SalesOrderRow: View {
    let order: SalesOrder
    let customerName: String

    var body: some View {
        HStack(spacing: 12) {
            VStack(alignment: .leading, spacing: 6) {
                HStack {
                    Text(order.orderNumber)
                        .font(.subheadline)
                        .fontWeight(.semibold)
                    StatusBadge(status: order.status, size: .small)
                }

                Text(customerName)
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
