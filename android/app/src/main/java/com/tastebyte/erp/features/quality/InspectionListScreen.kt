package com.tastebyte.erp.features.quality

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.outlined.Checklist
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FilterChip
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
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
fun InspectionListScreen(
    viewModel: QualityViewModel,
    onInspectionClick: (String) -> Unit,
    onBack: () -> Unit
) {
    val state by viewModel.listState.collectAsState()

    LaunchedEffect(Unit) {
        viewModel.loadInspectionLots()
    }

    val statusFilters = listOf(
        null to "All",
        "open" to "Open",
        "in_progress" to "In Progress",
        "completed" to "Completed"
    )

    Column(modifier = Modifier.fillMaxSize()) {
        TopAppBar(
            title = { Text("Inspections") },
            navigationIcon = {
                IconButton(onClick = onBack) {
                    Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Back")
                }
            }
        )

        // Status filter chips
        LazyRow(
            contentPadding = PaddingValues(horizontal = 16.dp),
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            items(statusFilters) { (filterValue, label) ->
                FilterChip(
                    selected = state.statusFilter == filterValue,
                    onClick = { viewModel.setStatusFilter(filterValue) },
                    label = { Text(label) }
                )
            }
        }

        PullToRefreshBox(
            isRefreshing = state.isLoading,
            onRefresh = viewModel::loadInspectionLots,
            modifier = Modifier.fillMaxSize()
        ) {
            when {
                state.isLoading && state.lots.isEmpty() -> LoadingIndicator()
                state.error != null && state.lots.isEmpty() -> {
                    EmptyState(
                        message = state.error ?: "Error",
                        icon = Icons.Outlined.Checklist,
                        actionLabel = "Retry",
                        onAction = viewModel::loadInspectionLots
                    )
                }
                state.lots.isEmpty() -> {
                    EmptyState(
                        message = "No inspection lots found",
                        icon = Icons.Outlined.Checklist
                    )
                }
                else -> {
                    LazyColumn(
                        contentPadding = PaddingValues(horizontal = 16.dp, vertical = 8.dp),
                        verticalArrangement = Arrangement.spacedBy(8.dp)
                    ) {
                        items(state.lots, key = { it.id }) { lot ->
                            ErpCard(
                                title = lot.lotNumber,
                                subtitle = buildString {
                                    append("Type: ${lot.inspectionType}")
                                    if (lot.materialName != null) {
                                        append(" | ${lot.materialName}")
                                    }
                                    append(" | Qty: ${lot.quantity.toInt()}")
                                },
                                status = lot.status,
                                onClick = { onInspectionClick(lot.id) }
                            )
                        }
                    }
                }
            }
        }
    }
}
