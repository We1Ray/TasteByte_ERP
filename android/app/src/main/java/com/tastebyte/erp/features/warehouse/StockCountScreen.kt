package com.tastebyte.erp.features.warehouse

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.ExposedDropdownMenuBox
import androidx.compose.material3.ExposedDropdownMenuDefaults
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.MenuAnchorType
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import com.tastebyte.erp.ui.LoadingIndicator

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun StockCountScreen(
    viewModel: WarehouseViewModel,
    onBack: () -> Unit
) {
    val state by viewModel.stockCountState.collectAsState()
    val snackbarHostState = remember { SnackbarHostState() }
    var warehouseDropdownExpanded by remember { mutableStateOf(false) }

    LaunchedEffect(Unit) {
        viewModel.loadStockCountData()
    }

    LaunchedEffect(state.successMessage) {
        state.successMessage?.let {
            snackbarHostState.showSnackbar(it)
            viewModel.clearSuccessMessage()
        }
    }

    Column(modifier = Modifier.fillMaxSize()) {
        TopAppBar(
            title = { Text("Stock Count") },
            navigationIcon = {
                IconButton(onClick = onBack) {
                    Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Back")
                }
            }
        )

        if (state.isLoading) {
            LoadingIndicator()
        } else {
            LazyColumn(
                modifier = Modifier
                    .fillMaxSize()
                    .weight(1f),
                contentPadding = androidx.compose.foundation.layout.PaddingValues(16.dp),
                verticalArrangement = Arrangement.spacedBy(12.dp)
            ) {
                // Warehouse Selector
                item {
                    ExposedDropdownMenuBox(
                        expanded = warehouseDropdownExpanded,
                        onExpandedChange = { warehouseDropdownExpanded = it }
                    ) {
                        OutlinedTextField(
                            value = state.selectedWarehouse?.name ?: "",
                            onValueChange = {},
                            readOnly = true,
                            label = { Text("Select Warehouse") },
                            trailingIcon = {
                                ExposedDropdownMenuDefaults.TrailingIcon(expanded = warehouseDropdownExpanded)
                            },
                            modifier = Modifier
                                .fillMaxWidth()
                                .menuAnchor(MenuAnchorType.PrimaryNotEditable)
                        )
                        ExposedDropdownMenu(
                            expanded = warehouseDropdownExpanded,
                            onDismissRequest = { warehouseDropdownExpanded = false }
                        ) {
                            state.warehouses.forEach { warehouse ->
                                DropdownMenuItem(
                                    text = {
                                        Text("${warehouse.warehouseNumber} - ${warehouse.name}")
                                    },
                                    onClick = {
                                        viewModel.selectWarehouse(warehouse)
                                        warehouseDropdownExpanded = false
                                    }
                                )
                            }
                        }
                    }
                }

                if (state.selectedWarehouse != null) {
                    item {
                        Text(
                            text = "Enter counted quantities:",
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.SemiBold
                        )
                    }

                    items(state.materials, key = { it.id }) { material ->
                        Card(
                            modifier = Modifier.fillMaxWidth(),
                            colors = CardDefaults.cardColors(
                                containerColor = MaterialTheme.colorScheme.surface
                            ),
                            elevation = CardDefaults.cardElevation(defaultElevation = 1.dp)
                        ) {
                            Row(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .padding(12.dp),
                                verticalAlignment = Alignment.CenterVertically
                            ) {
                                Column(modifier = Modifier.weight(1f)) {
                                    Text(
                                        text = material.name,
                                        style = MaterialTheme.typography.bodyMedium,
                                        fontWeight = FontWeight.Medium
                                    )
                                    Text(
                                        text = material.materialNumber,
                                        style = MaterialTheme.typography.bodySmall,
                                        color = MaterialTheme.colorScheme.onSurfaceVariant
                                    )
                                }
                                Spacer(modifier = Modifier.width(12.dp))
                                OutlinedTextField(
                                    value = state.countEntries[material.id] ?: "",
                                    onValueChange = { viewModel.updateCountEntry(material.id, it) },
                                    label = { Text("Qty") },
                                    keyboardOptions = KeyboardOptions(
                                        keyboardType = KeyboardType.Decimal
                                    ),
                                    singleLine = true,
                                    modifier = Modifier.width(100.dp)
                                )
                            }
                        }
                    }
                }

                // Error
                if (state.error != null) {
                    item {
                        Text(
                            text = state.error!!,
                            color = MaterialTheme.colorScheme.error,
                            style = MaterialTheme.typography.bodyMedium
                        )
                    }
                }
            }

            // Submit button
            if (state.selectedWarehouse != null) {
                Button(
                    onClick = viewModel::submitStockCount,
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(16.dp)
                        .height(48.dp),
                    enabled = !state.isSubmitting &&
                            state.countEntries.values.any { it.isNotBlank() }
                ) {
                    if (state.isSubmitting) {
                        CircularProgressIndicator(
                            modifier = Modifier.size(20.dp),
                            color = MaterialTheme.colorScheme.onPrimary,
                            strokeWidth = 2.dp
                        )
                    } else {
                        Text("Submit Stock Count")
                    }
                }
            }
        }

        SnackbarHost(hostState = snackbarHostState)
    }
}
