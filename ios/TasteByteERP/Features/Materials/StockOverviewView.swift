import SwiftUI

struct StockOverviewItem: Identifiable {
    var id: String { stock.id }
    let stock: PlantStock
    let materialName: String
    let materialNumber: String
}

struct StockOverviewView: View {
    @ObservedObject var viewModel: MaterialsViewModel

    var body: some View {
        Group {
            if viewModel.plantStocks.isEmpty {
                EmptyStateView(
                    icon: "chart.bar.doc.horizontal",
                    title: "No Stock Data",
                    message: "Stock information is not available yet."
                )
            } else {
                List {
                    Section {
                        ForEach(stockItems) { item in
                            StockOverviewRow(item: item)
                        }
                    } header: {
                        Text("\(stockItems.count) Stock Entries")
                    }
                }
                .listStyle(.insetGrouped)
            }
        }
        .navigationTitle("Stock Overview")
        .navigationBarTitleDisplayMode(.inline)
        .task {
            if viewModel.plantStocks.isEmpty {
                await viewModel.loadPlantStock()
            }
        }
    }

    private var stockItems: [StockOverviewItem] {
        viewModel.plantStocks.map { stock in
            let material = viewModel.materials.first { $0.id == stock.materialId }
            return StockOverviewItem(
                stock: stock,
                materialName: material?.name ?? "Unknown Material",
                materialNumber: material?.materialNumber ?? "---"
            )
        }
    }
}

struct StockOverviewRow: View {
    let item: StockOverviewItem

    var body: some View {
        HStack(spacing: 12) {
            VStack(alignment: .leading, spacing: 4) {
                Text(item.materialName)
                    .font(.subheadline)
                    .fontWeight(.medium)
                Text(item.materialNumber)
                    .font(.caption)
                    .foregroundStyle(.secondary)
            }

            Spacer()

            VStack(alignment: .trailing, spacing: 4) {
                Text(item.stock.quantity.quantityFormatted)
                    .font(.subheadline)
                    .fontWeight(.bold)
                    .foregroundStyle(item.stock.quantity > 0 ? Color.primary : Color.erpError)

                if item.stock.reservedQuantity > 0 {
                    Text("\(item.stock.reservedQuantity.quantityFormatted) reserved")
                        .font(.caption2)
                        .foregroundStyle(Color.erpWarning)
                }
            }
        }
        .padding(.vertical, 4)
    }
}
