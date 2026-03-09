import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/network/api_client.dart';
import '../models/sales_order.dart';

class SalesOrdersState {
  final List<SalesOrder> orders;
  final bool isLoading;
  final String? error;
  final String? statusFilter;
  final String searchQuery;

  const SalesOrdersState({
    this.orders = const [],
    this.isLoading = false,
    this.error,
    this.statusFilter,
    this.searchQuery = '',
  });

  SalesOrdersState copyWith({
    List<SalesOrder>? orders,
    bool? isLoading,
    String? error,
    String? statusFilter,
    String? searchQuery,
  }) {
    return SalesOrdersState(
      orders: orders ?? this.orders,
      isLoading: isLoading ?? this.isLoading,
      error: error,
      statusFilter: statusFilter ?? this.statusFilter,
      searchQuery: searchQuery ?? this.searchQuery,
    );
  }

  List<SalesOrder> get filteredOrders {
    var result = orders;
    if (statusFilter != null && statusFilter!.isNotEmpty) {
      result = result.where((o) => o.status == statusFilter).toList();
    }
    if (searchQuery.isNotEmpty) {
      final query = searchQuery.toLowerCase();
      result = result.where((o) {
        return o.orderNumber.toLowerCase().contains(query) ||
            o.customerName.toLowerCase().contains(query);
      }).toList();
    }
    return result;
  }
}

class SalesOrdersNotifier extends StateNotifier<SalesOrdersState> {
  final Dio _dio;

  SalesOrdersNotifier(this._dio) : super(const SalesOrdersState());

  Future<void> loadOrders() async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      final response = await _dio.get('/sd/sales-orders');
      final data = response.data as List<dynamic>? ?? [];
      final orders =
          data.map((e) => SalesOrder.fromJson(e as Map<String, dynamic>)).toList();
      state = state.copyWith(orders: orders, isLoading: false);
    } on DioException catch (e) {
      state = state.copyWith(
        orders: _demoOrders,
        isLoading: false,
        error: e.message,
      );
    }
  }

  void setStatusFilter(String? status) {
    state = state.copyWith(statusFilter: status);
  }

  void setSearchQuery(String query) {
    state = state.copyWith(searchQuery: query);
  }

  static final List<SalesOrder> _demoOrders = [
    SalesOrder(
      id: '1',
      orderNumber: 'SO-2024-0001',
      customerId: '1',
      customerName: 'Gourmet Foods Ltd.',
      status: 'completed',
      orderDate: DateTime(2024, 12, 1),
      deliveryDate: DateTime(2024, 12, 10),
      totalAmount: 125000,
      items: const [
        SalesOrderItem(
          id: '1',
          materialId: '1',
          materialNumber: 'MAT-001',
          description: 'Premium Olive Oil 500ml',
          quantity: 100,
          unitPrice: 250,
          totalPrice: 25000,
        ),
      ],
    ),
    SalesOrder(
      id: '2',
      orderNumber: 'SO-2024-0002',
      customerId: '2',
      customerName: 'Fresh Market Co.',
      status: 'in_progress',
      orderDate: DateTime(2024, 12, 5),
      deliveryDate: DateTime(2024, 12, 15),
      totalAmount: 89000,
    ),
    SalesOrder(
      id: '3',
      orderNumber: 'SO-2024-0003',
      customerId: '3',
      customerName: 'Restaurant Supply Inc.',
      status: 'draft',
      orderDate: DateTime(2024, 12, 10),
      totalAmount: 340000,
    ),
    SalesOrder(
      id: '4',
      orderNumber: 'SO-2024-0004',
      customerId: '4',
      customerName: 'Healthy Bites Shop',
      status: 'released',
      orderDate: DateTime(2024, 12, 12),
      deliveryDate: DateTime(2024, 12, 20),
      totalAmount: 56000,
    ),
    SalesOrder(
      id: '5',
      orderNumber: 'SO-2024-0005',
      customerId: '5',
      customerName: 'Cafe Deluxe',
      status: 'cancelled',
      orderDate: DateTime(2024, 12, 8),
      totalAmount: 22000,
    ),
  ];
}

final salesOrdersProvider =
    StateNotifierProvider<SalesOrdersNotifier, SalesOrdersState>((ref) {
  final dio = ref.watch(dioProvider);
  return SalesOrdersNotifier(dio);
});
