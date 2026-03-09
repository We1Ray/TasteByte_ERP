package com.tastebyte.erp.models

import com.google.gson.annotations.SerializedName

// FI Reports
data class TrialBalanceEntry(
    @SerializedName("account_number") val accountNumber: String,
    @SerializedName("account_name") val accountName: String,
    @SerializedName("debit_balance") val debitBalance: Double,
    @SerializedName("credit_balance") val creditBalance: Double
)

data class IncomeStatementEntry(
    @SerializedName("category") val category: String,
    @SerializedName("description") val description: String,
    @SerializedName("amount") val amount: Double
)

data class BalanceSheetEntry(
    @SerializedName("category") val category: String,
    @SerializedName("description") val description: String,
    @SerializedName("amount") val amount: Double
)

data class AgingEntry(
    @SerializedName("partner_id") val partnerId: String?,
    @SerializedName("partner_name") val partnerName: String?,
    @SerializedName("current_amount") val currentAmount: Double,
    @SerializedName("days_30") val days30: Double,
    @SerializedName("days_60") val days60: Double,
    @SerializedName("days_90") val days90: Double,
    @SerializedName("over_90") val over90: Double,
    @SerializedName("total") val total: Double
)

// MM Reports
data class StockValuationEntry(
    @SerializedName("material_id") val materialId: String?,
    @SerializedName("material_number") val materialNumber: String?,
    @SerializedName("material_name") val materialName: String?,
    @SerializedName("quantity") val quantity: Double,
    @SerializedName("unit_cost") val unitCost: Double,
    @SerializedName("total_value") val totalValue: Double
)

data class MovementSummaryEntry(
    @SerializedName("movement_type") val movementType: String,
    @SerializedName("count") val count: Int,
    @SerializedName("total_quantity") val totalQuantity: Double
)

data class SlowMovingEntry(
    @SerializedName("material_id") val materialId: String?,
    @SerializedName("material_number") val materialNumber: String?,
    @SerializedName("material_name") val materialName: String?,
    @SerializedName("quantity") val quantity: Double,
    @SerializedName("last_movement_date") val lastMovementDate: String?,
    @SerializedName("days_since_movement") val daysSinceMovement: Int
)

// SD Reports
data class SalesSummaryEntry(
    @SerializedName("period") val period: String?,
    @SerializedName("order_count") val orderCount: Int,
    @SerializedName("total_amount") val totalAmount: Double,
    @SerializedName("currency") val currency: String?
)

data class OrderFulfillmentEntry(
    @SerializedName("order_number") val orderNumber: String?,
    @SerializedName("order_date") val orderDate: String?,
    @SerializedName("status") val status: String,
    @SerializedName("total_amount") val totalAmount: Double,
    @SerializedName("delivered_percentage") val deliveredPercentage: Double
)

data class TopCustomerEntry(
    @SerializedName("customer_id") val customerId: String?,
    @SerializedName("customer_name") val customerName: String?,
    @SerializedName("order_count") val orderCount: Int,
    @SerializedName("total_amount") val totalAmount: Double
)
