package io.openmuse.openmuse_mobile

import android.content.Context
import android.security.keystore.KeyGenParameterSpec
import android.security.keystore.KeyInfo
import android.security.keystore.KeyProperties
import android.util.Base64
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodChannel
import java.security.KeyStore
import java.security.MessageDigest
import java.security.SecureRandom
import javax.crypto.Cipher
import javax.crypto.KeyGenerator
import javax.crypto.SecretKey
import javax.crypto.SecretKeyFactory

class MainActivity : FlutterActivity() {
    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)
        DeviceKeyStoreBridge(applicationContext).register(flutterEngine)
    }
}

private class DeviceKeyStoreBridge(context: Context) {
    private val preferences = context.getSharedPreferences(
        "openmuse_device_keys_v1",
        Context.MODE_PRIVATE,
    )
    private val keyStore = KeyStore.getInstance("AndroidKeyStore").apply { load(null) }

    fun register(engine: FlutterEngine) {
        MethodChannel(
            engine.dartExecutor.binaryMessenger,
            "io.openmuse/device_keystore",
        ).setMethodCallHandler { call, result ->
            try {
                when (call.method) {
                    "ensure" -> result.success(
                        ensure(
                            requireRef(call.argument<String>("accountRef")),
                            requireRef(call.argument<String>("deviceRef")),
                        ),
                    )
                    "delete" -> {
                        delete(requireRef(call.argument<String>("keyRef")))
                        result.success(null)
                    }
                    else -> result.notImplemented()
                }
            } catch (error: Exception) {
                result.error("device_keystore", "Device key operation failed", null)
            }
        }
    }

    private fun ensure(accountRef: String, deviceRef: String): Map<String, Any> {
        val digest = MessageDigest.getInstance("SHA-256")
            .digest("$accountRef\u0000$deviceRef".toByteArray(Charsets.UTF_8))
            .joinToString("") { "%02x".format(it) }
        val keyRef = "device-key:$digest"
        val alias = "openmuse.device.wrap.$digest"
        val existing = preferences.getString(keyRef, null)
        if (existing == null) {
            if (keyStore.containsAlias(alias)) keyStore.deleteEntry(alias)
            val generator = KeyGenerator.getInstance(
                KeyProperties.KEY_ALGORITHM_AES,
                "AndroidKeyStore",
            )
            generator.init(
                KeyGenParameterSpec.Builder(
                    alias,
                    KeyProperties.PURPOSE_ENCRYPT or KeyProperties.PURPOSE_DECRYPT,
                )
                    .setBlockModes(KeyProperties.BLOCK_MODE_GCM)
                    .setEncryptionPaddings(KeyProperties.ENCRYPTION_PADDING_NONE)
                    .setKeySize(256)
                    .setRandomizedEncryptionRequired(true)
                    .build(),
            )
            val wrappingKey = generator.generateKey()
            val seed = ByteArray(32).also(SecureRandom()::nextBytes)
            try {
                val cipher = Cipher.getInstance("AES/GCM/NoPadding")
                cipher.init(Cipher.ENCRYPT_MODE, wrappingKey)
                val encrypted = cipher.doFinal(seed)
                val blob = ByteArray(cipher.iv.size + encrypted.size)
                cipher.iv.copyInto(blob)
                encrypted.copyInto(blob, cipher.iv.size)
                preferences.edit()
                    .putString(keyRef, Base64.encodeToString(blob, Base64.NO_WRAP))
                    .apply()
                blob.fill(0)
                encrypted.fill(0)
            } finally {
                seed.fill(0)
            }
        }
        val key = keyStore.getKey(alias, null) as? SecretKey
            ?: error("device wrapping key is missing")
        val hardwareBacked = try {
            val factory = SecretKeyFactory.getInstance(key.algorithm, "AndroidKeyStore")
            val keyInfo = factory.getKeySpec(key, KeyInfo::class.java) as KeyInfo
            keyInfo.isInsideSecureHardware
        } catch (_: Exception) {
            false
        }
        return mapOf(
            "keyRef" to keyRef,
            "storage" to "android-keystore-wrapped",
            "hardwareBacked" to hardwareBacked,
            "created" to (existing == null),
        )
    }

    private fun delete(keyRef: String) {
        val digest = keyRef.removePrefix("device-key:")
        require(digest.length == 64 && digest.all { it in "0123456789abcdef" })
        keyStore.deleteEntry("openmuse.device.wrap.$digest")
        preferences.edit().remove(keyRef).apply()
    }

    private fun requireRef(value: String?): String {
        require(value != null && value.isNotEmpty() && value.length <= 256 && !value.contains('\u0000'))
        return value
    }
}
