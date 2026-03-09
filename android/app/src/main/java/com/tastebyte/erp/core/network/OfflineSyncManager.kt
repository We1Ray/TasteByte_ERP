package com.tastebyte.erp.core.network

import android.content.Context
import com.google.gson.Gson
import com.tastebyte.erp.core.auth.TokenStorage
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody
import java.io.File
import java.util.UUID

class OfflineSyncManager private constructor(context: Context) {

    private val queueDir = File(context.filesDir, "offline_queue").apply { mkdirs() }
    private val gson = Gson()
    private val scope = CoroutineScope(SupervisorJob() + Dispatchers.IO)
    private val httpClient = OkHttpClient()

    private val _pendingCount = MutableStateFlow(loadQueue().size)
    val pendingCount: StateFlow<Int> = _pendingCount.asStateFlow()

    fun enqueue(method: String, url: String, body: String?) {
        val op = PendingOperation(
            id = UUID.randomUUID().toString(),
            method = method,
            url = url,
            body = body,
            createdAt = System.currentTimeMillis()
        )
        val file = File(queueDir, "${op.id}.json")
        file.writeText(gson.toJson(op))
        _pendingCount.value = loadQueue().size
    }

    fun syncPendingOperations(tokenStorage: TokenStorage) {
        scope.launch {
            val operations = loadQueue()
            if (operations.isEmpty()) return@launch

            for (op in operations) {
                try {
                    executeOperation(op, tokenStorage)
                    File(queueDir, "${op.id}.json").delete()
                    _pendingCount.value = maxOf(0, _pendingCount.value - 1)
                } catch (e: Exception) {
                    // Stop on first failure to preserve order
                    break
                }
            }
        }
    }

    private fun executeOperation(op: PendingOperation, tokenStorage: TokenStorage) {
        val builder = Request.Builder().url(op.url)

        tokenStorage.getAccessToken()?.let { token ->
            builder.header("Authorization", "Bearer $token")
        }
        builder.header("Content-Type", "application/json")

        val requestBody = op.body?.toRequestBody("application/json".toMediaType())

        when (op.method) {
            "POST" -> builder.post(requestBody ?: "".toRequestBody("application/json".toMediaType()))
            "PUT" -> builder.put(requestBody ?: "".toRequestBody("application/json".toMediaType()))
        }

        val response = httpClient.newCall(builder.build()).execute()
        if (!response.isSuccessful) {
            throw Exception("Sync failed: ${response.code}")
        }
    }

    private fun loadQueue(): List<PendingOperation> {
        return queueDir.listFiles()
            ?.filter { it.extension == "json" }
            ?.mapNotNull { file ->
                try {
                    gson.fromJson(file.readText(), PendingOperation::class.java)
                } catch (e: Exception) {
                    null
                }
            }
            ?.sortedBy { it.createdAt }
            ?: emptyList()
    }

    companion object {
        @Volatile
        private var instance: OfflineSyncManager? = null

        fun init(context: Context) {
            if (instance == null) {
                synchronized(this) {
                    if (instance == null) {
                        instance = OfflineSyncManager(context.applicationContext)
                    }
                }
            }
        }

        fun getInstance(): OfflineSyncManager {
            return instance ?: throw IllegalStateException(
                "OfflineSyncManager not initialized. Call OfflineSyncManager.init(context) first."
            )
        }
    }
}

private data class PendingOperation(
    val id: String,
    val method: String,
    val url: String,
    val body: String?,
    val createdAt: Long
)
