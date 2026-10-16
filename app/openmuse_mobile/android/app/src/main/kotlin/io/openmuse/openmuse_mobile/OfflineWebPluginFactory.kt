package io.openmuse.openmuse_mobile

import android.content.Context
import android.graphics.Color
import android.view.View
import android.webkit.WebResourceRequest
import android.webkit.WebResourceResponse
import android.webkit.WebView
import android.webkit.WebViewClient
import androidx.webkit.WebViewAssetLoader
import io.flutter.plugin.common.StandardMessageCodec
import io.flutter.plugin.platform.PlatformView
import io.flutter.plugin.platform.PlatformViewFactory
import java.io.File
import java.io.FileInputStream

/** Serves verified installed web resources from app-private storage to an in-app WebView. */
internal class OfflineWebPluginFactory(private val appContext: Context) :
    PlatformViewFactory(StandardMessageCodec.INSTANCE) {
    override fun create(context: Context, viewId: Int, args: Any?): PlatformView {
        val digest = (args as? Map<*, *>)?.get("digest") as? String
        require(digest != null && digest.matches(Regex("[a-f0-9]{64}")))
        val root = File(appContext.filesDir, "mcp-web/$digest").canonicalFile
        require(File(root, "index.html").isFile)
        val domain = "p${digest.substring(0, 32)}.p${digest.substring(32)}.plugins.openmuse.io"
        val loader = WebViewAssetLoader.Builder()
            .setDomain(domain)
            .addPathHandler("/editor/", WebViewAssetLoader.PathHandler { path ->
                val file = File(root, path).canonicalFile
                if (!file.path.startsWith(root.path + File.separator) || !file.isFile) {
                    return@PathHandler null
                }
                WebResourceResponse(mimeType(file), null, FileInputStream(file))
            })
            .build()
        val webView = WebView(context)
        webView.setBackgroundColor(Color.WHITE)
        webView.settings.javaScriptEnabled = true
        webView.settings.domStorageEnabled = true
        webView.settings.allowFileAccess = false
        webView.settings.allowContentAccess = false
        webView.settings.setSupportMultipleWindows(false)
        webView.webViewClient = object : WebViewClient() {
            override fun shouldOverrideUrlLoading(view: WebView, request: WebResourceRequest): Boolean =
                request.url.host != domain || request.url.scheme != "https"

            override fun shouldInterceptRequest(
                view: WebView,
                request: WebResourceRequest,
            ): WebResourceResponse {
                if (request.url.host == domain && request.url.scheme == "https") {
                    loader.shouldInterceptRequest(request.url)?.let { return it }
                }
                return WebResourceResponse("text/plain", "UTF-8", 403, "Blocked", emptyMap(),
                    "Blocked".byteInputStream())
            }
        }
        webView.loadUrl("https://$domain/editor/index.html")
        return object : PlatformView {
            override fun getView(): View = webView
            override fun dispose() { webView.destroy() }
        }
    }

    private fun mimeType(file: File): String = when (file.extension.lowercase()) {
        "html" -> "text/html"
        "js", "mjs" -> "text/javascript"
        "wasm" -> "application/wasm"
        "json" -> "application/json"
        "svg" -> "image/svg+xml"
        "css" -> "text/css"
        "png" -> "image/png"
        "ttf" -> "font/ttf"
        "woff" -> "font/woff"
        "woff2" -> "font/woff2"
        else -> "application/octet-stream"
    }
}
