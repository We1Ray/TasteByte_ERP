package com.tastebyte.erp.features.hr

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Login
import androidx.compose.material.icons.filled.Logout
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.tastebyte.erp.core.theme.StatusCompleted
import com.tastebyte.erp.core.theme.StatusCancelled
import com.tastebyte.erp.features.materials.DetailRow
import com.tastebyte.erp.ui.LoadingIndicator
import com.tastebyte.erp.ui.StatusBadge

@Composable
fun AttendanceScreen(
    viewModel: HrViewModel
) {
    val state by viewModel.attendanceState.collectAsState()
    val snackbarHostState = remember { SnackbarHostState() }

    LaunchedEffect(state.successMessage) {
        state.successMessage?.let {
            snackbarHostState.showSnackbar(it)
            viewModel.clearSuccessMessage()
        }
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        Text(
            text = "Attendance",
            style = MaterialTheme.typography.headlineSmall,
            fontWeight = FontWeight.Bold,
            modifier = Modifier
                .fillMaxWidth()
                .padding(bottom = 16.dp)
        )

        if (state.isLoading) {
            LoadingIndicator()
        } else {
            // Status Card
            Card(
                modifier = Modifier.fillMaxWidth(),
                colors = CardDefaults.cardColors(
                    containerColor = MaterialTheme.colorScheme.surface
                ),
                elevation = CardDefaults.cardElevation(defaultElevation = 2.dp)
            ) {
                Column(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(24.dp),
                    horizontalAlignment = Alignment.CenterHorizontally
                ) {
                    Text(
                        text = "Current Status",
                        style = MaterialTheme.typography.titleMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                    Spacer(modifier = Modifier.height(8.dp))
                    StatusBadge(
                        status = if (state.isClockedIn) "clocked_in" else "absent"
                    )
                    Spacer(modifier = Modifier.height(8.dp))
                    Text(
                        text = if (state.isClockedIn) "You are clocked in" else "Not clocked in",
                        style = MaterialTheme.typography.headlineSmall,
                        fontWeight = FontWeight.Bold,
                        textAlign = TextAlign.Center
                    )
                }
            }

            Spacer(modifier = Modifier.height(24.dp))

            // Clock In/Out Buttons
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(16.dp)
            ) {
                Button(
                    onClick = viewModel::clockIn,
                    modifier = Modifier
                        .weight(1f)
                        .height(64.dp),
                    enabled = !state.isClockedIn && !state.isClocking,
                    colors = ButtonDefaults.buttonColors(
                        containerColor = StatusCompleted
                    )
                ) {
                    if (state.isClocking && !state.isClockedIn) {
                        CircularProgressIndicator(
                            modifier = Modifier.size(20.dp),
                            color = MaterialTheme.colorScheme.onPrimary,
                            strokeWidth = 2.dp
                        )
                    } else {
                        Icon(
                            imageVector = Icons.Default.Login,
                            contentDescription = null,
                            modifier = Modifier.padding(end = 8.dp)
                        )
                        Text("Clock In", style = MaterialTheme.typography.labelLarge)
                    }
                }

                Button(
                    onClick = viewModel::clockOut,
                    modifier = Modifier
                        .weight(1f)
                        .height(64.dp),
                    enabled = state.isClockedIn && !state.isClocking,
                    colors = ButtonDefaults.buttonColors(
                        containerColor = StatusCancelled
                    )
                ) {
                    if (state.isClocking && state.isClockedIn) {
                        CircularProgressIndicator(
                            modifier = Modifier.size(20.dp),
                            color = MaterialTheme.colorScheme.onPrimary,
                            strokeWidth = 2.dp
                        )
                    } else {
                        Icon(
                            imageVector = Icons.Default.Logout,
                            contentDescription = null,
                            modifier = Modifier.padding(end = 8.dp)
                        )
                        Text("Clock Out", style = MaterialTheme.typography.labelLarge)
                    }
                }
            }

            // Error message
            if (state.error != null) {
                Spacer(modifier = Modifier.height(16.dp))
                Text(
                    text = state.error!!,
                    color = MaterialTheme.colorScheme.error,
                    style = MaterialTheme.typography.bodyMedium,
                    textAlign = TextAlign.Center,
                    modifier = Modifier.fillMaxWidth()
                )
            }

            // Today's Record
            if (state.todayAttendance != null) {
                Spacer(modifier = Modifier.height(24.dp))
                Card(
                    modifier = Modifier.fillMaxWidth(),
                    colors = CardDefaults.cardColors(
                        containerColor = MaterialTheme.colorScheme.surface
                    ),
                    elevation = CardDefaults.cardElevation(defaultElevation = 1.dp)
                ) {
                    Column(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(16.dp)
                    ) {
                        Text(
                            text = "Today's Record",
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.SemiBold
                        )
                        Spacer(modifier = Modifier.height(8.dp))
                        HorizontalDivider()
                        Spacer(modifier = Modifier.height(8.dp))

                        val attendance = state.todayAttendance!!
                        DetailRow("Date", attendance.date)
                        if (attendance.clockIn != null) {
                            DetailRow("Clock In", attendance.clockIn)
                        }
                        if (attendance.clockOut != null) {
                            DetailRow("Clock Out", attendance.clockOut)
                        }
                        if (attendance.hoursWorked != null) {
                            DetailRow(
                                "Hours Worked",
                                String.format("%.2f hrs", attendance.hoursWorked)
                            )
                        }
                        DetailRow("Status", attendance.status)
                    }
                }
            }
        }

        SnackbarHost(hostState = snackbarHostState)
    }
}
