package com.tastebyte.erp.models

import com.google.gson.annotations.SerializedName

data class SalesOrder(
    @SerializedName("id") val id: String,
    @SerializedName("order_number") val orderNumber: String,
    @SerializedName("customer_id") val customerId: String,
    @SerializedName("order_date") val orderDate: String,
    @SerializedName("requested_delivery_date") val requestedDeliveryDate: String?,
    @SerializedName("status") val status: String,
    @SerializedName("total_amount") val totalAmount: Double,
    @SerializedName("currency") val currency: String,
    @SerializedName("notes") val notes: String?,
    @SerializedName("created_by") val createdBy: String?,
    @SerializedName("created_at") val createdAt: String,
    @SerializedName("updated_at") val updatedAt: String
)

data class SalesOrderItem(
    @SerializedName("id") val id: String,
    @SerializedName("sales_order_id") val salesOrderId: String,
    @SerializedName("line_number") val lineNumber: Int,
    @SerializedName("material_id") val materialId: String,
    @SerializedName("quantity") val quantity: Double,
    @SerializedName("unit_price") val unitPrice: Double,
    @SerializedName("total_price") val totalPrice: Double,
    @SerializedName("uom_id") val uomId: String?,
    @SerializedName("delivered_quantity") val deliveredQuantity: Double
)

data class CreateSalesOrderRequest(
    @SerializedName("customer_id") val customerId: String,
    @SerializedName("order_date") val orderDate: String,
    @SerializedName("requested_delivery_date") val requestedDeliveryDate: String? = null,
    @SerializedName("notes") val notes: String? = null,
    @SerializedName("items") val items: List<CreateSalesOrderItemRequest>
)

data class CreateSalesOrderItemRequest(
    @SerializedName("material_id") val materialId: String,
    @SerializedName("quantity") val quantity: Double,
    @SerializedName("unit_price") val unitPrice: Double,
    @SerializedName("uom_id") val uomId: String? = null
)

data class Customer(
    @SerializedName("id") val id: String,
    @SerializedName("customer_number") val customerNumber: String,
    @SerializedName("name") val name: String,
    @SerializedName("contact_person") val contactPerson: String?,
    @SerializedName("email") val email: String?,
    @SerializedName("phone") val phone: String?,
    @SerializedName("address") val address: String?,
    @SerializedName("payment_terms") val paymentTerms: Int,
    @SerializedName("credit_limit") val creditLimit: Double,
    @SerializedName("is_active") val isActive: Boolean,
    @SerializedName("created_at") val createdAt: String,
    @SerializedName("updated_at") val updatedAt: String
)

data class Delivery(
    @SerializedName("id") val id: String,
    @SerializedName("delivery_number") val deliveryNumber: String,
    @SerializedName("sales_order_id") val salesOrderId: String,
    @SerializedName("delivery_date") val deliveryDate: String,
    @SerializedName("status") val status: String,
    @SerializedName("shipped_by") val shippedBy: String?,
    @SerializedName("shipped_at") val shippedAt: String?,
    @SerializedName("created_at") val createdAt: String
)
