package com.tastebyte.erp.core.auth

import com.tastebyte.erp.core.network.ApiClient
import com.tastebyte.erp.models.LoginRequest
import com.tastebyte.erp.models.LogoutRequest
import com.tastebyte.erp.models.RefreshRequest
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow

class AuthManager(private val tokenStorage: TokenStorage) {

    private val _isAuthenticated = MutableStateFlow(tokenStorage.hasToken())
    val isAuthenticated: StateFlow<Boolean> = _isAuthenticated.asStateFlow()

    private val _currentUsername = MutableStateFlow(tokenStorage.getUsername())
    val currentUsername: StateFlow<String?> = _currentUsername.asStateFlow()

    suspend fun login(username: String, password: String): Result<Unit> {
        return try {
            val response = ApiClient.getService().login(
                LoginRequest(username = username, password = password)
            )
            if (response.success && response.data != null) {
                tokenStorage.saveAccessToken(response.data.accessToken)
                tokenStorage.saveRefreshToken(response.data.refreshToken)
                tokenStorage.saveTokenExpiry(response.data.expiresIn)
                tokenStorage.saveUsername(username)
                _isAuthenticated.value = true
                _currentUsername.value = username
                Result.success(Unit)
            } else {
                Result.failure(Exception(response.error ?: "Login failed"))
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }

    suspend fun refreshToken(): Boolean {
        return try {
            val refreshToken = tokenStorage.getRefreshToken() ?: return false
            val response = ApiClient.getService().refreshToken(
                RefreshRequest(refreshToken = refreshToken)
            )
            if (response.success && response.data != null) {
                tokenStorage.saveAccessToken(response.data.accessToken)
                tokenStorage.saveRefreshToken(response.data.refreshToken)
                tokenStorage.saveTokenExpiry(response.data.expiresIn)
                true
            } else {
                false
            }
        } catch (e: Exception) {
            false
        }
    }

    suspend fun logout() {
        try {
            val refreshToken = tokenStorage.getRefreshToken()
            if (refreshToken != null) {
                ApiClient.getService().logout(
                    LogoutRequest(refreshToken = refreshToken)
                )
            }
        } catch (_: Exception) {
            // Best-effort server logout; proceed with local cleanup regardless
        }
        tokenStorage.clearAll()
        ApiClient.reset()
        _isAuthenticated.value = false
        _currentUsername.value = null
    }

    fun checkAuthState() {
        _isAuthenticated.value = tokenStorage.hasToken()
        _currentUsername.value = tokenStorage.getUsername()
    }
}
