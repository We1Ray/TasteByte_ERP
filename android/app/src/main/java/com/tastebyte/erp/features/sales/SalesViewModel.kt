package com.tastebyte.erp.features.sales

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.tastebyte.erp.core.network.ApiClient
import com.tastebyte.erp.models.SalesOrder
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class SalesOrdersListState(
    val orders: List<SalesOrder> = emptyList(),
    val isLoading: Boolean = false,
    val error: String? = null,
    val searchQuery: String = "",
    val currentPage: Int = 1,
    val totalItems: Int = 0
)

data class SalesOrderDetailState(
    val order: SalesOrder? = null,
    val isLoading: Boolean = false,
    val error: String? = null,
    val isConfirming: Boolean = false,
    val confirmSuccess: String? = null
)

class SalesViewModel : ViewModel() {

    private val _listState = MutableStateFlow(SalesOrdersListState())
    val listState: StateFlow<SalesOrdersListState> = _listState.asStateFlow()

    private val _detailState = MutableStateFlow(SalesOrderDetailState())
    val detailState: StateFlow<SalesOrderDetailState> = _detailState.asStateFlow()

    init {
        loadSalesOrders()
    }

    fun loadSalesOrders() {
        viewModelScope.launch {
            _listState.value = _listState.value.copy(isLoading = true, error = null)
            try {
                val response = ApiClient.getService().listSalesOrders(
                    page = _listState.value.currentPage,
                    search = _listState.value.searchQuery.ifBlank { null }
                )
                if (response.success && response.data != null) {
                    _listState.value = _listState.value.copy(
                        orders = response.data.items,
                        totalItems = response.data.total,
                        isLoading = false
                    )
                } else {
                    _listState.value = _listState.value.copy(
                        isLoading = false,
                        error = response.error ?: "Failed to load sales orders"
                    )
                }
            } catch (e: Exception) {
                _listState.value = _listState.value.copy(
                    isLoading = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }

    fun onSearchQueryChanged(query: String) {
        _listState.value = _listState.value.copy(searchQuery = query, currentPage = 1)
        loadSalesOrders()
    }

    fun loadSalesOrderDetail(id: String) {
        viewModelScope.launch {
            _detailState.value = SalesOrderDetailState(isLoading = true)
            try {
                val response = ApiClient.getService().getSalesOrder(id)
                if (response.success && response.data != null) {
                    _detailState.value = SalesOrderDetailState(
                        order = response.data,
                        isLoading = false
                    )
                } else {
                    _detailState.value = SalesOrderDetailState(
                        isLoading = false,
                        error = response.error ?: "Failed to load order"
                    )
                }
            } catch (e: Exception) {
                _detailState.value = SalesOrderDetailState(
                    isLoading = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }

    fun confirmSalesOrder(id: String) {
        viewModelScope.launch {
            _detailState.value = _detailState.value.copy(
                isConfirming = true,
                confirmSuccess = null,
                error = null
            )
            try {
                val response = ApiClient.getService().confirmSalesOrder(id)
                if (response.success && response.data != null) {
                    _detailState.value = _detailState.value.copy(
                        order = response.data,
                        isConfirming = false,
                        confirmSuccess = "Order confirmed successfully"
                    )
                } else {
                    _detailState.value = _detailState.value.copy(
                        isConfirming = false,
                        error = response.error ?: "Failed to confirm order"
                    )
                }
            } catch (e: Exception) {
                _detailState.value = _detailState.value.copy(
                    isConfirming = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }
}
