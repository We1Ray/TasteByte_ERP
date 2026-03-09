package com.tastebyte.erp.models

import com.google.gson.annotations.SerializedName

data class Warehouse(
    @SerializedName("id") val id: String,
    @SerializedName("warehouse_number") val warehouseNumber: String,
    @SerializedName("name") val name: String,
    @SerializedName("description") val description: String?,
    @SerializedName("address") val address: String?,
    @SerializedName("capacity") val capacity: Int?,
    @SerializedName("is_active") val isActive: Boolean,
    @SerializedName("created_at") val createdAt: String,
    @SerializedName("updated_at") val updatedAt: String
)

data class StockCountRequest(
    @SerializedName("warehouse_id") val warehouseId: String,
    @SerializedName("items") val items: List<StockCountItem>
)

data class StockCountItem(
    @SerializedName("material_id") val materialId: String,
    @SerializedName("counted_quantity") val countedQuantity: Double,
    @SerializedName("uom_id") val uomId: String? = null
)

data class StockCountResult(
    @SerializedName("id") val id: String,
    @SerializedName("warehouse_id") val warehouseId: String,
    @SerializedName("count_date") val countDate: String,
    @SerializedName("status") val status: String,
    @SerializedName("items_count") val itemsCount: Int,
    @SerializedName("discrepancies") val discrepancies: Int,
    @SerializedName("created_by") val createdBy: String?,
    @SerializedName("created_at") val createdAt: String
)
