package com.openmuseai.mobile

import android.app.Application
import android.os.Handler
import android.os.Looper
import io.flutter.embedding.engine.FlutterJNI
import java.io.File

/** USB test diagnostics when OEM logcat is unavailable. Debug source set only.
 * The VM service keeps its authentication nonce and loopback binding. The URI
 * is readable only by this debug App UID (or explicitly authorized adb run-as).
 */
class DebugApplication : Application() {
    override fun onCreate() {
        super.onCreate()
        val receipt = File(filesDir, "debug-vm-service-uri")
        receipt.delete()
        val handler = Handler(Looper.getMainLooper())
        val deadline = System.currentTimeMillis() + 120_000
        val poll = object : Runnable {
            override fun run() {
                val uri = FlutterJNI.getVMServiceUri()
                if (!uri.isNullOrEmpty()) {
                    receipt.writeText(uri)
                    receipt.setReadable(false, false)
                    receipt.setReadable(true, true)
                    receipt.setWritable(false, false)
                    receipt.setWritable(true, true)
                } else if (System.currentTimeMillis() < deadline) {
                    handler.postDelayed(this, 100)
                }
            }
        }
        handler.post(poll)
    }
}
