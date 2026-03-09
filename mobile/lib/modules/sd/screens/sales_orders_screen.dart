import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:intl/intl.dart';
import '../../../core/constants.dart';
import '../../../shared/widgets/empty_state.dart';
import '../../../shared/widgets/loading_indicator.dart';
import '../../../shared/widgets/search_field.dart';
import '../../../shared/widgets/status_badge.dart';
import '../providers/sales_orders_provider.dart';

class SalesOrdersScreen extends ConsumerStatefulWidget {
  const SalesOrdersScreen({super.key});

  @override
  ConsumerState<SalesOrdersScreen> createState() => _SalesOrdersScreenState();
}

class _SalesOrdersScreenState extends ConsumerState<SalesOrdersScreen> {
  final _searchController = TextEditingController();

  @override
  void initState() {
    super.initState();
    Future.microtask(
        () => ref.read(salesOrdersProvider.notifier).loadOrders());
  }

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(salesOrdersProvider);
    final orders = state.filteredOrders;
    final currencyFormat =
        NumberFormat.currency(symbol: '\$', decimalDigits: 0);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Sales Orders'),
      ),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(
                AppSpacing.md, AppSpacing.md, AppSpacing.md, AppSpacing.sm),
            child: SearchField(
              controller: _searchController,
              hintText: 'Search orders...',
              onChanged: (query) {
                ref.read(salesOrdersProvider.notifier).setSearchQuery(query);
              },
              onClear: () {
                ref
                    .read(salesOrdersProvider.notifier)
                    .setSearchQuery('');
                setState(() {});
              },
            ),
          ),
          // Status filter chips
          SizedBox(
            height: 44,
            child: ListView(
              scrollDirection: Axis.horizontal,
              padding:
                  const EdgeInsets.symmetric(horizontal: AppSpacing.md),
              children: [
                _FilterChip(
                  label: 'All',
                  isSelected: state.statusFilter == null,
                  onSelected: () => ref
                      .read(salesOrdersProvider.notifier)
                      .setStatusFilter(null),
                ),
                const SizedBox(width: 8),
                _FilterChip(
                  label: 'Draft',
                  isSelected: state.statusFilter == 'draft',
                  onSelected: () => ref
                      .read(salesOrdersProvider.notifier)
                      .setStatusFilter('draft'),
                ),
                const SizedBox(width: 8),
                _FilterChip(
                  label: 'Released',
                  isSelected: state.statusFilter == 'released',
                  onSelected: () => ref
                      .read(salesOrdersProvider.notifier)
                      .setStatusFilter('released'),
                ),
                const SizedBox(width: 8),
                _FilterChip(
                  label: 'In Progress',
                  isSelected: state.statusFilter == 'in_progress',
                  onSelected: () => ref
                      .read(salesOrdersProvider.notifier)
                      .setStatusFilter('in_progress'),
                ),
                const SizedBox(width: 8),
                _FilterChip(
                  label: 'Completed',
                  isSelected: state.statusFilter == 'completed',
                  onSelected: () => ref
                      .read(salesOrdersProvider.notifier)
                      .setStatusFilter('completed'),
                ),
              ],
            ),
          ),
          const SizedBox(height: AppSpacing.sm),
          Expanded(
            child: state.isLoading
                ? const LoadingIndicator()
                : orders.isEmpty
                    ? const EmptyState(
                        icon: Icons.shopping_cart_outlined,
                        title: 'No Orders Found',
                        subtitle:
                            'Sales orders matching your criteria will appear here',
                      )
                    : RefreshIndicator(
                        onRefresh: () => ref
                            .read(salesOrdersProvider.notifier)
                            .loadOrders(),
                        child: ListView.builder(
                          itemCount: orders.length,
                          padding: const EdgeInsets.symmetric(
                              horizontal: AppSpacing.md),
                          itemBuilder: (context, index) {
                            final order = orders[index];
                            return Card(
                              margin: const EdgeInsets.only(
                                  bottom: AppSpacing.sm),
                              child: ListTile(
                                contentPadding: const EdgeInsets.symmetric(
                                  horizontal: 16,
                                  vertical: 8,
                                ),
                                title: Row(
                                  children: [
                                    Expanded(
                                      child: Text(
                                        order.orderNumber,
                                        style: const TextStyle(
                                          fontWeight: FontWeight.w600,
                                        ),
                                      ),
                                    ),
                                    StatusBadge(status: order.status),
                                  ],
                                ),
                                subtitle: Column(
                                  crossAxisAlignment:
                                      CrossAxisAlignment.start,
                                  children: [
                                    const SizedBox(height: 4),
                                    Text(
                                      order.customerName,
                                      style: const TextStyle(fontSize: 13),
                                    ),
                                    const SizedBox(height: 4),
                                    Row(
                                      children: [
                                        Text(
                                          DateFormat('MMM d, yyyy')
                                              .format(order.orderDate),
                                          style: const TextStyle(
                                            fontSize: 12,
                                            color: AppColors.textSecondary,
                                          ),
                                        ),
                                        const Spacer(),
                                        Text(
                                          currencyFormat
                                              .format(order.totalAmount),
                                          style: const TextStyle(
                                            fontSize: 15,
                                            fontWeight: FontWeight.w600,
                                            color: AppColors.primary,
                                          ),
                                        ),
                                      ],
                                    ),
                                  ],
                                ),
                                onTap: () =>
                                    context.go('/sd/orders/${order.id}'),
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
