class Attendance {
  final String id;
  final String employeeId;
  final String employeeName;
  final DateTime date;
  final DateTime? clockIn;
  final DateTime? clockOut;
  final String status;
  final double? hoursWorked;
  final String? notes;

  const Attendance({
    required this.id,
    required this.employeeId,
    required this.employeeName,
    required this.date,
    this.clockIn,
    this.clockOut,
    this.status = 'absent',
    this.hoursWorked,
    this.notes,
  });

  bool get isClockedIn => clockIn != null && clockOut == null;
  bool get isComplete => clockIn != null && clockOut != null;

  factory Attendance.fromJson(Map<String, dynamic> json) {
    return Attendance(
      id: json['id']?.toString() ?? '',
      employeeId: json['employee_id']?.toString() ?? '',
      employeeName: json['employee_name'] as String? ?? '',
      date: DateTime.tryParse(json['date'] as String? ?? '') ?? DateTime.now(),
      clockIn: json['clock_in'] != null
          ? DateTime.tryParse(json['clock_in'] as String)
          : null,
      clockOut: json['clock_out'] != null
          ? DateTime.tryParse(json['clock_out'] as String)
          : null,
      status: json['status'] as String? ?? 'absent',
      hoursWorked: (json['hours_worked'] as num?)?.toDouble(),
      notes: json['notes'] as String?,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'employee_id': employeeId,
      'employee_name': employeeName,
      'date': date.toIso8601String(),
      'clock_in': clockIn?.toIso8601String(),
      'clock_out': clockOut?.toIso8601String(),
      'status': status,
      'hours_worked': hoursWorked,
      'notes': notes,
    };
  }
}
