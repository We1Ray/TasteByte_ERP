package com.tastebyte.erp.features.materials

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.Inventory2
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
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
import com.tastebyte.erp.ui.SearchField

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MaterialsListScreen(
    viewModel: MaterialsViewModel,
    onMaterialClick: (String) -> Unit
) {
    val state by viewModel.listState.collectAsState()

    Column(modifier = Modifier.fillMaxSize()) {
        Text(
            text = "Materials",
            style = MaterialTheme.typography.headlineSmall,
            fontWeight = FontWeight.Bold,
            modifier = Modifier.padding(start = 16.dp, top = 16.dp, end = 16.dp)
        )

        SearchField(
            value = state.searchQuery,
            onValueChange = viewModel::onSearchQueryChanged,
            placeholder = "Search materials..."
        )

        PullToRefreshBox(
            isRefreshing = state.isLoading,
            onRefresh = viewModel::loadMaterials,
            modifier = Modifier.fillMaxSize()
        ) {
            when {
                state.isLoading && state.materials.isEmpty() -> {
                    LoadingIndicator()
                }
                state.error != null && state.materials.isEmpty() -> {
                    EmptyState(
                        message = state.error ?: "Error",
                        icon = Icons.Outlined.Inventory2,
                        actionLabel = "Retry",
                        onAction = viewModel::loadMaterials
                    )
                }
                state.materials.isEmpty() -> {
                    EmptyState(
                        message = if (state.searchQuery.isNotEmpty()) {
                            "No materials match \"${state.searchQuery}\""
                        } else {
                            "No materials found"
                        },
                        icon = Icons.Outlined.Inventory2
                    )
                }
                else -> {
                    LazyColumn(
                        contentPadding = PaddingValues(horizontal = 16.dp, vertical = 8.dp),
                        verticalArrangement = Arrangement.spacedBy(8.dp)
                    ) {
                        items(state.materials, key = { it.id }) { material ->
                            ErpCard(
                                title = material.name,
                                subtitle = "${material.materialNumber} | Type: ${material.materialType}",
                                status = if (material.isActive) "active" else "inactive",
                                onClick = { onMaterialClick(material.id) }
                            )
                        }
                    }
                }
            }
        }
    }
}
