import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:intl/intl.dart';
import '../../../core/constants.dart';
import '../../../shared/widgets/kpi_card.dart';
import '../../../shared/widgets/loading_indicator.dart';
import '../../auth/providers/auth_provider.dart';
import '../providers/dashboard_provider.dart';

class DashboardScreen extends ConsumerStatefulWidget {
  const DashboardScreen({super.key});

  @override
  ConsumerState<DashboardScreen> createState() => _DashboardScreenState();
}

class _DashboardScreenState extends ConsumerState<DashboardScreen> {
  @override
  void initState() {
    super.initState();
    Future.microtask(
        () => ref.read(dashboardProvider.notifier).loadDashboard());
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(dashboardProvider);
    final user = ref.watch(authProvider).user;
    final currencyFormat =
        NumberFormat.currency(symbol: '\$', decimalDigits: 0);

    return Scaffold(
      appBar: AppBar(
        title: const Text(AppConstants.appName),
        actions: [
          IconButton(
            icon: const Icon(Icons.notifications_outlined),
            onPressed: () {},
          ),
          PopupMenuButton<String>(
            icon: const Icon(Icons.account_circle),
            itemBuilder: (context) => <PopupMenuEntry<String>>[
              PopupMenuItem<String>(
                enabled: false,
                value: 'user',
                child: Text(
                  user?.fullName ?? 'User',
                  style: const TextStyle(fontWeight: FontWeight.w600),
                ),
              ),
              const PopupMenuDivider(),
              PopupMenuItem<String>(
                value: 'logout',
                child: const Row(
                  children: [
                    Icon(Icons.logout, size: 20, color: AppColors.error),
                    SizedBox(width: 8),
                    Text('Logout'),
                  ],
                ),
              ),
            ],
            onSelected: (value) {
              if (value == 'logout') {
                ref.read(authProvider.notifier).logout();
              }
            },
          ),
        ],
      ),
      body: state.isLoading
          ? const LoadingIndicator(message: 'Loading dashboard...')
          : RefreshIndicator(
              onRefresh: () =>
                  ref.read(dashboardProvider.notifier).loadDashboard(),
              child: ListView(
                padding: const EdgeInsets.all(AppSpacing.md),
                children: [
                  // Greeting
                  Text(
                    'Welcome, ${user?.fullName ?? 'User'}',
                    style: const TextStyle(
                      fontSize: 20,
                      fontWeight: FontWeight.bold,
                      color: AppColors.onSurface,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    DateFormat('EEEE, MMMM d, yyyy').format(DateTime.now()),
                    style: const TextStyle(
                      fontSize: 14,
                      color: AppColors.textSecondary,
                    ),
                  ),
                  const SizedBox(height: AppSpacing.lg),

                  // KPI Cards
                  GridView.count(
                    crossAxisCount: 2,
                    shrinkWrap: true,
                    physics: const NeverScrollableScrollPhysics(),
                    mainAxisSpacing: 8,
                    crossAxisSpacing: 8,
                    childAspectRatio: 1.3,
                    children: [
                      KpiCard(
                        title: 'Total Orders',
                        value: '${state.kpi.totalOrders}',
                        icon: Icons.shopping_cart,
                        color: AppColors.primary,
                        onTap: () => context.go('/sd/orders'),
                      ),
                      KpiCard(
                        title: 'Revenue',
                        value: currencyFormat.format(state.kpi.revenue),
                        icon: Icons.trending_up,
                        color: AppColors.success,
                      ),
                      KpiCard(
                        title: 'Inventory Items',
                        value: '${state.kpi.inventoryItems}',
                        icon: Icons.inventory_2,
                        color: AppColors.info,
                        onTap: () => context.go('/mm/materials'),
                      ),
                      KpiCard(
                        title: 'Pending POs',
                        value: '${state.kpi.pendingPOs}',
                        icon: Icons.assignment,
                        color: AppColors.warning,
                      ),
                    ],
                  ),
                  const SizedBox(height: AppSpacing.lg),

                  // Quick Actions
                  const Text(
                    'Quick Actions',
                    style: TextStyle(
                      fontSize: 16,
                      fontWeight: FontWeight.w600,
                      color: AppColors.onSurface,
                    ),
                  ),
                  const SizedBox(height: AppSpacing.sm),
                  Row(
                    children: [
                      Expanded(
                        child: _QuickActionButton(
                          icon: Icons.access_time,
                          label: 'Clock In',
                          color: AppColors.success,
                          onTap: () => context.go('/hr/attendance'),
                        ),
                      ),
                      const SizedBox(width: 8),
                      Expanded(
                        child: _QuickActionButton(
                          icon: Icons.add_shopping_cart,
                          label: 'New Order',
                          color: AppColors.primary,
                          onTap: () => context.go('/sd/orders'),
                        ),
                      ),
                      const SizedBox(width: 8),
                      Expanded(
                        child: _QuickActionButton(
                          icon: Icons.inventory,
                          label: 'Stock Check',
                          color: AppColors.info,
                          onTap: () => context.go('/mm/stock'),
                        ),
                      ),
                      const SizedBox(width: 8),
                      Expanded(
                        child: _QuickActionButton(
                          icon: Icons.checklist,
                          label: 'QC Check',
                          color: AppColors.warning,
                          onTap: () => context.go('/qm/inspections'),
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: AppSpacing.lg),

                  // Recent Activity
                  const Text(
                    'Recent Activity',
                    style: TextStyle(
                      fontSize: 16,
                      fontWeight: FontWeight.w600,
                      color: AppColors.onSurface,
                    ),
                  ),
                  const SizedBox(height: AppSpacing.sm),
                  Card(
                    child: ListView.separated(
                      shrinkWrap: true,
                      physics: const NeverScrollableScrollPhysics(),
                      itemCount: state.recentActivities.length,
                      separatorBuilder: (_, __) => const Divider(height: 1),
                      itemBuilder: (context, index) {
                        final activity = state.recentActivities[index];
                        return ListTile(
                          leading: CircleAvatar(
                            backgroundColor:
                                _getModuleColor(activity.module).withValues(alpha: 0.12),
                            radius: 20,
                            child: Icon(
                              _getModuleIcon(activity.module),
                              color: _getModuleColor(activity.module),
                              size: 20,
                            ),
                          ),
                          title: Text(
                            activity.description,
                            style: const TextStyle(fontSize: 14),
                          ),
                          subtitle: Text(
                            activity.timestamp,
                            style: const TextStyle(
                              fontSize: 12,
                              color: AppColors.textSecondary,
                            ),
                          ),
                          trailing: activity.module != null
                              ? Container(
                                  padding: const EdgeInsets.symmetric(
                                    horizontal: 8,
                                    vertical: 2,
                                  ),
                                  decoration: BoxDecoration(
                                    color: _getModuleColor(activity.module)
                                        .withValues(alpha: 0.1),
                                    borderRadius: BorderRadius.circular(4),
                                  ),
                                  child: Text(
                                    activity.module!,
                                    style: TextStyle(
                                      fontSize: 11,
                                      fontWeight: FontWeight.w600,
                                      color:
                                          _getModuleColor(activity.module),
                                    ),
                                  ),
                                )
                              : null,
                        );
                      },
                    ),
                  ),
                ],
              ),
            ),
    );
  }

  IconData _getModuleIcon(String? module) {
    switch (module) {
      case 'SD':
        return Icons.shopping_cart;
      case 'MM':
        return Icons.inventory_2;
      case 'HR':
        return Icons.people;
      case 'QM':
        return Icons.checklist;
      case 'WM':
        return Icons.warehouse;
      default:
        return Icons.info;
    }
  }

  Color _getModuleColor(String? module) {
    switch (module) {
      case 'SD':
        return AppColors.primary;
      case 'MM':
        return AppColors.info;
      case 'HR':
        return AppColors.success;
      case 'QM':
        return AppColors.warning;
      case 'WM':
        return AppColors.secondary;
      default:
        return AppColors.textSecondary;
    }
  }
}

class _QuickActionButton extends StatelessWidget {
  final IconData icon;
  final String label;
  final Color color;
  final VoidCallback onTap;

  const _QuickActionButton({
    required this.icon,
    required this.label,
    required this.color,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(12),
        child: Padding(
          padding: const EdgeInsets.symmetric(vertical: 16, horizontal: 8),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Container(
                padding: const EdgeInsets.all(10),
                decoration: BoxDecoration(
                  color: color.withValues(alpha: 0.12),
                  shape: BoxShape.circle,
                ),
                child: Icon(icon, color: color, size: 24),
              ),
              const SizedBox(height: 8),
              Text(
                label,
                style: const TextStyle(
                  fontSize: 12,
                  fontWeight: FontWeight.w500,
                ),
                textAlign: TextAlign.center,
              ),
            ],
          ),
        ),
      ),
    );
  }
}
