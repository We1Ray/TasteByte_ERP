package com.tastebyte.erp.models

import com.google.gson.annotations.SerializedName

data class Employee(
    @SerializedName("id") val id: String,
    @SerializedName("employee_number") val employeeNumber: String,
    @SerializedName("first_name") val firstName: String,
    @SerializedName("last_name") val lastName: String,
    @SerializedName("email") val email: String?,
    @SerializedName("phone") val phone: String?,
    @SerializedName("department") val department: String?,
    @SerializedName("position") val position: String?,
    @SerializedName("hire_date") val hireDate: String?,
    @SerializedName("is_active") val isActive: Boolean,
    @SerializedName("created_at") val createdAt: String,
    @SerializedName("updated_at") val updatedAt: String
) {
    val fullName: String get() = "$firstName $lastName"
}
