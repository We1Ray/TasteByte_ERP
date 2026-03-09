package com.tastebyte.erp.features.purchasing

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.outlined.ShoppingCart
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.tastebyte.erp.ui.EmptyState
import com.tastebyte.erp.ui.ErpCard
import com.tastebyte.erp.ui.LoadingIndicator

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun PurchaseOrdersScreen(
    viewModel: PurchasingViewModel,
    onOrderClick: (String) -> Unit,
    onBack: () -> Unit
) {
    val state by viewModel.listState.collectAsState()

    Column(modifier = Modifier.fillMaxSize()) {
        TopAppBar(
            title = { Text("Purchase Orders") },
            navigationIcon = {
                IconButton(onClick = onBack) {
                    Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Back")
                }
            }
        )

        PullToRefreshBox(
            isRefreshing = state.isLoading,
            onRefresh = viewModel::loadPurchaseOrders,
            modifier = Modifier.fillMaxSize()
        ) {
            when {
                state.isLoading && state.orders.isEmpty() -> LoadingIndicator()
                state.error != null && state.orders.isEmpty() -> {
                    EmptyState(
                        message = state.error ?: "Error",
                        icon = Icons.Outlined.ShoppingCart,
                        actionLabel = "Retry",
                        onAction = viewModel::loadPurchaseOrders
                    )
                }
                state.orders.isEmpty() -> {
                    EmptyState(
                        message = "No purchase orders found",
                        icon = Icons.Outlined.ShoppingCart
                    )
                }
                else -> {
                    LazyColumn(
                        contentPadding = PaddingValues(horizontal = 16.dp, vertical = 8.dp),
                        verticalArrangement = Arrangement.spacedBy(8.dp)
                    ) {
                        items(state.orders, key = { it.id }) { order ->
                            ErpCard(
                                title = order.poNumber,
                                subtitle = "Date: ${order.orderDate}",
                                status = order.status,
                                trailing = "${order.currency} ${String.format("%.2f", order.totalAmount)}",
                                onClick = { onOrderClick(order.id) }
                            )
                        }
                    }
                }
            }
        }
    }
}
