import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/network/api_client.dart';
import '../models/material.dart' as mm;
import '../models/stock.dart';

class MaterialsState {
  final List<mm.Material> materials;
  final List<PlantStock> stocks;
  final bool isLoading;
  final String? error;
  final String searchQuery;

  const MaterialsState({
    this.materials = const [],
    this.stocks = const [],
    this.isLoading = false,
    this.error,
    this.searchQuery = '',
  });

  MaterialsState copyWith({
    List<mm.Material>? materials,
    List<PlantStock>? stocks,
    bool? isLoading,
    String? error,
    String? searchQuery,
  }) {
    return MaterialsState(
      materials: materials ?? this.materials,
      stocks: stocks ?? this.stocks,
      isLoading: isLoading ?? this.isLoading,
      error: error,
      searchQuery: searchQuery ?? this.searchQuery,
    );
  }

  List<mm.Material> get filteredMaterials {
    if (searchQuery.isEmpty) return materials;
    final query = searchQuery.toLowerCase();
    return materials.where((m) {
      return m.materialNumber.toLowerCase().contains(query) ||
          m.description.toLowerCase().contains(query) ||
          m.materialType.toLowerCase().contains(query);
    }).toList();
  }
}

class MaterialsNotifier extends StateNotifier<MaterialsState> {
  final Dio _dio;

  MaterialsNotifier(this._dio) : super(const MaterialsState());

  Future<void> loadMaterials() async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      final response = await _dio.get('/mm/materials');
      final data = response.data as List<dynamic>? ?? [];
      final materials =
          data.map((e) => mm.Material.fromJson(e as Map<String, dynamic>)).toList();
      state = state.copyWith(materials: materials, isLoading: false);
    } on DioException {
      state = state.copyWith(
        materials: _demoMaterials,
        isLoading: false,
      );
    }
  }

  Future<void> loadStocks({String? materialId}) async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      final path = materialId != null
          ? '/mm/materials/$materialId/stock'
          : '/mm/stock';
      final response = await _dio.get(path);
      final data = response.data as List<dynamic>? ?? [];
      final stocks =
          data.map((e) => PlantStock.fromJson(e as Map<String, dynamic>)).toList();
      state = state.copyWith(stocks: stocks, isLoading: false);
    } on DioException {
      state = state.copyWith(
        stocks: _demoStocks,
        isLoading: false,
      );
    }
  }

  void setSearchQuery(String query) {
    state = state.copyWith(searchQuery: query);
  }

  static final List<mm.Material> _demoMaterials = [
    const mm.Material(
      id: '1',
      materialNumber: 'MAT-001',
      description: 'Premium Olive Oil 500ml',
      materialType: 'FERT',
      unitOfMeasure: 'EA',
      materialGroup: 'Oils',
      weight: 0.55,
      weightUnit: 'KG',
    ),
    const mm.Material(
      id: '2',
      materialNumber: 'MAT-002',
      description: 'Organic Flour 1kg',
      materialType: 'ROH',
      unitOfMeasure: 'KG',
      materialGroup: 'Grains',
      weight: 1.0,
      weightUnit: 'KG',
    ),
    const mm.Material(
      id: '3',
      materialNumber: 'MAT-003',
      description: 'Sea Salt 250g',
      materialType: 'ROH',
      unitOfMeasure: 'EA',
      materialGroup: 'Seasonings',
      weight: 0.25,
      weightUnit: 'KG',
    ),
    const mm.Material(
      id: '4',
      materialNumber: 'MAT-004',
      description: 'Dark Chocolate 70% 200g',
      materialType: 'FERT',
      unitOfMeasure: 'EA',
      materialGroup: 'Confectionery',
      weight: 0.2,
      weightUnit: 'KG',
    ),
    const mm.Material(
      id: '5',
      materialNumber: 'MAT-005',
      description: 'Packaging Box - Large',
      materialType: 'VERP',
      unitOfMeasure: 'EA',
      materialGroup: 'Packaging',
    ),
  ];

  static final List<PlantStock> _demoStocks = [
    const PlantStock(
      id: '1',
      materialId: '1',
      materialNumber: 'MAT-001',
      materialDescription: 'Premium Olive Oil 500ml',
      warehouseId: '1',
      warehouseName: 'WH-01 Main',
      quantity: 500,
      unitOfMeasure: 'EA',
      minStock: 100,
      maxStock: 1000,
    ),
    const PlantStock(
      id: '2',
      materialId: '2',
      materialNumber: 'MAT-002',
      materialDescription: 'Organic Flour 1kg',
      warehouseId: '1',
      warehouseName: 'WH-01 Main',
      quantity: 80,
      unitOfMeasure: 'KG',
      minStock: 200,
      maxStock: 2000,
    ),
    const PlantStock(
      id: '3',
      materialId: '3',
      materialNumber: 'MAT-003',
      materialDescription: 'Sea Salt 250g',
      warehouseId: '2',
      warehouseName: 'WH-02 Secondary',
      quantity: 300,
      unitOfMeasure: 'EA',
      minStock: 50,
      maxStock: 500,
    ),
  ];
}

final materialsProvider =
    StateNotifierProvider<MaterialsNotifier, MaterialsState>((ref) {
  final dio = ref.watch(dioProvider);
  return MaterialsNotifier(dio);
});
