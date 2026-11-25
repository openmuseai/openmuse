package com.openmuseai.mobile

import android.Manifest
import android.app.Activity
import android.app.AlertDialog
import android.content.pm.PackageManager
import android.webkit.PermissionRequest
import android.webkit.WebView

/** Activity-owned, explicit per-View microphone consent. No camera/resource
 * wildcard grant and no permission response can be reused by a later View. */
internal class OfflineWebMicrophone(private val activity: Activity) {
    private data class Pending(val view: WebView, val request: PermissionRequest, val host: String)
    private var pending: Pending? = null
    private var dialog: AlertDialog? = null
    private var requestingSystemPermission = false
    private val requestCode = 0x7c10

    fun open(view: WebView, request: PermissionRequest, host: String, declared: Boolean) {
        if (!MicrophoneOriginPolicy.permits(declared, request.origin.toString(), host, request.resources.toList()) ||
            !view.isAttachedToWindow || !view.isShown || !activity.hasWindowFocus() || pending != null || requestingSystemPermission) {
            request.deny(); return
        }
        pending = Pending(view, request, host)
        dialog = AlertDialog.Builder(activity)
            .setTitle(R.string.plugin_microphone_title)
            .setMessage(R.string.plugin_microphone_message)
            .setNegativeButton(android.R.string.cancel) { _, _ -> cancel(view) }
            .setPositiveButton(R.string.plugin_microphone_allow) { _, _ ->
                dialog = null
                if (pending?.request !== request) return@setPositiveButton
                if (activity.checkSelfPermission(Manifest.permission.RECORD_AUDIO) == PackageManager.PERMISSION_GRANTED) finish(true)
                else {
                    requestingSystemPermission = true
                    activity.requestPermissions(arrayOf(Manifest.permission.RECORD_AUDIO), requestCode)
                }
            }.setOnCancelListener { cancel(view) }.show()
    }

    fun receive(code: Int, grants: IntArray): Boolean {
        if (code != requestCode) return false
        requestingSystemPermission = false
        finish(grants.size == 1 && grants[0] == PackageManager.PERMISSION_GRANTED)
        return true
    }

    private fun finish(allowed: Boolean) {
        val p = pending ?: return
        pending = null
        if (allowed && p.view.isAttachedToWindow && p.view.isShown &&
            MicrophoneOriginPolicy.permits(true, p.request.origin.toString(), p.host, p.request.resources.toList()))
            p.request.grant(arrayOf(PermissionRequest.RESOURCE_AUDIO_CAPTURE))
        else p.request.deny()
    }
    fun cancel(view: WebView) {
        if (pending?.view !== view) return
        val p = pending; pending = null
        dialog?.setOnCancelListener(null); dialog?.dismiss(); dialog = null
        p?.request?.deny()
        // Keep a pending system request tombstoned until its callback arrives.
    }
    fun canceled(request: PermissionRequest) { pending?.takeIf { it.request === request }?.let { cancel(it.view) } }
    fun close() { pending?.let { cancel(it.view) } }
}
