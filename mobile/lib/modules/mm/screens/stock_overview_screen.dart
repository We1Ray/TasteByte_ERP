import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/constants.dart';
import '../../../shared/widgets/empty_state.dart';
import '../../../shared/widgets/loading_indicator.dart';
import '../providers/materials_provider.dart';

class StockOverviewScreen extends ConsumerStatefulWidget {
  const StockOverviewScreen({super.key});

  @override
  ConsumerState<StockOverviewScreen> createState() =>
      _StockOverviewScreenState();
}

class _StockOverviewScreenState extends ConsumerState<StockOverviewScreen> {
  @override
  void initState() {
    super.initState();
    Future.microtask(() => ref.read(materialsProvider.notifier).loadStocks());
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(materialsProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Stock Overview'),
      ),
      body: state.isLoading
          ? const LoadingIndicator()
          : state.stocks.isEmpty
              ? const EmptyState(
                  icon: Icons.inventory_outlined,
                  title: 'No Stock Data',
                  subtitle: 'Stock levels will appear here',
                )
              : RefreshIndicator(
                  onRefresh: () =>
                      ref.read(materialsProvider.notifier).loadStocks(),
                  child: ListView.builder(
                    itemCount: state.stocks.length,
                    padding: const EdgeInsets.all(AppSpacing.md),
                    itemBuilder: (context, index) {
                      final stock = state.stocks[index];
                      final ratio = stock.minStock != null && stock.minStock! > 0
                          ? stock.quantity / stock.minStock!
                          : 1.0;
                      final stockColor = ratio < 1.0
                          ? AppColors.error
                          : ratio < 1.5
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
                                  Expanded(
                                    child: Column(
                                      crossAxisAlignment:
                                          CrossAxisAlignment.start,
                                      children: [
                                        Text(
                                          stock.materialDescription,
                                          style: const TextStyle(
                                            fontWeight: FontWeight.w600,
                                            fontSize: 15,
                                          ),
                                        ),
                                        const SizedBox(height: 2),
                                        Text(
                                          '${stock.materialNumber} | ${stock.warehouseName}',
                                          style: const TextStyle(
                                            fontSize: 12,
                                            color: AppColors.textSecondary,
                                          ),
                                        ),
                                      ],
                                    ),
                                  ),
                                  Column(
                                    crossAxisAlignment:
                                        CrossAxisAlignment.end,
                                    children: [
                                      Text(
                                        stock.quantity.toStringAsFixed(0),
                                        style: TextStyle(
                                          fontSize: 20,
                                          fontWeight: FontWeight.bold,
                                          color: stockColor,
                                        ),
                                      ),
                                      Text(
                                        stock.unitOfMeasure,
                                        style: const TextStyle(
                                          fontSize: 11,
                                          color: AppColors.textSecondary,
                                        ),
                                      ),
                                    ],
                                  ),
                                ],
                              ),
                              if (stock.minStock != null) ...[
                                const SizedBox(height: 12),
                                ClipRRect(
                                  borderRadius: BorderRadius.circular(4),
                                  child: LinearProgressIndicator(
                                    value: stock.maxStock != null
                                        ? (stock.quantity / stock.maxStock!)
                                            .clamp(0.0, 1.0)
                                        : 0.5,
                                    backgroundColor:
                                        AppColors.divider,
                                    color: stockColor,
                                    minHeight: 6,
                                  ),
                                ),
                                const SizedBox(height: 4),
                                Row(
                                  mainAxisAlignment:
                                      MainAxisAlignment.spaceBetween,
                                  children: [
                                    Text(
                                      'Min: ${stock.minStock!.toStringAsFixed(0)}',
                                      style: const TextStyle(
                                        fontSize: 11,
                                        color: AppColors.textSecondary,
                                      ),
                                    ),
                                    if (stock.maxStock != null)
                                      Text(
                                        'Max: ${stock.maxStock!.toStringAsFixed(0)}',
                                        style: const TextStyle(
                                          fontSize: 11,
                                          color: AppColors.textSecondary,
                                        ),
                                      ),
                                  ],
                                ),
                                if (stock.isBelowMin)
                                  Padding(
                                    padding:
                                        const EdgeInsets.only(top: 8),
                                    child: Container(
                                      padding: const EdgeInsets.symmetric(
                                        horizontal: 8,
                                        vertical: 4,
                                      ),
                                      decoration: BoxDecoration(
                                        color: AppColors.error
                                            .withValues(alpha: 0.1),
                                        borderRadius:
                                            BorderRadius.circular(4),
                                      ),
                                      child: const Row(
                                        mainAxisSize: MainAxisSize.min,
                                        children: [
                                          Icon(
                                            Icons.warning_amber_rounded,
                                            size: 14,
                                            color: AppColors.error,
                                          ),
                                          SizedBox(width: 4),
                                          Text(
                                            'Below minimum stock',
                                            style: TextStyle(
                                              fontSize: 11,
                                              color: AppColors.error,
                                              fontWeight: FontWeight.w500,
                                            ),
                                          ),
                                        ],
                                      ),
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
