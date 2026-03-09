import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/network/api_client.dart';
import '../models/warehouse.dart';

class WarehouseState {
  final List<Warehouse> warehouses;
  final bool isLoading;
  final String? error;

  const WarehouseState({
    this.warehouses = const [],
    this.isLoading = false,
    this.error,
  });

  WarehouseState copyWith({
    List<Warehouse>? warehouses,
    bool? isLoading,
    String? error,
  }) {
    return WarehouseState(
      warehouses: warehouses ?? this.warehouses,
      isLoading: isLoading ?? this.isLoading,
      error: error,
    );
  }
}

class WarehouseNotifier extends StateNotifier<WarehouseState> {
  final Dio _dio;

  WarehouseNotifier(this._dio) : super(const WarehouseState());

  Future<void> loadWarehouses() async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      final response = await _dio.get('/wm/warehouses');
      final data = response.data as List<dynamic>? ?? [];
      final warehouses =
          data.map((e) => Warehouse.fromJson(e as Map<String, dynamic>)).toList();
      state = state.copyWith(warehouses: warehouses, isLoading: false);
    } on DioException catch (e) {
      state = state.copyWith(
        warehouses: _demoWarehouses,
        isLoading: false,
        error: e.message,
      );
    }
  }

  static final List<Warehouse> _demoWarehouses = const [
    Warehouse(
      id: '1',
      warehouseNumber: 'WH-01',
      description: 'Main Warehouse',
      address: '100 Industrial Road, Taipei',
      type: 'standard',
      totalCapacity: 10000,
      usedCapacity: 7500,
    ),
    Warehouse(
      id: '2',
      warehouseNumber: 'WH-02',
      description: 'Cold Storage',
      address: '100 Industrial Road, Taipei',
      type: 'cold_storage',
      totalCapacity: 5000,
      usedCapacity: 3200,
    ),
    Warehouse(
      id: '3',
      warehouseNumber: 'WH-03',
      description: 'Distribution Center',
      address: '50 Logistics Blvd, Taoyuan',
      type: 'distribution',
      totalCapacity: 8000,
      usedCapacity: 4500,
    ),
  ];
}

final warehouseProvider =
    StateNotifierProvider<WarehouseNotifier, WarehouseState>((ref) {
  final dio = ref.watch(dioProvider);
  return WarehouseNotifier(dio);
});
