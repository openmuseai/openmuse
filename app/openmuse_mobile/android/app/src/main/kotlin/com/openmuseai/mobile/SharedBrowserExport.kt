package com.openmuseai.mobile

import android.content.ContentValues
import android.content.Context
import android.os.Build
import android.os.Environment
import android.provider.MediaStore

/** Standard MCP download exports go to visible Downloads without library access.
 * Each export owns a fresh pending row; a failed write cannot damage other files. */
internal object SharedBrowserExport {
    fun save(context: Context, name: String, mime: String, bytes: ByteArray): Boolean {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.Q) return false
        require(name.isNotBlank() && name.length <= 200 && name.none { it == '/' || it == '\\' || it.code < 32 })
        val resolver = context.contentResolver
        val values = ContentValues().apply {
            put(MediaStore.Downloads.DISPLAY_NAME, name)
            put(MediaStore.Downloads.MIME_TYPE, mime)
            put(MediaStore.Downloads.RELATIVE_PATH, Environment.DIRECTORY_DOWNLOADS + "/OpenMuse")
            put(MediaStore.Downloads.IS_PENDING, 1)
        }
        val uri = resolver.insert(MediaStore.Downloads.EXTERNAL_CONTENT_URI, values)
            ?: error("Unable to allocate export")
        try {
            resolver.openOutputStream(uri, "w")?.use { it.write(bytes) }
                ?: error("Unable to write export")
            require(resolver.update(uri, ContentValues().apply { put(MediaStore.Downloads.IS_PENDING, 0) }, null, null) == 1)
        } catch (error: Exception) {
            resolver.delete(uri, null, null)
            throw error
        }
        return true
    }
}
