package com.tastebyte.erp.features.reports

import androidx.compose.runtime.Composable

@Composable
fun FiReportsScreen(
    onReportClick: (String) -> Unit,
    onBack: () -> Unit
) {
    val reports = listOf(
        ReportItem(
            title = "Trial Balance",
            description = "Account balances with debit and credit totals",
            route = "reports/fi/trial-balance"
        ),
        ReportItem(
            title = "Income Statement",
            description = "Revenue and expense summary",
            route = "reports/fi/income-statement"
        ),
        ReportItem(
            title = "Balance Sheet",
            description = "Assets, liabilities and equity",
            route = "reports/fi/balance-sheet"
        ),
        ReportItem(
            title = "AR Aging",
            description = "Accounts receivable aging analysis",
            route = "reports/fi/ar-aging"
        ),
        ReportItem(
            title = "AP Aging",
            description = "Accounts payable aging analysis",
            route = "reports/fi/ap-aging"
        )
    )

    ReportListScreen(
        title = "Financial Reports",
        reports = reports,
        onReportClick = onReportClick,
        onBack = onBack
    )
}
