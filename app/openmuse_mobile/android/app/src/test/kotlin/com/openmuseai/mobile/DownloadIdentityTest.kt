package com.openmuseai.mobile

import org.junit.Assert.*
import org.junit.Test

class DownloadIdentityTest {
    private val pptx = "application/vnd.openxmlformats-officedocument.presentationml.presentation"
    @Test fun savingTheGrantedResourceRequiresBothNameAndMediaType() {
        assertTrue(DownloadIdentity.mayReplace("slides.pptx", pptx, "slides.pptx", pptx, "/granted/slides.pptx"))
        assertFalse(DownloadIdentity.mayReplace("slides.pdf", "application/pdf", "slides.pptx", pptx, "/granted/slides.pptx"))
        assertFalse(DownloadIdentity.mayReplace("slides.pptx", "image/png", "slides.pptx", pptx, "/granted/slides.pptx"))
        assertFalse(DownloadIdentity.mayReplace("copy.pptx", pptx, "slides.pptx", pptx, "/granted/slides.pptx"))
        assertFalse(DownloadIdentity.mayReplace("slides.pptx", pptx, "slides.pptx", pptx, null))
        assertFalse(DownloadIdentity.mayReplace("slides.pptx", pptx, null, null, "/granted/slides.pptx"))
    }
}
