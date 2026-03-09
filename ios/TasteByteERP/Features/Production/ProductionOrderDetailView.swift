import SwiftUI

struct ProductionOrderDetailView: View {
    let order: ProductionOrder
    @ObservedObject var viewModel: ProductionViewModel

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
                            Text(viewModel.materialName(for: order.materialId))
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
                    Text("Production Details")
                        .font(.headline)

                    Divider()

                    ERPInfoRow(label: "Order Number", value: order.orderNumber)
                    ERPInfoRow(label: "Quantity", value: order.quantity.quantityFormatted)
                    ERPInfoRow(label: "Status", value: order.status.replacingOccurrences(of: "_", with: " ").capitalized,
                              valueColor: Color.statusColor(for: order.status))

                    if let startDate = order.plannedStartDate {
                        ERPInfoRow(label: "Planned Start", value: startDate.formatted(as: .medium))
                    }
                    if let endDate = order.plannedEndDate {
                        ERPInfoRow(label: "Planned End", value: endDate.formatted(as: .medium))
                    }
                    if let actualStart = order.actualStartDate {
                        ERPInfoRow(label: "Actual Start", value: actualStart.formatted(as: .medium))
                    }
                    if let actualEnd = order.actualEndDate {
                        ERPInfoRow(label: "Actual End", value: actualEnd.formatted(as: .medium))
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

                // Status Action Buttons
                VStack(spacing: 10) {
                    ForEach(availableTransitions, id: \.self) { status in
                        Button {
                            Task {
                                await viewModel.updateStatus(orderId: order.id, newStatus: status)
                            }
                        } label: {
                            HStack {
                                if viewModel.isLoading {
                                    ProgressView()
                                        .tint(.white)
                                }
                                Text(buttonLabel(for: status))
                                    .fontWeight(.semibold)
                            }
                            .frame(maxWidth: .infinity)
                            .padding(.vertical, 14)
                        }
                        .buttonStyle(.borderedProminent)
                        .tint(buttonColor(for: status))
                        .disabled(viewModel.isLoading)
                    }
                }
                .padding(.horizontal, 16)

                Spacer(minLength: 20)
            }
            .padding(.top, 12)
        }
        .background(Color.erpBackground)
        .navigationTitle("Production Detail")
        .navigationBarTitleDisplayMode(.inline)
        .alert("Error", isPresented: .constant(viewModel.errorMessage != nil)) {
            Button("OK") { viewModel.errorMessage = nil }
        } message: {
            Text(viewModel.errorMessage ?? "")
        }
    }

    private var availableTransitions: [String] {
        switch order.status.lowercased() {
        case "created", "draft":
            return ["released"]
        case "released":
            return ["in_progress"]
        case "in_progress":
            return ["completed"]
        default:
            return []
        }
    }

    private func buttonLabel(for status: String) -> String {
        switch status {
        case "released": return "Release Order"
        case "in_progress": return "Start Production"
        case "completed": return "Complete Production"
        default: return "Update to \(status.capitalized)"
        }
    }

    private func buttonColor(for status: String) -> Color {
        switch status {
        case "released": return .erpPrimary
        case "in_progress": return .erpAccent
        case "completed": return .erpSuccess
        default: return .erpPrimary
        }
    }
}
