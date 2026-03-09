package com.tastebyte.erp.features.warehouse

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.tastebyte.erp.core.network.ApiClient
import com.tastebyte.erp.models.Material
import com.tastebyte.erp.models.StockCountItem
import com.tastebyte.erp.models.StockCountRequest
import com.tastebyte.erp.models.Warehouse
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class WarehouseListState(
    val warehouses: List<Warehouse> = emptyList(),
    val isLoading: Boolean = false,
    val error: String? = null
)

data class StockCountState(
    val warehouses: List<Warehouse> = emptyList(),
    val selectedWarehouse: Warehouse? = null,
    val materials: List<Material> = emptyList(),
    val countEntries: Map<String, String> = emptyMap(),
    val isLoading: Boolean = false,
    val isSubmitting: Boolean = false,
    val error: String? = null,
    val successMessage: String? = null
)

class WarehouseViewModel : ViewModel() {

    private val _listState = MutableStateFlow(WarehouseListState())
    val listState: StateFlow<WarehouseListState> = _listState.asStateFlow()

    private val _stockCountState = MutableStateFlow(StockCountState())
    val stockCountState: StateFlow<StockCountState> = _stockCountState.asStateFlow()

    fun loadWarehouses() {
        viewModelScope.launch {
            _listState.value = WarehouseListState(isLoading = true)
            try {
                val response = ApiClient.getService().listWarehouses()
                if (response.success && response.data != null) {
                    _listState.value = WarehouseListState(
                        warehouses = response.data,
                        isLoading = false
                    )
                } else {
                    _listState.value = WarehouseListState(
                        isLoading = false,
                        error = response.error ?: "Failed to load warehouses"
                    )
                }
            } catch (e: Exception) {
                _listState.value = WarehouseListState(
                    isLoading = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }

    fun loadStockCountData() {
        viewModelScope.launch {
            _stockCountState.value = _stockCountState.value.copy(isLoading = true, error = null)
            try {
                val api = ApiClient.getService()
                val warehouseResponse = api.listWarehouses()
                val materialsResponse = api.listMaterials(pageSize = 100)

                _stockCountState.value = _stockCountState.value.copy(
                    warehouses = warehouseResponse.data ?: emptyList(),
                    materials = materialsResponse.data?.items ?: emptyList(),
                    isLoading = false
                )
            } catch (e: Exception) {
                _stockCountState.value = _stockCountState.value.copy(
                    isLoading = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }

    fun selectWarehouse(warehouse: Warehouse) {
        _stockCountState.value = _stockCountState.value.copy(
            selectedWarehouse = warehouse,
            countEntries = emptyMap()
        )
    }

    fun updateCountEntry(materialId: String, quantity: String) {
        val entries = _stockCountState.value.countEntries.toMutableMap()
        entries[materialId] = quantity
        _stockCountState.value = _stockCountState.value.copy(countEntries = entries)
    }

    fun submitStockCount() {
        val state = _stockCountState.value
        val warehouse = state.selectedWarehouse ?: return

        val items = state.countEntries
            .filter { it.value.isNotBlank() }
            .mapNotNull { (materialId, quantity) ->
                quantity.toDoubleOrNull()?.let { qty ->
                    StockCountItem(
                        materialId = materialId,
                        countedQuantity = qty
                    )
                }
            }

        if (items.isEmpty()) {
            _stockCountState.value = state.copy(error = "Enter at least one quantity")
            return
        }

        viewModelScope.launch {
            _stockCountState.value = state.copy(isSubmitting = true, error = null)
            try {
                val response = ApiClient.getService().submitStockCount(
                    StockCountRequest(
                        warehouseId = warehouse.id,
                        items = items
                    )
                )
                if (response.success) {
                    _stockCountState.value = _stockCountState.value.copy(
                        isSubmitting = false,
                        countEntries = emptyMap(),
                        successMessage = "Stock count submitted successfully"
                    )
                } else {
                    _stockCountState.value = _stockCountState.value.copy(
                        isSubmitting = false,
                        error = response.error ?: "Submission failed"
                    )
                }
            } catch (e: Exception) {
                _stockCountState.value = _stockCountState.value.copy(
                    isSubmitting = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }

    fun clearSuccessMessage() {
        _stockCountState.value = _stockCountState.value.copy(successMessage = null)
    }
}
