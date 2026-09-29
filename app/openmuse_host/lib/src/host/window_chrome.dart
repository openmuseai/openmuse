import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

/// Windows caption overlay: drag, maximize, and the native min/max/close
/// buttons live in the Flutter title strip so they share one row with OpenMuse.
final class OpenMuseWindowChrome {
  OpenMuseWindowChrome._();

  static const channel = MethodChannel('com.openmuse.host/window');
  static final maximized = ValueNotifier(false);
  static bool _bound = false;

  static void ensureBound() {
    if (_bound || kIsWeb || !Platform.isWindows) return;
    _bound = true;
    channel.setMethodCallHandler((call) async {
      if (call.method == 'state') {
        final arguments = call.arguments;
        maximized.value =
            arguments is Map && arguments['maximized'] == true;
      }
    });
    channel.invokeMapMethod<String, dynamic>('query').then((state) {
      if (state != null) maximized.value = state['maximized'] == true;
    }, onError: (_) {});
  }

  static Future<void> startDrag() async {
    if (!Platform.isWindows) return;
    try {
      await channel.invokeMethod<void>('startDrag');
    } on MissingPluginException {
      // Widget tests have no runner channel.
    }
  }

  static Future<void> minimize() async {
    if (!Platform.isWindows) return;
    try {
      await channel.invokeMethod<void>('minimize');
    } on MissingPluginException {
      // Widget tests have no runner channel.
    }
  }

  static Future<void> toggleMaximized() async {
    if (!Platform.isWindows) return;
    try {
      await channel.invokeMethod<void>('toggleMaximized');
    } on MissingPluginException {
      // Widget tests have no runner channel.
    }
  }

  static Future<void> close() async {
    if (!Platform.isWindows) return;
    try {
      await channel.invokeMethod<void>('close');
    } on MissingPluginException {
      // Widget tests have no runner channel.
    }
  }
}
