package com.tastebyte.erp.features.production

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.tastebyte.erp.ui.EmptyState
import com.tastebyte.erp.ui.ErpCard
import com.tastebyte.erp.ui.LoadingIndicator

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ProductionOrdersScreen(
    viewModel: ProductionViewModel,
    onOrderClick: (String) -> Unit,
    onBack: () -> Unit
) {
    val state by viewModel.listState.collectAsState()

    Column(modifier = Modifier.fillMaxSize()) {
        TopAppBar(
            title = { Text("Production Orders") },
            navigationIcon = {
                IconButton(onClick = onBack) {
                    Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Back")
                }
            }
        )

        PullToRefreshBox(
            isRefreshing = state.isLoading,
            onRefresh = viewModel::loadProductionOrders,
            modifier = Modifier.fillMaxSize()
        ) {
            when {
                state.isLoading && state.orders.isEmpty() -> LoadingIndicator()
                state.error != null && state.orders.isEmpty() -> {
                    EmptyState(
                        message = state.error ?: "Error",
                        actionLabel = "Retry",
                        onAction = viewModel::loadProductionOrders
                    )
                }
                state.orders.isEmpty() -> {
                    EmptyState(message = "No production orders found")
                }
                else -> {
                    LazyColumn(
                        contentPadding = PaddingValues(horizontal = 16.dp, vertical = 8.dp),
                        verticalArrangement = Arrangement.spacedBy(8.dp)
                    ) {
                        items(state.orders, key = { it.id }) { order ->
                            ErpCard(
                                title = order.orderNumber,
                                subtitle = "Material: ${order.materialName ?: order.materialId.take(8)} | Qty: ${String.format("%.0f", order.quantity)}",
                                status = order.status,
                                onClick = { onOrderClick(order.id) }
                            )
                        }
                    }
                }
            }
        }
    }
}
