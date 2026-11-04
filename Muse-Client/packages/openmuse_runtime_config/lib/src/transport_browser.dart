import 'dart:async';
import 'dart:js_interop';
import 'package:web/web.dart' as web;
import 'client.dart';

ConfigTransport createConfigTransport() => BrowserConfigTransport();

/// BrowserClient/XHR can still attach same-origin cookies. Fetch explicitly
/// omits credentials and rejects redirects for the anonymous config channel.
final class BrowserConfigTransport implements ConfigTransport {
  @override
  Future<List<int>> get(Uri uri, {required int maxBytes}) async {
    final abort = web.AbortController();
    final timer = Timer(const Duration(seconds: 2), () => abort.abort());
    try {
      final response = await web.window
          .fetch(
            uri.toString().toJS,
            web.RequestInit(
              credentials: 'omit',
              redirect: 'error',
              signal: abort.signal,
            ),
          )
          .toDart;
      final length = int.tryParse(response.headers.get('content-length') ?? '');
      if (response.status != 200 ||
          (length != null && length > maxBytes) ||
          response.body == null) {
        throw const FormatException('Config response rejected');
      }
      final reader = web.ReadableStreamDefaultReader(response.body!);
      final bytes = <int>[];
      try {
        while (true) {
          final item = await reader.read().toDart;
          if (item.done) break;
          final chunk = (item.value as JSUint8Array).toDart;
          if (bytes.length + chunk.length > maxBytes) {
            abort.abort();
            throw const FormatException('Config response exceeds size limit');
          }
          bytes.addAll(chunk);
        }
      } finally {
        reader.releaseLock();
      }
      return bytes;
    } finally {
      timer.cancel();
    }
  }
}
