import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import '../../../core/constants.dart';
import '../../../shared/widgets/loading_indicator.dart';
import '../models/inspection_lot.dart';
import '../providers/inspection_provider.dart';

class InspectionFormScreen extends ConsumerStatefulWidget {
  final String lotId;

  const InspectionFormScreen({required this.lotId, super.key});

  @override
  ConsumerState<InspectionFormScreen> createState() =>
      _InspectionFormScreenState();
}

class _InspectionFormScreenState extends ConsumerState<InspectionFormScreen> {
  final Map<String, TextEditingController> _controllers = {};
  final Map<String, bool> _passedValues = {};

  @override
  void dispose() {
    for (final controller in _controllers.values) {
      controller.dispose();
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(inspectionProvider);
    final lotList =
        state.lots.where((l) => l.id == widget.lotId).toList();

    if (state.isLoading) {
      return Scaffold(
        appBar: AppBar(title: const Text('Inspection')),
        body: const LoadingIndicator(),
      );
    }

    if (lotList.isEmpty) {
      return Scaffold(
        appBar: AppBar(title: const Text('Inspection')),
        body: const Center(child: Text('Inspection lot not found')),
      );
    }

    final lot = lotList.first;

    // Initialize controllers for results
    for (final result in lot.results) {
      _controllers.putIfAbsent(
          result.id, () => TextEditingController(text: result.actualValue));
      _passedValues.putIfAbsent(result.id, () => result.passed);
    }

    return Scaffold(
      appBar: AppBar(
        title: Text(lot.lotNumber),
        actions: [
          TextButton.icon(
            onPressed: () => _submitResults(lot),
            icon: const Icon(Icons.save, color: AppColors.onPrimary),
            label: const Text(
              'Submit',
              style: TextStyle(color: AppColors.onPrimary),
            ),
          ),
        ],
      ),
      body: ListView(
        padding: const EdgeInsets.all(AppSpacing.md),
        children: [
          // Lot info
          Card(
            child: Padding(
              padding: const EdgeInsets.all(AppSpacing.md),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    lot.materialDescription,
                    style: const TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    '${lot.materialNumber} | ${lot.inspectionType} | ${lot.quantity} ${lot.unitOfMeasure}',
                    style: const TextStyle(
                      fontSize: 13,
                      color: AppColors.textSecondary,
                    ),
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: AppSpacing.md),

          // Inspection characteristics
          const Text(
            'Inspection Characteristics',
            style: TextStyle(
              fontSize: 16,
              fontWeight: FontWeight.w600,
            ),
          ),
          const SizedBox(height: AppSpacing.sm),

          if (lot.results.isEmpty)
            Card(
              child: Padding(
                padding: const EdgeInsets.all(AppSpacing.lg),
                child: const Center(
                  child: Text(
                    'No characteristics defined for this inspection',
                    style: TextStyle(
                      color: AppColors.textSecondary,
                      fontStyle: FontStyle.italic,
                    ),
                  ),
                ),
              ),
            )
          else
            ...lot.results.map((result) => Card(
                  margin: const EdgeInsets.only(bottom: AppSpacing.sm),
                  child: Padding(
                    padding: const EdgeInsets.all(AppSpacing.md),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          children: [
                            Expanded(
                              child: Text(
                                result.characteristic,
                                style: const TextStyle(
                                  fontWeight: FontWeight.w600,
                                  fontSize: 15,
                                ),
                              ),
                            ),
                            _PassFailToggle(
                              value: _passedValues[result.id] ?? false,
                              onChanged: (value) {
                                setState(() {
                                  _passedValues[result.id] = value;
                                });
                              },
                            ),
                          ],
                        ),
                        const SizedBox(height: 8),
                        Row(
                          children: [
                            const Icon(Icons.flag_outlined,
                                size: 16, color: AppColors.textSecondary),
                            const SizedBox(width: 4),
                            Text(
                              'Target: ${result.targetValue}',
                              style: const TextStyle(
                                fontSize: 13,
                                color: AppColors.textSecondary,
                              ),
                            ),
                          ],
                        ),
                        const SizedBox(height: 8),
                        TextField(
                          controller: _controllers[result.id],
                          decoration: const InputDecoration(
                            labelText: 'Actual Value',
                            isDense: true,
                          ),
                        ),
                      ],
                    ),
                  ),
                )),
        ],
      ),
    );
  }

  void _submitResults(InspectionLot lot) {
    final results = lot.results.map((r) {
      return InspectionResult(
        id: r.id,
        characteristic: r.characteristic,
        targetValue: r.targetValue,
        actualValue: _controllers[r.id]?.text ?? '',
        passed: _passedValues[r.id] ?? false,
      );
    }).toList();

    final allPassed = results.every((r) => r.passed);
    final overallResult = allPassed ? 'passed' : 'failed';

    showDialog(
      context: context,
      builder: (ctx) => AlertDialog(
        title: const Text('Submit Inspection Results'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Overall Result: ${overallResult.toUpperCase()}'),
            const SizedBox(height: 8),
            Text(
              '${results.where((r) => r.passed).length}/${results.length} characteristics passed',
              style: const TextStyle(color: AppColors.textSecondary),
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(ctx),
            child: const Text('Cancel'),
          ),
          ElevatedButton(
            onPressed: () {
              Navigator.pop(ctx);
              ref.read(inspectionProvider.notifier).submitInspectionResult(
                    lotId: lot.id,
                    results: results,
                    overallResult: overallResult,
                  );
              context.go('/qm/inspections');
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(
                  content: Text('Inspection results submitted'),
                  backgroundColor: AppColors.success,
                ),
              );
            },
            child: const Text('Submit'),
          ),
        ],
      ),
    );
  }
}

class _PassFailToggle extends StatelessWidget {
  final bool value;
  final ValueChanged<bool> onChanged;

  const _PassFailToggle({
    required this.value,
    required this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        GestureDetector(
          onTap: () => onChanged(true),
          child: Container(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
            decoration: BoxDecoration(
              color: value
                  ? AppColors.success
                  : AppColors.success.withValues(alpha: 0.1),
              borderRadius:
                  const BorderRadius.horizontal(left: Radius.circular(16)),
            ),
            child: Text(
              'Pass',
              style: TextStyle(
                fontSize: 12,
                fontWeight: FontWeight.w600,
                color: value ? Colors.white : AppColors.success,
              ),
            ),
          ),
        ),
        GestureDetector(
          onTap: () => onChanged(false),
          child: Container(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
            decoration: BoxDecoration(
              color: !value
                  ? AppColors.error
                  : AppColors.error.withValues(alpha: 0.1),
              borderRadius:
                  const BorderRadius.horizontal(right: Radius.circular(16)),
            ),
            child: Text(
              'Fail',
              style: TextStyle(
                fontSize: 12,
                fontWeight: FontWeight.w600,
                color: !value ? Colors.white : AppColors.error,
              ),
            ),
          ),
        ),
      ],
    );
  }
}
