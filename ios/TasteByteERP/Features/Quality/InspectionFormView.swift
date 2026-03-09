import SwiftUI

struct InspectionFormView: View {
    let lot: InspectionLot
    @ObservedObject var viewModel: QualityViewModel
    @State private var resultEntries: [ResultEntry] = []
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        ScrollView {
            VStack(spacing: 16) {
                // Lot Details
                ERPCard {
                    HStack {
                        VStack(alignment: .leading, spacing: 6) {
                            Text(lot.lotNumber)
                                .font(.caption)
                                .foregroundStyle(.secondary)
                            Text(lot.materialName ?? "Inspection Lot")
                                .font(.title3)
                                .fontWeight(.bold)
                        }
                        Spacer()
                        StatusBadge(status: lot.status)
                    }

                    Divider()

                    ERPInfoRow(label: "Type", value: lot.inspectionType.capitalized)
                    ERPInfoRow(label: "Quantity", value: lot.quantity.quantityFormatted)
                    ERPInfoRow(label: "Created", value: lot.createdAt.formatted(as: .dateTime))
                }
                .padding(.horizontal, 16)

                // Characteristics
                ERPCard {
                    Text("Inspection Characteristics")
                        .font(.headline)

                    Divider()

                    if viewModel.characteristics.isEmpty {
                        HStack {
                            Spacer()
                            VStack(spacing: 8) {
                                ProgressView()
                                Text("Loading characteristics...")
                                    .font(.caption)
                                    .foregroundStyle(.secondary)
                            }
                            .padding()
                            Spacer()
                        }
                    } else {
                        ForEach(Array(resultEntries.enumerated()), id: \.offset) { index, entry in
                            VStack(alignment: .leading, spacing: 8) {
                                Text(entry.characteristicName)
                                    .font(.subheadline)
                                    .fontWeight(.medium)

                                if let target = entry.targetValue {
                                    HStack(spacing: 12) {
                                        Text("Target: \(target)")
                                            .font(.caption)
                                            .foregroundStyle(.secondary)
                                        if let lower = entry.lowerLimit, let upper = entry.upperLimit {
                                            Text("Range: \(lower.quantityFormatted) - \(upper.quantityFormatted)")
                                                .font(.caption)
                                                .foregroundStyle(.secondary)
                                        }
                                    }
                                }

                                HStack(spacing: 12) {
                                    VStack(alignment: .leading, spacing: 4) {
                                        Text("Actual Value")
                                            .font(.caption)
                                            .foregroundStyle(.secondary)

                                        TextField("Enter value", text: $resultEntries[index].actualValue)
                                            .textFieldStyle(.plain)
                                            .padding(10)
                                            .background(
                                                RoundedRectangle(cornerRadius: 8)
                                                    .fill(Color(uiColor: .tertiarySystemFill))
                                            )
                                    }

                                    VStack(alignment: .leading, spacing: 4) {
                                        Text("Conforming")
                                            .font(.caption)
                                            .foregroundStyle(.secondary)

                                        Toggle("", isOn: $resultEntries[index].isConforming)
                                            .labelsHidden()
                                            .tint(.erpSuccess)
                                    }
                                    .frame(width: 80)
                                }

                                if index < resultEntries.count - 1 {
                                    Divider()
                                        .padding(.top, 4)
                                }
                            }
                        }
                    }
                }
                .padding(.horizontal, 16)

                // Submit Button
                if !resultEntries.isEmpty {
                    Button {
                        submitResults()
                    } label: {
                        HStack {
                            if viewModel.isSubmitting {
                                ProgressView()
                                    .tint(.white)
                            } else {
                                Image(systemName: "checkmark.circle.fill")
                                Text("Submit Results")
                                    .fontWeight(.semibold)
                            }
                        }
                        .frame(maxWidth: .infinity)
                    }
                    .buttonStyle(.borderedProminent)
                    .tint(.erpPrimary)
                    .controlSize(.large)
                    .disabled(viewModel.isSubmitting || !allFieldsFilled)
                    .padding(.horizontal, 16)
                }

                // Success/Error
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

                Spacer(minLength: 20)
            }
            .padding(.top, 12)
        }
        .background(Color.erpBackground)
        .navigationTitle("Inspection Form")
        .navigationBarTitleDisplayMode(.inline)
        .task {
            await viewModel.loadCharacteristics(lotId: lot.id)
            buildResultEntries()
        }
    }

    private var allFieldsFilled: Bool {
        resultEntries.allSatisfy { !$0.actualValue.trimmingCharacters(in: .whitespaces).isEmpty }
    }

    private func buildResultEntries() {
        resultEntries = viewModel.characteristics.map { char in
            ResultEntry(
                characteristicId: char.id,
                characteristicName: char.name,
                targetValue: char.targetValue,
                lowerLimit: char.lowerLimit,
                upperLimit: char.upperLimit,
                actualValue: "",
                isConforming: true
            )
        }
    }

    private func submitResults() {
        let results = resultEntries.map { entry in
            CreateInspectionResult(
                characteristicId: entry.characteristicId,
                actualValue: entry.actualValue,
                isConforming: entry.isConforming
            )
        }
        Task {
            await viewModel.submitResults(lotId: lot.id, results: results)
        }
    }
}

struct ResultEntry {
    let characteristicId: String
    let characteristicName: String
    let targetValue: String?
    let lowerLimit: Double?
    let upperLimit: Double?
    var actualValue: String
    var isConforming: Bool
}
