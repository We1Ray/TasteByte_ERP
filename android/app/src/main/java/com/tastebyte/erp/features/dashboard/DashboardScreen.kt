package com.tastebyte.erp.features.dashboard

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Assignment
import androidx.compose.material.icons.filled.Checklist
import androidx.compose.material.icons.filled.Inventory2
import androidx.compose.material.icons.filled.People
import androidx.compose.material.icons.filled.Schedule
import androidx.compose.material.icons.filled.Warehouse
import androidx.compose.material3.AssistChip
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.tastebyte.erp.core.theme.*
import com.tastebyte.erp.ui.EmptyState
import com.tastebyte.erp.ui.ErpCard
import com.tastebyte.erp.ui.LoadingIndicator

data class QuickAction(
    val label: String,
    val icon: ImageVector,
    val route: String
)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DashboardScreen(
    viewModel: DashboardViewModel,
    onNavigate: (String) -> Unit
) {
    val uiState by viewModel.uiState.collectAsState()

    val quickActions = listOf(
        QuickAction("Materials", Icons.Default.Inventory2, "materials"),
        QuickAction("Sales Orders", Icons.Default.Assignment, "sales-orders"),
        QuickAction("Attendance", Icons.Default.Schedule, "attendance"),
        QuickAction("Employees", Icons.Default.People, "employees"),
        QuickAction("Warehouses", Icons.Default.Warehouse, "warehouses"),
        QuickAction("Inspections", Icons.Default.Checklist, "inspections")
    )

    val kpiColors = listOf(KpiBlue, KpiGreen, KpiOrange, KpiPurple, KpiTeal, KpiRed)

    PullToRefreshBox(
        isRefreshing = uiState.isLoading,
        onRefresh = viewModel::loadDashboard,
        modifier = Modifier.fillMaxSize()
    ) {
        if (uiState.isLoading && uiState.kpis.isEmpty()) {
            LoadingIndicator()
        } else if (uiState.error != null && uiState.kpis.isEmpty()) {
            EmptyState(
                message = uiState.error ?: "Error loading dashboard",
                actionLabel = "Retry",
                onAction = viewModel::loadDashboard
            )
        } else {
            LazyColumn(
                modifier = Modifier.fillMaxSize(),
                contentPadding = PaddingValues(16.dp),
                verticalArrangement = Arrangement.spacedBy(16.dp)
            ) {
                // KPI Cards Grid
                item {
                    Text(
                        text = "Overview",
                        style = MaterialTheme.typography.titleLarge,
                        fontWeight = FontWeight.Bold
                    )
                    Spacer(modifier = Modifier.height(8.dp))
                    val gridHeight = ((uiState.kpis.size + 1) / 2 * 110).dp
                    LazyVerticalGrid(
                        columns = GridCells.Fixed(2),
                        modifier = Modifier
                            .fillMaxWidth()
                            .height(gridHeight),
                        horizontalArrangement = Arrangement.spacedBy(12.dp),
                        verticalArrangement = Arrangement.spacedBy(12.dp)
                    ) {
                        items(uiState.kpis) { kpi ->
                            KpiCard(
                                title = kpi.title,
                                value = kpi.value,
                                subtitle = kpi.subtitle,
                                color = kpiColors[uiState.kpis.indexOf(kpi) % kpiColors.size]
                            )
                        }
                    }
                }

                // Quick Actions
                item {
                    Text(
                        text = "Quick Actions",
                        style = MaterialTheme.typography.titleMedium,
                        fontWeight = FontWeight.SemiBold
                    )
                    Spacer(modifier = Modifier.height(8.dp))
                    LazyRow(
                        horizontalArrangement = Arrangement.spacedBy(8.dp)
                    ) {
                        items(quickActions) { action ->
                            AssistChip(
                                onClick = { onNavigate(action.route) },
                                label = { Text(action.label) },
                                leadingIcon = {
                                    Icon(
                                        imageVector = action.icon,
                                        contentDescription = action.label
                                    )
                                }
                            )
                        }
                    }
                }

                // Recent Materials
                if (uiState.recentMaterials.isNotEmpty()) {
                    item {
                        Row(
                            modifier = Modifier.fillMaxWidth(),
                            horizontalArrangement = Arrangement.SpaceBetween,
                            verticalAlignment = Alignment.CenterVertically
                        ) {
                            Text(
                                text = "Recent Materials",
                                style = MaterialTheme.typography.titleMedium,
                                fontWeight = FontWeight.SemiBold
                            )
                            Text(
                                text = "View All",
                                style = MaterialTheme.typography.labelLarge,
                                color = MaterialTheme.colorScheme.primary,
                                modifier = Modifier.clickable { onNavigate("materials") }
                            )
                        }
                    }
                    items(uiState.recentMaterials) { material ->
                        ErpCard(
                            title = material.name,
                            subtitle = "${material.materialNumber} - ${material.materialType}",
                            status = if (material.isActive) "active" else "inactive",
                            onClick = { onNavigate("materials/${material.id}") }
                        )
                    }
                }

                // Recent Sales Orders
                if (uiState.recentSalesOrders.isNotEmpty()) {
                    item {
                        Spacer(modifier = Modifier.height(8.dp))
                        Row(
                            modifier = Modifier.fillMaxWidth(),
                            horizontalArrangement = Arrangement.SpaceBetween,
                            verticalAlignment = Alignment.CenterVertically
                        ) {
                            Text(
                                text = "Recent Sales Orders",
                                style = MaterialTheme.typography.titleMedium,
                                fontWeight = FontWeight.SemiBold
                            )
                            Text(
                                text = "View All",
                                style = MaterialTheme.typography.labelLarge,
                                color = MaterialTheme.colorScheme.primary,
                                modifier = Modifier.clickable { onNavigate("sales-orders") }
                            )
                        }
                    }
                    items(uiState.recentSalesOrders) { order ->
                        ErpCard(
                            title = order.orderNumber,
                            subtitle = "Date: ${order.orderDate}",
                            status = order.status,
                            trailing = "${order.currency} ${String.format("%.2f", order.totalAmount)}",
                            onClick = { onNavigate("sales-orders/${order.id}") }
                        )
                    }
                }
            }
        }
    }
}
