package com.tastebyte.erp.features.dashboard

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.tastebyte.erp.core.network.ApiClient
import com.tastebyte.erp.models.Material
import com.tastebyte.erp.models.SalesOrder
import kotlinx.coroutines.async
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class KpiData(
    val title: String,
    val value: String,
    val subtitle: String
)

data class DashboardUiState(
    val kpis: List<KpiData> = emptyList(),
    val recentMaterials: List<Material> = emptyList(),
    val recentSalesOrders: List<SalesOrder> = emptyList(),
    val isLoading: Boolean = false,
    val error: String? = null
)

class DashboardViewModel : ViewModel() {

    private val _uiState = MutableStateFlow(DashboardUiState())
    val uiState: StateFlow<DashboardUiState> = _uiState.asStateFlow()

    init {
        loadDashboard()
    }

    fun loadDashboard() {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(isLoading = true, error = null)
            try {
                val api = ApiClient.getService()

                val kpisDeferred = async {
                    try {
                        api.getDashboardKpis()
                    } catch (e: Exception) {
                        null
                    }
                }
                val materialsDeferred = async {
                    try {
                        api.listMaterials(page = 1, pageSize = 5)
                    } catch (e: Exception) {
                        null
                    }
                }
                val salesOrdersDeferred = async {
                    try {
                        api.listSalesOrders(page = 1, pageSize = 5)
                    } catch (e: Exception) {
                        null
                    }
                }

                val kpisResponse = kpisDeferred.await()
                val materialsResponse = materialsDeferred.await()
                val salesOrdersResponse = salesOrdersDeferred.await()

                val kpis = if (kpisResponse?.success == true && kpisResponse.data != null) {
                    val d = kpisResponse.data
                    listOf(
                        KpiData(
                            "Total Revenue",
                            String.format("$%,.0f", d.totalRevenue),
                            "All time revenue"
                        ),
                        KpiData(
                            "Orders",
                            d.totalOrderCount.toString(),
                            "Total order count"
                        ),
                        KpiData(
                            "Inventory",
                            String.format("%,.0f", d.totalInventoryQuantity),
                            "Total stock quantity"
                        ),
                        KpiData(
                            "Production",
                            d.pendingProductionOrders.toString(),
                            "Pending orders"
                        ),
                        KpiData(
                            "Open AR",
                            String.format("$%,.0f", d.openArAmount),
                            "Accounts receivable"
                        ),
                        KpiData(
                            "Open AP",
                            String.format("$%,.0f", d.openApAmount),
                            "Accounts payable"
                        )
                    )
                } else {
                    // Fallback to derived KPIs
                    val materialCount = materialsResponse?.data?.total ?: 0
                    val salesOrderCount = salesOrdersResponse?.data?.total ?: 0
                    listOf(
                        KpiData("Materials", materialCount.toString(), "Total registered"),
                        KpiData("Sales Orders", salesOrderCount.toString(), "Total orders"),
                        KpiData("Stock Value", "N/A", "Total quantity"),
                        KpiData("Open Orders", "N/A", "Pending fulfillment")
                    )
                }

                _uiState.value = DashboardUiState(
                    kpis = kpis,
                    recentMaterials = materialsResponse?.data?.items ?: emptyList(),
                    recentSalesOrders = salesOrdersResponse?.data?.items ?: emptyList(),
                    isLoading = false
                )
            } catch (e: Exception) {
                _uiState.value = _uiState.value.copy(
                    isLoading = false,
                    error = e.message ?: "Failed to load dashboard"
                )
            }
        }
    }
}
