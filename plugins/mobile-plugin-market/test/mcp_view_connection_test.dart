import 'dart:async';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';

void main() {
  test(
    'WebView transport preserves requests, readiness and revocation',
    () async {
      final sent = <Map<String, Object?>>[];
      final connection = RemoteMcpViewConnection.webView(
        (message) async => sent.add(message),
      );
      await connection.handleMessage({
        'jsonrpc': '2.0',
        'method': 'ui/notifications/initialized',
      });
      expect(connection.ready.isCompleted, isTrue);
      final pending = connection.request('tools/list', {});
      await Future<void>.delayed(Duration.zero);
      final request = sent.single;
      await connection.handleMessage({
        'jsonrpc': '2.0',
        'id': request['id'],
        'result': {'tools': []},
      });
      expect(await pending, {'tools': []});
      connection.onRequest = (method, params) async => {'accepted': method};
      await connection.handleMessage({
        'jsonrpc': '2.0',
        'id': 'view-1',
        'method': 'tools/call',
        'params': {},
      });
      expect(sent.last['result'], {'accepted': 'tools/call'});
      final inFlight = connection.request('tools/list', {});
      final closed = expectLater(inFlight, throwsStateError);
      await Future<void>.delayed(Duration.zero);
      connection.dispose();
      await closed;
      await expectLater(connection.request('tools/list', {}), throwsStateError);
      final count = sent.length;
      await connection.receive({'jsonrpc': '2.0', 'method': 'ignored'});
      expect(sent.length, count);
    },
  );
  test(
    'resource save acknowledges only after commit and propagates conflicts',
    () async {
      final sent = <Map<String, Object?>>[];
      final connection = RemoteMcpViewConnection.webView(
        (message) async => sent.add(message),
      );
      final commit = Completer<void>();
      connection.onDownload = (_) => commit.future;
      final saving = connection.handleMessage({
        'jsonrpc': '2.0',
        'id': 'save-1',
        'method': 'ui/download-file',
        'params': <String, Object?>{},
      });
      await Future<void>.delayed(Duration.zero);
      expect(
        sent,
        isEmpty,
        reason:
            'The editor must not report saved before the resource commit completes',
      );
      commit.complete();
      await saving;
      expect(sent.single['result'], isEmpty);
      connection.onDownload = (_) async =>
          throw StateError('RESOURCE_CONFLICT');
      await connection.handleMessage({
        'jsonrpc': '2.0',
        'id': 'save-2',
        'method': 'ui/download-file',
        'params': <String, Object?>{},
      });
      expect((sent.last['error'] as Map)['message'], 'RESOURCE_CONFLICT');
      expect(sent.last.containsKey('result'), isFalse);
      connection.dispose();
    },
  );
}
