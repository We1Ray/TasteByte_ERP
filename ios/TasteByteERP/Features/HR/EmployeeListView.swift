import SwiftUI

struct EmployeeListView: View {
    @ObservedObject var viewModel: HRViewModel

    var body: some View {
        Group {
            if viewModel.isLoading && viewModel.employees.isEmpty {
                LoadingView(message: "Loading employees...")
            } else if viewModel.employees.isEmpty {
                EmptyStateView(
                    icon: "person.2",
                    title: "No Employees",
                    message: "Employee records will appear here.",
                    action: { Task { await viewModel.loadEmployees() } },
                    actionLabel: "Refresh"
                )
            } else {
                employeeList
            }
        }
        .navigationTitle("Employees")
        .navigationBarTitleDisplayMode(.inline)
        .refreshable {
            await viewModel.loadEmployees()
        }
        .task {
            if viewModel.employees.isEmpty {
                await viewModel.loadEmployees()
            }
        }
    }

    private var employeeList: some View {
        List {
            Section {
                SearchField(placeholder: "Search employees...", text: $viewModel.searchText)
                    .listRowInsets(EdgeInsets(top: 4, leading: 16, bottom: 4, trailing: 16))
                    .listRowSeparator(.hidden)
            }

            Section {
                ForEach(viewModel.filteredEmployees) { employee in
                    EmployeeRow(employee: employee)
                }
            } header: {
                Text("\(viewModel.filteredEmployees.count) Employees")
            }
        }
        .listStyle(.insetGrouped)
    }
}

struct EmployeeRow: View {
    let employee: Employee

    var body: some View {
        HStack(spacing: 12) {
            Text(employee.fullName.prefix(2).uppercased())
                .font(.subheadline)
                .fontWeight(.bold)
                .foregroundStyle(.white)
                .frame(width: 40, height: 40)
                .background(Color.erpPrimary, in: Circle())

            VStack(alignment: .leading, spacing: 4) {
                HStack {
                    Text(employee.fullName)
                        .font(.subheadline)
                        .fontWeight(.medium)
                    if !employee.isActive {
                        StatusBadge(status: "Inactive", size: .small)
                    }
                }

                HStack(spacing: 8) {
                    Text(employee.employeeNumber)
                        .font(.caption)
                        .foregroundStyle(.secondary)

                    if let dept = employee.department {
                        Text(dept)
                            .font(.caption)
                            .foregroundStyle(.erpPrimary)
                    }
                }
            }

            Spacer()

            if let position = employee.position {
                Text(position)
                    .font(.caption)
                    .foregroundStyle(.secondary)
                    .lineLimit(1)
            }
        }
        .padding(.vertical, 4)
    }
}
