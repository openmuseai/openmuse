package io.openmuse.task_notifications

import android.Manifest
import android.app.Activity
import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Build
import android.provider.Settings
import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.embedding.engine.plugins.activity.ActivityAware
import io.flutter.embedding.engine.plugins.activity.ActivityPluginBinding
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.common.PluginRegistry

class TaskNotificationsPlugin : FlutterPlugin, MethodChannel.MethodCallHandler,
    ActivityAware, PluginRegistry.RequestPermissionsResultListener,
    PluginRegistry.NewIntentListener {
    private lateinit var context: Context
    private lateinit var channel: MethodChannel
    private var binding: ActivityPluginBinding? = null
    private var pending: MethodChannel.Result? = null
    private var activated = false
    private val channelId = "openmuse_task_outcomes_v1"
    private val tag = "openmuse.task"
    private val activationKey = "openmuse.task.activate"
    private val requestCode = 10301
    private val manager get() = context.getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
    private val preferences get() = context.getSharedPreferences("openmuse_task_notifications", Context.MODE_PRIVATE)

    override fun onAttachedToEngine(binding: FlutterPlugin.FlutterPluginBinding) {
        context = binding.applicationContext
        channel = MethodChannel(binding.binaryMessenger, "io.openmuse/task_notifications")
        channel.setMethodCallHandler(this)
    }
    override fun onDetachedFromEngine(binding: FlutterPlugin.FlutterPluginBinding) {
        channel.setMethodCallHandler(null)
        pending?.error("unavailable", "Notification activity is unavailable", null)
        pending = null
    }
    private fun status(): String {
        if (Build.VERSION.SDK_INT >= 33 && context.checkSelfPermission(Manifest.permission.POST_NOTIFICATIONS) != PackageManager.PERMISSION_GRANTED) {
            return if (preferences.getBoolean("requested", false)) "denied" else "notDetermined"
        }
        if (!manager.areNotificationsEnabled()) return "denied"
        if (Build.VERSION.SDK_INT >= 26 && manager.getNotificationChannel(channelId)?.importance == NotificationManager.IMPORTANCE_NONE) return "denied"
        return "granted"
    }
    override fun onMethodCall(call: MethodCall, result: MethodChannel.Result) {
        try {
            when (call.method) {
                "authorization" -> result.success(status())
                "request" -> {
                    if (status() != "notDetermined" || Build.VERSION.SDK_INT < 33) {
                        result.success(status()); return
                    }
                    val activity = binding?.activity
                    if (activity == null || pending != null) {
                        result.error("unavailable", "Notification permission is unavailable", null); return
                    }
                    pending = result
                    preferences.edit().putBoolean("requested", true).apply()
                    activity.requestPermissions(arrayOf(Manifest.permission.POST_NOTIFICATIONS), requestCode)
                }
                "settings" -> {
                    val intent = if (Build.VERSION.SDK_INT >= 26) Intent(Settings.ACTION_APP_NOTIFICATION_SETTINGS).putExtra(Settings.EXTRA_APP_PACKAGE, context.packageName)
                        else Intent(Settings.ACTION_APPLICATION_DETAILS_SETTINGS, android.net.Uri.parse("package:" + context.packageName))
                    intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
                    context.startActivity(intent)
                    result.success(null)
                }
                "show" -> {
                    if (status() != "granted") { result.success(null); return }
                    val id = call.argument<Int>("id")?.takeIf { it > 0 } ?: throw IllegalArgumentException()
                    fun text(key: String): String = call.argument<String>(key)?.takeIf { it.isNotBlank() && it.length <= 256 } ?: throw IllegalArgumentException()
                    val title = text("title"); val body = text("body"); val name = text("channelName")
                    if (Build.VERSION.SDK_INT >= 26) manager.createNotificationChannel(NotificationChannel(channelId, name, NotificationManager.IMPORTANCE_DEFAULT))
                    val intent = context.packageManager.getLaunchIntentForPackage(context.packageName) ?: throw IllegalStateException()
                    intent.addFlags(Intent.FLAG_ACTIVITY_SINGLE_TOP or Intent.FLAG_ACTIVITY_CLEAR_TOP)
                    intent.putExtra(activationKey, true)
                    val tap = PendingIntent.getActivity(context, id, intent, PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE)
                    val builder = if (Build.VERSION.SDK_INT >= 26) Notification.Builder(context, channelId) else Notification.Builder(context)
                    val notification = builder.setSmallIcon(android.R.drawable.ic_dialog_info)
                        .setContentTitle(title).setContentText(body).setContentIntent(tap)
                        .setVisibility(Notification.VISIBILITY_PRIVATE).setAutoCancel(true).build()
                    manager.notify(tag, id, notification)
                    result.success(null)
                }
                "clear" -> {
                    for (notification in manager.activeNotifications) if (notification.tag == tag) manager.cancel(tag, notification.id)
                    result.success(null)
                }
                "takeActivation" -> { result.success(activated); activated = false }
                else -> result.notImplemented()
            }
        } catch (_: Exception) {
            result.error("notification_operation", "The local notification operation failed", null)
        }
    }
    override fun onRequestPermissionsResult(code: Int, permissions: Array<out String>, results: IntArray): Boolean {
        if (code != requestCode) return false
        pending?.success(status()); pending = null
        return true
    }
    override fun onNewIntent(intent: Intent): Boolean {
        if (!intent.getBooleanExtra(activationKey, false)) return false
        intent.removeExtra(activationKey)
        activated = true
        channel.invokeMethod("activated", null)
        return false // Other plugins may also need the launch intent.
    }
    override fun onAttachedToActivity(binding: ActivityPluginBinding) {
        this.binding = binding
        binding.addRequestPermissionsResultListener(this)
        binding.addOnNewIntentListener(this)
        onNewIntent(binding.activity.intent)
    }
    private fun detach() {
        binding?.removeRequestPermissionsResultListener(this)
        binding?.removeOnNewIntentListener(this)
        binding = null
        pending?.error("unavailable", "Notification activity was detached", null)
        pending = null
    }
    override fun onDetachedFromActivity() = detach()
    override fun onDetachedFromActivityForConfigChanges() = detach()
    override fun onReattachedToActivityForConfigChanges(binding: ActivityPluginBinding) = onAttachedToActivity(binding)
}
