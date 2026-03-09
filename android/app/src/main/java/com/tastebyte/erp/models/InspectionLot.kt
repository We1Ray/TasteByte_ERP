package com.tastebyte.erp.models

import com.google.gson.annotations.SerializedName

data class InspectionLot(
    @SerializedName("id") val id: String,
    @SerializedName("lot_number") val lotNumber: String,
    @SerializedName("material_id") val materialId: String,
    @SerializedName("material_name") val materialName: String?,
    @SerializedName("inspection_type") val inspectionType: String,
    @SerializedName("quantity") val quantity: Double,
    @SerializedName("status") val status: String,
    @SerializedName("inspector_id") val inspectorId: String?,
    @SerializedName("inspection_date") val inspectionDate: String?,
    @SerializedName("result") val result: String?,
    @SerializedName("characteristics") val characteristics: List<InspectionCharacteristic>?,
    @SerializedName("notes") val notes: String?,
    @SerializedName("created_at") val createdAt: String,
    @SerializedName("updated_at") val updatedAt: String
)

data class InspectionCharacteristic(
    @SerializedName("id") val id: String,
    @SerializedName("name") val name: String,
    @SerializedName("description") val description: String?,
    @SerializedName("target_value") val targetValue: Double?,
    @SerializedName("lower_limit") val lowerLimit: Double?,
    @SerializedName("upper_limit") val upperLimit: Double?,
    @SerializedName("measured_value") val measuredValue: Double?,
    @SerializedName("unit") val unit: String?,
    @SerializedName("result") val result: String?
)

data class CreateInspectionLotRequest(
    @SerializedName("material_id") val materialId: String,
    @SerializedName("inspection_type") val inspectionType: String,
    @SerializedName("quantity") val quantity: Double,
    @SerializedName("notes") val notes: String? = null
)

data class InspectionResultsRequest(
    @SerializedName("result") val result: String,
    @SerializedName("characteristics") val characteristics: List<CharacteristicResult>,
    @SerializedName("notes") val notes: String? = null
)

data class CharacteristicResult(
    @SerializedName("characteristic_id") val characteristicId: String,
    @SerializedName("measured_value") val measuredValue: Double,
    @SerializedName("result") val result: String
)
