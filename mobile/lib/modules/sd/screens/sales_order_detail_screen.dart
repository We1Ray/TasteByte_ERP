import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:intl/intl.dart';
import '../../../core/constants.dart';
import '../../../shared/widgets/loading_indicator.dart';
import '../../../shared/widgets/status_badge.dart';
import '../providers/sales_orders_provider.dart';

class SalesOrderDetailScreen extends ConsumerWidget {
  final String orderId;

  const SalesOrderDetailScreen({required this.orderId, super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(salesOrdersProvider);
    final orderList = state.orders.where((o) => o.id == orderId).toList();
    final currencyFormat =
        NumberFormat.currency(symbol: '\$', decimalDigits: 0);

    if (state.isLoading) {
      return Scaffold(
        appBar: AppBar(title: const Text('Order Detail')),
        body: const LoadingIndicator(),
      );
    }

    if (orderList.isEmpty) {
      return Scaffold(
        appBar: AppBar(title: const Text('Order Detail')),
        body: const Center(child: Text('Order not found')),
      );
    }

    final order = orderList.first;

    return Scaffold(
      appBar: AppBar(
        title: Text(order.orderNumber),
      ),
      body: ListView(
        padding: const EdgeInsets.all(AppSpacing.md),
        children: [
          // Header
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
                          order.orderNumber,
                          style: const TextStyle(
                            fontSize: 20,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ),
                      StatusBadge(status: order.status),
                    ],
                  ),
                  const SizedBox(height: 8),
                  Text(
                    order.customerName,
                    style: const TextStyle(
                      fontSize: 16,
                      color: AppColors.onSurface,
                    ),
                  ),
                  const Divider(height: 24),
                  _InfoRow(
                    'Order Date',
                    DateFormat('MMM d, yyyy').format(order.orderDate),
                  ),
                  if (order.deliveryDate != null)
                    _InfoRow(
                      'Delivery Date',
                      DateFormat('MMM d, yyyy').format(order.deliveryDate!),
                    ),
                  _InfoRow('Currency', order.currency),
                  const Divider(height: 24),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      const Text(
                        'Total Amount',
                        style: TextStyle(
                          fontSize: 16,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                      Text(
                        currencyFormat.format(order.totalAmount),
                        style: const TextStyle(
                          fontSize: 22,
                          fontWeight: FontWeight.bold,
                          color: AppColors.primary,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ),
          const SizedBox(height: AppSpacing.md),

          // Items
          Card(
            child: Padding(
              padding: const EdgeInsets.all(AppSpacing.md),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'Order Items (${order.items.length})',
                    style: const TextStyle(
                      fontSize: 16,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                  const Divider(),
                  if (order.items.isEmpty)
                    const Padding(
                      padding: EdgeInsets.symmetric(vertical: 16),
                      child: Center(
                        child: Text(
                          'No items',
                          style: TextStyle(
                            color: AppColors.textSecondary,
                            fontStyle: FontStyle.italic,
                          ),
                        ),
                      ),
                    )
                  else
                    ...order.items.map((item) => Padding(
                          padding: const EdgeInsets.symmetric(vertical: 8),
                          child: Row(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Expanded(
                                child: Column(
                                  crossAxisAlignment:
                                      CrossAxisAlignment.start,
                                  children: [
                                    Text(
                                      item.description,
                                      style: const TextStyle(
                                        fontWeight: FontWeight.w500,
                                      ),
                                    ),
                                    Text(
                                      '${item.materialNumber} | ${item.quantity} ${item.unitOfMeasure} x ${currencyFormat.format(item.unitPrice)}',
                                      style: const TextStyle(
                                        fontSize: 12,
                                        color: AppColors.textSecondary,
                                      ),
                                    ),
                                  ],
                                ),
                              ),
                              Text(
                                currencyFormat.format(item.totalPrice),
                                style: const TextStyle(
                                  fontWeight: FontWeight.w600,
                                ),
                              ),
                            ],
                          ),
                        )),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _InfoRow extends StatelessWidget {
  final String label;
  final String value;

  const _InfoRow(this.label, this.value);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(
            label,
            style: const TextStyle(
              color: AppColors.textSecondary,
              fontSize: 14,
            ),
          ),
          Text(
            value,
            style: const TextStyle(
              fontWeight: FontWeight.w500,
              fontSize: 14,
            ),
          ),
        ],
      ),
    );
  }
}
