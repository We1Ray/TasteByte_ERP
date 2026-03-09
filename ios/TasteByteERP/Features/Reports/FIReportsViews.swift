import SwiftUI

// MARK: - Trial Balance

struct TrialBalanceView: View {
    @StateObject private var viewModel = ReportsViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.trialBalance.isEmpty {
                LoadingView(message: "Loading trial balance...")
            } else if viewModel.trialBalance.isEmpty {
                EmptyStateView(
                    icon: "list.bullet.rectangle",
                    title: "No Data",
                    message: "Trial balance data is not available.",
                    action: { Task { await viewModel.loadTrialBalance() } },
                    actionLabel: "Refresh"
                )
            } else {
                List {
                    Section {
                        ForEach(viewModel.trialBalance) { entry in
                            VStack(alignment: .leading, spacing: 6) {
                                HStack {
                                    Text(entry.accountNumber)
                                        .font(.caption)
                                        .fontWeight(.medium)
                                        .foregroundStyle(.secondary)
                                    Spacer()
                                    Text(entry.balance.currencyFormatted)
                                        .font(.subheadline)
                                        .fontWeight(.bold)
                                }
                                Text(entry.accountName)
                                    .font(.subheadline)
                                HStack {
                                    Text("Debit: \(entry.debit.currencyFormatted)")
                                        .font(.caption)
                                        .foregroundStyle(.secondary)
                                    Spacer()
                                    Text("Credit: \(entry.credit.currencyFormatted)")
                                        .font(.caption)
                                        .foregroundStyle(.secondary)
                                }
                            }
                            .padding(.vertical, 2)
                        }
                    } header: {
                        Text("\(viewModel.trialBalance.count) Accounts")
                    }
                }
                .listStyle(.insetGrouped)
            }
        }
        .navigationTitle("Trial Balance")
        .navigationBarTitleDisplayMode(.inline)
        .refreshable { await viewModel.loadTrialBalance() }
        .task { await viewModel.loadTrialBalance() }
    }
}

// MARK: - Income Statement

struct IncomeStatementView: View {
    @StateObject private var viewModel = ReportsViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.incomeStatement.isEmpty {
                LoadingView(message: "Loading income statement...")
            } else if viewModel.incomeStatement.isEmpty {
                EmptyStateView(
                    icon: "chart.line.uptrend.xyaxis",
                    title: "No Data",
                    message: "Income statement data is not available.",
                    action: { Task { await viewModel.loadIncomeStatement() } },
                    actionLabel: "Refresh"
                )
            } else {
                List {
                    ForEach(viewModel.incomeStatement) { entry in
                        HStack {
                            Text(entry.category)
                                .font(.subheadline)
                            Spacer()
                            Text(entry.amount.currencyFormatted)
                                .font(.subheadline)
                                .fontWeight(.semibold)
                                .foregroundStyle(entry.amount >= 0 ? .erpSuccess : .erpError)
                        }
                    }
                }
                .listStyle(.insetGrouped)
            }
        }
        .navigationTitle("Income Statement")
        .navigationBarTitleDisplayMode(.inline)
        .refreshable { await viewModel.loadIncomeStatement() }
        .task { await viewModel.loadIncomeStatement() }
    }
}

// MARK: - Balance Sheet

struct BalanceSheetView: View {
    @StateObject private var viewModel = ReportsViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.balanceSheet.isEmpty {
                LoadingView(message: "Loading balance sheet...")
            } else if viewModel.balanceSheet.isEmpty {
                EmptyStateView(
                    icon: "scale.3d",
                    title: "No Data",
                    message: "Balance sheet data is not available.",
                    action: { Task { await viewModel.loadBalanceSheet() } },
                    actionLabel: "Refresh"
                )
            } else {
                List {
                    ForEach(viewModel.balanceSheet) { entry in
                        HStack {
                            Text(entry.category)
                                .font(.subheadline)
                            Spacer()
                            Text(entry.amount.currencyFormatted)
                                .font(.subheadline)
                                .fontWeight(.semibold)
                        }
                    }
                }
                .listStyle(.insetGrouped)
            }
        }
        .navigationTitle("Balance Sheet")
        .navigationBarTitleDisplayMode(.inline)
        .refreshable { await viewModel.loadBalanceSheet() }
        .task { await viewModel.loadBalanceSheet() }
    }
}

// MARK: - AR Aging

struct ArAgingView: View {
    @StateObject private var viewModel = ReportsViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.arAging.isEmpty {
                LoadingView(message: "Loading AR aging...")
            } else if viewModel.arAging.isEmpty {
                EmptyStateView(
                    icon: "arrow.down.circle",
                    title: "No Data",
                    message: "AR aging data is not available.",
                    action: { Task { await viewModel.loadArAging() } },
                    actionLabel: "Refresh"
                )
            } else {
                List {
                    ForEach(viewModel.arAging) { entry in
                        VStack(alignment: .leading, spacing: 6) {
                            HStack {
                                Text(entry.customerName)
                                    .font(.subheadline)
                                    .fontWeight(.medium)
                                Spacer()
                                Text(entry.total.currencyFormatted)
                                    .font(.subheadline)
                                    .fontWeight(.bold)
                            }
                            HStack(spacing: 12) {
                                AgingBucket(label: "Current", value: entry.current)
                                AgingBucket(label: "30d", value: entry.days30)
                                AgingBucket(label: "60d", value: entry.days60)
                                AgingBucket(label: "90d+", value: entry.days90Plus)
                            }
                        }
                        .padding(.vertical, 2)
                    }
                }
                .listStyle(.insetGrouped)
            }
        }
        .navigationTitle("AR Aging")
        .navigationBarTitleDisplayMode(.inline)
        .refreshable { await viewModel.loadArAging() }
        .task { await viewModel.loadArAging() }
    }
}

// MARK: - AP Aging

struct ApAgingView: View {
    @StateObject private var viewModel = ReportsViewModel()

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.apAging.isEmpty {
                LoadingView(message: "Loading AP aging...")
            } else if viewModel.apAging.isEmpty {
                EmptyStateView(
                    icon: "arrow.up.circle",
                    title: "No Data",
                    message: "AP aging data is not available.",
                    action: { Task { await viewModel.loadApAging() } },
                    actionLabel: "Refresh"
                )
            } else {
                List {
                    ForEach(viewModel.apAging) { entry in
                        VStack(alignment: .leading, spacing: 6) {
                            HStack {
                                Text(entry.vendorName)
                                    .font(.subheadline)
                                    .fontWeight(.medium)
                                Spacer()
                                Text(entry.total.currencyFormatted)
                                    .font(.subheadline)
                                    .fontWeight(.bold)
                            }
                            HStack(spacing: 12) {
                                AgingBucket(label: "Current", value: entry.current)
                                AgingBucket(label: "30d", value: entry.days30)
                                AgingBucket(label: "60d", value: entry.days60)
                                AgingBucket(label: "90d+", value: entry.days90Plus)
                            }
                        }
                        .padding(.vertical, 2)
                    }
                }
                .listStyle(.insetGrouped)
            }
        }
        .navigationTitle("AP Aging")
        .navigationBarTitleDisplayMode(.inline)
        .refreshable { await viewModel.loadApAging() }
        .task { await viewModel.loadApAging() }
    }
}

// MARK: - Shared Component

struct AgingBucket: View {
    let label: String
    let value: Double

    var body: some View {
        VStack(spacing: 2) {
            Text(label)
                .font(.caption2)
                .foregroundStyle(.secondary)
            Text(value.currencyFormatted)
                .font(.caption2)
                .fontWeight(.medium)
        }
    }
}
