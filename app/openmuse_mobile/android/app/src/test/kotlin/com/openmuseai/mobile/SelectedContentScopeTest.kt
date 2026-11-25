package com.openmuseai.mobile

import org.junit.Assert.*
import org.junit.Test

class SelectedContentScopeTest {
    @Test fun grantsAreExactAndIsolatedBetweenPluginViews() {
        val scope = SelectedContentScope<Any>()
        val first = Any(); val other = Any()
        val uri = "content://picker/image/42"
        assertFalse(scope.permits(first, uri))
        scope.replace(first, listOf(uri))
        assertTrue(scope.permits(first, uri))
        assertFalse(scope.permits(other, uri))
        assertFalse(scope.permits(first, "$uri/child"))
        assertFalse(scope.permits(first, "$uri?other=1"))
        assertFalse(scope.permits(first, "content://picker/image/43"))
    }

    @Test fun replacementCancellationAndDisposalRevokeOldSelections() {
        val scope = SelectedContentScope<Any>()
        val first = Any(); val other = Any()
        val previous = mutableListOf("content://picker/image/42")
        scope.replace(first, previous)
        previous.add("content://picker/image/unselected")
        assertFalse(scope.permits(first, previous[1]))
        scope.replace(first, listOf("content://picker/image/43"))
        assertFalse(scope.permits(first, previous[0]))
        scope.replace(other, listOf(previous[0]))
        scope.revoke(first)
        assertFalse(scope.permits(first, "content://picker/image/43"))
        assertTrue(scope.permits(other, previous[0]))
        scope.clear()
        assertFalse(scope.permits(other, previous[0]))
    }
}
