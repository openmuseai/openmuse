package com.openmuseai.mobile

import android.app.Activity
import android.content.Intent
import android.net.Uri
import android.provider.DocumentsContract
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodChannel
import java.io.File

/** Host implementation of the PDF editor's Android document-save port. */
internal class PdfDocumentSaveBridge(private val activity: Activity) {
    private val saveRequestCode = 7821
    private val batchRequestCode = 7822
    private var pendingSave: MethodChannel.Result? = null
    private var pendingSourcePath: String? = null
    private var pendingBatch: List<Pair<String, File>>? = null

    fun register(engine: FlutterEngine) {
        MethodChannel(
            engine.dartExecutor.binaryMessenger,
            "cn.wps.pdf_rebuild_flutter/document_io",
        ).setMethodCallHandler { call, result ->
            if (pendingSave != null) {
                result.error("busy", "A document save is already in progress", null)
                return@setMethodCallHandler
            }
            when (call.method) {
                "savePdf" -> {
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
                    launch(intent, saveRequestCode, result)
                }
                "savePdfBatch" -> {
                    val descriptors = call.argument<List<*>>("files")
                    val files = descriptors?.mapNotNull { item ->
                        val fields = item as? Map<*, *> ?: return@mapNotNull null
                        val name = fields["name"] as? String ?: return@mapNotNull null
                        val path = fields["sourcePath"] as? String ?: return@mapNotNull null
                        if (name.isBlank() || name == "." || name == ".." ||
                            name.contains('/') || name.contains('\\') ||
                            !name.endsWith(".pdf", ignoreCase = true)
                        ) return@mapNotNull null
                        name to File(path)
                    }
                    if (files == null || files.size != descriptors?.size || files.isEmpty() ||
                        files.size > 1000 || files.any { !it.second.isFile }
                    ) {
                        result.error("source", "PDF batch source files are invalid", null)
                        return@setMethodCallHandler
                    }
                    pendingSave = result
                    pendingBatch = files
                    val intent = Intent(Intent.ACTION_OPEN_DOCUMENT_TREE).apply {
                        addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION or
                            Intent.FLAG_GRANT_WRITE_URI_PERMISSION)
                    }
                    launch(intent, batchRequestCode, result)
                }
                else -> result.notImplemented()
            }
        }
    }

    private fun launch(intent: Intent, requestCode: Int, result: MethodChannel.Result) {
        try {
            activity.startActivityForResult(intent, requestCode)
        } catch (error: Exception) {
            pendingSave = null
            pendingSourcePath = null
            pendingBatch = null
            result.error("picker", error.message, null)
        }
    }

    fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        if (requestCode != saveRequestCode && requestCode != batchRequestCode) return
        val result = pendingSave ?: return
        val sourcePath = pendingSourcePath
        val batch = pendingBatch
        pendingSave = null
        pendingSourcePath = null
        pendingBatch = null
        if (resultCode != Activity.RESULT_OK || data?.data == null) {
            result.success(if (requestCode == batchRequestCode) 0 else false)
            return
        }
        if (requestCode == batchRequestCode) {
            if (batch == null) {
                result.error("source", "PDF batch source files are missing", null)
                return
            }
            saveBatch(data.data!!, batch, result)
            return
        }
        if (sourcePath == null) {
            result.error("source", "PDF source file is missing", null)
            return
        }
        val target = data.data!!
        Thread {
            try {
                val stream = activity.contentResolver.openOutputStream(target, "w")
                    ?: throw IllegalStateException("Cannot open selected document")
                stream.use { output -> File(sourcePath).inputStream().use { it.copyTo(output) } }
                activity.runOnUiThread { result.success(true) }
            } catch (error: Exception) {
                activity.runOnUiThread { result.error("write", error.message, null) }
            }
        }.start()
    }

    private fun saveBatch(
        tree: Uri,
        files: List<Pair<String, File>>,
        result: MethodChannel.Result,
    ) {
        Thread {
            val created = mutableListOf<Uri>()
            try {
                val resolver = activity.contentResolver
                val parent = DocumentsContract.buildDocumentUriUsingTree(
                    tree,
                    DocumentsContract.getTreeDocumentId(tree),
                )
                for ((name, source) in files) {
                    val target = DocumentsContract.createDocument(
                        resolver, parent, "application/pdf", name,
                    ) ?: throw IllegalStateException("Cannot create $name")
                    created.add(target)
                    val stream = resolver.openOutputStream(target, "w")
                        ?: throw IllegalStateException("Cannot write $name")
                    stream.use { output -> source.inputStream().use { it.copyTo(output) } }
                }
                activity.runOnUiThread { result.success(created.size) }
            } catch (error: Exception) {
                for (uri in created.asReversed()) {
                    try {
                        DocumentsContract.deleteDocument(activity.contentResolver, uri)
                    } catch (_: Exception) { /* Best-effort rollback. */ }
                }
                activity.runOnUiThread { result.error("write", error.message, null) }
            }
        }.start()
    }
}
