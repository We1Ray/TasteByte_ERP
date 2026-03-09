import SwiftUI

struct ReportsView: View {
    var body: some View {
        List {
            Section("Financial (FI)") {
                NavigationLink {
                    TrialBalanceView()
                } label: {
                    Label("Trial Balance", systemImage: "list.bullet.rectangle")
                }
                NavigationLink {
                    IncomeStatementView()
                } label: {
                    Label("Income Statement", systemImage: "chart.line.uptrend.xyaxis")
                }
                NavigationLink {
                    BalanceSheetView()
                } label: {
                    Label("Balance Sheet", systemImage: "scale.3d")
                }
                NavigationLink {
                    ArAgingView()
                } label: {
                    Label("AR Aging", systemImage: "arrow.down.circle")
                }
                NavigationLink {
                    ApAgingView()
                } label: {
                    Label("AP Aging", systemImage: "arrow.up.circle")
                }
            }

            Section("Materials Management (MM)") {
                NavigationLink {
                    StockValuationView()
                } label: {
                    Label("Stock Valuation", systemImage: "cube.box")
                }
                NavigationLink {
                    MovementSummaryView()
                } label: {
                    Label("Movement Summary", systemImage: "arrow.left.arrow.right")
                }
                NavigationLink {
                    SlowMovingView()
                } label: {
                    Label("Slow-Moving Items", systemImage: "tortoise")
                }
            }

            Section("Sales & Distribution (SD)") {
                NavigationLink {
                    SalesSummaryView()
                } label: {
                    Label("Sales Summary", systemImage: "chart.bar")
                }
                NavigationLink {
                    OrderFulfillmentView()
                } label: {
                    Label("Order Fulfillment", systemImage: "checkmark.circle")
                }
                NavigationLink {
                    TopCustomersView()
                } label: {
                    Label("Top Customers", systemImage: "person.3")
                }
            }
        }
        .listStyle(.insetGrouped)
        .navigationTitle("Reports")
    }
}
