import Foundation

struct Attendance: Codable, Identifiable {
    let id: String
    let employeeId: String
    let date: Date
    let clockIn: Date?
    let clockOut: Date?
    let status: String
    let hoursWorked: Double?
    let notes: String?
}

struct ClockInResponse: Codable {
    let id: String
    let employeeId: String
    let clockIn: Date
    let status: String
}

struct ClockOutResponse: Codable {
    let id: String
    let employeeId: String
    let clockIn: Date
    let clockOut: Date
    let hoursWorked: Double
    let status: String
}
