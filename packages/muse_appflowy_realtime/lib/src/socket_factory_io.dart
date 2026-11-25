import 'dart:async';
import 'dart:io';
import 'package:web_socket_channel/web_socket_channel.dart';
import 'package:web_socket_channel/io.dart';

/// Own the HTTP handshake client per session. dart:io's shared WebSocket client
/// can predate the installed service's network configuration and DNS recovery.
WebSocketChannel createAppFlowySocket(Uri uri) {
  final client = HttpClient();
  final channel = IOWebSocketChannel.connect(
    uri,
    customClient: client,
    connectTimeout: const Duration(seconds: 20),
  );
  // Keep handshake transport lifetime tied to its WebSocket. Closing it when
  // ready races consumers registering subscriptions on some native runtimes.
  unawaited(
    channel.sink.done.then<void>(
      (_) => client.close(force: true),
      onError: (Object _) => client.close(force: true),
    ),
  );
  unawaited(
    channel.ready.then<void>(
      (_) {},
      onError: (Object _) => client.close(force: true),
    ),
  );
  return channel;
}
