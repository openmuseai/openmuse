package com.openmuseai.mobile
import org.junit.Assert.*
import org.junit.Test
class MicrophoneOriginPolicyTest {
    private val host = "vverified.plugins.openmuse.io"
    private val audio = listOf("android.webkit.resource.AUDIO_CAPTURE")
    @Test fun onlyDeclaredAudioFromTheExactDedicatedOriginIsPermitted() {
        assertTrue(MicrophoneOriginPolicy.permits(true, "https://$host/", host, audio))
        assertFalse(MicrophoneOriginPolicy.permits(false, "https://$host/", host, audio))
        for (uri in listOf("null", "http://$host/", "https://$host.evil.test/", "https://user@$host/", "https://$host:444/", "https://$host/?other=1", "https://host.plugins.openmuse.io/"))
            assertFalse(uri, MicrophoneOriginPolicy.permits(true, uri, host, audio))
        assertFalse(MicrophoneOriginPolicy.permits(true, "https://$host/", host, audio + "android.webkit.resource.VIDEO_CAPTURE"))
        assertFalse(MicrophoneOriginPolicy.permits(true, "https://$host/", host, emptyList()))
    }
}
