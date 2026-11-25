package com.openmuseai.mobile

import android.app.Activity
import android.content.Intent
import android.net.Uri
import android.webkit.ValueCallback
import android.webkit.WebChromeClient
import android.webkit.WebView

/** Activity-owned, user-mediated file input. No storage permission or raw path. */
internal class OfflineWebFilePicker(private val activity: Activity) {
    private data class Pending(val view: WebView, var callback: ValueCallback<Array<Uri>>?)
    private val pending = mutableMapOf<Int, Pending>()
    private val selectedContent = SelectedContentScope<WebView>()
    private var nextCode = 0x7e00

    fun open(view: WebView, callback: ValueCallback<Array<Uri>>, params: WebChromeClient.FileChooserParams): Boolean {
        // A cancelled View keeps its request-code tombstone until its result
        // arrives: a late picker result must never reach another plugin View.
        cancel(view)
        val code = (0 until 256).map { 0x7e00 + ((nextCode++ - 0x7e00) and 255) }
            .firstOrNull { !pending.containsKey(it) } ?: return false
        val intent = try { params.createIntent() } catch (_: Exception) { return false }
        intent.addCategory(Intent.CATEGORY_OPENABLE)
        intent.addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
        pending[code] = Pending(view, callback)
        return try {
            activity.startActivityForResult(intent, code)
            true
        } catch (_: Exception) {
            pending.remove(code)
            false
        }
    }

    fun receive(code: Int, resultCode: Int, data: Intent?): Boolean {
        val request = pending.remove(code) ?: return false
        val selected = WebChromeClient.FileChooserParams.parseResult(resultCode, data)
        // Keep the browser sandbox closed to file:// and arbitrary network URIs.
        val allowed = selected?.takeIf { it.size <= 20 && it.all { uri -> uri.scheme == "content" } }
        if (request.callback != null && allowed != null) {
            selectedContent.replace(request.view, allowed.map { it.toString() })
        }
        request.callback?.onReceiveValue(allowed)
        return true
    }

    fun cancel(view: WebView) {
        selectedContent.revoke(view)
        pending.values.filter { it.view === view }.forEach { request ->
            request.callback?.onReceiveValue(null)
            request.callback = null
        }
    }

    fun permitsContent(view: WebView, uri: Uri): Boolean =
        uri.scheme == "content" && selectedContent.permits(view, uri.toString())

    fun close() {
        pending.values.forEach { it.callback?.onReceiveValue(null) }
        pending.clear()
        selectedContent.clear()
    }
}
