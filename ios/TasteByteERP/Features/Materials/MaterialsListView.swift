import SwiftUI

struct MaterialsListView: View {
    @StateObject private var viewModel = MaterialsViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.materials.isEmpty {
                LoadingView(message: "Loading materials...")
            } else if viewModel.materials.isEmpty {
                EmptyStateView(
                    icon: "cube.box",
                    title: "No Materials",
                    message: "No materials found. Materials will appear here once created in the system.",
                    action: { Task { await viewModel.loadMaterials() } },
                    actionLabel: "Refresh"
                )
            } else {
                materialsList
            }
        }
        .navigationTitle("Materials")
        .toolbar {
            ToolbarItem(placement: .topBarTrailing) {
                NavigationLink {
                    StockOverviewView(viewModel: viewModel)
                } label: {
                    Image(systemName: "chart.bar.doc.horizontal")
                }
            }
        }
        .refreshable {
            await viewModel.loadMaterials()
        }
        .task {
            await viewModel.loadMaterials()
            await viewModel.loadPlantStock()
        }
        .alert("Error", isPresented: .constant(viewModel.errorMessage != nil)) {
            Button("OK") { viewModel.errorMessage = nil }
        } message: {
            Text(viewModel.errorMessage ?? "")
        }
    }

    private var materialsList: some View {
        List {
            Section {
                SearchField(placeholder: "Search materials...", text: $viewModel.searchText)
                    .listRowInsets(EdgeInsets(top: 4, leading: 16, bottom: 4, trailing: 16))
                    .listRowSeparator(.hidden)
            }

            Section {
                ForEach(viewModel.filteredMaterials) { material in
                    NavigationLink {
                        MaterialDetailView(material: material, viewModel: viewModel)
                    } label: {
                        MaterialRow(material: material, stock: viewModel.stockForMaterial(material.id))
                    }
                }
            } header: {
                Text("\(viewModel.filteredMaterials.count) Materials")
            }
        }
        .listStyle(.insetGrouped)
    }
}

struct MaterialRow: View {
    let material: Material
    let stock: PlantStock?

    var body: some View {
        HStack(spacing: 12) {
            Image(systemName: materialIcon)
                .font(.title3)
                .foregroundStyle(.erpPrimary)
                .frame(width: 40, height: 40)
                .background(Color.erpPrimary.opacity(0.1), in: RoundedRectangle(cornerRadius: 8))

            VStack(alignment: .leading, spacing: 4) {
                Text(material.name)
                    .font(.subheadline)
                    .fontWeight(.medium)
                    .lineLimit(1)

                HStack(spacing: 8) {
                    Text(material.materialNumber)
                        .font(.caption)
                        .foregroundStyle(.secondary)

                    StatusBadge(status: material.materialType, size: .small)
                }
            }

            Spacer()

            if let stock = stock {
                VStack(alignment: .trailing, spacing: 2) {
                    Text(stock.quantity.quantityFormatted)
                        .font(.subheadline)
                        .fontWeight(.semibold)
                    Text("In Stock")
                        .font(.caption2)
                        .foregroundStyle(.secondary)
                }
            }

            if !material.isActive {
                Image(systemName: "xmark.circle.fill")
                    .foregroundStyle(.erpError)
                    .font(.caption)
            }
        }
        .padding(.vertical, 4)
    }

    private var materialIcon: String {
        switch material.materialType.lowercased() {
        case "raw": return "leaf.fill"
        case "finished": return "shippingbox.fill"
        case "semi_finished": return "gearshape.fill"
        case "packaging": return "archivebox.fill"
        default: return "cube.fill"
        }
    }
}
