package com.tastebyte.erp.features.production

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.tastebyte.erp.core.network.ApiClient
import com.tastebyte.erp.models.ProductionOrder
import com.tastebyte.erp.models.ProductionOrderStatusRequest
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class ProductionOrdersListState(
    val orders: List<ProductionOrder> = emptyList(),
    val isLoading: Boolean = false,
    val error: String? = null,
    val currentPage: Int = 1,
    val totalItems: Int = 0
)

data class ProductionOrderDetailState(
    val order: ProductionOrder? = null,
    val isLoading: Boolean = false,
    val error: String? = null,
    val isUpdating: Boolean = false,
    val updateSuccess: String? = null
)

class ProductionViewModel : ViewModel() {

    private val _listState = MutableStateFlow(ProductionOrdersListState())
    val listState: StateFlow<ProductionOrdersListState> = _listState.asStateFlow()

    private val _detailState = MutableStateFlow(ProductionOrderDetailState())
    val detailState: StateFlow<ProductionOrderDetailState> = _detailState.asStateFlow()

    init {
        loadProductionOrders()
    }

    fun loadProductionOrders() {
        viewModelScope.launch {
            _listState.value = _listState.value.copy(isLoading = true, error = null)
            try {
                val response = ApiClient.getService().listProductionOrders(
                    page = _listState.value.currentPage
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
                        error = response.error ?: "Failed to load production orders"
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

    fun loadProductionOrderDetail(id: String) {
        viewModelScope.launch {
            _detailState.value = ProductionOrderDetailState(isLoading = true)
            try {
                val response = ApiClient.getService().getProductionOrder(id)
                if (response.success && response.data != null) {
                    _detailState.value = ProductionOrderDetailState(
                        order = response.data,
                        isLoading = false
                    )
                } else {
                    _detailState.value = ProductionOrderDetailState(
                        isLoading = false,
                        error = response.error ?: "Failed to load production order"
                    )
                }
            } catch (e: Exception) {
                _detailState.value = ProductionOrderDetailState(
                    isLoading = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }

    fun updateStatus(id: String, newStatus: String) {
        viewModelScope.launch {
            _detailState.value = _detailState.value.copy(
                isUpdating = true,
                updateSuccess = null,
                error = null
            )
            try {
                val response = ApiClient.getService().updateProductionOrderStatus(
                    id,
                    ProductionOrderStatusRequest(newStatus)
                )
                if (response.success && response.data != null) {
                    _detailState.value = _detailState.value.copy(
                        order = response.data,
                        isUpdating = false,
                        updateSuccess = "Status updated to ${newStatus.replace("_", " ")}"
                    )
                } else {
                    _detailState.value = _detailState.value.copy(
                        isUpdating = false,
                        error = response.error ?: "Failed to update status"
                    )
                }
            } catch (e: Exception) {
                _detailState.value = _detailState.value.copy(
                    isUpdating = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }
}
