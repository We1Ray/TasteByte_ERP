package com.tastebyte.erp.features.hr

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.tastebyte.erp.core.network.ApiClient
import com.tastebyte.erp.models.Attendance
import com.tastebyte.erp.models.Employee
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

data class AttendanceState(
    val todayAttendance: Attendance? = null,
    val isClockedIn: Boolean = false,
    val isLoading: Boolean = false,
    val isClocking: Boolean = false,
    val error: String? = null,
    val successMessage: String? = null
)

data class EmployeeListState(
    val employees: List<Employee> = emptyList(),
    val isLoading: Boolean = false,
    val error: String? = null,
    val searchQuery: String = ""
)

class HrViewModel : ViewModel() {

    private val _attendanceState = MutableStateFlow(AttendanceState())
    val attendanceState: StateFlow<AttendanceState> = _attendanceState.asStateFlow()

    private val _employeeListState = MutableStateFlow(EmployeeListState())
    val employeeListState: StateFlow<EmployeeListState> = _employeeListState.asStateFlow()

    init {
        loadTodayAttendance()
    }

    fun loadTodayAttendance() {
        viewModelScope.launch {
            _attendanceState.value = _attendanceState.value.copy(isLoading = true, error = null)
            try {
                val response = ApiClient.getService().getTodayAttendance()
                if (response.success && response.data != null) {
                    val attendance = response.data
                    _attendanceState.value = AttendanceState(
                        todayAttendance = attendance,
                        isClockedIn = attendance.clockIn != null && attendance.clockOut == null,
                        isLoading = false
                    )
                } else {
                    _attendanceState.value = AttendanceState(
                        isClockedIn = false,
                        isLoading = false
                    )
                }
            } catch (e: Exception) {
                _attendanceState.value = AttendanceState(
                    isLoading = false,
                    error = e.message ?: "Failed to load attendance"
                )
            }
        }
    }

    fun clockIn() {
        viewModelScope.launch {
            _attendanceState.value = _attendanceState.value.copy(
                isClocking = true,
                error = null,
                successMessage = null
            )
            try {
                val response = ApiClient.getService().clockIn()
                if (response.success && response.data != null) {
                    _attendanceState.value = _attendanceState.value.copy(
                        todayAttendance = response.data,
                        isClockedIn = true,
                        isClocking = false,
                        successMessage = "Clocked in successfully"
                    )
                } else {
                    _attendanceState.value = _attendanceState.value.copy(
                        isClocking = false,
                        error = response.error ?: "Clock in failed"
                    )
                }
            } catch (e: Exception) {
                _attendanceState.value = _attendanceState.value.copy(
                    isClocking = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }

    fun clockOut() {
        viewModelScope.launch {
            _attendanceState.value = _attendanceState.value.copy(
                isClocking = true,
                error = null,
                successMessage = null
            )
            try {
                val response = ApiClient.getService().clockOut()
                if (response.success && response.data != null) {
                    _attendanceState.value = _attendanceState.value.copy(
                        todayAttendance = response.data,
                        isClockedIn = false,
                        isClocking = false,
                        successMessage = "Clocked out successfully"
                    )
                } else {
                    _attendanceState.value = _attendanceState.value.copy(
                        isClocking = false,
                        error = response.error ?: "Clock out failed"
                    )
                }
            } catch (e: Exception) {
                _attendanceState.value = _attendanceState.value.copy(
                    isClocking = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }

    fun loadEmployees() {
        viewModelScope.launch {
            _employeeListState.value = _employeeListState.value.copy(isLoading = true, error = null)
            try {
                val response = ApiClient.getService().listEmployees(
                    search = _employeeListState.value.searchQuery.ifBlank { null }
                )
                if (response.success && response.data != null) {
                    _employeeListState.value = _employeeListState.value.copy(
                        employees = response.data.items,
                        isLoading = false
                    )
                } else {
                    _employeeListState.value = _employeeListState.value.copy(
                        isLoading = false,
                        error = response.error ?: "Failed to load employees"
                    )
                }
            } catch (e: Exception) {
                _employeeListState.value = _employeeListState.value.copy(
                    isLoading = false,
                    error = e.message ?: "Network error"
                )
            }
        }
    }

    fun onEmployeeSearchChanged(query: String) {
        _employeeListState.value = _employeeListState.value.copy(searchQuery = query)
        loadEmployees()
    }

    fun clearSuccessMessage() {
        _attendanceState.value = _attendanceState.value.copy(successMessage = null)
    }
}
