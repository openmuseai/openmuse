import 'dart:convert';
import 'package:flutter/services.dart';
import 'package:muse_resource_center/muse_resource_center.dart';

abstract final class ResourceClipboard {
  static const _channel = MethodChannel('io.openmuse/resource_clipboard');
  static Future<void> write(ResourceReference reference) async {
    final text = reference.toUri().toString();
    try {
      await _channel.invokeMethod<void>('write', {
        'text': text,
        'reference': jsonEncode(reference.toJson()),
      });
    } on MissingPluginException {
      await Clipboard.setData(ClipboardData(text: text));
    }
  }

  /// Explicit user action only. No clipboard observer or launch-time reads.
  static Future<ResourceReference> read() async {
    try {
      final typed = await _channel.invokeMethod<String>('read');
      if (typed != null) {
        if (typed.length > 8192)
          throw const ResourceFailure('INVALID_REFERENCE');
        return ResourceReference.fromJson(jsonDecode(typed));
      }
    } on MissingPluginException {
      /* Web/Linux fall back to the same canonical URI. */
    }
    final text = (await Clipboard.getData(Clipboard.kTextPlain))?.text;
    if (text == null) throw const ResourceFailure('INVALID_REFERENCE');
    return ResourceReference.fromUri(text.trim());
  }
}
