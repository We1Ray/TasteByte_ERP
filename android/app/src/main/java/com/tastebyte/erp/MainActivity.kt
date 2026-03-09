package com.tastebyte.erp

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Dashboard
import androidx.compose.material.icons.filled.Inventory2
import androidx.compose.material.icons.filled.MoreHoriz
import androidx.compose.material.icons.filled.People
import androidx.compose.material.icons.filled.Receipt
import androidx.compose.material.icons.outlined.Dashboard
import androidx.compose.material.icons.outlined.Inventory2
import androidx.compose.material.icons.outlined.MoreHoriz
import androidx.compose.material.icons.outlined.People
import androidx.compose.material.icons.outlined.Receipt
import androidx.compose.material3.DropdownMenu
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.navigation.NavDestination.Companion.hierarchy
import androidx.navigation.NavGraph.Companion.findStartDestination
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import kotlinx.coroutines.launch
import com.tastebyte.erp.core.auth.AuthManager
import com.tastebyte.erp.core.auth.TokenStorage
import com.tastebyte.erp.core.network.ApiClient
import com.tastebyte.erp.core.network.NetworkMonitor
import com.tastebyte.erp.core.network.OfflineSyncManager
import com.tastebyte.erp.core.theme.TasteByteTheme
import com.tastebyte.erp.navigation.NavGraph
import com.tastebyte.erp.navigation.Screen
import com.tastebyte.erp.ui.OfflineBanner

data class BottomNavItem(
    val title: String,
    val route: String,
    val selectedIcon: ImageVector,
    val unselectedIcon: ImageVector
)

class MainActivity : ComponentActivity() {

    private lateinit var authManager: AuthManager

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        val tokenStorage = TokenStorage(this)
        ApiClient.init(tokenStorage)
        authManager = AuthManager(tokenStorage)

        setContent {
            TasteByteTheme {
                MainApp(authManager = authManager, tokenStorage = tokenStorage)
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainApp(authManager: AuthManager, tokenStorage: TokenStorage) {
    val navController = rememberNavController()
    val coroutineScope = rememberCoroutineScope()
    val isAuthenticated by authManager.isAuthenticated.collectAsState()
    val currentUsername by authManager.currentUsername.collectAsState()
    val isConnected by NetworkMonitor.getInstance().isConnected.collectAsState()

    // Sync pending operations when network is restored
    LaunchedEffect(isConnected) {
        if (isConnected) {
            OfflineSyncManager.getInstance().syncPendingOperations(tokenStorage)
        }
    }

    val bottomNavItems = listOf(
        BottomNavItem("Dashboard", Screen.Dashboard.route, Icons.Filled.Dashboard, Icons.Outlined.Dashboard),
        BottomNavItem("Materials", Screen.Materials.route, Icons.Filled.Inventory2, Icons.Outlined.Inventory2),
        BottomNavItem("Sales", Screen.SalesOrders.route, Icons.Filled.Receipt, Icons.Outlined.Receipt),
        BottomNavItem("HR", Screen.Attendance.route, Icons.Filled.People, Icons.Outlined.People),
        BottomNavItem("More", "more", Icons.Filled.MoreHoriz, Icons.Outlined.MoreHoriz)
    )

    val navBackStackEntry by navController.currentBackStackEntryAsState()
    val currentDestination = navBackStackEntry?.destination
    val currentRoute = currentDestination?.route

    val showBottomBar = isAuthenticated && currentRoute in listOf(
        Screen.Dashboard.route,
        Screen.Materials.route,
        Screen.SalesOrders.route,
        Screen.Attendance.route
    )

    val screensWithOwnTopBar = listOf(
        Screen.MaterialDetail.route,
        Screen.SalesOrderDetail.route,
        Screen.Stock.route,
        Screen.Employees.route,
        Screen.Warehouses.route,
        Screen.StockCount.route,
        Screen.Inspections.route,
        Screen.InspectionForm.route,
        Screen.PurchaseOrders.route,
        Screen.PurchaseOrderDetail.route,
        Screen.ProductionOrders.route,
        Screen.ProductionOrderDetail.route,
        Screen.FiReports.route,
        Screen.MmReports.route,
        Screen.SdReports.route,
        Screen.TrialBalance.route,
        Screen.IncomeStatement.route,
        Screen.BalanceSheet.route,
        Screen.ArAging.route,
        Screen.ApAging.route,
        Screen.StockValuation.route,
        Screen.MovementSummary.route,
        Screen.SlowMoving.route,
        Screen.SalesSummary.route,
        Screen.OrderFulfillment.route,
        Screen.TopCustomers.route
    )

    val showTopBar = isAuthenticated && currentRoute != Screen.Login.route &&
            currentRoute !in screensWithOwnTopBar

    var showMoreMenu by remember { mutableStateOf(false) }

    val startDestination = if (isAuthenticated) Screen.Dashboard.route else Screen.Login.route

    Scaffold(
        modifier = Modifier.fillMaxSize(),
        topBar = {
            if (showTopBar) {
                TopAppBar(
                    title = {
                        Text(
                            text = "TasteByte ERP",
                            style = MaterialTheme.typography.titleLarge
                        )
                    },
                    colors = TopAppBarDefaults.topAppBarColors(
                        containerColor = MaterialTheme.colorScheme.primary,
                        titleContentColor = MaterialTheme.colorScheme.onPrimary,
                        actionIconContentColor = MaterialTheme.colorScheme.onPrimary
                    ),
                    actions = {
                        if (currentUsername != null) {
                            Text(
                                text = currentUsername!!,
                                style = MaterialTheme.typography.labelMedium,
                                color = MaterialTheme.colorScheme.onPrimary
                            )
                        }
                        IconButton(onClick = {
                            coroutineScope.launch {
                                authManager.logout()
                                navController.navigate(Screen.Login.route) {
                                    popUpTo(0) { inclusive = true }
                                }
                            }
                        }) {
                            Icon(
                                imageVector = Icons.Outlined.People,
                                contentDescription = "Logout"
                            )
                        }
                    }
                )
            }
        },
        bottomBar = {
            if (showBottomBar) {
                NavigationBar {
                    bottomNavItems.forEach { item ->
                        val isSelected = if (item.route == "more") {
                            false
                        } else {
                            currentDestination?.hierarchy?.any { it.route == item.route } == true
                        }

                        NavigationBarItem(
                            selected = isSelected,
                            onClick = {
                                if (item.route == "more") {
                                    showMoreMenu = true
                                } else {
                                    navController.navigate(item.route) {
                                        popUpTo(navController.graph.findStartDestination().id) {
                                            saveState = true
                                        }
                                        launchSingleTop = true
                                        restoreState = true
                                    }
                                }
                            },
                            icon = {
                                Icon(
                                    imageVector = if (isSelected) item.selectedIcon else item.unselectedIcon,
                                    contentDescription = item.title
                                )
                            },
                            label = { Text(item.title) }
                        )
                    }
                }

                DropdownMenu(
                    expanded = showMoreMenu,
                    onDismissRequest = { showMoreMenu = false }
                ) {
                    DropdownMenuItem(
                        text = { Text("Purchase Orders") },
                        onClick = {
                            showMoreMenu = false
                            navController.navigate(Screen.PurchaseOrders.route)
                        }
                    )
                    DropdownMenuItem(
                        text = { Text("Production Orders") },
                        onClick = {
                            showMoreMenu = false
                            navController.navigate(Screen.ProductionOrders.route)
                        }
                    )
                    DropdownMenuItem(
                        text = { Text("Stock Overview") },
                        onClick = {
                            showMoreMenu = false
                            navController.navigate(Screen.Stock.route)
                        }
                    )
                    DropdownMenuItem(
                        text = { Text("Employees") },
                        onClick = {
                            showMoreMenu = false
                            navController.navigate(Screen.Employees.route)
                        }
                    )
                    DropdownMenuItem(
                        text = { Text("Warehouses") },
                        onClick = {
                            showMoreMenu = false
                            navController.navigate(Screen.Warehouses.route)
                        }
                    )
                    DropdownMenuItem(
                        text = { Text("Stock Count") },
                        onClick = {
                            showMoreMenu = false
                            navController.navigate(Screen.StockCount.route)
                        }
                    )
                    DropdownMenuItem(
                        text = { Text("Quality Inspections") },
                        onClick = {
                            showMoreMenu = false
                            navController.navigate(Screen.Inspections.route)
                        }
                    )
                    HorizontalDivider()
                    DropdownMenuItem(
                        text = { Text("FI Reports") },
                        onClick = {
                            showMoreMenu = false
                            navController.navigate(Screen.FiReports.route)
                        }
                    )
                    DropdownMenuItem(
                        text = { Text("MM Reports") },
                        onClick = {
                            showMoreMenu = false
                            navController.navigate(Screen.MmReports.route)
                        }
                    )
                    DropdownMenuItem(
                        text = { Text("SD Reports") },
                        onClick = {
                            showMoreMenu = false
                            navController.navigate(Screen.SdReports.route)
                        }
                    )
                    HorizontalDivider()
                    DropdownMenuItem(
                        text = { Text("Logout") },
                        onClick = {
                            showMoreMenu = false
                            coroutineScope.launch {
                                authManager.logout()
                                navController.navigate(Screen.Login.route) {
                                    popUpTo(0) { inclusive = true }
                                }
                            }
                        }
                    )
                }
            }
        }
    ) { innerPadding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(innerPadding)
        ) {
            if (isAuthenticated) {
                OfflineBanner()
            }
            NavGraph(
                navController = navController,
                authManager = authManager,
                startDestination = startDestination,
                modifier = Modifier
                    .fillMaxSize()
                    .weight(1f)
            )
        }
    }
}
