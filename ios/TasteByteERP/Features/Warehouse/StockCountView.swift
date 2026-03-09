import SwiftUI

struct StockCountView: View {
    @StateObject private var viewModel = WarehouseViewModel()
    @State private var selectedMaterialId: String?
    @State private var countedQuantity = ""
    @State private var showingSubmitAlert = false

    var body: some View {
        ScrollView {
            VStack(spacing: 20) {
                // Warehouse Selector
                ERPCard {
                    Text("Select Warehouse")
                        .font(.headline)

                    Divider()

                    if viewModel.warehouses.isEmpty {
                        HStack {
                            Spacer()
                            ProgressView()
                                .padding()
                            Spacer()
                        }
                    } else {
                        Picker("Warehouse", selection: $viewModel.selectedWarehouseId) {
                            Text("Select...").tag(nil as String?)
                            ForEach(viewModel.warehouses) { warehouse in
                                Text("\(warehouse.warehouseNumber) - \(warehouse.name)")
                                    .tag(warehouse.id as String?)
                            }
                        }
                        .pickerStyle(.menu)
                    }
                }
                .padding(.horizontal, 16)

                // Material List for Count
                if viewModel.selectedWarehouseId != nil {
                    ERPCard {
                        Text("Count Entry")
                            .font(.headline)

                        Divider()

                        // Material Picker
                        Picker("Material", selection: $selectedMaterialId) {
                            Text("Select material...").tag(nil as String?)
                            ForEach(viewModel.materials) { material in
                                Text("\(material.materialNumber) - \(material.name)")
                                    .tag(material.id as String?)
                            }
                        }
                        .pickerStyle(.menu)

                        // Counted Quantity
                        VStack(alignment: .leading, spacing: 6) {
                            Text("Counted Quantity")
                                .font(.subheadline)
                                .foregroundStyle(.secondary)

                            TextField("Enter counted quantity", text: $countedQuantity)
                                .textFieldStyle(.plain)
                                .padding(12)
                                .background(
                                    RoundedRectangle(cornerRadius: 10)
                                        .fill(Color(uiColor: .tertiarySystemFill))
                                )
                                .keyboardType(.decimalPad)
                        }

                        // Submit Button
                        Button {
                            showingSubmitAlert = true
                        } label: {
                            HStack {
                                if viewModel.isSubmitting {
                                    ProgressView()
                                        .tint(.white)
                                } else {
                                    Image(systemName: "checkmark.circle.fill")
                                    Text("Submit Count")
                                        .fontWeight(.semibold)
                                }
                            }
                            .frame(maxWidth: .infinity)
                        }
                        .buttonStyle(.borderedProminent)
                        .tint(.erpPrimary)
                        .disabled(selectedMaterialId == nil || countedQuantity.isEmpty || viewModel.isSubmitting)
                    }
                    .padding(.horizontal, 16)

                    // Success/Error Messages
                    if let success = viewModel.successMessage {
                        HStack(spacing: 6) {
                            Image(systemName: "checkmark.circle.fill")
                            Text(success)
                        }
                        .font(.subheadline)
                        .foregroundStyle(.erpSuccess)
                        .padding(.horizontal, 16)
                    }

                    if let error = viewModel.errorMessage {
                        HStack(spacing: 6) {
                            Image(systemName: "exclamationmark.triangle.fill")
                            Text(error)
                        }
                        .font(.subheadline)
                        .foregroundStyle(.erpError)
                        .padding(.horizontal, 16)
                    }

                    // Existing Stock Counts
                    if !viewModel.stockCounts.isEmpty {
                        ERPCard {
                            Text("Recent Counts")
                                .font(.headline)

                            Divider()

                            ForEach(viewModel.stockCounts) { count in
                                VStack(spacing: 6) {
                                    HStack {
                                        Text(viewModel.materialName(for: count.materialId))
                                            .font(.subheadline)
                                            .fontWeight(.medium)
                                        Spacer()
                                        StatusBadge(status: count.status, size: .small)
                                    }

                                    HStack {
                                        VStack(alignment: .leading, spacing: 2) {
                                            Text("Book: \(count.bookQuantity.quantityFormatted)")
                                                .font(.caption)
                                                .foregroundStyle(.secondary)
                                            if let counted = count.countedQuantity {
                                                Text("Counted: \(counted.quantityFormatted)")
                                                    .font(.caption)
                                                    .foregroundStyle(.secondary)
                                            }
                                        }
                                        Spacer()
                                        if let diff = count.difference {
                                            Text(diff >= 0 ? "+\(diff.quantityFormatted)" : diff.quantityFormatted)
                                                .font(.caption)
                                                .fontWeight(.semibold)
                                                .foregroundStyle(diff == 0 ? .erpSuccess : .erpWarning)
                                        }
                                    }

                                    if count.id != viewModel.stockCounts.last?.id {
                                        Divider()
                                    }
                                }
                            }
                        }
                        .padding(.horizontal, 16)
                    }
                }

                Spacer(minLength: 20)
            }
            .padding(.top, 12)
        }
        .background(Color.erpBackground)
        .navigationTitle("Stock Count")
        .navigationBarTitleDisplayMode(.inline)
        .task {
            await viewModel.loadWarehouses()
            await viewModel.loadMaterials()
        }
        .onChange(of: viewModel.selectedWarehouseId) { _, newValue in
            if let warehouseId = newValue {
                Task { await viewModel.loadStockCounts(warehouseId: warehouseId) }
            }
        }
        .alert("Confirm Stock Count", isPresented: $showingSubmitAlert) {
            Button("Cancel", role: .cancel) {}
            Button("Submit") {
                guard let warehouseId = viewModel.selectedWarehouseId,
                      let materialId = selectedMaterialId,
                      let quantity = Double(countedQuantity) else { return }
                Task {
                    await viewModel.submitStockCount(
                        warehouseId: warehouseId,
                        materialId: materialId,
                        countedQuantity: quantity
                    )
                    countedQuantity = ""
                    selectedMaterialId = nil
                }
            }
        } message: {
            Text("Submit a count of \(countedQuantity) for the selected material?")
        }
    }
}
