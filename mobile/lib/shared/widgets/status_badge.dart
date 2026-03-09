import 'package:flutter/material.dart';
import '../../core/constants.dart';

class StatusBadge extends StatelessWidget {
  final String status;
  final double fontSize;

  const StatusBadge({
    required this.status,
    this.fontSize = 12,
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    final config = _getStatusConfig(status);
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
      decoration: BoxDecoration(
        color: config.color.withValues(alpha: 0.12),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: config.color.withValues(alpha: 0.3)),
      ),
      child: Text(
        config.label,
        style: TextStyle(
          color: config.color,
          fontSize: fontSize,
          fontWeight: FontWeight.w600,
        ),
      ),
    );
  }

  _StatusConfig _getStatusConfig(String status) {
    switch (status.toLowerCase()) {
      case 'draft':
        return _StatusConfig('Draft', AppColors.draft);
      case 'created':
        return _StatusConfig('Created', AppColors.draft);
      case 'released':
      case 'confirmed':
        return _StatusConfig('Released', AppColors.released);
      case 'in_progress':
      case 'processing':
        return _StatusConfig('In Progress', AppColors.inProgress);
      case 'completed':
      case 'done':
        return _StatusConfig('Completed', AppColors.completed);
      case 'cancelled':
      case 'rejected':
        return _StatusConfig('Cancelled', AppColors.cancelled);
      case 'active':
        return _StatusConfig('Active', AppColors.success);
      case 'inactive':
        return _StatusConfig('Inactive', AppColors.draft);
      case 'present':
      case 'clocked_in':
        return _StatusConfig('Present', AppColors.success);
      case 'absent':
        return _StatusConfig('Absent', AppColors.error);
      case 'passed':
      case 'accepted':
        return _StatusConfig('Passed', AppColors.success);
      case 'failed':
        return _StatusConfig('Failed', AppColors.error);
      default:
        return _StatusConfig(
          status[0].toUpperCase() + status.substring(1),
          AppColors.draft,
        );
    }
  }
}

class _StatusConfig {
  final String label;
  final Color color;

  const _StatusConfig(this.label, this.color);
}
