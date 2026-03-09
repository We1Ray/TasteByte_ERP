package com.tastebyte.erp.features.quality

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FilterChip
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import com.tastebyte.erp.features.materials.DetailRow
import com.tastebyte.erp.ui.LoadingIndicator
import com.tastebyte.erp.ui.StatusBadge

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun InspectionFormScreen(
    inspectionId: String,
    viewModel: QualityViewModel,
    onBack: () -> Unit
) {
    val state by viewModel.formState.collectAsState()

    LaunchedEffect(inspectionId) {
        viewModel.loadInspectionLot(inspectionId)
    }

    LaunchedEffect(state.submitSuccess) {
        if (state.submitSuccess) {
            onBack()
        }
    }

    Column(modifier = Modifier.fillMaxSize()) {
        TopAppBar(
            title = { Text("Inspection Form") },
            navigationIcon = {
                IconButton(onClick = onBack) {
                    Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Back")
                }
            }
        )

        when {
            state.isLoading -> LoadingIndicator()
            state.error != null && state.lot == null -> {
                Text(
                    text = state.error ?: "Error",
                    color = MaterialTheme.colorScheme.error,
                    modifier = Modifier.padding(16.dp)
                )
            }
            state.lot != null -> {
                val lot = state.lot!!

                LazyColumn(
                    modifier = Modifier.weight(1f),
                    contentPadding = PaddingValues(16.dp),
                    verticalArrangement = Arrangement.spacedBy(12.dp)
                ) {
                    // Lot Info
                    item {
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
                                Row(
                                    modifier = Modifier.fillMaxWidth(),
                                    horizontalArrangement = Arrangement.SpaceBetween,
                                    verticalAlignment = Alignment.CenterVertically
                                ) {
                                    Text(
                                        text = lot.lotNumber,
                                        style = MaterialTheme.typography.titleMedium,
                                        fontWeight = FontWeight.Bold
                                    )
                                    StatusBadge(status = lot.status)
                                }
                                Spacer(modifier = Modifier.height(8.dp))
                                DetailRow("Type", lot.inspectionType)
                                DetailRow("Quantity", lot.quantity.toInt().toString())
                                if (lot.materialName != null) {
                                    DetailRow("Material", lot.materialName)
                                }
                            }
                        }
                    }

                    // Characteristics
                    if (lot.characteristics != null && lot.characteristics.isNotEmpty()) {
                        item {
                            Text(
                                text = "Characteristics",
                                style = MaterialTheme.typography.titleMedium,
                                fontWeight = FontWeight.SemiBold
                            )
                        }

                        items(lot.characteristics) { char ->
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
                                        .padding(12.dp)
                                ) {
                                    Text(
                                        text = char.name,
                                        style = MaterialTheme.typography.titleSmall,
                                        fontWeight = FontWeight.SemiBold
                                    )
                                    if (char.description != null) {
                                        Text(
                                            text = char.description,
                                            style = MaterialTheme.typography.bodySmall,
                                            color = MaterialTheme.colorScheme.onSurfaceVariant
                                        )
                                    }

                                    Spacer(modifier = Modifier.height(8.dp))

                                    // Target/limits info
                                    Row(
                                        modifier = Modifier.fillMaxWidth(),
                                        horizontalArrangement = Arrangement.SpaceBetween
                                    ) {
                                        if (char.targetValue != null) {
                                            Text(
                                                text = "Target: ${char.targetValue}${char.unit?.let { " $it" } ?: ""}",
                                                style = MaterialTheme.typography.labelSmall,
                                                color = MaterialTheme.colorScheme.onSurfaceVariant
                                            )
                                        }
                                        if (char.lowerLimit != null && char.upperLimit != null) {
                                            Text(
                                                text = "Range: ${char.lowerLimit} - ${char.upperLimit}",
                                                style = MaterialTheme.typography.labelSmall,
                                                color = MaterialTheme.colorScheme.onSurfaceVariant
                                            )
                                        }
                                    }

                                    Spacer(modifier = Modifier.height(8.dp))
                                    HorizontalDivider()
                                    Spacer(modifier = Modifier.height(8.dp))

                                    // Measured value input
                                    OutlinedTextField(
                                        value = state.measuredValues[char.id] ?: "",
                                        onValueChange = {
                                            viewModel.updateMeasuredValue(char.id, it)
                                        },
                                        label = {
                                            Text("Measured Value${char.unit?.let { " ($it)" } ?: ""}")
                                        },
                                        keyboardOptions = KeyboardOptions(
                                            keyboardType = KeyboardType.Decimal
                                        ),
                                        singleLine = true,
                                        modifier = Modifier.fillMaxWidth()
                                    )

                                    Spacer(modifier = Modifier.height(8.dp))

                                    // Result selection
                                    Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                                        listOf("pass", "fail").forEach { result ->
                                            FilterChip(
                                                selected = state.characteristicResults[char.id] == result,
                                                onClick = {
                                                    viewModel.updateCharacteristicResult(
                                                        char.id,
                                                        result
                                                    )
                                                },
                                                label = {
                                                    Text(result.replaceFirstChar { it.uppercase() })
                                                }
                                            )
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Overall Result
                    item {
                        Spacer(modifier = Modifier.height(8.dp))
                        Text(
                            text = "Overall Result",
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.SemiBold
                        )
                        Spacer(modifier = Modifier.height(8.dp))
                        Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                            listOf("pass", "fail", "conditional").forEach { result ->
                                FilterChip(
                                    selected = state.overallResult == result,
                                    onClick = { viewModel.updateOverallResult(result) },
                                    label = {
                                        Text(result.replaceFirstChar { it.uppercase() })
                                    }
                                )
                            }
                        }
                    }

                    // Notes
                    item {
                        OutlinedTextField(
                            value = state.notes,
                            onValueChange = viewModel::updateNotes,
                            label = { Text("Notes (optional)") },
                            modifier = Modifier.fillMaxWidth(),
                            minLines = 3,
                            maxLines = 5
                        )
                    }

                    // Error display
                    if (state.error != null) {
                        item {
                            Text(
                                text = state.error!!,
                                color = MaterialTheme.colorScheme.error,
                                style = MaterialTheme.typography.bodyMedium
                            )
                        }
                    }
                }

                // Submit button
                Button(
                    onClick = viewModel::submitResults,
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(16.dp)
                        .height(48.dp),
                    enabled = !state.isSubmitting && state.overallResult.isNotBlank()
                ) {
                    if (state.isSubmitting) {
                        CircularProgressIndicator(
                            modifier = Modifier.size(20.dp),
                            color = MaterialTheme.colorScheme.onPrimary,
                            strokeWidth = 2.dp
                        )
                    } else {
                        Text("Submit Results")
                    }
                }
            }
        }
    }
}
