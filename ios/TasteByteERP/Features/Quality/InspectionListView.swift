import SwiftUI

struct InspectionListView: View {
    @StateObject private var viewModel = QualityViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.inspectionLots.isEmpty {
                LoadingView(message: "Loading inspection lots...")
            } else if viewModel.inspectionLots.isEmpty {
                EmptyStateView(
                    icon: "checkmark.shield",
                    title: "No Inspection Lots",
                    message: "Quality inspection lots will appear here once created.",
                    action: { Task { await viewModel.loadInspectionLots() } },
                    actionLabel: "Refresh"
                )
            } else {
                inspectionList
            }
        }
        .navigationTitle("Inspection Lots")
        .navigationBarTitleDisplayMode(.inline)
        .refreshable {
            await viewModel.loadInspectionLots()
        }
        .task {
            await viewModel.loadInspectionLots()
        }
        .alert("Error", isPresented: .constant(viewModel.errorMessage != nil)) {
            Button("OK") { viewModel.errorMessage = nil }
        } message: {
            Text(viewModel.errorMessage ?? "")
        }
    }

    private var inspectionList: some View {
        List {
            Section {
                SearchField(placeholder: "Search inspections...", text: $viewModel.searchText)
                    .listRowInsets(EdgeInsets(top: 4, leading: 16, bottom: 4, trailing: 16))
                    .listRowSeparator(.hidden)
            }

            Section {
                ForEach(viewModel.filteredLots) { lot in
                    NavigationLink {
                        InspectionFormView(lot: lot, viewModel: viewModel)
                    } label: {
                        InspectionLotRow(lot: lot)
                    }
                }
            } header: {
                Text("\(viewModel.filteredLots.count) Lots")
            }
        }
        .listStyle(.insetGrouped)
    }
}

struct InspectionLotRow: View {
    let lot: InspectionLot

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: "checkmark.shield.fill")
                .font(.title3)
                .foregroundStyle(Color.statusColor(for: lot.status))
                .frame(width: 40, height: 40)
                .background(Color.statusColor(for: lot.status).opacity(0.1), in: RoundedRectangle(cornerRadius: 8))

            VStack(alignment: .leading, spacing: 4) {
                HStack {
                    Text(lot.lotNumber)
                        .font(.subheadline)
                        .fontWeight(.semibold)
                    StatusBadge(status: lot.status, size: .small)
                }

                if let name = lot.materialName {
                    Text(name)
                        .font(.caption)
                        .foregroundStyle(.secondary)
                        .lineLimit(1)
                }

                HStack(spacing: 8) {
                    Text(lot.inspectionType.capitalized)
                        .font(.caption2)
                        .foregroundStyle(.erpPrimary)
                    Text("Qty: \(lot.quantity.quantityFormatted)")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
            }

            Spacer()

            Text(lot.createdAt.formatted(as: .dayMonth))
                .font(.caption2)
                .foregroundStyle(.secondary)
        }
        .padding(.vertical, 4)
    }
}
