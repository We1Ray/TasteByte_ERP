package com.tastebyte.erp.features.hr

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.outlined.People
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.tastebyte.erp.ui.EmptyState
import com.tastebyte.erp.ui.ErpCard
import com.tastebyte.erp.ui.LoadingIndicator
import com.tastebyte.erp.ui.SearchField

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun EmployeeListScreen(
    viewModel: HrViewModel,
    onBack: () -> Unit
) {
    val state by viewModel.employeeListState.collectAsState()

    LaunchedEffect(Unit) {
        viewModel.loadEmployees()
    }

    Column(modifier = Modifier.fillMaxSize()) {
        TopAppBar(
            title = { Text("Employees") },
            navigationIcon = {
                IconButton(onClick = onBack) {
                    Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Back")
                }
            }
        )

        SearchField(
            value = state.searchQuery,
            onValueChange = viewModel::onEmployeeSearchChanged,
            placeholder = "Search employees..."
        )

        PullToRefreshBox(
            isRefreshing = state.isLoading,
            onRefresh = viewModel::loadEmployees,
            modifier = Modifier.fillMaxSize()
        ) {
            when {
                state.isLoading && state.employees.isEmpty() -> LoadingIndicator()
                state.error != null && state.employees.isEmpty() -> {
                    EmptyState(
                        message = state.error ?: "Error",
                        icon = Icons.Outlined.People,
                        actionLabel = "Retry",
                        onAction = viewModel::loadEmployees
                    )
                }
                state.employees.isEmpty() -> {
                    EmptyState(
                        message = "No employees found",
                        icon = Icons.Outlined.People
                    )
                }
                else -> {
                    LazyColumn(
                        contentPadding = PaddingValues(horizontal = 16.dp, vertical = 8.dp),
                        verticalArrangement = Arrangement.spacedBy(8.dp)
                    ) {
                        items(state.employees, key = { it.id }) { employee ->
                            ErpCard(
                                title = employee.fullName,
                                subtitle = buildString {
                                    append(employee.employeeNumber)
                                    if (employee.department != null) {
                                        append(" | ${employee.department}")
                                    }
                                    if (employee.position != null) {
                                        append(" | ${employee.position}")
                                    }
                                },
                                status = if (employee.isActive) "active" else "inactive"
                            )
                        }
                    }
                }
            }
        }
    }
}
