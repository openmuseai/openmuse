import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/mcp_resource_extensions.dart';
import 'package:openmuse_host/src/host/resource_agent_broker.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

class Contributor implements OpenMuseToolContributor {
  int calls = 0, disposed = 0;
  @override
  List<Map<String, Object?>> get contributedTools => [
    {
      'name': 'example_operation',
      'inputSchema': {'type': 'object'},
    },
  ];
  @override
  Future<Map<String, Object?>> callContributedTool(
    String caller,
    String name,
    Map<String, Object?> args,
  ) async {
    calls++;
    return {'content': []};
  }

  @override
  Future<void> disposeToolCaller(String caller) async {
    disposed++;
  }
}

void main() {
  test(
    'extension grants stay within caller/resource/package lifetime',
    () async {
      final broker = ResourceAgentBroker()..activeUri = 'resource://one';
      final plugin = Contributor();
      var approvals = 0;
      var current = true;
      final extension = ResourceExtensionSession(
        caller: 'consumer',
        resourceUri: 'resource://one',
        broker: broker,
        contributors: () => [(id: 'provider', plugin: plugin)],
        approve: (_, _) async {
          approvals++;
          return true;
        },
        isCurrent: () async => current,
      );
      Map<String, Object?> request(String name) => {
        'name': 'call_plugin_tool',
        'arguments': {'plugin': 'provider', 'name': name, 'arguments': {}},
      };
      await extension.dispatch('tools/call', request('example_operation'));
      await extension.dispatch('tools/call', request('example_operation'));
      expect(approvals, 1);
      expect(plugin.calls, 2);
      await expectLater(
        extension.dispatch('tools/call', request('invented_operation')),
        throwsStateError,
      );
      await expectLater(
        extension.dispatch('tools/call', {
          'name': 'call_resource_tool',
          'arguments': {
            'uri': 'resource://other',
            'name': 'x',
            'arguments': {},
          },
        }),
        throwsStateError,
      );
      broker.activeUri = 'resource://other';
      await expectLater(extension.dispatch('tools/list', {}), throwsStateError);
      broker.activeUri = 'resource://one';
      current = false;
      await expectLater(extension.dispatch('tools/list', {}), throwsStateError);
      await extension.close();
      expect(plugin.disposed, 1);
      current = true;
      await expectLater(extension.dispatch('tools/list', {}), throwsStateError);
      expect(plugin.calls, 2);
      broker.dispose();
    },
  );
}
