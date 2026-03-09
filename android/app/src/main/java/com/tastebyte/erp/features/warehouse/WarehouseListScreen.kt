package com.tastebyte.erp.features.warehouse

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.outlined.Warehouse
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.tastebyte.erp.ui.EmptyState
import com.tastebyte.erp.ui.ErpCard
import com.tastebyte.erp.ui.LoadingIndicator

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun WarehouseListScreen(
    viewModel: WarehouseViewModel,
    onBack: () -> Unit
) {
    val state by viewModel.listState.collectAsState()

    LaunchedEffect(Unit) {
        viewModel.loadWarehouses()
    }

    Column(modifier = Modifier.fillMaxSize()) {
        TopAppBar(
            title = { Text("Warehouses") },
            navigationIcon = {
                IconButton(onClick = onBack) {
                    Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Back")
                }
            }
        )

        PullToRefreshBox(
            isRefreshing = state.isLoading,
            onRefresh = viewModel::loadWarehouses,
            modifier = Modifier.fillMaxSize()
        ) {
            when {
                state.isLoading && state.warehouses.isEmpty() -> LoadingIndicator()
                state.error != null && state.warehouses.isEmpty() -> {
                    EmptyState(
                        message = state.error ?: "Error",
                        icon = Icons.Outlined.Warehouse,
                        actionLabel = "Retry",
                        onAction = viewModel::loadWarehouses
                    )
                }
                state.warehouses.isEmpty() -> {
                    EmptyState(
                        message = "No warehouses found",
                        icon = Icons.Outlined.Warehouse
                    )
                }
                else -> {
                    LazyColumn(
                        contentPadding = PaddingValues(horizontal = 16.dp, vertical = 8.dp),
                        verticalArrangement = Arrangement.spacedBy(8.dp)
                    ) {
                        items(state.warehouses, key = { it.id }) { warehouse ->
                            ErpCard(
                                title = warehouse.name,
                                subtitle = buildString {
                                    append(warehouse.warehouseNumber)
                                    if (warehouse.description != null) {
                                        append(" - ${warehouse.description}")
                                    }
                                    if (warehouse.address != null) {
                                        append("\n${warehouse.address}")
                                    }
                                },
                                status = if (warehouse.isActive) "active" else "inactive",
                                trailing = warehouse.capacity?.let { "${it}" }
                            )
                        }
                    }
                }
            }
        }
    }
}
