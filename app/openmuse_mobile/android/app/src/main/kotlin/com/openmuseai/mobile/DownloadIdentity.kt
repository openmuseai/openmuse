package com.openmuseai.mobile

/** A download may update only the exact resource granted to this View. Exported
 * images/PDFs/copies are different resources even when a plugin reuses a name. */
internal object DownloadIdentity {
    fun matches(name: String, mime: String, selectedName: String?, selectedMime: String?): Boolean =
        selectedName != null && selectedMime != null && name == selectedName && mime.equals(selectedMime, ignoreCase = true)
    fun mayReplace(name: String, mime: String, selectedName: String?, selectedMime: String?, grantedPath: String?): Boolean =
        grantedPath != null && matches(name, mime, selectedName, selectedMime)
}
