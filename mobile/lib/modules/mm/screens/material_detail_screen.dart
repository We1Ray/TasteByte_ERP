import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/constants.dart';
import '../../../shared/widgets/loading_indicator.dart';
import '../../../shared/widgets/status_badge.dart';
import '../providers/materials_provider.dart';

class MaterialDetailScreen extends ConsumerWidget {
  final String materialId;

  const MaterialDetailScreen({required this.materialId, super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(materialsProvider);
    final material = state.materials
        .where((m) => m.id == materialId)
        .toList();

    if (state.isLoading) {
      return Scaffold(
        appBar: AppBar(title: const Text('Material Detail')),
        body: const LoadingIndicator(),
      );
    }

    if (material.isEmpty) {
      return Scaffold(
        appBar: AppBar(title: const Text('Material Detail')),
        body: const Center(child: Text('Material not found')),
      );
    }

    final mat = material.first;

    return Scaffold(
      appBar: AppBar(
        title: Text(mat.materialNumber),
      ),
      body: ListView(
        padding: const EdgeInsets.all(AppSpacing.md),
        children: [
          // Header card
          Card(
            child: Padding(
              padding: const EdgeInsets.all(AppSpacing.md),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Expanded(
                        child: Text(
                          mat.description,
                          style: const TextStyle(
                            fontSize: 20,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ),
                      StatusBadge(
                        status: mat.isActive ? 'active' : 'inactive',
                      ),
                    ],
                  ),
                  const SizedBox(height: 4),
                  Text(
                    mat.materialNumber,
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

          // Details card
          Card(
            child: Padding(
              padding: const EdgeInsets.all(AppSpacing.md),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    'Details',
                    style: TextStyle(
                      fontSize: 16,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  const Divider(),
                  _DetailRow('Material Type', mat.materialType),
                  _DetailRow('Material Group', mat.materialGroup),
                  _DetailRow('Unit of Measure', mat.unitOfMeasure),
                  if (mat.weight != null)
                    _DetailRow(
                      'Weight',
                      '${mat.weight} ${mat.weightUnit ?? 'KG'}',
                    ),
                  if (mat.createdAt != null)
                    _DetailRow(
                      'Created',
                      '${mat.createdAt!.year}-${mat.createdAt!.month.toString().padLeft(2, '0')}-${mat.createdAt!.day.toString().padLeft(2, '0')}',
                    ),
                ],
              ),
            ),
          ),
          const SizedBox(height: AppSpacing.md),

          // Stock section
          Card(
            child: Padding(
              padding: const EdgeInsets.all(AppSpacing.md),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    'Stock Levels',
                    style: TextStyle(
                      fontSize: 16,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  const Divider(),
                  ...state.stocks
                      .where((s) => s.materialId == materialId)
                      .map((stock) => Padding(
                            padding:
                                const EdgeInsets.symmetric(vertical: 8),
                            child: Row(
                              children: [
                                Expanded(
                                  child: Column(
                                    crossAxisAlignment:
                                        CrossAxisAlignment.start,
                                    children: [
                                      Text(
                                        stock.warehouseName,
                                        style: const TextStyle(
                                          fontWeight: FontWeight.w500,
                                        ),
                                      ),
                                      if (stock.minStock != null)
                                        Text(
                                          'Min: ${stock.minStock} | Max: ${stock.maxStock ?? '-'}',
                                          style: const TextStyle(
                                            fontSize: 12,
                                            color: AppColors.textSecondary,
                                          ),
                                        ),
                                    ],
                                  ),
                                ),
                                Text(
                                  '${stock.quantity} ${stock.unitOfMeasure}',
                                  style: TextStyle(
                                    fontSize: 16,
                                    fontWeight: FontWeight.w600,
                                    color: stock.isBelowMin
                                        ? AppColors.error
                                        : AppColors.onSurface,
                                  ),
                                ),
                              ],
                            ),
                          )),
                  if (state.stocks
                      .where((s) => s.materialId == materialId)
                      .isEmpty)
                    const Padding(
                      padding: EdgeInsets.symmetric(vertical: 8),
                      child: Text(
                        'No stock data available',
                        style: TextStyle(
                          color: AppColors.textSecondary,
                          fontStyle: FontStyle.italic,
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

class _DetailRow extends StatelessWidget {
  final String label;
  final String value;

  const _DetailRow(this.label, this.value);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 6),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            width: 140,
            child: Text(
              label,
              style: const TextStyle(
                fontSize: 14,
                color: AppColors.textSecondary,
              ),
            ),
          ),
          Expanded(
            child: Text(
              value.isNotEmpty ? value : '-',
              style: const TextStyle(
                fontSize: 14,
                fontWeight: FontWeight.w500,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
