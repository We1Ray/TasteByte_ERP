package com.tastebyte.erp.features.reports

import androidx.compose.runtime.Composable

@Composable
fun SdReportsScreen(
    onReportClick: (String) -> Unit,
    onBack: () -> Unit
) {
    val reports = listOf(
        ReportItem(
            title = "Sales Summary",
            description = "Sales totals by period",
            route = "reports/sd/sales-summary"
        ),
        ReportItem(
            title = "Order Fulfillment",
            description = "Order delivery status and completion rates",
            route = "reports/sd/order-fulfillment"
        ),
        ReportItem(
            title = "Top Customers",
            description = "Highest value customers by order volume",
            route = "reports/sd/top-customers"
        )
    )

    ReportListScreen(
        title = "Sales Reports",
        reports = reports,
        onReportClick = onReportClick,
        onBack = onBack
    )
}
