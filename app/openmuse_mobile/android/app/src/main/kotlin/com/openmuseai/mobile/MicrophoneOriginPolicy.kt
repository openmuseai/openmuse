package com.openmuseai.mobile

import java.net.URI

internal object MicrophoneOriginPolicy {
    fun permits(declared: Boolean, requestedOrigin: String, expectedHost: String, resources: List<String>): Boolean {
        if (!declared || resources != listOf("android.webkit.resource.AUDIO_CAPTURE")) return false
        return try {
            val uri = URI(requestedOrigin)
            uri.scheme == "https" && uri.host == expectedHost && (uri.port == -1 || uri.port == 443) &&
                uri.rawUserInfo == null && uri.rawQuery == null && uri.rawFragment == null &&
                (uri.rawPath.isNullOrEmpty() || uri.rawPath == "/")
        } catch (_: Exception) { false }
    }
}
