package com.tastebyte.erp.core.network

import com.google.gson.annotations.SerializedName

data class ApiResponse<T>(
    @SerializedName("success") val success: Boolean,
    @SerializedName("data") val data: T?,
    @SerializedName("error") val error: String?,
    @SerializedName("message") val message: String?
)

data class PaginatedData<T>(
    @SerializedName("items") val items: List<T>,
    @SerializedName("total") val total: Int,
    @SerializedName("page") val page: Int,
    @SerializedName("page_size") val pageSize: Int,
    @SerializedName("total_pages") val totalPages: Int
)
