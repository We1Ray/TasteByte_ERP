package com.tastebyte.erp.features.materials

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.tastebyte.erp.core.network.ApiClient
import com.tastebyte.erp.models.Material
import com.tastebyte.erp.models.PlantStock
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class MaterialsListState(
    val materials: List<Material> = emptyList(),
    val isLoading: Boolean = false,
    val error: String? = null,
    val searchQuery: String = "",
    val currentPage: Int = 1,
    val totalItems: Int = 0
)

data class MaterialDetailState(
    val material: Material? = null,
    val stockLevels: List<PlantStock> = emptyList(),
    val isLoading: Boolean = false,
    val error: String? = null
)

data class StockOverviewState(
    val stockItems: List<PlantStock> = emptyList(),
    val isLoading: Boolean = false,
    val error: String? = null
)

class MaterialsViewModel : ViewModel() {

    private val _listState = MutableStateFlow(MaterialsListState())
    val listState: StateFlow<MaterialsListState> = _listState.asStateFlow()

    private val _detailState = MutableStateFlow(MaterialDetailState())
    val detailState: StateFlow<MaterialDetailState> = _detailState.asStateFlow()

    private val _stockState = MutableStateFlow(StockOverviewState())
    val stockState: StateFlow<StockOverviewState> = _stockState.asStateFlow()

    init {
        loadMaterials()
    }

    fun loadMaterials() {
        viewModelScope.launch {
            _listState.value = _listState.value.copy(isLoading = true, error = null)
            try {
                val response = ApiClient.getService().listMaterials(
                    page = _listState.value.currentPage,
                    search = _listState.value.searchQuery.ifBlank { null }
                )
                if (response.success && response.data != null) {
                    _listState.value = _listState.value.copy(
                        materials = response.data.items,
                        totalItems = response.data.total,
                        isLoading = false
                    )
                } else {
                    _listState.value = _listState.value.copy(
                        isLoading = false,
                        error = response.error ?: "Failed to load materials"
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
        loadMaterials()
    }

    fun loadMaterialDetail(id: String) {
        viewModelScope.launch {
            _detailState.value = MaterialDetailState(isLoading = true)
            try {
                val api = ApiClient.getService()
                val materialResponse = api.getMaterial(id)
                val stockResponse = try {
                    api.listPlantStock(materialId = id)
                } catch (e: Exception) {
                    null
                }

                if (materialResponse.success && materialResponse.data != null) {
                    _detailState.value = MaterialDetailState(
                        material = materialResponse.data,
                        stockLevels = stockResponse?.data ?: emptyList(),
                        isLoading = false
                    )
                } else {
                    _detailState.value = MaterialDetailState(
                        isLoading = false,
                        error = materialResponse.error ?: "Failed to load material"
                    )
                }
            } catch (e: Exception) {
                _detailState.value = MaterialDetailState(
                    isLoading = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }

    fun loadStockOverview() {
        viewModelScope.launch {
            _stockState.value = StockOverviewState(isLoading = true)
            try {
                val response = ApiClient.getService().listPlantStock()
                if (response.success && response.data != null) {
                    _stockState.value = StockOverviewState(
                        stockItems = response.data,
                        isLoading = false
                    )
                } else {
                    _stockState.value = StockOverviewState(
                        isLoading = false,
                        error = response.error ?: "Failed to load stock"
                    )
                }
            } catch (e: Exception) {
                _stockState.value = StockOverviewState(
                    isLoading = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }
}
