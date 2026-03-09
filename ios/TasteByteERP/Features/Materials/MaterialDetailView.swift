import SwiftUI

struct MaterialDetailView: View {
    let material: Material
    @ObservedObject var viewModel: MaterialsViewModel

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                // Header Card
                ERPCard {
                    HStack {
                        VStack(alignment: .leading, spacing: 6) {
                            Text(material.materialNumber)
                                .font(.caption)
                                .foregroundStyle(.secondary)
                            Text(material.name)
                                .font(.title2)
                                .fontWeight(.bold)
                        }
                        Spacer()
                        StatusBadge(status: material.isActive ? "Active" : "Inactive")
                    }

                    if let description = material.description {
                        Text(description)
                            .font(.subheadline)
                            .foregroundStyle(.secondary)
                    }
                }
                .padding(.horizontal, 16)

                // General Data
                ERPCard {
                    Text("General Data")
                        .font(.headline)

                    Divider()

                    ERPInfoRow(label: "Material Number", value: material.materialNumber)
                    ERPInfoRow(label: "Material Type", value: material.materialType.capitalized)
                    ERPInfoRow(label: "Status", value: material.isActive ? "Active" : "Inactive",
                              valueColor: material.isActive ? .erpSuccess : .erpError)

                    if let weight = material.weight {
                        ERPInfoRow(
                            label: "Weight",
                            value: "\(weight.quantityFormatted) \(material.weightUom ?? "")"
                        )
                    }

                    ERPInfoRow(
                        label: "Created",
                        value: material.createdAt.formatted(as: .dateTime)
                    )
                    ERPInfoRow(
                        label: "Last Updated",
                        value: material.updatedAt.formatted(as: .dateTime)
                    )
                }
                .padding(.horizontal, 16)

                // Stock Information
                if let stock = viewModel.stockForMaterial(material.id) {
                    ERPCard {
                        Text("Stock Information")
                            .font(.headline)

                        Divider()

                        ERPInfoRow(
                            label: "Available Quantity",
                            value: stock.quantity.quantityFormatted
                        )
                        ERPInfoRow(
                            label: "Reserved Quantity",
                            value: stock.reservedQuantity.quantityFormatted
                        )
                        ERPInfoRow(
                            label: "Unrestricted",
                            value: (stock.quantity - stock.reservedQuantity).quantityFormatted,
                            valueColor: .erpSuccess
                        )

                        if let lastCount = stock.lastCountDate {
                            ERPInfoRow(
                                label: "Last Count",
                                value: lastCount.formatted(as: .medium)
                            )
                        }
                    }
                    .padding(.horizontal, 16)
                }

                Spacer(minLength: 20)
            }
            .padding(.top, 12)
        }
        .background(Color.erpBackground)
        .navigationTitle("Material Detail")
        .navigationBarTitleDisplayMode(.inline)
    }
}
