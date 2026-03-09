import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import '../../../core/constants.dart';
import '../../../shared/widgets/empty_state.dart';
import '../../../shared/widgets/loading_indicator.dart';
import '../providers/warehouse_provider.dart';

class WarehouseListScreen extends ConsumerStatefulWidget {
  const WarehouseListScreen({super.key});

  @override
  ConsumerState<WarehouseListScreen> createState() =>
      _WarehouseListScreenState();
}

class _WarehouseListScreenState extends ConsumerState<WarehouseListScreen> {
  @override
  void initState() {
    super.initState();
    Future.microtask(
        () => ref.read(warehouseProvider.notifier).loadWarehouses());
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(warehouseProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Warehouses'),
        actions: [
          IconButton(
            icon: const Icon(Icons.qr_code_scanner),
            onPressed: () => context.go('/wm/stock-count'),
            tooltip: 'Stock Count',
          ),
        ],
      ),
      body: state.isLoading
          ? const LoadingIndicator()
          : state.warehouses.isEmpty
              ? const EmptyState(
                  icon: Icons.warehouse_outlined,
                  title: 'No Warehouses',
                  subtitle: 'Warehouse data will appear here',
                )
              : RefreshIndicator(
                  onRefresh: () =>
                      ref.read(warehouseProvider.notifier).loadWarehouses(),
                  child: ListView.builder(
                    itemCount: state.warehouses.length,
                    padding: const EdgeInsets.all(AppSpacing.md),
                    itemBuilder: (context, index) {
                      final warehouse = state.warehouses[index];
                      final utilization = warehouse.utilizationPercent;
                      final utilizationColor = utilization > 90
                          ? AppColors.error
                          : utilization > 70
                              ? AppColors.warning
                              : AppColors.success;

                      return Card(
                        margin: const EdgeInsets.only(bottom: AppSpacing.sm),
                        child: Padding(
                          padding: const EdgeInsets.all(AppSpacing.md),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Row(
                                children: [
                                  Container(
                                    padding: const EdgeInsets.all(10),
                                    decoration: BoxDecoration(
                                      color: AppColors.secondary
                                          .withValues(alpha: 0.1),
                                      borderRadius:
                                          BorderRadius.circular(8),
                                    ),
                                    child: const Icon(
                                      Icons.warehouse,
                                      color: AppColors.secondary,
                                      size: 24,
                                    ),
                                  ),
                                  const SizedBox(width: 12),
                                  Expanded(
                                    child: Column(
                                      crossAxisAlignment:
                                          CrossAxisAlignment.start,
                                      children: [
                                        Text(
                                          '${warehouse.warehouseNumber} - ${warehouse.description}',
                                          style: const TextStyle(
                                            fontWeight: FontWeight.w600,
                                            fontSize: 15,
                                          ),
                                        ),
                                        Text(
                                          warehouse.type
                                              .replaceAll('_', ' ')
                                              .toUpperCase(),
                                          style: const TextStyle(
                                            fontSize: 12,
                                            color: AppColors.textSecondary,
                                          ),
                                        ),
                                      ],
                                    ),
                                  ),
                                  Container(
                                    padding: const EdgeInsets.symmetric(
                                      horizontal: 10,
                                      vertical: 4,
                                    ),
                                    decoration: BoxDecoration(
                                      color: warehouse.isActive
                                          ? AppColors.success
                                              .withValues(alpha: 0.1)
                                          : AppColors.draft
                                              .withValues(alpha: 0.1),
                                      borderRadius:
                                          BorderRadius.circular(12),
                                    ),
                                    child: Text(
                                      warehouse.isActive
                                          ? 'Active'
                                          : 'Inactive',
                                      style: TextStyle(
                                        fontSize: 11,
                                        fontWeight: FontWeight.w600,
                                        color: warehouse.isActive
                                            ? AppColors.success
                                            : AppColors.draft,
                                      ),
                                    ),
                                  ),
                                ],
                              ),
                              if (warehouse.address.isNotEmpty) ...[
                                const SizedBox(height: 8),
                                Row(
                                  children: [
                                    const Icon(
                                      Icons.location_on_outlined,
                                      size: 14,
                                      color: AppColors.textSecondary,
                                    ),
                                    const SizedBox(width: 4),
                                    Text(
                                      warehouse.address,
                                      style: const TextStyle(
                                        fontSize: 12,
                                        color: AppColors.textSecondary,
                                      ),
                                    ),
                                  ],
                                ),
                              ],
                              if (warehouse.totalCapacity > 0) ...[
                                const SizedBox(height: 12),
                                Row(
                                  mainAxisAlignment:
                                      MainAxisAlignment.spaceBetween,
                                  children: [
                                    Text(
                                      'Capacity',
                                      style: const TextStyle(
                                        fontSize: 12,
                                        color: AppColors.textSecondary,
                                      ),
                                    ),
                                    Text(
                                      '${warehouse.usedCapacity} / ${warehouse.totalCapacity} (${utilization.toStringAsFixed(0)}%)',
                                      style: TextStyle(
                                        fontSize: 12,
                                        fontWeight: FontWeight.w500,
                                        color: utilizationColor,
                                      ),
                                    ),
                                  ],
                                ),
                                const SizedBox(height: 6),
                                ClipRRect(
                                  borderRadius: BorderRadius.circular(4),
                                  child: LinearProgressIndicator(
                                    value: (utilization / 100)
                                        .clamp(0.0, 1.0),
                                    backgroundColor: AppColors.divider,
                                    color: utilizationColor,
                                    minHeight: 6,
                                  ),
                                ),
                              ],
                            ],
                          ),
                        ),
                      );
                    },
                  ),
                ),
    );
  }
}
