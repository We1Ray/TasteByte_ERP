import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:intl/intl.dart';
import '../../../core/constants.dart';
import '../../../shared/widgets/empty_state.dart';
import '../../../shared/widgets/loading_indicator.dart';
import '../../../shared/widgets/status_badge.dart';
import '../providers/inspection_provider.dart';

class InspectionListScreen extends ConsumerStatefulWidget {
  const InspectionListScreen({super.key});

  @override
  ConsumerState<InspectionListScreen> createState() =>
      _InspectionListScreenState();
}

class _InspectionListScreenState extends ConsumerState<InspectionListScreen> {
  @override
  void initState() {
    super.initState();
    Future.microtask(
        () => ref.read(inspectionProvider.notifier).loadInspectionLots());
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(inspectionProvider);
    final lots = state.filteredLots;

    return Scaffold(
      appBar: AppBar(
        title: const Text('Quality Inspections'),
      ),
      body: Column(
        children: [
          // Filter chips
          SizedBox(
            height: 52,
            child: ListView(
              scrollDirection: Axis.horizontal,
              padding: const EdgeInsets.symmetric(
                  horizontal: AppSpacing.md, vertical: AppSpacing.sm),
              children: [
                _FilterChip(
                  label: 'All',
                  isSelected: state.statusFilter == null,
                  onSelected: () => ref
                      .read(inspectionProvider.notifier)
                      .setStatusFilter(null),
                ),
                const SizedBox(width: 8),
                _FilterChip(
                  label: 'Created',
                  isSelected: state.statusFilter == 'created',
                  onSelected: () => ref
                      .read(inspectionProvider.notifier)
                      .setStatusFilter('created'),
                ),
                const SizedBox(width: 8),
                _FilterChip(
                  label: 'In Progress',
                  isSelected: state.statusFilter == 'in_progress',
                  onSelected: () => ref
                      .read(inspectionProvider.notifier)
                      .setStatusFilter('in_progress'),
                ),
                const SizedBox(width: 8),
                _FilterChip(
                  label: 'Completed',
                  isSelected: state.statusFilter == 'completed',
                  onSelected: () => ref
                      .read(inspectionProvider.notifier)
                      .setStatusFilter('completed'),
                ),
              ],
            ),
          ),
          Expanded(
            child: state.isLoading
                ? const LoadingIndicator()
                : lots.isEmpty
                    ? const EmptyState(
                        icon: Icons.checklist,
                        title: 'No Inspection Lots',
                        subtitle:
                            'Inspection lots matching your filter will appear here',
                      )
                    : RefreshIndicator(
                        onRefresh: () => ref
                            .read(inspectionProvider.notifier)
                            .loadInspectionLots(),
                        child: ListView.builder(
                          itemCount: lots.length,
                          padding: const EdgeInsets.symmetric(
                              horizontal: AppSpacing.md),
                          itemBuilder: (context, index) {
                            final lot = lots[index];
                            return Card(
                              margin: const EdgeInsets.only(
                                  bottom: AppSpacing.sm),
                              child: ListTile(
                                contentPadding: const EdgeInsets.symmetric(
                                  horizontal: 16,
                                  vertical: 8,
                                ),
                                leading: CircleAvatar(
                                  backgroundColor: _getTypeColor(
                                          lot.inspectionType)
                                      .withValues(alpha: 0.1),
                                  child: Icon(
                                    Icons.science,
                                    color: _getTypeColor(
                                        lot.inspectionType),
                                    size: 20,
                                  ),
                                ),
                                title: Row(
                                  children: [
                                    Expanded(
                                      child: Text(
                                        lot.lotNumber,
                                        style: const TextStyle(
                                          fontWeight: FontWeight.w600,
                                        ),
                                      ),
                                    ),
                                    StatusBadge(
                                      status: lot.result ?? lot.status,
                                    ),
                                  ],
                                ),
                                subtitle: Column(
                                  crossAxisAlignment:
                                      CrossAxisAlignment.start,
                                  children: [
                                    const SizedBox(height: 4),
                                    Text(
                                      lot.materialDescription,
                                      style:
                                          const TextStyle(fontSize: 13),
                                    ),
                                    const SizedBox(height: 4),
                                    Row(
                                      children: [
                                        Text(
                                          lot.inspectionType,
                                          style: const TextStyle(
                                            fontSize: 12,
                                            color:
                                                AppColors.textSecondary,
                                          ),
                                        ),
                                        const Text(' | ',
                                            style: TextStyle(
                                                color: AppColors
                                                    .textSecondary)),
                                        Text(
                                          '${lot.quantity} ${lot.unitOfMeasure}',
                                          style: const TextStyle(
                                            fontSize: 12,
                                            color:
                                                AppColors.textSecondary,
                                          ),
                                        ),
                                        const Spacer(),
                                        Text(
                                          DateFormat('MMM d')
                                              .format(lot.createdAt),
                                          style: const TextStyle(
                                            fontSize: 12,
                                            color:
                                                AppColors.textSecondary,
                                          ),
                                        ),
                                      ],
                                    ),
                                  ],
                                ),
                                onTap: lot.status != 'completed'
                                    ? () => context.go(
                                        '/qm/inspections/${lot.id}')
                                    : null,
                              ),
                            );
                          },
                        ),
                      ),
          ),
        ],
      ),
    );
  }

  Color _getTypeColor(String type) {
    switch (type.toLowerCase()) {
      case 'incoming':
        return AppColors.info;
      case 'production':
        return AppColors.warning;
      case 'final':
        return AppColors.success;
      default:
        return AppColors.textSecondary;
    }
  }
}

class _FilterChip extends StatelessWidget {
  final String label;
  final bool isSelected;
  final VoidCallback onSelected;

  const _FilterChip({
    required this.label,
    required this.isSelected,
    required this.onSelected,
  });

  @override
  Widget build(BuildContext context) {
    return FilterChip(
      label: Text(
        label,
        style: TextStyle(
          fontSize: 12,
          color: isSelected ? AppColors.onPrimary : AppColors.onSurface,
        ),
      ),
      selected: isSelected,
      onSelected: (_) => onSelected(),
      selectedColor: AppColors.primary,
      backgroundColor: AppColors.surface,
      checkmarkColor: AppColors.onPrimary,
      side: BorderSide(
        color: isSelected ? AppColors.primary : AppColors.divider,
      ),
      padding: const EdgeInsets.symmetric(horizontal: 4),
    );
  }
}
