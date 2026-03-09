package com.tastebyte.erp.models

import com.google.gson.annotations.SerializedName

data class ProductionOrder(
    @SerializedName("id") val id: String,
    @SerializedName("order_number") val orderNumber: String,
    @SerializedName("material_id") val materialId: String,
    @SerializedName("material_name") val materialName: String?,
    @SerializedName("quantity") val quantity: Double,
    @SerializedName("uom_id") val uomId: String?,
    @SerializedName("status") val status: String,
    @SerializedName("planned_start") val plannedStart: String?,
    @SerializedName("planned_end") val plannedEnd: String?,
    @SerializedName("actual_start") val actualStart: String?,
    @SerializedName("actual_end") val actualEnd: String?,
    @SerializedName("notes") val notes: String?,
    @SerializedName("created_by") val createdBy: String?,
    @SerializedName("created_at") val createdAt: String,
    @SerializedName("updated_at") val updatedAt: String
)

data class ProductionOrderStatusRequest(
    @SerializedName("status") val status: String
)
