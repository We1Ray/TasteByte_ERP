package com.tastebyte.erp.models

import com.google.gson.annotations.SerializedName

data class DashboardKpi(
    @SerializedName("total_revenue") val totalRevenue: Double,
    @SerializedName("total_order_count") val totalOrderCount: Int,
    @SerializedName("total_inventory_quantity") val totalInventoryQuantity: Double,
    @SerializedName("pending_production_orders") val pendingProductionOrders: Int,
    @SerializedName("open_ar_amount") val openArAmount: Double,
    @SerializedName("open_ap_amount") val openApAmount: Double
)
