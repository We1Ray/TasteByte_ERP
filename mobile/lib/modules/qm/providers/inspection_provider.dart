import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/network/api_client.dart';
import '../models/inspection_lot.dart';

class InspectionState {
  final List<InspectionLot> lots;
  final bool isLoading;
  final String? error;
  final String? statusFilter;

  const InspectionState({
    this.lots = const [],
    this.isLoading = false,
    this.error,
    this.statusFilter,
  });

  InspectionState copyWith({
    List<InspectionLot>? lots,
    bool? isLoading,
    String? error,
    String? statusFilter,
  }) {
    return InspectionState(
      lots: lots ?? this.lots,
      isLoading: isLoading ?? this.isLoading,
      error: error,
      statusFilter: statusFilter ?? this.statusFilter,
    );
  }

  List<InspectionLot> get filteredLots {
    if (statusFilter == null || statusFilter!.isEmpty) return lots;
    return lots.where((l) => l.status == statusFilter).toList();
  }
}

class InspectionNotifier extends StateNotifier<InspectionState> {
  final Dio _dio;

  InspectionNotifier(this._dio) : super(const InspectionState());

  Future<void> loadInspectionLots() async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      final response = await _dio.get('/qm/inspection-lots');
      final data = response.data as List<dynamic>? ?? [];
      final lots = data
          .map((e) => InspectionLot.fromJson(e as Map<String, dynamic>))
          .toList();
      state = state.copyWith(lots: lots, isLoading: false);
    } on DioException catch (e) {
      state = state.copyWith(
        lots: _demoLots,
        isLoading: false,
        error: e.message,
      );
    }
  }

  Future<void> submitInspectionResult({
    required String lotId,
    required List<InspectionResult> results,
    required String overallResult,
  }) async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      await _dio.post('/qm/inspection-lots/$lotId/results', data: {
        'results': results.map((r) => r.toJson()).toList(),
        'overall_result': overallResult,
      });
      await loadInspectionLots();
    } on DioException catch (e) {
      // Demo: update local state
      final updatedLots = state.lots.map((lot) {
        if (lot.id == lotId) {
          return InspectionLot(
            id: lot.id,
            lotNumber: lot.lotNumber,
            materialId: lot.materialId,
            materialNumber: lot.materialNumber,
            materialDescription: lot.materialDescription,
            inspectionType: lot.inspectionType,
            status: 'completed',
            quantity: lot.quantity,
            unitOfMeasure: lot.unitOfMeasure,
            createdAt: lot.createdAt,
            completedAt: DateTime.now(),
            result: overallResult,
            inspector: 'Demo User',
            results: results,
          );
        }
        return lot;
      }).toList();
      state = state.copyWith(
        lots: updatedLots,
        isLoading: false,
        error: e.message,
      );
    }
  }

  void setStatusFilter(String? status) {
    state = state.copyWith(statusFilter: status);
  }

  static final List<InspectionLot> _demoLots = [
    InspectionLot(
      id: '1',
      lotNumber: 'IL-2024-0001',
      materialId: '1',
      materialNumber: 'MAT-001',
      materialDescription: 'Premium Olive Oil 500ml',
      inspectionType: 'Incoming',
      status: 'created',
      quantity: 500,
      createdAt: DateTime(2024, 12, 10),
      results: const [
        InspectionResult(
          id: '1',
          characteristic: 'Color',
          targetValue: 'Golden Yellow',
          actualValue: '',
          passed: false,
        ),
        InspectionResult(
          id: '2',
          characteristic: 'Viscosity',
          targetValue: '80-100 mPas',
          actualValue: '',
          passed: false,
        ),
        InspectionResult(
          id: '3',
          characteristic: 'Acidity',
          targetValue: '<= 0.8%',
          actualValue: '',
          passed: false,
        ),
      ],
    ),
    InspectionLot(
      id: '2',
      lotNumber: 'IL-2024-0002',
      materialId: '2',
      materialNumber: 'MAT-002',
      materialDescription: 'Organic Flour 1kg',
      inspectionType: 'Incoming',
      status: 'in_progress',
      quantity: 1000,
      createdAt: DateTime(2024, 12, 11),
      inspector: 'Alice Wang',
    ),
    InspectionLot(
      id: '3',
      lotNumber: 'IL-2024-0003',
      materialId: '4',
      materialNumber: 'MAT-004',
      materialDescription: 'Dark Chocolate 70% 200g',
      inspectionType: 'Production',
      status: 'completed',
      quantity: 200,
      createdAt: DateTime(2024, 12, 8),
      completedAt: DateTime(2024, 12, 9),
      result: 'passed',
      inspector: 'Alice Wang',
    ),
  ];
}

final inspectionProvider =
    StateNotifierProvider<InspectionNotifier, InspectionState>((ref) {
  final dio = ref.watch(dioProvider);
  return InspectionNotifier(dio);
});
