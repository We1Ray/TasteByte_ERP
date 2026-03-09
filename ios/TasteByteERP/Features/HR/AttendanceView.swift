import SwiftUI

struct AttendanceView: View {
    @StateObject private var viewModel = HRViewModel()

    var body: some View {
        ScrollView {
            VStack(spacing: 24) {
                // Current Time Display
                VStack(spacing: 8) {
                    Text(viewModel.currentTime.formatted(as: .long))
                        .font(.subheadline)
                        .foregroundStyle(.secondary)

                    Text(timeString(from: viewModel.currentTime))
                        .font(.system(size: 56, weight: .light, design: .monospaced))
                        .foregroundStyle(.primary)
                }
                .padding(.top, 20)

                // Status Indicator
                HStack(spacing: 8) {
                    Circle()
                        .fill(viewModel.isClockedIn ? Color.erpSuccess : Color.gray)
                        .frame(width: 10, height: 10)
                    Text(viewModel.isClockedIn ? "Clocked In" : "Not Clocked In")
                        .font(.subheadline)
                        .fontWeight(.medium)
                        .foregroundStyle(viewModel.isClockedIn ? .erpSuccess : .secondary)
                }
                .padding(.horizontal, 16)
                .padding(.vertical, 8)
                .background(
                    (viewModel.isClockedIn ? Color.erpSuccess : Color.gray).opacity(0.1),
                    in: Capsule()
                )

                // Clock In/Out Button
                Button {
                    Task {
                        if viewModel.isClockedIn {
                            await viewModel.clockOut()
                        } else {
                            await viewModel.clockIn()
                        }
                    }
                } label: {
                    VStack(spacing: 8) {
                        if viewModel.isClocking {
                            ProgressView()
                                .tint(.white)
                                .frame(width: 40, height: 40)
                        } else {
                            Image(systemName: viewModel.isClockedIn ? "clock.badge.xmark.fill" : "clock.badge.checkmark.fill")
                                .font(.system(size: 40))
                        }
                        Text(viewModel.isClockedIn ? "Clock Out" : "Clock In")
                            .font(.title3)
                            .fontWeight(.semibold)
                    }
                    .foregroundStyle(.white)
                    .frame(width: 160, height: 160)
                    .background(
                        viewModel.isClockedIn ? Color.erpError : Color.erpSuccess,
                        in: Circle()
                    )
                    .shadow(color: (viewModel.isClockedIn ? Color.erpError : Color.erpSuccess).opacity(0.3),
                            radius: 10, y: 4)
                }
                .disabled(viewModel.isClocking)

                // Success/Error Messages
                if let success = viewModel.successMessage {
                    HStack(spacing: 6) {
                        Image(systemName: "checkmark.circle.fill")
                        Text(success)
                    }
                    .font(.subheadline)
                    .foregroundStyle(.erpSuccess)
                    .padding(.horizontal, 16)
                    .padding(.vertical, 8)
                    .background(Color.erpSuccess.opacity(0.1), in: Capsule())
                }

                if let error = viewModel.errorMessage {
                    HStack(spacing: 6) {
                        Image(systemName: "exclamationmark.triangle.fill")
                        Text(error)
                    }
                    .font(.subheadline)
                    .foregroundStyle(.erpError)
                    .padding(.horizontal, 16)
                    .padding(.vertical, 8)
                    .background(Color.erpError.opacity(0.1), in: Capsule())
                }

                // Today's Record
                ERPCard {
                    Text("Today's Record")
                        .font(.headline)

                    Divider()

                    if let attendance = viewModel.todayAttendance {
                        if let clockIn = attendance.clockIn {
                            ERPInfoRow(
                                label: "Clock In",
                                value: clockIn.formatted(as: .timeOnly),
                                valueColor: .erpSuccess
                            )
                        } else {
                            ERPInfoRow(label: "Clock In", value: "---")
                        }

                        if let clockOut = attendance.clockOut {
                            ERPInfoRow(
                                label: "Clock Out",
                                value: clockOut.formatted(as: .timeOnly),
                                valueColor: .erpPrimary
                            )
                        } else {
                            ERPInfoRow(label: "Clock Out", value: "---")
                        }

                        if let hours = attendance.hoursWorked, hours > 0 {
                            ERPInfoRow(
                                label: "Hours Worked",
                                value: String(format: "%.1fh", hours),
                                valueColor: .erpSuccess
                            )
                        }

                        ERPInfoRow(label: "Status", value: attendance.status.capitalized)
                    } else {
                        Text("No attendance record for today.")
                            .font(.subheadline)
                            .foregroundStyle(.secondary)
                    }
                }
                .padding(.horizontal, 16)

                // Employee List Link
                NavigationLink {
                    EmployeeListView(viewModel: viewModel)
                } label: {
                    HStack {
                        Image(systemName: "person.2.fill")
                            .foregroundStyle(.erpPrimary)
                        Text("View Employee Directory")
                            .fontWeight(.medium)
                        Spacer()
                        Image(systemName: "chevron.right")
                            .font(.caption)
                            .foregroundStyle(.secondary)
                    }
                    .padding(16)
                    .background(Color(uiColor: .secondarySystemGroupedBackground))
                    .clipShape(RoundedRectangle(cornerRadius: 12))
                }
                .buttonStyle(.plain)
                .padding(.horizontal, 16)

                Spacer(minLength: 20)
            }
        }
        .background(Color.erpBackground)
        .navigationTitle("Attendance")
        .onAppear {
            viewModel.startClock()
        }
        .onDisappear {
            viewModel.stopClock()
        }
        .task {
            await viewModel.loadTodayAttendance()
        }
        .refreshable {
            await viewModel.loadTodayAttendance()
        }
    }

    private func timeString(from date: Date) -> String {
        let formatter = DateFormatter()
        formatter.dateFormat = "HH:mm:ss"
        return formatter.string(from: date)
    }
}
