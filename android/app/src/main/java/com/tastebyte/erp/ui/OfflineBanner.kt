package com.tastebyte.erp.ui

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.expandVertically
import androidx.compose.animation.shrinkVertically
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.WifiOff
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.tastebyte.erp.core.network.NetworkMonitor
import com.tastebyte.erp.core.network.OfflineSyncManager
import com.tastebyte.erp.core.theme.KpiOrange

@Composable
fun OfflineBanner() {
    val isConnected by NetworkMonitor.getInstance().isConnected.collectAsState()
    val pendingCount by OfflineSyncManager.getInstance().pendingCount.collectAsState()

    AnimatedVisibility(
        visible = !isConnected,
        enter = expandVertically(),
        exit = shrinkVertically()
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .background(KpiOrange)
                .padding(horizontal = 16.dp, vertical = 8.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(8.dp)
        ) {
            Icon(
                imageVector = Icons.Default.WifiOff,
                contentDescription = "Offline",
                tint = Color.White
            )
            Text(
                text = "Offline Mode",
                color = Color.White,
                fontWeight = FontWeight.Medium,
                fontSize = 14.sp
            )
            if (pendingCount > 0) {
                androidx.compose.foundation.layout.Spacer(modifier = Modifier.weight(1f))
                Text(
                    text = "$pendingCount pending",
                    color = Color.White.copy(alpha = 0.9f),
                    fontSize = 12.sp,
                    modifier = Modifier
                        .background(
                            color = Color.White.copy(alpha = 0.25f),
                            shape = RoundedCornerShape(12.dp)
                        )
                        .padding(horizontal = 8.dp, vertical = 2.dp)
                )
            }
        }
    }
}
