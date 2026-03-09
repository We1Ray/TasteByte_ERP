import SwiftUI

struct ContentView: View {
    @State private var selectedTab = 0

    var body: some View {
        VStack(spacing: 0) {
            OfflineBanner()

            TabView(selection: $selectedTab) {
                DashboardView()
                    .tabItem {
                        Label("Dashboard", systemImage: "chart.bar.fill")
                    }
                    .tag(0)

                NavigationStack {
                    MaterialsListView()
                }
                .tabItem {
                    Label("Materials", systemImage: "cube.box.fill")
                }
                .tag(1)

                NavigationStack {
                    SalesOrdersView()
                }
                .tabItem {
                    Label("Sales", systemImage: "cart.fill")
                }
                .tag(2)

                NavigationStack {
                    AttendanceView()
                }
                .tabItem {
                    Label("HR", systemImage: "person.2.fill")
                }
                .tag(3)

                MoreView()
                    .tabItem {
                        Label("More", systemImage: "ellipsis.circle.fill")
                    }
                    .tag(4)
            }
            .tint(Color.erpPrimary)
        }
    }
}

struct MoreView: View {
    @EnvironmentObject var authManager: AuthManager

    var body: some View {
        NavigationStack {
            List {
                Section("Purchasing") {
                    NavigationLink {
                        PurchaseOrdersView()
                    } label: {
                        Label("Purchase Orders", systemImage: "doc.plaintext.fill")
                    }
                }

                Section("Production Planning") {
                    NavigationLink {
                        ProductionOrdersView()
                    } label: {
                        Label("Production Orders", systemImage: "gearshape.2.fill")
                    }
                }

                Section("Warehouse Management") {
                    NavigationLink {
                        WarehouseListView()
                    } label: {
                        Label("Warehouses", systemImage: "building.2.fill")
                    }

                    NavigationLink {
                        StockCountView()
                    } label: {
                        Label("Stock Count", systemImage: "checklist")
                    }
                }

                Section("Quality Management") {
                    NavigationLink {
                        InspectionListView()
                    } label: {
                        Label("Inspection Lots", systemImage: "checkmark.shield.fill")
                    }
                }

                Section("Reports") {
                    NavigationLink {
                        ReportsView()
                    } label: {
                        Label("All Reports", systemImage: "chart.bar.doc.horizontal")
                    }
                }

                Section("Account") {
                    if let user = authManager.currentUser {
                        HStack {
                            Text("User")
                                .foregroundStyle(.secondary)
                            Spacer()
                            Text(user.displayName ?? user.username)
                        }
                        HStack {
                            Text("Email")
                                .foregroundStyle(.secondary)
                            Spacer()
                            Text(user.email)
                                .foregroundStyle(.secondary)
                        }
                    }

                    Button(role: .destructive) {
                        Task { await authManager.logout() }
                    } label: {
                        Label("Logout", systemImage: "rectangle.portrait.and.arrow.right")
                    }
                }
            }
            .navigationTitle("More")
        }
    }
}
