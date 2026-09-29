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
import javax.crypto.spec.GCMParameterSpec

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
                    "publicIdentity" -> result.success(
                        publicIdentity(requireRef(call.argument<String>("keyRef"))),
                    )
                    "issueOffer" -> result.success(
                        issueOffer(
                            requireRef(call.argument<String>("keyRef")),
                            requireRef(call.argument<String>("accountRef")),
                            requireRef(call.argument<String>("deviceRef")),
                            call.argument<Number>("registrationGeneration")?.toLong()
                                ?: error("registration generation is missing"),
                        ),
                    )
                    else -> result.notImplemented()
                }
            } catch (error: Exception) {
                result.error("device_keystore", "Device key operation failed", null)
            }
        }
    }

    private fun publicIdentity(keyRef: String): Map<String, ByteArray> {
        return withDeviceSeed(keyRef) { seed ->
            val public = PairedCryptoNative.devicePublic(seed)
                ?: error("public identity derivation failed")
            try {
                require(public.size == 64)
                mapOf(
                    "signingPublic" to public.copyOfRange(0, 32),
                    "agreementPublic" to public.copyOfRange(32, 64),
                )
            } finally {
                public.fill(0)
            }
        }
    }

    private fun issueOffer(
        keyRef: String,
        accountRef: String,
        deviceRef: String,
        registrationGeneration: Long,
    ): String {
        require(registrationGeneration > 0)
        val nonce = ByteArray(32).also(SecureRandom()::nextBytes)
        return try {
            withDeviceSeed(keyRef) { seed ->
                val json = PairedCryptoNative.issueOffer(
                    seed,
                    accountRef,
                    deviceRef,
                    nonce,
                    registrationGeneration,
                ) ?: error("pairing offer operation failed")
                try {
                    json.toString(Charsets.UTF_8)
                } finally {
                    json.fill(0)
                }
            }
        } finally {
            nonce.fill(0)
        }
    }

    private fun <T> withDeviceSeed(keyRef: String, operation: (ByteArray) -> T): T {
        val digest = validatedDigest(keyRef)
        val alias = "openmuse.device.wrap.$digest"
        val encoded = preferences.getString(keyRef, null)
            ?: error("device seed is missing")
        val blob = Base64.decode(encoded, Base64.NO_WRAP)
        require(blob.size > GCM_IV_BYTES + GCM_TAG_BYTES)
        val encrypted = blob.copyOfRange(GCM_IV_BYTES, blob.size)
        val key = keyStore.getKey(alias, null) as? SecretKey
            ?: error("device wrapping key is missing")
        val cipher = Cipher.getInstance("AES/GCM/NoPadding")
        cipher.init(
            Cipher.DECRYPT_MODE,
            key,
            GCMParameterSpec(GCM_TAG_BYTES * 8, blob.copyOfRange(0, GCM_IV_BYTES)),
        )
        val seed = cipher.doFinal(encrypted)
        require(seed.size == 32)
        try {
            return operation(seed)
        } finally {
            seed.fill(0)
            encrypted.fill(0)
            blob.fill(0)
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
        val digest = validatedDigest(keyRef)
        keyStore.deleteEntry("openmuse.device.wrap.$digest")
        preferences.edit().remove(keyRef).apply()
    }

    private fun validatedDigest(keyRef: String): String {
        require(keyRef.startsWith("device-key:"))
        val digest = keyRef.removePrefix("device-key:")
        require(digest.length == 64 && digest.all { it in "0123456789abcdef" })
        return digest
    }

    private fun requireRef(value: String?): String {
        require(value != null && value.isNotEmpty() && value.length <= 256 && !value.contains('\u0000'))
        return value
    }

    private companion object {
        const val GCM_IV_BYTES = 12
        const val GCM_TAG_BYTES = 16
    }
}

private object PairedCryptoNative {
    init {
        System.loadLibrary("openmuse_paired_relay")
    }

    external fun devicePublic(seed: ByteArray): ByteArray?

    external fun issueOffer(
        seed: ByteArray,
        accountRef: String,
        deviceRef: String,
        nonce: ByteArray,
        registrationGeneration: Long,
    ): ByteArray?
}
