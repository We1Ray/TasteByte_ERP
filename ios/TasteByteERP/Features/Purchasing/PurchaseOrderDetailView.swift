import SwiftUI

struct PurchaseOrderDetailView: View {
    let order: PurchaseOrder
    @ObservedObject var viewModel: PurchaseOrderViewModel

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                // Header Card
                ERPCard {
                    HStack {
                        VStack(alignment: .leading, spacing: 6) {
                            Text(order.orderNumber)
                                .font(.caption)
                                .foregroundStyle(.secondary)
                            Text(viewModel.vendorName(for: order.vendorId))
                                .font(.title2)
                                .fontWeight(.bold)
                        }
                        Spacer()
                        StatusBadge(status: order.status)
                    }
                }
                .padding(.horizontal, 16)

                // Order Details
                ERPCard {
                    Text("Order Details")
                        .font(.headline)

                    Divider()

                    ERPInfoRow(label: "Order Number", value: order.orderNumber)
                    ERPInfoRow(label: "Order Date", value: order.orderDate.formatted(as: .medium))

                    if let deliveryDate = order.expectedDeliveryDate {
                        ERPInfoRow(label: "Expected Delivery", value: deliveryDate.formatted(as: .medium))
                    }

                    ERPInfoRow(label: "Status", value: order.status.capitalized,
                              valueColor: Color.statusColor(for: order.status))
                    ERPInfoRow(label: "Currency", value: order.currency)

                    Divider()

                    HStack {
                        Text("Total Amount")
                            .font(.subheadline)
                            .fontWeight(.semibold)
                        Spacer()
                        Text(order.totalAmount.currencyFormatted)
                            .font(.title3)
                            .fontWeight(.bold)
                            .foregroundStyle(.erpPrimary)
                    }
                }
                .padding(.horizontal, 16)

                // Line Items
                ERPCard {
                    Text("Line Items")
                        .font(.headline)

                    Divider()

                    if viewModel.selectedOrderItems.isEmpty {
                        HStack {
                            Spacer()
                            ProgressView()
                                .padding()
                            Spacer()
                        }
                    } else {
                        ForEach(viewModel.selectedOrderItems) { item in
                            VStack(spacing: 8) {
                                HStack {
                                    Text("Line \(item.lineNumber)")
                                        .font(.caption)
                                        .fontWeight(.medium)
                                        .foregroundStyle(.secondary)
                                    Spacer()
                                }

                                HStack {
                                    VStack(alignment: .leading, spacing: 2) {
                                        Text("Qty: \(item.quantity.quantityFormatted)")
                                            .font(.subheadline)
                                        Text("Unit Price: \(item.unitPrice.currencyFormatted)")
                                            .font(.caption)
                                            .foregroundStyle(.secondary)
                                    }
                                    Spacer()
                                    VStack(alignment: .trailing, spacing: 2) {
                                        Text(item.totalPrice.currencyFormatted)
                                            .font(.subheadline)
                                            .fontWeight(.semibold)
                                        if item.receivedQuantity > 0 {
                                            Text("Received: \(item.receivedQuantity.quantityFormatted)")
                                                .font(.caption)
                                                .foregroundStyle(.erpSuccess)
                                        }
                                    }
                                }

                                if item.id != viewModel.selectedOrderItems.last?.id {
                                    Divider()
                                }
                            }
                        }
                    }
                }
                .padding(.horizontal, 16)

                // Notes
                if let notes = order.notes, !notes.isEmpty {
                    ERPCard {
                        Text("Notes")
                            .font(.headline)
                        Divider()
                        Text(notes)
                            .font(.subheadline)
                            .foregroundStyle(.secondary)
                    }
                    .padding(.horizontal, 16)
                }

                // Receive Button
                if order.status.lowercased() == "released" || order.status.lowercased() == "open" || order.status.lowercased() == "confirmed" {
                    Button {
                        Task {
                            await viewModel.receiveOrder(orderId: order.id)
                        }
                    } label: {
                        HStack {
                            if viewModel.isLoading {
                                ProgressView()
                                    .tint(.white)
                            }
                            Text("Receive Goods")
                                .fontWeight(.semibold)
                        }
                        .frame(maxWidth: .infinity)
                        .padding(.vertical, 14)
                    }
                    .buttonStyle(.borderedProminent)
                    .tint(.erpSuccess)
                    .disabled(viewModel.isLoading)
                    .padding(.horizontal, 16)
                }

                Spacer(minLength: 20)
            }
            .padding(.top, 12)
        }
        .background(Color.erpBackground)
        .navigationTitle("PO Detail")
        .navigationBarTitleDisplayMode(.inline)
        .task {
            await viewModel.loadOrderItems(orderId: order.id)
        }
        .alert("Error", isPresented: .constant(viewModel.errorMessage != nil)) {
            Button("OK") { viewModel.errorMessage = nil }
        } message: {
            Text(viewModel.errorMessage ?? "")
        }
    }
}
