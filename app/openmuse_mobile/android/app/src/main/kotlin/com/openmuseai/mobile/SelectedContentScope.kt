package com.openmuseai.mobile

import java.util.Collections
import java.util.WeakHashMap

/** Exact picker results belong to one live plugin View, never a URI prefix. */
internal class SelectedContentScope<K : Any> {
    private val selections = Collections.synchronizedMap(WeakHashMap<K, Set<String>>())

    fun replace(owner: K, uris: List<String>) { selections[owner] = uris.toSet() }
    fun permits(owner: K, uri: String): Boolean = selections[owner]?.contains(uri) == true
    fun revoke(owner: K) { selections.remove(owner) }
    fun clear() { selections.clear() }
}
