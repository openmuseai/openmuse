import 'dart:typed_data';
import 'package:http/http.dart' as http;
import 'codec.dart';
import 'core_api.dart';

/// Caller supplies the connection's already validated Cloud origin. Entire
/// profile bytes remain transient and are discarded after extracting only UID.
Future<String> readAppFlowyActorUid({
  required Uri origin,
  required String token,
  required AppFlowyRealtimeCodec codec,
  http.Client? client,
}) async {
  final owned = client == null, transport = client ?? http.Client();
  Future<String> load() async {
    final request = http.Request('GET', origin.resolve('/api/user/profile'))
      ..followRedirects = false
      ..headers['Authorization'] = 'Bearer $token';
    final response = await transport.send(request);
    if (response.statusCode != 200 || (response.contentLength ?? 0) > 65536)
      throw const RealtimeFailure('ACTOR_IDENTITY_UNAVAILABLE');
    final bytes = BytesBuilder(copy: false);
    var size = 0;
    await for (final chunk in response.stream) {
      size += chunk.length;
      if (size > 65536) throw const RealtimeFailure('PROFILE_QUOTA_EXCEEDED');
      bytes.add(chunk);
    }
    return codec.profileActorUid(bytes.takeBytes());
  }

  try {
    return await load().timeout(const Duration(seconds: 5));
  } on Object {
    throw const RealtimeFailure('ACTOR_IDENTITY_UNAVAILABLE');
  } finally {
    if (owned) transport.close();
  }
}
