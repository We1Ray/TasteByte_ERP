import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/network/api_client.dart';
import '../models/attendance.dart';
import '../models/employee.dart';

class AttendanceState {
  final Attendance? todayRecord;
  final List<Attendance> history;
  final List<Employee> employees;
  final bool isLoading;
  final String? error;
  final bool isClockedIn;

  const AttendanceState({
    this.todayRecord,
    this.history = const [],
    this.employees = const [],
    this.isLoading = false,
    this.error,
    this.isClockedIn = false,
  });

  AttendanceState copyWith({
    Attendance? todayRecord,
    List<Attendance>? history,
    List<Employee>? employees,
    bool? isLoading,
    String? error,
    bool? isClockedIn,
  }) {
    return AttendanceState(
      todayRecord: todayRecord ?? this.todayRecord,
      history: history ?? this.history,
      employees: employees ?? this.employees,
      isLoading: isLoading ?? this.isLoading,
      error: error,
      isClockedIn: isClockedIn ?? this.isClockedIn,
    );
  }
}

class AttendanceNotifier extends StateNotifier<AttendanceState> {
  final Dio _dio;

  AttendanceNotifier(this._dio) : super(const AttendanceState());

  Future<void> loadTodayAttendance() async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      final response = await _dio.get('/hr/attendance/today');
      final record =
          Attendance.fromJson(response.data as Map<String, dynamic>);
      state = state.copyWith(
        todayRecord: record,
        isClockedIn: record.isClockedIn,
        isLoading: false,
      );
    } on DioException catch (e) {
      state = state.copyWith(
        todayRecord: null,
        isClockedIn: false,
        isLoading: false,
        error: e.message,
      );
    }
  }

  Future<void> clockIn() async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      final response = await _dio.post('/hr/attendance/clock-in');
      final record =
          Attendance.fromJson(response.data as Map<String, dynamic>);
      state = state.copyWith(
        todayRecord: record,
        isClockedIn: true,
        isLoading: false,
      );
    } on DioException catch (e) {
      // Demo clock in
      final now = DateTime.now();
      final record = Attendance(
        id: 'demo-1',
        employeeId: '1',
        employeeName: 'Demo User',
        date: DateTime(now.year, now.month, now.day),
        clockIn: now,
        status: 'present',
      );
      state = state.copyWith(
        todayRecord: record,
        isClockedIn: true,
        isLoading: false,
        error: e.message,
      );
    }
  }

  Future<void> clockOut() async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      final response = await _dio.post('/hr/attendance/clock-out');
      final record =
          Attendance.fromJson(response.data as Map<String, dynamic>);
      state = state.copyWith(
        todayRecord: record,
        isClockedIn: false,
        isLoading: false,
      );
    } on DioException catch (e) {
      // Demo clock out
      final now = DateTime.now();
      final existing = state.todayRecord;
      final record = Attendance(
        id: existing?.id ?? 'demo-1',
        employeeId: existing?.employeeId ?? '1',
        employeeName: existing?.employeeName ?? 'Demo User',
        date: DateTime(now.year, now.month, now.day),
        clockIn: existing?.clockIn ?? now.subtract(const Duration(hours: 8)),
        clockOut: now,
        status: 'present',
        hoursWorked: 8.0,
      );
      state = state.copyWith(
        todayRecord: record,
        isClockedIn: false,
        isLoading: false,
        error: e.message,
      );
    }
  }

  Future<void> loadEmployees() async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      final response = await _dio.get('/hr/employees');
      final data = response.data as List<dynamic>? ?? [];
      final employees =
          data.map((e) => Employee.fromJson(e as Map<String, dynamic>)).toList();
      state = state.copyWith(employees: employees, isLoading: false);
    } on DioException catch (e) {
      state = state.copyWith(
        employees: _demoEmployees,
        isLoading: false,
        error: e.message,
      );
    }
  }

  static final List<Employee> _demoEmployees = [
    Employee(
      id: '1',
      employeeNumber: 'EMP-001',
      firstName: 'John',
      lastName: 'Doe',
      email: 'john.doe@tastebyte.com',
      department: 'Operations',
      position: 'Operations Manager',
      hireDate: DateTime(2022, 3, 15),
      phoneNumber: '+886-912-345-678',
    ),
    Employee(
      id: '2',
      employeeNumber: 'EMP-002',
      firstName: 'Jane',
      lastName: 'Smith',
      email: 'jane.smith@tastebyte.com',
      department: 'Sales',
      position: 'Sales Representative',
      hireDate: DateTime(2023, 1, 10),
      phoneNumber: '+886-923-456-789',
    ),
    Employee(
      id: '3',
      employeeNumber: 'EMP-003',
      firstName: 'Bob',
      lastName: 'Chen',
      email: 'bob.chen@tastebyte.com',
      department: 'Warehouse',
      position: 'Warehouse Supervisor',
      hireDate: DateTime(2021, 8, 1),
      phoneNumber: '+886-934-567-890',
    ),
    Employee(
      id: '4',
      employeeNumber: 'EMP-004',
      firstName: 'Alice',
      lastName: 'Wang',
      email: 'alice.wang@tastebyte.com',
      department: 'Quality',
      position: 'QA Inspector',
      hireDate: DateTime(2023, 6, 20),
      phoneNumber: '+886-945-678-901',
    ),
    Employee(
      id: '5',
      employeeNumber: 'EMP-005',
      firstName: 'David',
      lastName: 'Lin',
      email: 'david.lin@tastebyte.com',
      department: 'HR',
      position: 'HR Specialist',
      hireDate: DateTime(2022, 11, 5),
      phoneNumber: '+886-956-789-012',
    ),
  ];
}

final attendanceProvider =
    StateNotifierProvider<AttendanceNotifier, AttendanceState>((ref) {
  final dio = ref.watch(dioProvider);
  return AttendanceNotifier(dio);
});
