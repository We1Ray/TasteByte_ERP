package com.tastebyte.erp.features.reports

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.itemsIndexed
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.tastebyte.erp.features.materials.DetailRow
import com.tastebyte.erp.models.*
import com.tastebyte.erp.ui.EmptyState
import com.tastebyte.erp.ui.LoadingIndicator
import kotlinx.coroutines.flow.StateFlow

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ReportDetailScreen(
    title: String,
    state: StateFlow<ReportState<List<Any>>>,
    onLoad: () -> Unit,
    onBack: () -> Unit
) {
    val reportState by state.collectAsState()

    LaunchedEffect(Unit) {
        onLoad()
    }

    Column(modifier = Modifier.fillMaxSize()) {
        TopAppBar(
            title = { Text(title) },
            navigationIcon = {
                IconButton(onClick = onBack) {
                    Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Back")
                }
            }
        )

        when {
            reportState.isLoading -> LoadingIndicator()
            reportState.error != null -> {
                EmptyState(
                    message = reportState.error ?: "Error loading report",
                    actionLabel = "Retry",
                    onAction = onLoad
                )
            }
            reportState.data.isNullOrEmpty() -> {
                EmptyState(message = "No data available")
            }
            else -> {
                val items = reportState.data!!
                LazyColumn(
                    contentPadding = PaddingValues(16.dp),
                    verticalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    item {
                        Text(
                            text = "${items.size} records",
                            style = MaterialTheme.typography.bodyMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                    }

                    itemsIndexed(items) { index, item ->
                        ReportItemCard(item)
                    }
                }
            }
        }
    }
}

@Composable
fun ReportItemCard(item: Any) {
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
            when (item) {
                is TrialBalanceEntry -> {
                    Text(
                        text = "${item.accountNumber} - ${item.accountName}",
                        style = MaterialTheme.typography.titleSmall,
                        fontWeight = FontWeight.SemiBold
                    )
                    DetailRow("Debit", String.format("$%,.2f", item.debitBalance))
                    DetailRow("Credit", String.format("$%,.2f", item.creditBalance))
                }
                is IncomeStatementEntry -> {
                    Text(
                        text = item.category,
                        style = MaterialTheme.typography.labelMedium,
                        color = MaterialTheme.colorScheme.primary
                    )
                    DetailRow(item.description, String.format("$%,.2f", item.amount))
                }
                is BalanceSheetEntry -> {
                    Text(
                        text = item.category,
                        style = MaterialTheme.typography.labelMedium,
                        color = MaterialTheme.colorScheme.primary
                    )
                    DetailRow(item.description, String.format("$%,.2f", item.amount))
                }
                is AgingEntry -> {
                    Text(
                        text = item.partnerName ?: "Unknown",
                        style = MaterialTheme.typography.titleSmall,
                        fontWeight = FontWeight.SemiBold
                    )
                    DetailRow("Current", String.format("$%,.2f", item.currentAmount))
                    DetailRow("30 Days", String.format("$%,.2f", item.days30))
                    DetailRow("60 Days", String.format("$%,.2f", item.days60))
                    DetailRow("90 Days", String.format("$%,.2f", item.days90))
                    DetailRow("Over 90", String.format("$%,.2f", item.over90))
                    HorizontalDivider(modifier = Modifier.padding(vertical = 4.dp))
                    DetailRow("Total", String.format("$%,.2f", item.total))
                }
                is StockValuationEntry -> {
                    Text(
                        text = item.materialName ?: item.materialNumber ?: "Unknown",
                        style = MaterialTheme.typography.titleSmall,
                        fontWeight = FontWeight.SemiBold
                    )
                    if (item.materialNumber != null) {
                        DetailRow("Material #", item.materialNumber)
                    }
                    DetailRow("Quantity", String.format("%,.2f", item.quantity))
                    DetailRow("Unit Cost", String.format("$%,.2f", item.unitCost))
                    DetailRow("Total Value", String.format("$%,.2f", item.totalValue))
                }
                is MovementSummaryEntry -> {
                    Text(
                        text = item.movementType,
                        style = MaterialTheme.typography.titleSmall,
                        fontWeight = FontWeight.SemiBold
                    )
                    DetailRow("Count", item.count.toString())
                    DetailRow("Total Qty", String.format("%,.2f", item.totalQuantity))
                }
                is SlowMovingEntry -> {
                    Text(
                        text = item.materialName ?: item.materialNumber ?: "Unknown",
                        style = MaterialTheme.typography.titleSmall,
                        fontWeight = FontWeight.SemiBold
                    )
                    DetailRow("Quantity", String.format("%,.2f", item.quantity))
                    DetailRow("Last Movement", item.lastMovementDate ?: "Never")
                    DetailRow("Days Idle", item.daysSinceMovement.toString())
                }
                is SalesSummaryEntry -> {
                    Text(
                        text = item.period ?: "All Time",
                        style = MaterialTheme.typography.titleSmall,
                        fontWeight = FontWeight.SemiBold
                    )
                    DetailRow("Orders", item.orderCount.toString())
                    DetailRow("Total", String.format("$%,.2f", item.totalAmount))
                }
                is OrderFulfillmentEntry -> {
                    Text(
                        text = item.orderNumber ?: "Unknown",
                        style = MaterialTheme.typography.titleSmall,
                        fontWeight = FontWeight.SemiBold
                    )
                    DetailRow("Status", item.status)
                    DetailRow("Amount", String.format("$%,.2f", item.totalAmount))
                    DetailRow("Delivered", String.format("%.1f%%", item.deliveredPercentage))
                }
                is TopCustomerEntry -> {
                    Text(
                        text = item.customerName ?: "Unknown",
                        style = MaterialTheme.typography.titleSmall,
                        fontWeight = FontWeight.SemiBold
                    )
                    DetailRow("Orders", item.orderCount.toString())
                    DetailRow("Total", String.format("$%,.2f", item.totalAmount))
                }
                else -> {
                    Text(
                        text = item.toString(),
                        style = MaterialTheme.typography.bodyMedium
                    )
                }
            }
        }
    }
}
