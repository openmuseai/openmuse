package cn.wps.pdf_rebuild_flutter

import android.app.Activity
import android.content.Intent
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodChannel
import java.io.File

class MainActivity : FlutterActivity() {
    private val saveRequestCode = 7821
    private var pendingSave: MethodChannel.Result? = null
    private var pendingSourcePath: String? = null

    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)
        MethodChannel(
            flutterEngine.dartExecutor.binaryMessenger,
            "cn.wps.pdf_rebuild_flutter/document_io"
        ).setMethodCallHandler { call, result ->
            if (call.method != "savePdf") {
                result.notImplemented()
                return@setMethodCallHandler
            }
            if (pendingSave != null) {
                result.error("busy", "A document save is already in progress", null)
                return@setMethodCallHandler
            }
            val sourcePath = call.argument<String>("sourcePath")
            val name = call.argument<String>("name")?.takeIf { it.isNotBlank() }
                ?: "document.pdf"
            if (sourcePath == null || !File(sourcePath).isFile) {
                result.error("source", "PDF source file is missing", null)
                return@setMethodCallHandler
            }
            pendingSave = result
            pendingSourcePath = sourcePath
            val intent = Intent(Intent.ACTION_CREATE_DOCUMENT).apply {
                addCategory(Intent.CATEGORY_OPENABLE)
                type = "application/pdf"
                putExtra(Intent.EXTRA_TITLE, name)
                addFlags(Intent.FLAG_GRANT_WRITE_URI_PERMISSION)
            }
            try {
                startActivityForResult(intent, saveRequestCode)
            } catch (error: Exception) {
                pendingSave = null
                pendingSourcePath = null
                result.error("picker", error.message, null)
            }
        }
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        if (requestCode != saveRequestCode) return
        val result = pendingSave ?: return
        val sourcePath = pendingSourcePath
        pendingSave = null
        pendingSourcePath = null
        if (resultCode != Activity.RESULT_OK || data?.data == null) {
            result.success(false)
            return
        }
        if (sourcePath == null) {
            result.error("source", "PDF source file is missing", null)
            return
        }
        val target = data.data!!
        Thread {
            try {
                val stream = contentResolver.openOutputStream(target, "w")
                    ?: throw IllegalStateException("Cannot open selected document")
                stream.use { output -> File(sourcePath).inputStream().use { it.copyTo(output) } }
                runOnUiThread { result.success(true) }
            } catch (error: Exception) {
                runOnUiThread { result.error("write", error.message, null) }
            }
        }.start()
    }
}
