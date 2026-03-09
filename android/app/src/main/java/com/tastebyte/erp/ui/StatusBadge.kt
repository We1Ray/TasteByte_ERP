package com.tastebyte.erp.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.tastebyte.erp.core.theme.*

@Composable
fun StatusBadge(
    status: String,
    modifier: Modifier = Modifier
) {
    val (backgroundColor, textColor) = getStatusColors(status)

    Box(
        modifier = modifier
            .clip(RoundedCornerShape(4.dp))
            .background(backgroundColor.copy(alpha = 0.12f))
            .padding(horizontal = 8.dp, vertical = 4.dp)
    ) {
        Text(
            text = status.replaceFirstChar { it.uppercase() },
            style = MaterialTheme.typography.labelSmall,
            fontWeight = FontWeight.SemiBold,
            color = textColor
        )
    }
}

fun getStatusColors(status: String): Pair<Color, Color> {
    return when (status.lowercase()) {
        "draft" -> StatusDraft to StatusDraft
        "released", "active", "open" -> StatusReleased to StatusReleased
        "in_process", "in_progress", "processing" -> StatusInProcess to StatusInProcess
        "completed", "complete", "approved" -> StatusCompleted to StatusCompleted
        "closed", "inactive" -> StatusClosed to StatusClosed
        "cancelled", "rejected", "failed" -> StatusCancelled to StatusCancelled
        "clocked_in", "present" -> StatusCompleted to StatusCompleted
        "absent" -> StatusCancelled to StatusCancelled
        "pass", "passed" -> StatusCompleted to StatusCompleted
        "fail" -> StatusCancelled to StatusCancelled
        else -> StatusDraft to StatusDraft
    }
}
