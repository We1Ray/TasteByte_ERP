import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../core/network/api_client.dart';

class DashboardKpi {
  final int totalOrders;
  final double revenue;
  final int inventoryItems;
  final int pendingPOs;
  final int activeEmployees;
  final int pendingInspections;

  const DashboardKpi({
    this.totalOrders = 0,
    this.revenue = 0,
    this.inventoryItems = 0,
    this.pendingPOs = 0,
    this.activeEmployees = 0,
    this.pendingInspections = 0,
  });

  factory DashboardKpi.fromJson(Map<String, dynamic> json) {
    return DashboardKpi(
      totalOrders: json['total_orders'] as int? ?? 0,
      revenue: (json['revenue'] as num?)?.toDouble() ?? 0,
      inventoryItems: json['inventory_items'] as int? ?? 0,
      pendingPOs: json['pending_pos'] as int? ?? 0,
      activeEmployees: json['active_employees'] as int? ?? 0,
      pendingInspections: json['pending_inspections'] as int? ?? 0,
    );
  }
}

class RecentActivity {
  final String id;
  final String type;
  final String description;
  final String timestamp;
  final String? module;

  const RecentActivity({
    required this.id,
    required this.type,
    required this.description,
    required this.timestamp,
    this.module,
  });

  factory RecentActivity.fromJson(Map<String, dynamic> json) {
    return RecentActivity(
      id: json['id']?.toString() ?? '',
      type: json['type'] as String? ?? '',
      description: json['description'] as String? ?? '',
      timestamp: json['timestamp'] as String? ?? '',
      module: json['module'] as String?,
    );
  }
}

class DashboardState {
  final DashboardKpi kpi;
  final List<RecentActivity> recentActivities;
  final bool isLoading;
  final String? error;

  const DashboardState({
    this.kpi = const DashboardKpi(),
    this.recentActivities = const [],
    this.isLoading = false,
    this.error,
  });

  DashboardState copyWith({
    DashboardKpi? kpi,
    List<RecentActivity>? recentActivities,
    bool? isLoading,
    String? error,
  }) {
    return DashboardState(
      kpi: kpi ?? this.kpi,
      recentActivities: recentActivities ?? this.recentActivities,
      isLoading: isLoading ?? this.isLoading,
      error: error,
    );
  }
}

class DashboardNotifier extends StateNotifier<DashboardState> {
  final Dio _dio;

  DashboardNotifier(this._dio) : super(const DashboardState());

  Future<void> loadDashboard() async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      final response = await _dio.get('/dashboard/kpi');
      final kpi = DashboardKpi.fromJson(response.data as Map<String, dynamic>);

      List<RecentActivity> activities = [];
      try {
        final activityResponse = await _dio.get('/dashboard/recent-activity');
        final activityList = activityResponse.data as List<dynamic>? ?? [];
        activities = activityList
            .map((e) => RecentActivity.fromJson(e as Map<String, dynamic>))
            .toList();
      } catch (_) {
        // Activities are optional
      }

      state = state.copyWith(
        kpi: kpi,
        recentActivities: activities,
        isLoading: false,
      );
    } on DioException {
      // Fall back to demo data if API is not available
      state = state.copyWith(
        kpi: const DashboardKpi(
          totalOrders: 156,
          revenue: 2450000,
          inventoryItems: 1243,
          pendingPOs: 23,
          activeEmployees: 89,
          pendingInspections: 7,
        ),
        recentActivities: const [
          RecentActivity(
            id: '1',
            type: 'order',
            description: 'Sales Order SO-2024-0156 created',
            timestamp: '2 min ago',
            module: 'SD',
          ),
          RecentActivity(
            id: '2',
            type: 'inventory',
            description: 'Stock updated for MAT-001 in WH-01',
            timestamp: '15 min ago',
            module: 'MM',
          ),
          RecentActivity(
            id: '3',
            type: 'hr',
            description: 'Employee John Doe clocked in',
            timestamp: '30 min ago',
            module: 'HR',
          ),
          RecentActivity(
            id: '4',
            type: 'quality',
            description: 'Inspection lot IL-0089 passed',
            timestamp: '1 hr ago',
            module: 'QM',
          ),
          RecentActivity(
            id: '5',
            type: 'warehouse',
            description: 'Goods receipt posted for PO-2024-0078',
            timestamp: '2 hr ago',
            module: 'WM',
          ),
        ],
        isLoading: false,
      );
    }
  }
}

final dashboardProvider =
    StateNotifierProvider<DashboardNotifier, DashboardState>((ref) {
  final dio = ref.watch(dioProvider);
  return DashboardNotifier(dio);
});
