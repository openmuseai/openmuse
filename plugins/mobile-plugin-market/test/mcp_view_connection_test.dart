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
}
