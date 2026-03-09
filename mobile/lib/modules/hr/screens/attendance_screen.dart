import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:intl/intl.dart';
import '../../../core/constants.dart';
import '../../../shared/widgets/loading_indicator.dart';
import '../providers/attendance_provider.dart';

class AttendanceScreen extends ConsumerStatefulWidget {
  const AttendanceScreen({super.key});

  @override
  ConsumerState<AttendanceScreen> createState() => _AttendanceScreenState();
}

class _AttendanceScreenState extends ConsumerState<AttendanceScreen> {
  @override
  void initState() {
    super.initState();
    Future.microtask(
        () => ref.read(attendanceProvider.notifier).loadTodayAttendance());
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(attendanceProvider);
    final timeFormat = DateFormat('HH:mm:ss');
    final now = DateTime.now();

    return Scaffold(
      appBar: AppBar(
        title: const Text('Attendance'),
        actions: [
          IconButton(
            icon: const Icon(Icons.people_outline),
            onPressed: () {},
            tooltip: 'Employee List',
          ),
        ],
      ),
      body: state.isLoading
          ? const LoadingIndicator()
          : ListView(
              padding: const EdgeInsets.all(AppSpacing.md),
              children: [
                // Today's date and time
                Card(
                  child: Padding(
                    padding: const EdgeInsets.all(AppSpacing.lg),
                    child: Column(
                      children: [
                        const Icon(
                          Icons.calendar_today,
                          size: 40,
                          color: AppColors.primary,
                        ),
                        const SizedBox(height: 8),
                        Text(
                          DateFormat('EEEE, MMMM d').format(now),
                          style: const TextStyle(
                            fontSize: 18,
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                        Text(
                          DateFormat('yyyy').format(now),
                          style: const TextStyle(
                            fontSize: 14,
                            color: AppColors.textSecondary,
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: AppSpacing.md),

                // Clock In/Out button
                Card(
                  child: Padding(
                    padding: const EdgeInsets.all(AppSpacing.lg),
                    child: Column(
                      children: [
                        Container(
                          width: 120,
                          height: 120,
                          decoration: BoxDecoration(
                            shape: BoxShape.circle,
                            color: state.isClockedIn
                                ? AppColors.error.withValues(alpha: 0.1)
                                : AppColors.success.withValues(alpha: 0.1),
                            border: Border.all(
                              color: state.isClockedIn
                                  ? AppColors.error
                                  : AppColors.success,
                              width: 3,
                            ),
                          ),
                          child: Material(
                            color: Colors.transparent,
                            child: InkWell(
                              customBorder: const CircleBorder(),
                              onTap: state.isLoading
                                  ? null
                                  : () {
                                      if (state.isClockedIn) {
                                        ref
                                            .read(
                                                attendanceProvider.notifier)
                                            .clockOut();
                                      } else {
                                        ref
                                            .read(
                                                attendanceProvider.notifier)
                                            .clockIn();
                                      }
                                    },
                              child: Column(
                                mainAxisAlignment: MainAxisAlignment.center,
                                children: [
                                  Icon(
                                    state.isClockedIn
                                        ? Icons.logout
                                        : Icons.login,
                                    size: 36,
                                    color: state.isClockedIn
                                        ? AppColors.error
                                        : AppColors.success,
                                  ),
                                  const SizedBox(height: 4),
                                  Text(
                                    state.isClockedIn
                                        ? 'Clock Out'
                                        : 'Clock In',
                                    style: TextStyle(
                                      fontSize: 14,
                                      fontWeight: FontWeight.w600,
                                      color: state.isClockedIn
                                          ? AppColors.error
                                          : AppColors.success,
                                    ),
                                  ),
                                ],
                              ),
                            ),
                          ),
                        ),
                        const SizedBox(height: 16),
                        Text(
                          state.isClockedIn
                              ? 'You are currently clocked in'
                              : 'Tap to clock in',
                          style: const TextStyle(
                            fontSize: 14,
                            color: AppColors.textSecondary,
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
                const SizedBox(height: AppSpacing.md),

                // Today's record
                Card(
                  child: Padding(
                    padding: const EdgeInsets.all(AppSpacing.md),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text(
                          "Today's Record",
                          style: TextStyle(
                            fontSize: 16,
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                        const Divider(),
                        if (state.todayRecord != null) ...[
                          _RecordRow(
                            icon: Icons.login,
                            label: 'Clock In',
                            value: state.todayRecord!.clockIn != null
                                ? timeFormat
                                    .format(state.todayRecord!.clockIn!)
                                : '--:--:--',
                            color: AppColors.success,
                          ),
                          const SizedBox(height: 12),
                          _RecordRow(
                            icon: Icons.logout,
                            label: 'Clock Out',
                            value: state.todayRecord!.clockOut != null
                                ? timeFormat
                                    .format(state.todayRecord!.clockOut!)
                                : '--:--:--',
                            color: AppColors.error,
                          ),
                          if (state.todayRecord!.hoursWorked != null) ...[
                            const SizedBox(height: 12),
                            _RecordRow(
                              icon: Icons.timer,
                              label: 'Hours Worked',
                              value:
                                  '${state.todayRecord!.hoursWorked!.toStringAsFixed(1)} hrs',
                              color: AppColors.info,
                            ),
                          ],
                        ] else
                          const Padding(
                            padding: EdgeInsets.symmetric(vertical: 16),
                            child: Center(
                              child: Text(
                                'No attendance record for today',
                                style: TextStyle(
                                  color: AppColors.textSecondary,
                                  fontStyle: FontStyle.italic,
                                ),
                              ),
                            ),
                          ),
                      ],
                    ),
                  ),
                ),
              ],
            ),
    );
  }
}

class _RecordRow extends StatelessWidget {
  final IconData icon;
  final String label;
  final String value;
  final Color color;

  const _RecordRow({
    required this.icon,
    required this.label,
    required this.value,
    required this.color,
  });

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Container(
          padding: const EdgeInsets.all(8),
          decoration: BoxDecoration(
            color: color.withValues(alpha: 0.1),
            borderRadius: BorderRadius.circular(8),
          ),
          child: Icon(icon, color: color, size: 20),
        ),
        const SizedBox(width: 12),
        Expanded(
          child: Text(
            label,
            style: const TextStyle(
              fontSize: 14,
              color: AppColors.textSecondary,
            ),
          ),
        ),
        Text(
          value,
          style: const TextStyle(
            fontSize: 16,
            fontWeight: FontWeight.w600,
          ),
        ),
      ],
    );
  }
}
