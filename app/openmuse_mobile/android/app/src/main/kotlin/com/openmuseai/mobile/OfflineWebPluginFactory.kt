package com.openmuseai.mobile

import android.content.Context
import android.graphics.Color
import android.util.Base64
import android.view.View
import android.webkit.WebResourceRequest
import android.webkit.WebResourceResponse
import android.webkit.WebView
import android.webkit.WebViewClient
import androidx.webkit.WebViewAssetLoader
import androidx.webkit.WebViewCompat
import androidx.webkit.WebViewFeature
import android.os.Environment
import android.net.Uri
import io.flutter.plugin.common.StandardMessageCodec
import io.flutter.plugin.common.BinaryMessenger
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.platform.PlatformView
import io.flutter.plugin.platform.PlatformViewFactory
import java.io.File
import java.io.FileInputStream
import java.io.FileOutputStream
import android.util.Base64OutputStream
import org.json.JSONObject

/** Serves verified installed web resources from app-private storage to an in-app WebView. */
internal class OfflineWebPluginFactory(private val appContext: Context, private val messenger: BinaryMessenger) :
    PlatformViewFactory(StandardMessageCodec.INSTANCE) {
    override fun create(context: Context, viewId: Int, args: Any?): PlatformView {
        val html = (args as? Map<*, *>)?.get("html") as? String
        require(html == null || html.length <= 128 * 1024 * 1024)
        val digest = (args as? Map<*, *>)?.get("digest") as? String
        require(digest != null && digest.matches(Regex("[a-f0-9]{64}")))
        val root = File(appContext.filesDir, "mcp-web/$digest").canonicalFile
        require(html != null || File(root, "index.html").isFile)
        val params = args as? Map<*, *>
        val fileName = params?.get("fileName") as? String
        val mediaType = params?.get("mediaType") as? String
        val fileBytes = params?.get("fileBytes") as? ByteArray
        require(fileBytes == null || (fileName != null && mediaType != null && fileBytes.size <= 64 * 1024 * 1024))
        val domain = "p${digest.substring(0, 32)}.p${digest.substring(32)}.plugins.openmuse.io"
        val hostRoot = File(appContext.cacheDir, "mcp-host/$viewId").apply { mkdirs() }
        if (html != null) {
            File(hostRoot, "index.html").writeText(html)
            FileOutputStream(File(hostRoot, "arguments.json")).use { output ->
                output.write(("{\"name\":" + JSONObject.quote(fileName ?: "Document") + ",\"content\":\"").toByteArray())
                val encoded = Base64OutputStream(output, Base64.NO_WRAP or Base64.NO_CLOSE)
                encoded.write(fileBytes ?: ByteArray(0))
                encoded.close()
                output.write("\"}".toByteArray())
            }
        }
        val loader = WebViewAssetLoader.Builder()
            .setDomain(domain)
            .addPathHandler("/host/", WebViewAssetLoader.PathHandler { path ->
                val file = File(hostRoot, path).canonicalFile
                if (!file.path.startsWith(hostRoot.canonicalPath + File.separator) || !file.isFile) return@PathHandler null
                WebResourceResponse(mimeType(file), null, FileInputStream(file))
            })
            .addPathHandler("/editor/", WebViewAssetLoader.PathHandler { path ->
                val file = File(root, path).canonicalFile
                if (!file.path.startsWith(root.path + File.separator) || !file.isFile) {
                    return@PathHandler null
                }
                WebResourceResponse(mimeType(file), null, FileInputStream(file))
            })
            .build()
        if ((appContext.applicationInfo.flags and android.content.pm.ApplicationInfo.FLAG_DEBUGGABLE) != 0) {
            WebView.setWebContentsDebuggingEnabled(true)
        }
        val webView = WebView(context)
        webView.setBackgroundColor(Color.WHITE)
        webView.settings.javaScriptEnabled = true
        webView.settings.domStorageEnabled = true
        webView.settings.allowFileAccess = false
        webView.settings.allowContentAccess = false
        webView.settings.setSupportMultipleWindows(false)
        webView.webViewClient = object : WebViewClient() {
            override fun onPageFinished(view: WebView, url: String) {
                if (html != null || fileBytes == null || fileName == null || mediaType == null) return
                val encoded = Base64.encodeToString(fileBytes, Base64.NO_WRAP)
                val script = """(function(){
                    const binary=atob(${JSONObject.quote(encoded)});
                    const bytes=Uint8Array.from(binary,c=>c.charCodeAt(0));
                    const file=new File([bytes],${JSONObject.quote(fileName)},
                        {type:${JSONObject.quote(mediaType)}});
                    window.postMessage(file,window.location.origin);
                })()""".trimIndent()
                view.evaluateJavascript(script, null)
            }
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
        val channel = MethodChannel(messenger, "io.openmuse/mcp_view/$viewId")
        channel.setMethodCallHandler { call, result ->
            if (call.method != "receive") { result.notImplemented(); return@setMethodCallHandler }
            val raw = call.arguments as? String
            if (raw == null || raw.length > 2 * 1024 * 1024) {
                result.error("invalid_message", "Message exceeds budget", null)
            } else {
                webView.evaluateJavascript("window.__mcpHostReceive($raw)", null)
                result.success(null)
            }
        }
        if (html != null) {
            require(WebViewFeature.isFeatureSupported(WebViewFeature.WEB_MESSAGE_LISTENER))
            WebViewCompat.addWebMessageListener(webView, "McpHost", setOf("https://$domain")) { view, message, _, isMainFrame, _ ->
                if (!isMainFrame) return@addWebMessageListener
                val raw = message.data ?: return@addWebMessageListener
                if (raw.length > 96 * 1024 * 1024) return@addWebMessageListener
                val incoming = try { JSONObject(raw) } catch (_: Exception) { return@addWebMessageListener }
                if (incoming.optString("method") != "ui/download-file") {
                    channel.invokeMethod("message", raw)
                    return@addWebMessageListener
                }
                Thread {
                    val response = JSONObject().put("jsonrpc", "2.0")
                    try {
                        val request = JSONObject(raw)
                        response.put("id", request.get("id"))
                        require(request.getString("method") == "ui/download-file")
                        val contents = request.getJSONObject("params").getJSONArray("contents")
                        require(contents.length() == 1)
                        val resource = contents.getJSONObject(0).getJSONObject("resource")
                        val bytes = if (resource.has("blob")) Base64.decode(resource.getString("blob"), Base64.DEFAULT)
                            else resource.getString("text").toByteArray(Charsets.UTF_8)
                        require(bytes.size <= 64 * 1024 * 1024)
                        val grantedPath = params?.get("filePath") as? String
                        val name = Uri.parse(resource.getString("uri")).lastPathSegment ?: "Export"
                        require(!name.contains('/') && !name.contains('\\') && name.length <= 200)
                        val destination = if (grantedPath != null) File(grantedPath)
                            else File(appContext.getExternalFilesDir(Environment.DIRECTORY_DOCUMENTS), name)
                        destination.parentFile?.mkdirs()
                        val temporary = File(destination.parentFile, destination.name + ".saving")
                        temporary.writeBytes(bytes)
                        require(temporary.renameTo(destination))
                        response.put("result", JSONObject())
                    } catch (error: Exception) {
                        response.put("error", JSONObject().put("code", -32603).put("message", "Unable to save the authorized resource"))
                    }
                    view.post { view.evaluateJavascript("window.__mcpHostReceive($response)", null) }
                }.start()
            }
            webView.loadUrl("https://$domain/host/index.html")
        } else webView.loadUrl("https://$domain/editor/index.html")
        return object : PlatformView {
            override fun getView(): View = webView
            override fun dispose() { channel.setMethodCallHandler(null); webView.destroy(); hostRoot.deleteRecursively() }
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
