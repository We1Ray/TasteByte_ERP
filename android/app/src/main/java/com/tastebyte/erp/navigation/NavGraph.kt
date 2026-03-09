package com.tastebyte.erp.navigation

import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavHostController
import androidx.navigation.NavType
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.navArgument
import com.tastebyte.erp.core.auth.AuthManager
import com.tastebyte.erp.features.auth.LoginScreen
import com.tastebyte.erp.features.auth.LoginViewModel
import com.tastebyte.erp.features.dashboard.DashboardScreen
import com.tastebyte.erp.features.dashboard.DashboardViewModel
import com.tastebyte.erp.features.hr.AttendanceScreen
import com.tastebyte.erp.features.hr.EmployeeListScreen
import com.tastebyte.erp.features.hr.HrViewModel
import com.tastebyte.erp.features.materials.MaterialDetailScreen
import com.tastebyte.erp.features.materials.MaterialsListScreen
import com.tastebyte.erp.features.materials.MaterialsViewModel
import com.tastebyte.erp.features.materials.StockOverviewScreen
import com.tastebyte.erp.features.production.ProductionOrderDetailScreen
import com.tastebyte.erp.features.production.ProductionOrdersScreen
import com.tastebyte.erp.features.production.ProductionViewModel
import com.tastebyte.erp.features.purchasing.PurchaseOrderDetailScreen
import com.tastebyte.erp.features.purchasing.PurchaseOrdersScreen
import com.tastebyte.erp.features.purchasing.PurchasingViewModel
import com.tastebyte.erp.features.quality.InspectionFormScreen
import com.tastebyte.erp.features.quality.InspectionListScreen
import com.tastebyte.erp.features.quality.QualityViewModel
import com.tastebyte.erp.features.reports.FiReportsScreen
import com.tastebyte.erp.features.reports.MmReportsScreen
import com.tastebyte.erp.features.reports.ReportDetailScreen
import com.tastebyte.erp.features.reports.ReportsViewModel
import com.tastebyte.erp.features.reports.SdReportsScreen
import com.tastebyte.erp.features.sales.SalesOrderDetailScreen
import com.tastebyte.erp.features.sales.SalesOrdersScreen
import com.tastebyte.erp.features.sales.SalesViewModel
import com.tastebyte.erp.features.warehouse.StockCountScreen
import com.tastebyte.erp.features.warehouse.WarehouseListScreen
import com.tastebyte.erp.features.warehouse.WarehouseViewModel

sealed class Screen(val route: String) {
    data object Login : Screen("login")
    data object Dashboard : Screen("dashboard")
    data object Materials : Screen("materials")
    data object MaterialDetail : Screen("materials/{id}") {
        fun createRoute(id: String) = "materials/$id"
    }
    data object Stock : Screen("stock")
    data object SalesOrders : Screen("sales-orders")
    data object SalesOrderDetail : Screen("sales-orders/{id}") {
        fun createRoute(id: String) = "sales-orders/$id"
    }
    data object PurchaseOrders : Screen("purchase-orders")
    data object PurchaseOrderDetail : Screen("purchase-orders/{id}") {
        fun createRoute(id: String) = "purchase-orders/$id"
    }
    data object ProductionOrders : Screen("production-orders")
    data object ProductionOrderDetail : Screen("production-orders/{id}") {
        fun createRoute(id: String) = "production-orders/$id"
    }
    data object Attendance : Screen("attendance")
    data object Employees : Screen("employees")
    data object Warehouses : Screen("warehouses")
    data object StockCount : Screen("stock-count")
    data object Inspections : Screen("inspections")
    data object InspectionForm : Screen("inspections/{id}/form") {
        fun createRoute(id: String) = "inspections/$id/form"
    }
    // Report list screens
    data object FiReports : Screen("reports/fi")
    data object MmReports : Screen("reports/mm")
    data object SdReports : Screen("reports/sd")
    // Individual report screens
    data object TrialBalance : Screen("reports/fi/trial-balance")
    data object IncomeStatement : Screen("reports/fi/income-statement")
    data object BalanceSheet : Screen("reports/fi/balance-sheet")
    data object ArAging : Screen("reports/fi/ar-aging")
    data object ApAging : Screen("reports/fi/ap-aging")
    data object StockValuation : Screen("reports/mm/stock-valuation")
    data object MovementSummary : Screen("reports/mm/movement-summary")
    data object SlowMoving : Screen("reports/mm/slow-moving")
    data object SalesSummary : Screen("reports/sd/sales-summary")
    data object OrderFulfillment : Screen("reports/sd/order-fulfillment")
    data object TopCustomers : Screen("reports/sd/top-customers")
}

@Composable
fun NavGraph(
    navController: NavHostController,
    authManager: AuthManager,
    startDestination: String,
    modifier: Modifier = Modifier
) {
    val materialsViewModel: MaterialsViewModel = viewModel()
    val salesViewModel: SalesViewModel = viewModel()
    val hrViewModel: HrViewModel = viewModel()
    val warehouseViewModel: WarehouseViewModel = viewModel()
    val qualityViewModel: QualityViewModel = viewModel()
    val purchasingViewModel: PurchasingViewModel = viewModel()
    val productionViewModel: ProductionViewModel = viewModel()
    val reportsViewModel: ReportsViewModel = viewModel()

    NavHost(
        navController = navController,
        startDestination = startDestination,
        modifier = modifier
    ) {
        composable(Screen.Login.route) {
            val loginViewModel = remember {
                LoginViewModel(authManager)
            }
            LoginScreen(
                viewModel = loginViewModel,
                onLoginSuccess = {
                    navController.navigate(Screen.Dashboard.route) {
                        popUpTo(Screen.Login.route) { inclusive = true }
                    }
                }
            )
        }

        composable(Screen.Dashboard.route) {
            val dashboardViewModel: DashboardViewModel = viewModel()
            DashboardScreen(
                viewModel = dashboardViewModel,
                onNavigate = { route ->
                    navController.navigate(route)
                }
            )
        }

        composable(Screen.Materials.route) {
            MaterialsListScreen(
                viewModel = materialsViewModel,
                onMaterialClick = { id ->
                    navController.navigate(Screen.MaterialDetail.createRoute(id))
                }
            )
        }

        composable(
            route = Screen.MaterialDetail.route,
            arguments = listOf(navArgument("id") { type = NavType.StringType })
        ) { backStackEntry ->
            val id = backStackEntry.arguments?.getString("id") ?: return@composable
            MaterialDetailScreen(
                materialId = id,
                viewModel = materialsViewModel,
                onBack = { navController.popBackStack() }
            )
        }

        composable(Screen.Stock.route) {
            StockOverviewScreen(
                viewModel = materialsViewModel,
                onBack = { navController.popBackStack() }
            )
        }

        composable(Screen.SalesOrders.route) {
            SalesOrdersScreen(
                viewModel = salesViewModel,
                onOrderClick = { id ->
                    navController.navigate(Screen.SalesOrderDetail.createRoute(id))
                }
            )
        }

        composable(
            route = Screen.SalesOrderDetail.route,
            arguments = listOf(navArgument("id") { type = NavType.StringType })
        ) { backStackEntry ->
            val id = backStackEntry.arguments?.getString("id") ?: return@composable
            SalesOrderDetailScreen(
                orderId = id,
                viewModel = salesViewModel,
                onBack = { navController.popBackStack() }
            )
        }

        // Purchase Orders
        composable(Screen.PurchaseOrders.route) {
            PurchaseOrdersScreen(
                viewModel = purchasingViewModel,
                onOrderClick = { id ->
                    navController.navigate(Screen.PurchaseOrderDetail.createRoute(id))
                },
                onBack = { navController.popBackStack() }
            )
        }

        composable(
            route = Screen.PurchaseOrderDetail.route,
            arguments = listOf(navArgument("id") { type = NavType.StringType })
        ) { backStackEntry ->
            val id = backStackEntry.arguments?.getString("id") ?: return@composable
            PurchaseOrderDetailScreen(
                orderId = id,
                viewModel = purchasingViewModel,
                onBack = { navController.popBackStack() }
            )
        }

        // Production Orders
        composable(Screen.ProductionOrders.route) {
            ProductionOrdersScreen(
                viewModel = productionViewModel,
                onOrderClick = { id ->
                    navController.navigate(Screen.ProductionOrderDetail.createRoute(id))
                },
                onBack = { navController.popBackStack() }
            )
        }

        composable(
            route = Screen.ProductionOrderDetail.route,
            arguments = listOf(navArgument("id") { type = NavType.StringType })
        ) { backStackEntry ->
            val id = backStackEntry.arguments?.getString("id") ?: return@composable
            ProductionOrderDetailScreen(
                orderId = id,
                viewModel = productionViewModel,
                onBack = { navController.popBackStack() }
            )
        }

        composable(Screen.Attendance.route) {
            AttendanceScreen(viewModel = hrViewModel)
        }

        composable(Screen.Employees.route) {
            EmployeeListScreen(
                viewModel = hrViewModel,
                onBack = { navController.popBackStack() }
            )
        }

        composable(Screen.Warehouses.route) {
            WarehouseListScreen(
                viewModel = warehouseViewModel,
                onBack = { navController.popBackStack() }
            )
        }

        composable(Screen.StockCount.route) {
            StockCountScreen(
                viewModel = warehouseViewModel,
                onBack = { navController.popBackStack() }
            )
        }

        composable(Screen.Inspections.route) {
            InspectionListScreen(
                viewModel = qualityViewModel,
                onInspectionClick = { id ->
                    navController.navigate(Screen.InspectionForm.createRoute(id))
                },
                onBack = { navController.popBackStack() }
            )
        }

        composable(
            route = Screen.InspectionForm.route,
            arguments = listOf(navArgument("id") { type = NavType.StringType })
        ) { backStackEntry ->
            val id = backStackEntry.arguments?.getString("id") ?: return@composable
            InspectionFormScreen(
                inspectionId = id,
                viewModel = qualityViewModel,
                onBack = { navController.popBackStack() }
            )
        }

        // Report list screens
        composable(Screen.FiReports.route) {
            FiReportsScreen(
                onReportClick = { route -> navController.navigate(route) },
                onBack = { navController.popBackStack() }
            )
        }

        composable(Screen.MmReports.route) {
            MmReportsScreen(
                onReportClick = { route -> navController.navigate(route) },
                onBack = { navController.popBackStack() }
            )
        }

        composable(Screen.SdReports.route) {
            SdReportsScreen(
                onReportClick = { route -> navController.navigate(route) },
                onBack = { navController.popBackStack() }
            )
        }

        // FI Report detail screens
        composable(Screen.TrialBalance.route) {
            ReportDetailScreen(
                title = "Trial Balance",
                state = reportsViewModel.trialBalance,
                onLoad = reportsViewModel::loadTrialBalance,
                onBack = { navController.popBackStack() }
            )
        }

        composable(Screen.IncomeStatement.route) {
            ReportDetailScreen(
                title = "Income Statement",
                state = reportsViewModel.incomeStatement,
                onLoad = reportsViewModel::loadIncomeStatement,
                onBack = { navController.popBackStack() }
            )
        }

        composable(Screen.BalanceSheet.route) {
            ReportDetailScreen(
                title = "Balance Sheet",
                state = reportsViewModel.balanceSheet,
                onLoad = reportsViewModel::loadBalanceSheet,
                onBack = { navController.popBackStack() }
            )
        }

        composable(Screen.ArAging.route) {
            ReportDetailScreen(
                title = "AR Aging",
                state = reportsViewModel.arAging,
                onLoad = reportsViewModel::loadArAging,
                onBack = { navController.popBackStack() }
            )
        }

        composable(Screen.ApAging.route) {
            ReportDetailScreen(
                title = "AP Aging",
                state = reportsViewModel.apAging,
                onLoad = reportsViewModel::loadApAging,
                onBack = { navController.popBackStack() }
            )
        }

        // MM Report detail screens
        composable(Screen.StockValuation.route) {
            ReportDetailScreen(
                title = "Stock Valuation",
                state = reportsViewModel.stockValuation,
                onLoad = reportsViewModel::loadStockValuation,
                onBack = { navController.popBackStack() }
            )
        }

        composable(Screen.MovementSummary.route) {
            ReportDetailScreen(
                title = "Movement Summary",
                state = reportsViewModel.movementSummary,
                onLoad = reportsViewModel::loadMovementSummary,
                onBack = { navController.popBackStack() }
            )
        }

        composable(Screen.SlowMoving.route) {
            ReportDetailScreen(
                title = "Slow Moving Items",
                state = reportsViewModel.slowMoving,
                onLoad = reportsViewModel::loadSlowMoving,
                onBack = { navController.popBackStack() }
            )
        }

        // SD Report detail screens
        composable(Screen.SalesSummary.route) {
            ReportDetailScreen(
                title = "Sales Summary",
                state = reportsViewModel.salesSummary,
                onLoad = reportsViewModel::loadSalesSummary,
                onBack = { navController.popBackStack() }
            )
        }

        composable(Screen.OrderFulfillment.route) {
            ReportDetailScreen(
                title = "Order Fulfillment",
                state = reportsViewModel.orderFulfillment,
                onLoad = reportsViewModel::loadOrderFulfillment,
                onBack = { navController.popBackStack() }
            )
        }

        composable(Screen.TopCustomers.route) {
            ReportDetailScreen(
                title = "Top Customers",
                state = reportsViewModel.topCustomers,
                onLoad = reportsViewModel::loadTopCustomers,
                onBack = { navController.popBackStack() }
            )
        }
    }
}
