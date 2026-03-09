package com.tastebyte.erp.features.reports

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.tastebyte.erp.core.network.ApiClient
import com.tastebyte.erp.core.network.ApiResponse
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class ReportState<T>(
    val data: T? = null,
    val isLoading: Boolean = false,
    val error: String? = null
)

class ReportsViewModel : ViewModel() {

    // FI Reports
    private val _trialBalance = MutableStateFlow(ReportState<List<Any>>())
    val trialBalance: StateFlow<ReportState<List<Any>>> = _trialBalance.asStateFlow()

    private val _incomeStatement = MutableStateFlow(ReportState<List<Any>>())
    val incomeStatement: StateFlow<ReportState<List<Any>>> = _incomeStatement.asStateFlow()

    private val _balanceSheet = MutableStateFlow(ReportState<List<Any>>())
    val balanceSheet: StateFlow<ReportState<List<Any>>> = _balanceSheet.asStateFlow()

    private val _arAging = MutableStateFlow(ReportState<List<Any>>())
    val arAging: StateFlow<ReportState<List<Any>>> = _arAging.asStateFlow()

    private val _apAging = MutableStateFlow(ReportState<List<Any>>())
    val apAging: StateFlow<ReportState<List<Any>>> = _apAging.asStateFlow()

    // MM Reports
    private val _stockValuation = MutableStateFlow(ReportState<List<Any>>())
    val stockValuation: StateFlow<ReportState<List<Any>>> = _stockValuation.asStateFlow()

    private val _movementSummary = MutableStateFlow(ReportState<List<Any>>())
    val movementSummary: StateFlow<ReportState<List<Any>>> = _movementSummary.asStateFlow()

    private val _slowMoving = MutableStateFlow(ReportState<List<Any>>())
    val slowMoving: StateFlow<ReportState<List<Any>>> = _slowMoving.asStateFlow()

    // SD Reports
    private val _salesSummary = MutableStateFlow(ReportState<List<Any>>())
    val salesSummary: StateFlow<ReportState<List<Any>>> = _salesSummary.asStateFlow()

    private val _orderFulfillment = MutableStateFlow(ReportState<List<Any>>())
    val orderFulfillment: StateFlow<ReportState<List<Any>>> = _orderFulfillment.asStateFlow()

    private val _topCustomers = MutableStateFlow(ReportState<List<Any>>())
    val topCustomers: StateFlow<ReportState<List<Any>>> = _topCustomers.asStateFlow()

    private fun <T> loadReport(
        state: MutableStateFlow<ReportState<List<Any>>>,
        apiCall: suspend () -> ApiResponse<List<T>>
    ) {
        viewModelScope.launch {
            state.value = ReportState(isLoading = true)
            try {
                val response = apiCall()
                if (response.success && response.data != null) {
                    @Suppress("UNCHECKED_CAST")
                    state.value = ReportState(data = response.data as List<Any>, isLoading = false)
                } else {
                    state.value = ReportState(
                        isLoading = false,
                        error = response.error ?: "Failed to load report"
                    )
                }
            } catch (e: Exception) {
                state.value = ReportState(
                    isLoading = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }

    // FI report loaders
    fun loadTrialBalance() = loadReport(_trialBalance) {
        ApiClient.getService().getTrialBalance()
    }

    fun loadIncomeStatement() = loadReport(_incomeStatement) {
        ApiClient.getService().getIncomeStatement()
    }

    fun loadBalanceSheet() = loadReport(_balanceSheet) {
        ApiClient.getService().getBalanceSheet()
    }

    fun loadArAging() = loadReport(_arAging) {
        ApiClient.getService().getArAging()
    }

    fun loadApAging() = loadReport(_apAging) {
        ApiClient.getService().getApAging()
    }

    // MM report loaders
    fun loadStockValuation() = loadReport(_stockValuation) {
        ApiClient.getService().getStockValuation()
    }

    fun loadMovementSummary() = loadReport(_movementSummary) {
        ApiClient.getService().getMovementSummary()
    }

    fun loadSlowMoving() = loadReport(_slowMoving) {
        ApiClient.getService().getSlowMoving()
    }

    // SD report loaders
    fun loadSalesSummary() = loadReport(_salesSummary) {
        ApiClient.getService().getSalesSummary()
    }

    fun loadOrderFulfillment() = loadReport(_orderFulfillment) {
        ApiClient.getService().getOrderFulfillment()
    }

    fun loadTopCustomers() = loadReport(_topCustomers) {
        ApiClient.getService().getTopCustomers()
    }
}
