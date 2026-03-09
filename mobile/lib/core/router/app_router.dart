import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import '../../modules/auth/providers/auth_provider.dart';
import '../../modules/auth/screens/login_screen.dart';
import '../../modules/dashboard/screens/dashboard_screen.dart';
import '../../modules/hr/screens/attendance_screen.dart';
import '../../modules/hr/screens/employee_list_screen.dart';
import '../../modules/mm/screens/material_detail_screen.dart';
import '../../modules/mm/screens/materials_list_screen.dart';
import '../../modules/mm/screens/stock_overview_screen.dart';
import '../../modules/more/screens/more_screen.dart';
import '../../modules/qm/screens/inspection_form_screen.dart';
import '../../modules/qm/screens/inspection_list_screen.dart';
import '../../modules/sd/screens/sales_order_detail_screen.dart';
import '../../modules/sd/screens/sales_orders_screen.dart';
import '../../modules/wm/screens/stock_count_screen.dart';
import '../../modules/wm/screens/warehouse_list_screen.dart';
import '../../shared/widgets/app_scaffold.dart';

final _rootNavigatorKey = GlobalKey<NavigatorState>();
final _shellNavigatorKey = GlobalKey<NavigatorState>();

final routerProvider = Provider<GoRouter>((ref) {
  final authState = ref.watch(authProvider);

  return GoRouter(
    navigatorKey: _rootNavigatorKey,
    initialLocation: '/dashboard',
    debugLogDiagnostics: true,
    redirect: (context, state) {
      final isAuthenticated = authState.status == AuthStatus.authenticated;
      final isLoginRoute = state.uri.toString() == '/login';

      if (!isAuthenticated && !isLoginRoute) {
        return '/login';
      }
      if (isAuthenticated && isLoginRoute) {
        return '/dashboard';
      }
      return null;
    },
    routes: [
      GoRoute(
        path: '/login',
        builder: (context, state) => const LoginScreen(),
      ),
      ShellRoute(
        navigatorKey: _shellNavigatorKey,
        builder: (context, state, child) => AppScaffold(child: child),
        routes: [
          // Dashboard
          GoRoute(
            path: '/dashboard',
            builder: (context, state) => const DashboardScreen(),
          ),

          // MM - Materials Management
          GoRoute(
            path: '/mm/materials',
            builder: (context, state) => const MaterialsListScreen(),
          ),
          GoRoute(
            path: '/mm/materials/:id',
            builder: (context, state) => MaterialDetailScreen(
              materialId: state.pathParameters['id']!,
            ),
          ),
          GoRoute(
            path: '/mm/stock',
            builder: (context, state) => const StockOverviewScreen(),
          ),

          // SD - Sales & Distribution
          GoRoute(
            path: '/sd/orders',
            builder: (context, state) => const SalesOrdersScreen(),
          ),
          GoRoute(
            path: '/sd/orders/:id',
            builder: (context, state) => SalesOrderDetailScreen(
              orderId: state.pathParameters['id']!,
            ),
          ),

          // HR - Human Resources
          GoRoute(
            path: '/hr/attendance',
            builder: (context, state) => const AttendanceScreen(),
          ),
          GoRoute(
            path: '/hr/employees',
            builder: (context, state) => const EmployeeListScreen(),
          ),

          // WM - Warehouse Management
          GoRoute(
            path: '/wm/warehouses',
            builder: (context, state) => const WarehouseListScreen(),
          ),
          GoRoute(
            path: '/wm/stock-count',
            builder: (context, state) => const StockCountScreen(),
          ),

          // QM - Quality Management
          GoRoute(
            path: '/qm/inspections',
            builder: (context, state) => const InspectionListScreen(),
          ),
          GoRoute(
            path: '/qm/inspections/:id',
            builder: (context, state) => InspectionFormScreen(
              lotId: state.pathParameters['id']!,
            ),
          ),

          // More
          GoRoute(
            path: '/more',
            builder: (context, state) => const MoreScreen(),
          ),
        ],
      ),
    ],
  );
});
