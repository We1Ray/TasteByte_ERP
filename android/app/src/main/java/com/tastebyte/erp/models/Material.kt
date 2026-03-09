package com.tastebyte.erp.models

import com.google.gson.annotations.SerializedName

data class Material(
    @SerializedName("id") val id: String,
    @SerializedName("material_number") val materialNumber: String,
    @SerializedName("name") val name: String,
    @SerializedName("description") val description: String?,
    @SerializedName("material_group_id") val materialGroupId: String?,
    @SerializedName("base_uom_id") val baseUomId: String?,
    @SerializedName("material_type") val materialType: String,
    @SerializedName("weight") val weight: Double?,
    @SerializedName("weight_uom") val weightUom: String?,
    @SerializedName("is_active") val isActive: Boolean,
    @SerializedName("created_at") val createdAt: String,
    @SerializedName("updated_at") val updatedAt: String
)

data class CreateMaterialRequest(
    @SerializedName("name") val name: String,
    @SerializedName("description") val description: String? = null,
    @SerializedName("material_group_id") val materialGroupId: String? = null,
    @SerializedName("base_uom_id") val baseUomId: String? = null,
    @SerializedName("material_type") val materialType: String? = null,
    @SerializedName("weight") val weight: Double? = null,
    @SerializedName("weight_uom") val weightUom: String? = null
)

data class UpdateMaterialRequest(
    @SerializedName("name") val name: String? = null,
    @SerializedName("description") val description: String? = null,
    @SerializedName("material_group_id") val materialGroupId: String? = null,
    @SerializedName("base_uom_id") val baseUomId: String? = null,
    @SerializedName("weight") val weight: Double? = null,
    @SerializedName("weight_uom") val weightUom: String? = null,
    @SerializedName("is_active") val isActive: Boolean? = null
)

data class PlantStock(
    @SerializedName("id") val id: String,
    @SerializedName("material_id") val materialId: String,
    @SerializedName("warehouse_id") val warehouseId: String?,
    @SerializedName("quantity") val quantity: Double,
    @SerializedName("reserved_quantity") val reservedQuantity: Double,
    @SerializedName("uom_id") val uomId: String?,
    @SerializedName("last_count_date") val lastCountDate: String?,
    @SerializedName("updated_at") val updatedAt: String
)

data class MaterialMovement(
    @SerializedName("id") val id: String,
    @SerializedName("document_number") val documentNumber: String,
    @SerializedName("movement_type") val movementType: String,
    @SerializedName("material_id") val materialId: String,
    @SerializedName("warehouse_id") val warehouseId: String?,
    @SerializedName("quantity") val quantity: Double,
    @SerializedName("uom_id") val uomId: String?,
    @SerializedName("reference_type") val referenceType: String?,
    @SerializedName("reference_id") val referenceId: String?,
    @SerializedName("posted_by") val postedBy: String?,
    @SerializedName("posted_at") val postedAt: String
)

data class CreateMovementRequest(
    @SerializedName("movement_type") val movementType: String,
    @SerializedName("material_id") val materialId: String,
    @SerializedName("warehouse_id") val warehouseId: String? = null,
    @SerializedName("quantity") val quantity: Double,
    @SerializedName("uom_id") val uomId: String? = null,
    @SerializedName("reference_type") val referenceType: String? = null,
    @SerializedName("reference_id") val referenceId: String? = null
)

data class Vendor(
    @SerializedName("id") val id: String,
    @SerializedName("vendor_number") val vendorNumber: String,
    @SerializedName("name") val name: String,
    @SerializedName("contact_person") val contactPerson: String?,
    @SerializedName("email") val email: String?,
    @SerializedName("phone") val phone: String?,
    @SerializedName("address") val address: String?,
    @SerializedName("payment_terms") val paymentTerms: Int,
    @SerializedName("is_active") val isActive: Boolean,
    @SerializedName("created_at") val createdAt: String,
    @SerializedName("updated_at") val updatedAt: String
)

data class PurchaseOrder(
    @SerializedName("id") val id: String,
    @SerializedName("po_number") val poNumber: String,
    @SerializedName("vendor_id") val vendorId: String,
    @SerializedName("order_date") val orderDate: String,
    @SerializedName("delivery_date") val deliveryDate: String?,
    @SerializedName("status") val status: String,
    @SerializedName("total_amount") val totalAmount: Double,
    @SerializedName("currency") val currency: String,
    @SerializedName("notes") val notes: String?,
    @SerializedName("created_by") val createdBy: String?,
    @SerializedName("created_at") val createdAt: String,
    @SerializedName("updated_at") val updatedAt: String
)
