package com.tastebyte.erp.features.quality

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.tastebyte.erp.core.network.ApiClient
import com.tastebyte.erp.models.CharacteristicResult
import com.tastebyte.erp.models.InspectionLot
import com.tastebyte.erp.models.InspectionResultsRequest
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class InspectionListState(
    val lots: List<InspectionLot> = emptyList(),
    val isLoading: Boolean = false,
    val error: String? = null,
    val statusFilter: String? = null
)

data class InspectionFormState(
    val lot: InspectionLot? = null,
    val measuredValues: Map<String, String> = emptyMap(),
    val characteristicResults: Map<String, String> = emptyMap(),
    val overallResult: String = "",
    val notes: String = "",
    val isLoading: Boolean = false,
    val isSubmitting: Boolean = false,
    val error: String? = null,
    val submitSuccess: Boolean = false
)

class QualityViewModel : ViewModel() {

    private val _listState = MutableStateFlow(InspectionListState())
    val listState: StateFlow<InspectionListState> = _listState.asStateFlow()

    private val _formState = MutableStateFlow(InspectionFormState())
    val formState: StateFlow<InspectionFormState> = _formState.asStateFlow()

    fun loadInspectionLots() {
        viewModelScope.launch {
            _listState.value = _listState.value.copy(isLoading = true, error = null)
            try {
                val response = ApiClient.getService().listInspectionLots(
                    status = _listState.value.statusFilter
                )
                if (response.success && response.data != null) {
                    _listState.value = _listState.value.copy(
                        lots = response.data.items,
                        isLoading = false
                    )
                } else {
                    _listState.value = _listState.value.copy(
                        isLoading = false,
                        error = response.error ?: "Failed to load inspection lots"
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

    fun setStatusFilter(status: String?) {
        _listState.value = _listState.value.copy(statusFilter = status)
        loadInspectionLots()
    }

    fun loadInspectionLot(id: String) {
        viewModelScope.launch {
            _formState.value = InspectionFormState(isLoading = true)
            try {
                val response = ApiClient.getService().getInspectionLot(id)
                if (response.success && response.data != null) {
                    _formState.value = InspectionFormState(
                        lot = response.data,
                        isLoading = false
                    )
                } else {
                    _formState.value = InspectionFormState(
                        isLoading = false,
                        error = response.error ?: "Failed to load inspection lot"
                    )
                }
            } catch (e: Exception) {
                _formState.value = InspectionFormState(
                    isLoading = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }

    fun updateMeasuredValue(characteristicId: String, value: String) {
        val values = _formState.value.measuredValues.toMutableMap()
        values[characteristicId] = value
        _formState.value = _formState.value.copy(measuredValues = values)
    }

    fun updateCharacteristicResult(characteristicId: String, result: String) {
        val results = _formState.value.characteristicResults.toMutableMap()
        results[characteristicId] = result
        _formState.value = _formState.value.copy(characteristicResults = results)
    }

    fun updateOverallResult(result: String) {
        _formState.value = _formState.value.copy(overallResult = result)
    }

    fun updateNotes(notes: String) {
        _formState.value = _formState.value.copy(notes = notes)
    }

    fun submitResults() {
        val state = _formState.value
        val lot = state.lot ?: return

        if (state.overallResult.isBlank()) {
            _formState.value = state.copy(error = "Please select an overall result")
            return
        }

        val characteristics = lot.characteristics?.mapNotNull { char ->
            val value = state.measuredValues[char.id]?.toDoubleOrNull()
            val result = state.characteristicResults[char.id]
            if (value != null && result != null) {
                CharacteristicResult(
                    characteristicId = char.id,
                    measuredValue = value,
                    result = result
                )
            } else {
                null
            }
        } ?: emptyList()

        viewModelScope.launch {
            _formState.value = state.copy(isSubmitting = true, error = null)
            try {
                val response = ApiClient.getService().submitInspectionResults(
                    id = lot.id,
                    request = InspectionResultsRequest(
                        result = state.overallResult,
                        characteristics = characteristics,
                        notes = state.notes.ifBlank { null }
                    )
                )
                if (response.success) {
                    _formState.value = _formState.value.copy(
                        isSubmitting = false,
                        submitSuccess = true
                    )
                } else {
                    _formState.value = _formState.value.copy(
                        isSubmitting = false,
                        error = response.error ?: "Submission failed"
                    )
                }
            } catch (e: Exception) {
                _formState.value = _formState.value.copy(
                    isSubmitting = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }
}
