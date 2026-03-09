class Employee {
  final String id;
  final String employeeNumber;
  final String firstName;
  final String lastName;
  final String email;
  final String department;
  final String position;
  final String status;
  final DateTime? hireDate;
  final String? phoneNumber;

  const Employee({
    required this.id,
    required this.employeeNumber,
    required this.firstName,
    required this.lastName,
    required this.email,
    required this.department,
    required this.position,
    this.status = 'active',
    this.hireDate,
    this.phoneNumber,
  });

  String get fullName => '$firstName $lastName';

  factory Employee.fromJson(Map<String, dynamic> json) {
    return Employee(
      id: json['id']?.toString() ?? '',
      employeeNumber: json['employee_number'] as String? ?? '',
      firstName: json['first_name'] as String? ?? '',
      lastName: json['last_name'] as String? ?? '',
      email: json['email'] as String? ?? '',
      department: json['department'] as String? ?? '',
      position: json['position'] as String? ?? '',
      status: json['status'] as String? ?? 'active',
      hireDate: json['hire_date'] != null
          ? DateTime.tryParse(json['hire_date'] as String)
          : null,
      phoneNumber: json['phone_number'] as String?,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'employee_number': employeeNumber,
      'first_name': firstName,
      'last_name': lastName,
      'email': email,
      'department': department,
      'position': position,
      'status': status,
      'hire_date': hireDate?.toIso8601String(),
      'phone_number': phoneNumber,
    };
  }
}
