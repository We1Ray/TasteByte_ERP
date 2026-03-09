package com.tastebyte.erp.core.storage

import android.content.Context
import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import java.io.File

class CacheManager private constructor(context: Context) {

    private val cacheDir = File(context.cacheDir, "api_cache").apply { mkdirs() }
    private val gson = Gson()

    fun <T> save(key: String, data: T, ttlMillis: Long = DEFAULT_TTL) {
        val entry = CacheEntry(
            data = gson.toJson(data),
            expiresAt = System.currentTimeMillis() + ttlMillis
        )
        val file = File(cacheDir, key.sanitize())
        file.writeText(gson.toJson(entry))
    }

    inline fun <reified T> get(key: String): T? {
        return get(key, object : TypeToken<T>() {})
    }

    fun <T> get(key: String, typeToken: TypeToken<T>): T? {
        val file = File(cacheDir, key.sanitize())
        if (!file.exists()) return null

        return try {
            val entry = gson.fromJson(file.readText(), CacheEntry::class.java)
            if (entry.expiresAt < System.currentTimeMillis()) {
                file.delete()
                null
            } else {
                gson.fromJson(entry.data, typeToken.type)
            }
        } catch (e: Exception) {
            file.delete()
            null
        }
    }

    fun clear() {
        cacheDir.listFiles()?.forEach { it.delete() }
    }

    private fun String.sanitize(): String {
        return this.replace(Regex("[^a-zA-Z0-9_-]"), "_")
    }

    companion object {
        private const val DEFAULT_TTL = 3600_000L // 1 hour

        @Volatile
        private var instance: CacheManager? = null

        fun init(context: Context) {
            if (instance == null) {
                synchronized(this) {
                    if (instance == null) {
                        instance = CacheManager(context.applicationContext)
                    }
                }
            }
        }

        fun getInstance(): CacheManager {
            return instance ?: throw IllegalStateException(
                "CacheManager not initialized. Call CacheManager.init(context) first."
            )
        }
    }
}

private data class CacheEntry(
    val data: String,
    val expiresAt: Long
)
