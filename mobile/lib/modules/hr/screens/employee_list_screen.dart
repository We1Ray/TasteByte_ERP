import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/constants.dart';
import '../../../shared/widgets/empty_state.dart';
import '../../../shared/widgets/loading_indicator.dart';
import '../../../shared/widgets/status_badge.dart';
import '../providers/attendance_provider.dart';

class EmployeeListScreen extends ConsumerStatefulWidget {
  const EmployeeListScreen({super.key});

  @override
  ConsumerState<EmployeeListScreen> createState() =>
      _EmployeeListScreenState();
}

class _EmployeeListScreenState extends ConsumerState<EmployeeListScreen> {
  @override
  void initState() {
    super.initState();
    Future.microtask(
        () => ref.read(attendanceProvider.notifier).loadEmployees());
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(attendanceProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Employees'),
      ),
      body: state.isLoading
          ? const LoadingIndicator()
          : state.employees.isEmpty
              ? const EmptyState(
                  icon: Icons.people_outline,
                  title: 'No Employees',
                  subtitle: 'Employee directory is empty',
                )
              : RefreshIndicator(
                  onRefresh: () =>
                      ref.read(attendanceProvider.notifier).loadEmployees(),
                  child: ListView.builder(
                    itemCount: state.employees.length,
                    padding: const EdgeInsets.all(AppSpacing.md),
                    itemBuilder: (context, index) {
                      final employee = state.employees[index];
                      return Card(
                        margin: const EdgeInsets.only(bottom: AppSpacing.sm),
                        child: ListTile(
                          leading: CircleAvatar(
                            backgroundColor:
                                AppColors.primary.withValues(alpha: 0.1),
                            child: Text(
                              '${employee.firstName[0]}${employee.lastName[0]}',
                              style: const TextStyle(
                                color: AppColors.primary,
                                fontWeight: FontWeight.w600,
                              ),
                            ),
                          ),
                          title: Text(
                            employee.fullName,
                            style: const TextStyle(
                              fontWeight: FontWeight.w500,
                            ),
                          ),
                          subtitle: Text(
                            '${employee.department} - ${employee.position}',
                            style: const TextStyle(
                              fontSize: 12,
                              color: AppColors.textSecondary,
                            ),
                          ),
                          trailing: StatusBadge(status: employee.status),
                          onTap: () {
                            _showEmployeeDetail(context, employee);
                          },
                        ),
                      );
                    },
                  ),
                ),
    );
  }

  void _showEmployeeDetail(BuildContext context, dynamic employee) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(16)),
      ),
      builder: (context) => DraggableScrollableSheet(
        initialChildSize: 0.5,
        minChildSize: 0.3,
        maxChildSize: 0.8,
        expand: false,
        builder: (context, scrollController) => SingleChildScrollView(
          controller: scrollController,
          padding: const EdgeInsets.all(AppSpacing.lg),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Center(
                child: Container(
                  width: 40,
                  height: 4,
                  decoration: BoxDecoration(
                    color: AppColors.divider,
                    borderRadius: BorderRadius.circular(2),
                  ),
                ),
              ),
              const SizedBox(height: AppSpacing.lg),
              Center(
                child: CircleAvatar(
                  radius: 36,
                  backgroundColor: AppColors.primary.withValues(alpha: 0.1),
                  child: Text(
                    '${employee.firstName[0]}${employee.lastName[0]}',
                    style: const TextStyle(
                      fontSize: 24,
                      color: AppColors.primary,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                ),
              ),
              const SizedBox(height: 12),
              Center(
                child: Text(
                  employee.fullName,
                  style: const TextStyle(
                    fontSize: 20,
                    fontWeight: FontWeight.bold,
                  ),
                ),
              ),
              Center(
                child: Text(
                  employee.position,
                  style: const TextStyle(
                    fontSize: 14,
                    color: AppColors.textSecondary,
                  ),
                ),
              ),
              const SizedBox(height: AppSpacing.lg),
              _DetailItem(
                  Icons.badge, 'Employee ID', employee.employeeNumber),
              _DetailItem(Icons.business, 'Department', employee.department),
              _DetailItem(Icons.email, 'Email', employee.email),
              if (employee.phoneNumber != null)
                _DetailItem(Icons.phone, 'Phone', employee.phoneNumber!),
            ],
          ),
        ),
      ),
    );
  }
}

class _DetailItem extends StatelessWidget {
  final IconData icon;
  final String label;
  final String value;

  const _DetailItem(this.icon, this.label, this.value);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
          Icon(icon, color: AppColors.textSecondary, size: 20),
          const SizedBox(width: 12),
          Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                label,
                style: const TextStyle(
                  fontSize: 12,
                  color: AppColors.textSecondary,
                ),
              ),
              Text(
                value,
                style: const TextStyle(
                  fontSize: 14,
                  fontWeight: FontWeight.w500,
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
