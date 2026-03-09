package com.tastebyte.erp.core.network

import com.google.gson.Gson
import com.google.gson.annotations.SerializedName
import com.tastebyte.erp.core.auth.TokenStorage
import okhttp3.Interceptor
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody
import okhttp3.Response

class AuthInterceptor(private val tokenStorage: TokenStorage) : Interceptor {

    @Volatile
    private var isRefreshing = false

    private val gson = Gson()

    override fun intercept(chain: Interceptor.Chain): Response {
        val original = chain.request()
        val request = addAuthHeader(original)
        val response = chain.proceed(request)

        if (response.code == 401 && !isAuthEndpoint(original)) {
            synchronized(this) {
                // Double-check: another thread may have already refreshed
                val currentToken = tokenStorage.getAccessToken()
                val requestToken = request.header("Authorization")?.removePrefix("Bearer ")

                if (currentToken != null && currentToken != requestToken) {
                    // Token was already refreshed by another request; retry with new token
                    response.close()
                    return chain.proceed(addAuthHeader(original))
                }

                if (isRefreshing) {
                    return response
                }

                isRefreshing = true
                try {
                    val refreshToken = tokenStorage.getRefreshToken()
                    if (refreshToken != null) {
                        val refreshResult = attemptTokenRefresh(chain, refreshToken)
                        if (refreshResult) {
                            response.close()
                            return chain.proceed(addAuthHeader(original))
                        }
                    }
                    // Refresh failed - clear tokens
                    tokenStorage.clearAll()
                    ApiClient.reset()
                } finally {
                    isRefreshing = false
                }
            }
        }

        return response
    }

    private fun addAuthHeader(request: Request): Request {
        val token = tokenStorage.getAccessToken()
        return if (token != null && !isAuthEndpoint(request)) {
            request.newBuilder()
                .header("Authorization", "Bearer $token")
                .build()
        } else {
            request
        }
    }

    private fun isAuthEndpoint(request: Request): Boolean {
        val path = request.url.encodedPath
        return path.contains("/auth/login") ||
                path.contains("/auth/refresh") ||
                path.contains("/auth/logout")
    }

    private fun attemptTokenRefresh(chain: Interceptor.Chain, refreshToken: String): Boolean {
        return try {
            val body = gson.toJson(RefreshBody(refreshToken = refreshToken))
            val mediaType = "application/json; charset=utf-8".toMediaType()
            val requestBody = body.toRequestBody(mediaType)

            val originalUrl = chain.request().url
            val baseUrl = "${originalUrl.scheme}://${originalUrl.host}" +
                    if (originalUrl.port != 80 && originalUrl.port != 443) ":${originalUrl.port}" else ""

            val refreshRequest = Request.Builder()
                .url("$baseUrl/api/v1/auth/refresh")
                .post(requestBody)
                .header("Content-Type", "application/json")
                .build()

            val refreshResponse = chain.proceed(refreshRequest)
            if (refreshResponse.isSuccessful) {
                val responseBody = refreshResponse.body?.string()
                refreshResponse.close()
                if (responseBody != null) {
                    val tokenResponse = gson.fromJson(responseBody, RefreshApiResponse::class.java)
                    if (tokenResponse.success && tokenResponse.data != null) {
                        tokenStorage.saveAccessToken(tokenResponse.data.accessToken)
                        tokenStorage.saveRefreshToken(tokenResponse.data.refreshToken)
                        tokenStorage.saveTokenExpiry(tokenResponse.data.expiresIn)
                        return true
                    }
                }
            } else {
                refreshResponse.close()
            }
            false
        } catch (e: Exception) {
            false
        }
    }

    private data class RefreshBody(
        @SerializedName("refresh_token") val refreshToken: String
    )

    private data class RefreshApiResponse(
        val success: Boolean,
        val data: RefreshTokenData?,
        val error: String? = null
    )

    private data class RefreshTokenData(
        @SerializedName("access_token") val accessToken: String,
        @SerializedName("refresh_token") val refreshToken: String,
        @SerializedName("expires_in") val expiresIn: Long
    )
}
