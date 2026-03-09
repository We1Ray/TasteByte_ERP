package com.tastebyte.erp.features.purchasing

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.tastebyte.erp.core.network.ApiClient
import com.tastebyte.erp.models.PurchaseOrder
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class PurchaseOrdersListState(
    val orders: List<PurchaseOrder> = emptyList(),
    val isLoading: Boolean = false,
    val error: String? = null,
    val currentPage: Int = 1,
    val totalItems: Int = 0
)

data class PurchaseOrderDetailState(
    val order: PurchaseOrder? = null,
    val isLoading: Boolean = false,
    val error: String? = null,
    val isReceiving: Boolean = false,
    val receiveSuccess: String? = null
)

class PurchasingViewModel : ViewModel() {

    private val _listState = MutableStateFlow(PurchaseOrdersListState())
    val listState: StateFlow<PurchaseOrdersListState> = _listState.asStateFlow()

    private val _detailState = MutableStateFlow(PurchaseOrderDetailState())
    val detailState: StateFlow<PurchaseOrderDetailState> = _detailState.asStateFlow()

    init {
        loadPurchaseOrders()
    }

    fun loadPurchaseOrders() {
        viewModelScope.launch {
            _listState.value = _listState.value.copy(isLoading = true, error = null)
            try {
                val response = ApiClient.getService().listPurchaseOrders(
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
                        error = response.error ?: "Failed to load purchase orders"
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

    fun loadPurchaseOrderDetail(id: String) {
        viewModelScope.launch {
            _detailState.value = PurchaseOrderDetailState(isLoading = true)
            try {
                val response = ApiClient.getService().getPurchaseOrder(id)
                if (response.success && response.data != null) {
                    _detailState.value = PurchaseOrderDetailState(
                        order = response.data,
                        isLoading = false
                    )
                } else {
                    _detailState.value = PurchaseOrderDetailState(
                        isLoading = false,
                        error = response.error ?: "Failed to load purchase order"
                    )
                }
            } catch (e: Exception) {
                _detailState.value = PurchaseOrderDetailState(
                    isLoading = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }

    fun receivePurchaseOrder(id: String) {
        viewModelScope.launch {
            _detailState.value = _detailState.value.copy(
                isReceiving = true,
                receiveSuccess = null,
                error = null
            )
            try {
                val response = ApiClient.getService().receivePurchaseOrder(id)
                if (response.success && response.data != null) {
                    _detailState.value = _detailState.value.copy(
                        order = response.data,
                        isReceiving = false,
                        receiveSuccess = "Goods received successfully"
                    )
                } else {
                    _detailState.value = _detailState.value.copy(
                        isReceiving = false,
                        error = response.error ?: "Failed to receive goods"
                    )
                }
            } catch (e: Exception) {
                _detailState.value = _detailState.value.copy(
                    isReceiving = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }
}
