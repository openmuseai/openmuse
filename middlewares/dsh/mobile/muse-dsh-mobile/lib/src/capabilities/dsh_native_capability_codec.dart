import 'dart:convert';
import 'dart:typed_data';

/// Frozen `muse.native-capability/v1` codec. Rejects oversized, unknown, or
/// forbidden fields. Does not interpret file bytes or credentials.
class DshNativeCapabilityCodec {
  static const protocol = 'muse.native-capability/v1';

  /// Flutter [JavaScriptChannel] names must be valid JS identifiers. The
  /// frozen contract id stays `muse.native-capability`.
  static const javascriptChannelName = 'MuseNativeCapability';
  static const eventName = 'muse-native-capability';
  static const maxBytes = 8192;

  static const webToFlutter = {
    'capabilities.get',
    'speech.start',
    'speech.cancel',
    'share.open',
    'back.result',
  };

  static const flutterToWeb = {
    'capabilities',
    'speech.partial',
    'speech.final',
    'speech.error',
    'back.request',
    'lifecycle.changed',
  };

  static const forbiddenKeys = {
    'token',
    'path',
    'contentUri',
    'base64',
    'arbitraryMethod',
    'fetchUrl',
    'intent',
  };

  static DshNativeInbound? decode(String raw) {
    if (utf8.encode(raw).length > maxBytes) return null;
    final decoded = jsonDecode(raw);
    if (decoded is! Map<String, dynamic>) return null;
    if (_containsForbidden(decoded)) return null;
    if (decoded['protocol'] != protocol) return null;
    final type = decoded['type'];
    if (type is! String || !webToFlutter.contains(type)) return null;
    final requestId = decoded['requestId'];
    if (requestId is! String || requestId.isEmpty || requestId.length > 128) {
      return null;
    }
    final generation = decoded['generation'];
    if (type != 'back.result' && type != 'share.open') {
      if (generation is! int || generation < 0) return null;
    }
    if (type == 'back.request') return null;
    return DshNativeInbound(
      type: type,
      requestId: requestId,
      generation: generation is int ? generation : null,
      sessionId: decoded['sessionId'] is String
          ? decoded['sessionId'] as String
          : null,
      locale: _localeOf(decoded),
      textOrHttpsUrl: _shareTargetOf(decoded),
      consumed: decoded['consumed'] is bool
          ? decoded['consumed'] as bool
          : null,
    );
  }

  static String encodeOutbound(Map<String, Object?> message) {
    final payload = <String, Object?>{'protocol': protocol, ...message};
    final raw = jsonEncode(payload);
    if (utf8.encode(raw).length > maxBytes) {
      throw const FormatException('NATIVE_CAPABILITY_TOO_LARGE');
    }
    return raw;
  }

  /// Safe `runJavaScript` snippet: user text is JSON-encoded, never concatenated.
  static String dispatchScript(Map<String, Object?> message) {
    final encoded = encodeOutbound(message);
    return 'window.dispatchEvent(new CustomEvent("$eventName",{detail:JSON.parse(${jsonEncode(encoded)})}));';
  }

  static bool _containsForbidden(Object? value) {
    if (value is Map) {
      for (final entry in value.entries) {
        if (entry.key is String &&
            forbiddenKeys.contains(entry.key as String)) {
          return true;
        }
        if (_containsForbidden(entry.value)) return true;
      }
    } else if (value is List) {
      for (final item in value) {
        if (_containsForbidden(item)) return true;
      }
    } else if (value is TypedData) {
      return true;
    }
    return false;
  }

  static String? _localeOf(Map<String, dynamic> decoded) {
    final payload = decoded['payload'];
    if (payload is Map && payload['locale'] is String) {
      return payload['locale'] as String;
    }
    if (decoded['locale'] is String) return decoded['locale'] as String;
    return null;
  }

  static String? _shareTargetOf(Map<String, dynamic> decoded) {
    final payload = decoded['payload'];
    if (payload is Map && payload['textOrHttpsUrl'] is String) {
      return payload['textOrHttpsUrl'] as String;
    }
    if (decoded['textOrHttpsUrl'] is String) {
      return decoded['textOrHttpsUrl'] as String;
    }
    return null;
  }
}

class DshNativeInbound {
  const DshNativeInbound({
    required this.type,
    required this.requestId,
    this.generation,
    this.sessionId,
    this.locale,
    this.textOrHttpsUrl,
    this.consumed,
  });

  final String type;
  final String requestId;
  final int? generation;
  final String? sessionId;
  final String? locale;
  final String? textOrHttpsUrl;
  final bool? consumed;
}
