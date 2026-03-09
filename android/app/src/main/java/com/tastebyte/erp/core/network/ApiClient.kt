package com.tastebyte.erp.core.network

import com.tastebyte.erp.BuildConfig
import com.tastebyte.erp.core.auth.TokenStorage
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory
import java.util.concurrent.TimeUnit

object ApiClient {

    private var apiService: ApiService? = null
    private var tokenStorage: TokenStorage? = null

    fun init(tokenStorage: TokenStorage) {
        this.tokenStorage = tokenStorage
        apiService = null
    }

    fun getService(): ApiService {
        if (apiService == null) {
            val storage = tokenStorage
                ?: throw IllegalStateException("ApiClient not initialized. Call init() first.")

            val loggingInterceptor = HttpLoggingInterceptor().apply {
                level = if (BuildConfig.DEBUG) {
                    HttpLoggingInterceptor.Level.BODY
                } else {
                    HttpLoggingInterceptor.Level.NONE
                }
            }

            val client = OkHttpClient.Builder()
                .addInterceptor(AuthInterceptor(storage))
                .addInterceptor(OfflineCacheInterceptor())
                .addInterceptor(loggingInterceptor)
                .connectTimeout(30, TimeUnit.SECONDS)
                .readTimeout(30, TimeUnit.SECONDS)
                .writeTimeout(30, TimeUnit.SECONDS)
                .build()

            val retrofit = Retrofit.Builder()
                .baseUrl(BuildConfig.API_BASE_URL + "/")
                .client(client)
                .addConverterFactory(GsonConverterFactory.create())
                .build()

            apiService = retrofit.create(ApiService::class.java)
        }
        return apiService!!
    }

    fun reset() {
        apiService = null
    }
}
