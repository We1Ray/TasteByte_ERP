package com.tastebyte.erp.features.reports

import androidx.compose.runtime.Composable

@Composable
fun MmReportsScreen(
    onReportClick: (String) -> Unit,
    onBack: () -> Unit
) {
    val reports = listOf(
        ReportItem(
            title = "Stock Valuation",
            description = "Current inventory value by material",
            route = "reports/mm/stock-valuation"
        ),
        ReportItem(
            title = "Movement Summary",
            description = "Material movement statistics by type",
            route = "reports/mm/movement-summary"
        ),
        ReportItem(
            title = "Slow Moving Items",
            description = "Materials with low or no movement",
            route = "reports/mm/slow-moving"
        )
    )

    ReportListScreen(
        title = "Materials Reports",
        reports = reports,
        onReportClick = onReportClick,
        onBack = onBack
    )
}
