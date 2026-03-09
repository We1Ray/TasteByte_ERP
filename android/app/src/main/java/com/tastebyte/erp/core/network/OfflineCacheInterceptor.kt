package com.tastebyte.erp.core.network

import com.tastebyte.erp.core.storage.CacheManager
import okhttp3.Interceptor
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.Protocol
import okhttp3.Response
import okhttp3.ResponseBody.Companion.toResponseBody

class OfflineCacheInterceptor : Interceptor {

    override fun intercept(chain: Interceptor.Chain): Response {
        val request = chain.request()
        val isGet = request.method == "GET"
        val cacheKey = request.url.toString()

        if (isGet) {
            return try {
                val response = chain.proceed(request)
                if (response.isSuccessful) {
                    val bodyString = response.body?.string() ?: ""
                    CacheManager.getInstance().save(cacheKey, bodyString)
                    response.newBuilder()
                        .body(bodyString.toResponseBody(response.body?.contentType()))
                        .build()
                } else {
                    response
                }
            } catch (e: Exception) {
                val cached: String? = CacheManager.getInstance().get(cacheKey)
                if (cached != null) {
                    Response.Builder()
                        .request(request)
                        .protocol(Protocol.HTTP_1_1)
                        .code(200)
                        .message("OK (cached)")
                        .body(cached.toResponseBody("application/json".toMediaType()))
                        .build()
                } else {
                    throw e
                }
            }
        }

        // For POST/PUT mutations, queue on failure
        val isMutation = request.method in listOf("POST", "PUT")
        val isAuthRequest = request.url.encodedPath.contains("/auth/")

        return try {
            chain.proceed(request)
        } catch (e: Exception) {
            if (isMutation && !isAuthRequest) {
                val bodyBuffer = okio.Buffer()
                request.body?.writeTo(bodyBuffer)
                val bodyString = bodyBuffer.readUtf8()

                OfflineSyncManager.getInstance().enqueue(
                    method = request.method,
                    url = request.url.toString(),
                    body = bodyString.ifEmpty { null }
                )
            }
            throw e
        }
    }
}
