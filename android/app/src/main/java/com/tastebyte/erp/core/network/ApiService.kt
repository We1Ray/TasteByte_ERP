package com.tastebyte.erp.core.network

import com.tastebyte.erp.models.*
import retrofit2.http.*

interface ApiService {

    // Auth
    @POST("auth/login")
    suspend fun login(@Body request: LoginRequest): ApiResponse<TokenResponse>

    @POST("auth/register")
    suspend fun register(@Body request: RegisterRequest): ApiResponse<TokenResponse>

    @POST("auth/refresh")
    suspend fun refreshToken(@Body request: RefreshRequest): ApiResponse<TokenResponse>

    @POST("auth/logout")
    suspend fun logout(@Body request: LogoutRequest): ApiResponse<Unit>

    // Materials Management (MM)
    @GET("mm/materials")
    suspend fun listMaterials(
        @Query("page") page: Int = 1,
        @Query("page_size") pageSize: Int = 20,
        @Query("search") search: String? = null
    ): ApiResponse<PaginatedData<Material>>

    @GET("mm/materials/{id}")
    suspend fun getMaterial(@Path("id") id: String): ApiResponse<Material>

    @POST("mm/materials")
    suspend fun createMaterial(@Body request: CreateMaterialRequest): ApiResponse<Material>

    @PUT("mm/materials/{id}")
    suspend fun updateMaterial(
        @Path("id") id: String,
        @Body request: UpdateMaterialRequest
    ): ApiResponse<Material>

    @GET("mm/plant-stock")
    suspend fun listPlantStock(
        @Query("material_id") materialId: String? = null,
        @Query("warehouse_id") warehouseId: String? = null
    ): ApiResponse<List<PlantStock>>

    @GET("mm/material-movements")
    suspend fun listMaterialMovements(
        @Query("page") page: Int = 1,
        @Query("page_size") pageSize: Int = 20
    ): ApiResponse<PaginatedData<MaterialMovement>>

    @POST("mm/material-movements")
    suspend fun createMaterialMovement(@Body request: CreateMovementRequest): ApiResponse<MaterialMovement>

    @GET("mm/vendors")
    suspend fun listVendors(): ApiResponse<PaginatedData<Vendor>>

    @GET("mm/purchase-orders")
    suspend fun listPurchaseOrders(
        @Query("page") page: Int = 1,
        @Query("page_size") pageSize: Int = 20
    ): ApiResponse<PaginatedData<PurchaseOrder>>

    @GET("mm/purchase-orders/{id}")
    suspend fun getPurchaseOrder(@Path("id") id: String): ApiResponse<PurchaseOrder>

    @POST("mm/purchase-orders/{id}/receive")
    suspend fun receivePurchaseOrder(@Path("id") id: String): ApiResponse<PurchaseOrder>

    // Sales & Distribution (SD)
    @GET("sd/sales-orders")
    suspend fun listSalesOrders(
        @Query("page") page: Int = 1,
        @Query("page_size") pageSize: Int = 20,
        @Query("search") search: String? = null
    ): ApiResponse<PaginatedData<SalesOrder>>

    @GET("sd/sales-orders/{id}")
    suspend fun getSalesOrder(@Path("id") id: String): ApiResponse<SalesOrder>

    @POST("sd/sales-orders")
    suspend fun createSalesOrder(@Body request: CreateSalesOrderRequest): ApiResponse<SalesOrder>

    @GET("sd/customers")
    suspend fun listCustomers(): ApiResponse<PaginatedData<Customer>>

    @POST("sd/sales-orders/{id}/confirm")
    suspend fun confirmSalesOrder(@Path("id") id: String): ApiResponse<SalesOrder>

    @GET("sd/deliveries")
    suspend fun listDeliveries(
        @Query("page") page: Int = 1,
        @Query("page_size") pageSize: Int = 20
    ): ApiResponse<PaginatedData<Delivery>>

    // Production Planning (PP)
    @GET("pp/production-orders")
    suspend fun listProductionOrders(
        @Query("page") page: Int = 1,
        @Query("page_size") pageSize: Int = 20
    ): ApiResponse<PaginatedData<ProductionOrder>>

    @GET("pp/production-orders/{id}")
    suspend fun getProductionOrder(@Path("id") id: String): ApiResponse<ProductionOrder>

    @PUT("pp/production-orders/{id}/status")
    suspend fun updateProductionOrderStatus(
        @Path("id") id: String,
        @Body request: ProductionOrderStatusRequest
    ): ApiResponse<ProductionOrder>

    // Dashboard
    @GET("dashboard/kpis")
    suspend fun getDashboardKpis(): ApiResponse<DashboardKpi>

    // Reports - FI
    @GET("fi/reports/trial-balance")
    suspend fun getTrialBalance(): ApiResponse<List<TrialBalanceEntry>>

    @GET("fi/reports/income-statement")
    suspend fun getIncomeStatement(): ApiResponse<List<IncomeStatementEntry>>

    @GET("fi/reports/balance-sheet")
    suspend fun getBalanceSheet(): ApiResponse<List<BalanceSheetEntry>>

    @GET("fi/reports/ar-aging")
    suspend fun getArAging(): ApiResponse<List<AgingEntry>>

    @GET("fi/reports/ap-aging")
    suspend fun getApAging(): ApiResponse<List<AgingEntry>>

    // Reports - MM
    @GET("mm/reports/stock-valuation")
    suspend fun getStockValuation(): ApiResponse<List<StockValuationEntry>>

    @GET("mm/reports/movement-summary")
    suspend fun getMovementSummary(): ApiResponse<List<MovementSummaryEntry>>

    @GET("mm/reports/slow-moving")
    suspend fun getSlowMoving(): ApiResponse<List<SlowMovingEntry>>

    // Reports - SD
    @GET("sd/reports/sales-summary")
    suspend fun getSalesSummary(): ApiResponse<List<SalesSummaryEntry>>

    @GET("sd/reports/order-fulfillment")
    suspend fun getOrderFulfillment(): ApiResponse<List<OrderFulfillmentEntry>>

    @GET("sd/reports/top-customers")
    suspend fun getTopCustomers(): ApiResponse<List<TopCustomerEntry>>

    // Human Resources (HR)
    @GET("hr/employees")
    suspend fun listEmployees(
        @Query("page") page: Int = 1,
        @Query("page_size") pageSize: Int = 20,
        @Query("search") search: String? = null
    ): ApiResponse<PaginatedData<Employee>>

    @GET("hr/employees/{id}")
    suspend fun getEmployee(@Path("id") id: String): ApiResponse<Employee>

    @POST("hr/attendance/clock-in")
    suspend fun clockIn(): ApiResponse<Attendance>

    @POST("hr/attendance/clock-out")
    suspend fun clockOut(): ApiResponse<Attendance>

    @GET("hr/attendance")
    suspend fun listAttendance(
        @Query("date") date: String? = null,
        @Query("employee_id") employeeId: String? = null
    ): ApiResponse<List<Attendance>>

    @GET("hr/attendance/today")
    suspend fun getTodayAttendance(): ApiResponse<Attendance>

    // Warehouse Management (WM)
    @GET("wm/warehouses")
    suspend fun listWarehouses(): ApiResponse<List<Warehouse>>

    @GET("wm/warehouses/{id}")
    suspend fun getWarehouse(@Path("id") id: String): ApiResponse<Warehouse>

    @POST("wm/stock-count")
    suspend fun submitStockCount(@Body request: StockCountRequest): ApiResponse<StockCountResult>

    @GET("wm/stock-count")
    suspend fun listStockCounts(
        @Query("warehouse_id") warehouseId: String? = null
    ): ApiResponse<List<StockCountResult>>

    // Quality Management (QM)
    @GET("qm/inspection-lots")
    suspend fun listInspectionLots(
        @Query("page") page: Int = 1,
        @Query("page_size") pageSize: Int = 20,
        @Query("status") status: String? = null
    ): ApiResponse<PaginatedData<InspectionLot>>

    @GET("qm/inspection-lots/{id}")
    suspend fun getInspectionLot(@Path("id") id: String): ApiResponse<InspectionLot>

    @POST("qm/inspection-lots")
    suspend fun createInspectionLot(@Body request: CreateInspectionLotRequest): ApiResponse<InspectionLot>

    @PUT("qm/inspection-lots/{id}/results")
    suspend fun submitInspectionResults(
        @Path("id") id: String,
        @Body request: InspectionResultsRequest
    ): ApiResponse<InspectionLot>
}
