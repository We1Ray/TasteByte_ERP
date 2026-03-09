package com.tastebyte.erp

import android.app.Application
import com.tastebyte.erp.core.network.NetworkMonitor
import com.tastebyte.erp.core.network.OfflineSyncManager
import com.tastebyte.erp.core.storage.CacheManager

class TasteByteApp : Application() {
    override fun onCreate() {
        super.onCreate()
        instance = this
        CacheManager.init(this)
        NetworkMonitor.init(this)
        OfflineSyncManager.init(this)
    }

    companion object {
        lateinit var instance: TasteByteApp
            private set
    }
}
