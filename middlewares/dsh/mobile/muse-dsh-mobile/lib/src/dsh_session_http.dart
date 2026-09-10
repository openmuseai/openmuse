import 'dart:convert';
import 'dart:io';

/// Default POST used by [DshSessionApi] on Android. Inject a fake in tests.
Future<Map<String, dynamic>> dshSessionHttpPost(
  Uri uri,
  Map<String, String> headers,
  String body,
) async {
  final client = HttpClient();
  try {
    final request = await client.postUrl(uri);
    headers.forEach(request.headers.set);
    request.add(utf8.encode(body));
    final response = await request.close();
    final text = await utf8.decoder.bind(response).join();
    if (text.isEmpty) return <String, dynamic>{};
    final decoded = jsonDecode(text);
    if (decoded is Map<String, dynamic>) return decoded;
    return <String, dynamic>{'data': decoded};
  } finally {
    client.close(force: true);
  }
}
