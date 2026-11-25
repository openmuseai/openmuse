package com.openmuseai.mobile

import android.app.Activity
import android.os.Bundle
import android.os.CancellationSignal
import android.os.Handler
import android.os.Looper
import android.os.ParcelFileDescriptor
import android.print.PageRange
import android.print.PrintAttributes
import android.print.PrintDocumentAdapter
import android.print.PrintJob
import android.print.PrintManager
import android.webkit.WebResourceRequest
import android.webkit.WebResourceResponse
import android.webkit.WebView
import android.webkit.WebViewClient
import java.io.ByteArrayInputStream

/** Generic isolated HTML snapshot printing. No plugin bridge, JS, file grants
 * or networking is installed on this temporary print surface. */
internal class OfflineWebPrint(private val activity: Activity) {
    private var source: WebView? = null
    private var surface: WebView? = null
    private var job: PrintJob? = null
    private var pendingReply: ((Boolean) -> Unit)? = null
    private val handler = Handler(Looper.getMainLooper())

    fun open(view: WebView, html: String, title: String, widthMils: Int, heightMils: Int, reply: (Boolean) -> Unit) {
        require(source == null && view.isAttachedToWindow && view.isShown && activity.hasWindowFocus())
        require(html.toByteArray(Charsets.UTF_8).size <= 64 * 1024 * 1024)
        require(widthMils in 1000..30000 && heightMils in 1000..30000)
        source = view
        var answered = false
        val answer = { ok: Boolean -> if (!answered) { answered = true; pendingReply = null; reply(ok) } }
        pendingReply = answer
        val printView = WebView(activity)
        surface = printView
        printView.settings.javaScriptEnabled = false
        printView.settings.allowFileAccess = false
        printView.settings.allowContentAccess = false
        printView.settings.blockNetworkLoads = true
        printView.webViewClient = object : WebViewClient() {
            override fun shouldOverrideUrlLoading(v: WebView, request: WebResourceRequest) = true
            override fun shouldInterceptRequest(v: WebView, request: WebResourceRequest): WebResourceResponse? =
                if (request.url.scheme in listOf("http", "https", "file", "content"))
                    WebResourceResponse("text/plain", "UTF-8", 403, "Blocked", emptyMap(), ByteArrayInputStream(ByteArray(0))) else null

            override fun onPageFinished(v: WebView, url: String) {
                if (surface !== v || answered) return
                try {
                    val delegate = v.createPrintDocumentAdapter(title.take(200))
                    val adapter = object : PrintDocumentAdapter() {
                        override fun onStart() = delegate.onStart()
                        override fun onLayout(old: PrintAttributes?, new: PrintAttributes, signal: CancellationSignal, callback: LayoutResultCallback, extras: Bundle?) =
                            delegate.onLayout(old, new, signal, callback, extras)
                        override fun onWrite(pages: Array<out PageRange>, destination: ParcelFileDescriptor, signal: CancellationSignal, callback: WriteResultCallback) =
                            delegate.onWrite(pages, destination, signal, callback)
                        override fun onFinish() { delegate.onFinish(); release(v) }
                    }
                    val attributes = PrintAttributes.Builder().setMediaSize(PrintAttributes.MediaSize("OPENMUSE", "Document", widthMils, heightMils))
                        .setMinMargins(PrintAttributes.Margins.NO_MARGINS).build()
                    job = (activity.getSystemService(Activity.PRINT_SERVICE) as PrintManager).print(title.take(200), adapter, attributes)
                    answer(true) // System dialog opened; physical printing is not claimed.
                } catch (_: Exception) { release(v); answer(false) }
            }
        }
        handler.postDelayed({ if (!answered && surface === printView) { release(printView); answer(false) } }, 30000)
        printView.loadDataWithBaseURL("https://print.openmuse.invalid/", html, "text/html", "UTF-8", null)
    }

    private fun release(view: WebView) {
        if (surface !== view) return
        surface = null; source = null; job = null
        view.destroy()
        pendingReply?.invoke(false)
        pendingReply = null
    }
    fun cancel(view: WebView) { if (source === view) { job?.cancel(); surface?.let { release(it) } } }
    fun close() { source?.let { cancel(it) } }
}
