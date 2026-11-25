package com.openmuseai.mobile

import android.content.Context
import android.graphics.Color
import android.util.Base64
import android.view.View
import android.webkit.WebResourceRequest
import android.webkit.WebResourceResponse
import android.webkit.WebView
import android.webkit.WebViewClient
import android.webkit.WebChromeClient
import android.webkit.ValueCallback
import android.webkit.PermissionRequest
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
internal class OfflineWebPluginFactory(private val appContext: Context, private val messenger: BinaryMessenger, private val filePicker: OfflineWebFilePicker, private val microphone: OfflineWebMicrophone, private val printer: OfflineWebPrint) :
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
        val editorDomain = "v${digest.substring(0, 32)}.p${digest.substring(32)}.plugins.openmuse.io"
        val hostRoot = File(appContext.cacheDir, "mcp-host/$viewId").apply { mkdirs() }
        val downloadLock = Any()
        val exportTargets = mutableMapOf<String, File>()
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
        // Dedicated View origin has only verified editor resources. It never
        // serves Host arguments or receives the native bridge injection.
        val editorLoader = WebViewAssetLoader.Builder().setDomain(editorDomain)
            .addPathHandler("/editor/", WebViewAssetLoader.PathHandler { path ->
                val file = File(root, path).canonicalFile
                if (!file.path.startsWith(root.path + File.separator) || !file.isFile) return@PathHandler null
                WebResourceResponse(mimeType(file), null, FileInputStream(file))
            }).build()
        if ((appContext.applicationInfo.flags and android.content.pm.ApplicationInfo.FLAG_DEBUGGABLE) != 0) {
            WebView.setWebContentsDebuggingEnabled(true)
        }
        val webView = WebView(context)
        webView.setBackgroundColor(Color.WHITE)
        webView.settings.javaScriptEnabled = true
        webView.settings.domStorageEnabled = true
        webView.settings.allowFileAccess = false
        // File inputs need the user's content:// grant. The request gate below
        // admits only the exact selection for this View; no library permission.
        webView.settings.allowContentAccess = true
        webView.settings.setSupportMultipleWindows(false)
        webView.webChromeClient = object : WebChromeClient() {
            override fun onShowFileChooser(view: WebView, callback: ValueCallback<Array<Uri>>, params: FileChooserParams): Boolean =
                filePicker.open(view, callback, params)
            override fun onPermissionRequest(request: PermissionRequest) {
                webView.post { microphone.open(webView, request, editorDomain, html != null && params?.get("microphoneRequested") == true) }
            }
            override fun onPermissionRequestCanceled(request: PermissionRequest) { microphone.canceled(request) }
        }
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
                request.url.scheme != "https" || (request.url.host != domain &&
                    (request.isForMainFrame || request.url.host != editorDomain || !request.url.path.orEmpty().startsWith("/editor/")))

            override fun shouldInterceptRequest(
                view: WebView,
                request: WebResourceRequest,
            ): WebResourceResponse? {
                if (filePicker.permitsContent(view, request.url)) return super.shouldInterceptRequest(view, request)
                if (request.url.host == domain && request.url.scheme == "https") {
                    loader.shouldInterceptRequest(request.url)?.let { return it }
                }
                if (request.url.host == editorDomain && request.url.scheme == "https") {
                    editorLoader.shouldInterceptRequest(request.url)?.let { return it }
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
                if (incoming.optString("method") == "io.openmuse/browser-print") {
                    val response = JSONObject().put("jsonrpc", "2.0").put("id", incoming.opt("id"))
                    val reply: (Boolean) -> Unit = { ok ->
                        if (ok) response.put("result", JSONObject().put("dialogOpened", true))
                        else response.put("error", JSONObject().put("code", -32603).put("message", "System print dialog unavailable"))
                        view.post { view.evaluateJavascript("window.__mcpHostReceive($response)", null) }
                    }
                    try {
                        val input = incoming.getJSONObject("params")
                        printer.open(view, input.getString("html"), input.optString("title", "Document"), input.getInt("widthMils"), input.getInt("heightMils"), reply)
                    } catch (_: Exception) { reply(false) }
                    return@addWebMessageListener
                }
                val resource = incoming.optJSONObject("params")?.optJSONArray("contents")?.let { if (it.length() == 1) it.optJSONObject(0)?.optJSONObject("resource") else null }
                val isOriginal = resource?.let { DownloadIdentity.matches(Uri.parse(it.optString("uri")).lastPathSegment ?: "", it.optString("mimeType"), fileName, mediaType) } == true
                if (incoming.optString("method") != "ui/download-file" || params?.get("forwardDownloads") == true && isOriginal) {
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
                        val mime = resource.optString("mimeType", "application/octet-stream")
                        synchronized(downloadLock) {
                            if (!DownloadIdentity.mayReplace(name, mime, fileName, mediaType, grantedPath) && SharedBrowserExport.save(appContext, name, mime, bytes)) {
                                return@synchronized
                            }
                            val destination = if (DownloadIdentity.mayReplace(name, mime, fileName, mediaType, grantedPath)) File(grantedPath!!)
                                else exportTargets.getOrPut("$name\u0000$mime") {
                                    val directory = File(appContext.getExternalFilesDir(Environment.DIRECTORY_DOCUMENTS), "Exports").apply { mkdirs() }
                                    var candidate = File(directory, name)
                                    var suffix = 1
                                    while (!candidate.createNewFile()) {
                                        val stem = File(name).nameWithoutExtension
                                        val extension = File(name).extension.let { if (it.isEmpty()) "" else ".$it" }
                                        candidate = File(directory, "$stem (${suffix++})$extension")
                                    }
                                    candidate
                                }
                            destination.parentFile?.mkdirs()
                            val temporary = File(destination.parentFile, destination.name + ".saving-" + java.util.UUID.randomUUID())
                            try {
                                temporary.writeBytes(bytes)
                                require(temporary.renameTo(destination))
                            } finally { temporary.delete() }
                        }
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
            override fun dispose() { printer.cancel(webView); microphone.cancel(webView); filePicker.cancel(webView); channel.setMethodCallHandler(null); webView.destroy(); hostRoot.deleteRecursively() }
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
