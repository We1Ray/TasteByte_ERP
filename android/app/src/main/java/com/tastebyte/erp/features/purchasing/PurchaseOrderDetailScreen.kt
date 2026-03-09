package com.tastebyte.erp.features.purchasing

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.tastebyte.erp.features.materials.DetailRow
import com.tastebyte.erp.ui.LoadingIndicator
import com.tastebyte.erp.ui.StatusBadge

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun PurchaseOrderDetailScreen(
    orderId: String,
    viewModel: PurchasingViewModel,
    onBack: () -> Unit
) {
    val state by viewModel.detailState.collectAsState()
    val snackbarHostState = remember { SnackbarHostState() }

    LaunchedEffect(orderId) {
        viewModel.loadPurchaseOrderDetail(orderId)
    }

    LaunchedEffect(state.receiveSuccess) {
        state.receiveSuccess?.let {
            snackbarHostState.showSnackbar(it)
        }
    }

    Column(modifier = Modifier.fillMaxSize()) {
        TopAppBar(
            title = { Text("PO Details") },
            navigationIcon = {
                IconButton(onClick = onBack) {
                    Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Back")
                }
            }
        )

        SnackbarHost(hostState = snackbarHostState)

        when {
            state.isLoading -> LoadingIndicator()
            state.error != null && state.order == null -> {
                Text(
                    text = state.error ?: "Error",
                    color = MaterialTheme.colorScheme.error,
                    modifier = Modifier.padding(16.dp)
                )
            }
            state.order != null -> {
                val order = state.order!!
                LazyColumn(
                    contentPadding = PaddingValues(16.dp),
                    verticalArrangement = Arrangement.spacedBy(16.dp)
                ) {
                    item {
                        Card(
                            modifier = Modifier.fillMaxWidth(),
                            colors = CardDefaults.cardColors(
                                containerColor = MaterialTheme.colorScheme.surface
                            ),
                            elevation = CardDefaults.cardElevation(defaultElevation = 1.dp)
                        ) {
                            Column(
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .padding(16.dp)
                            ) {
                                Row(
                                    modifier = Modifier.fillMaxWidth(),
                                    horizontalArrangement = Arrangement.SpaceBetween,
                                    verticalAlignment = Alignment.CenterVertically
                                ) {
                                    Text(
                                        text = order.poNumber,
                                        style = MaterialTheme.typography.titleLarge,
                                        fontWeight = FontWeight.Bold
                                    )
                                    StatusBadge(status = order.status)
                                }

                                Spacer(modifier = Modifier.height(16.dp))
                                HorizontalDivider()
                                Spacer(modifier = Modifier.height(12.dp))

                                DetailRow("Order Date", order.orderDate)
                                if (order.deliveryDate != null) {
                                    DetailRow("Delivery Date", order.deliveryDate)
                                }
                                DetailRow("Vendor ID", order.vendorId.take(8) + "...")
                                DetailRow("Currency", order.currency)
                                DetailRow(
                                    "Total Amount",
                                    "${order.currency} ${String.format("%.2f", order.totalAmount)}"
                                )
                                if (order.notes != null) {
                                    Spacer(modifier = Modifier.height(8.dp))
                                    Text(
                                        text = "Notes",
                                        style = MaterialTheme.typography.labelMedium,
                                        color = MaterialTheme.colorScheme.onSurfaceVariant
                                    )
                                    Text(
                                        text = order.notes,
                                        style = MaterialTheme.typography.bodyMedium
                                    )
                                }
                                Spacer(modifier = Modifier.height(8.dp))
                                DetailRow("Created", order.createdAt.take(10))
                                DetailRow("Updated", order.updatedAt.take(10))
                            }
                        }
                    }

                    // Receive button for released/open orders
                    if (order.status.lowercase() in listOf("released", "open", "approved")) {
                        item {
                            Button(
                                onClick = { viewModel.receivePurchaseOrder(order.id) },
                                modifier = Modifier.fillMaxWidth(),
                                enabled = !state.isReceiving
                            ) {
                                if (state.isReceiving) {
                                    CircularProgressIndicator(
                                        modifier = Modifier.height(20.dp),
                                        strokeWidth = 2.dp,
                                        color = MaterialTheme.colorScheme.onPrimary
                                    )
                                } else {
                                    Text("Receive Goods")
                                }
                            }
                        }
                    }

                    if (state.error != null && state.order != null) {
                        item {
                            Text(
                                text = state.error!!,
                                color = MaterialTheme.colorScheme.error,
                                style = MaterialTheme.typography.bodyMedium
                            )
                        }
                    }
                }
            }
        }
    }
}
