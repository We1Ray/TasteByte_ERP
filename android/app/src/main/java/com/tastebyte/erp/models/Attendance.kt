package com.tastebyte.erp.models

import com.google.gson.annotations.SerializedName

data class Attendance(
    @SerializedName("id") val id: String,
    @SerializedName("employee_id") val employeeId: String,
    @SerializedName("date") val date: String,
    @SerializedName("clock_in") val clockIn: String?,
    @SerializedName("clock_out") val clockOut: String?,
    @SerializedName("status") val status: String,
    @SerializedName("hours_worked") val hoursWorked: Double?,
    @SerializedName("notes") val notes: String?,
    @SerializedName("created_at") val createdAt: String
)
