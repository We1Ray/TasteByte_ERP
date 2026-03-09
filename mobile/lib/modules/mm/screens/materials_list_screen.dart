import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import '../../../core/constants.dart';
import '../../../shared/widgets/empty_state.dart';
import '../../../shared/widgets/loading_indicator.dart';
import '../../../shared/widgets/search_field.dart';
import '../providers/materials_provider.dart';

class MaterialsListScreen extends ConsumerStatefulWidget {
  const MaterialsListScreen({super.key});

  @override
  ConsumerState<MaterialsListScreen> createState() =>
      _MaterialsListScreenState();
}

class _MaterialsListScreenState extends ConsumerState<MaterialsListScreen> {
  final _searchController = TextEditingController();

  @override
  void initState() {
    super.initState();
    Future.microtask(
        () => ref.read(materialsProvider.notifier).loadMaterials());
  }

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(materialsProvider);
    final materials = state.filteredMaterials;

    return Scaffold(
      appBar: AppBar(
        title: const Text('Materials'),
        actions: [
          IconButton(
            icon: const Icon(Icons.bar_chart),
            onPressed: () => context.go('/mm/stock'),
            tooltip: 'Stock Overview',
          ),
        ],
      ),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(AppSpacing.md),
            child: SearchField(
              controller: _searchController,
              hintText: 'Search materials...',
              onChanged: (query) {
                ref.read(materialsProvider.notifier).setSearchQuery(query);
              },
              onClear: () {
                ref.read(materialsProvider.notifier).setSearchQuery('');
                setState(() {});
              },
            ),
          ),
          Expanded(
            child: state.isLoading
                ? const LoadingIndicator()
                : materials.isEmpty
                    ? const EmptyState(
                        icon: Icons.inventory_2_outlined,
                        title: 'No Materials Found',
                        subtitle: 'No materials match your search criteria',
                      )
                    : RefreshIndicator(
                        onRefresh: () => ref
                            .read(materialsProvider.notifier)
                            .loadMaterials(),
                        child: ListView.builder(
                          itemCount: materials.length,
                          padding: const EdgeInsets.symmetric(
                              horizontal: AppSpacing.md),
                          itemBuilder: (context, index) {
                            final material = materials[index];
                            return Card(
                              margin:
                                  const EdgeInsets.only(bottom: AppSpacing.sm),
                              child: ListTile(
                                leading: CircleAvatar(
                                  backgroundColor:
                                      AppColors.primary.withValues(alpha: 0.1),
                                  child: Text(
                                    material.materialType.isNotEmpty
                                        ? material.materialType
                                            .substring(0, 2)
                                        : 'M',
                                    style: const TextStyle(
                                      color: AppColors.primary,
                                      fontWeight: FontWeight.w600,
                                      fontSize: 12,
                                    ),
                                  ),
                                ),
                                title: Text(
                                  material.description,
                                  style: const TextStyle(
                                    fontWeight: FontWeight.w500,
                                  ),
                                ),
                                subtitle: Text(
                                  '${material.materialNumber} | ${material.materialGroup}',
                                  style: const TextStyle(
                                    fontSize: 12,
                                    color: AppColors.textSecondary,
                                  ),
                                ),
                                trailing: Column(
                                  mainAxisAlignment: MainAxisAlignment.center,
                                  crossAxisAlignment: CrossAxisAlignment.end,
                                  children: [
                                    Text(
                                      material.unitOfMeasure,
                                      style: const TextStyle(
                                        fontSize: 12,
                                        color: AppColors.textSecondary,
                                      ),
                                    ),
                                    if (material.weight != null)
                                      Text(
                                        '${material.weight} ${material.weightUnit ?? 'KG'}',
                                        style: const TextStyle(
                                          fontSize: 11,
                                          color: AppColors.textSecondary,
                                        ),
                                      ),
                                  ],
                                ),
                                onTap: () =>
                                    context.go('/mm/materials/${material.id}'),
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
}
