import 'package:flutter_test/flutter_test.dart';
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
    'resource extension can call only tools declared by its immutable package',
    () async {
      final broker = ResourceAgentBroker()..activeUri = 'resource://test';
      var calls = 0;
      broker.register(
        ResourceAppSession(
          uri: 'resource://test',
          title: 'Test',
          request: (method, params) async {
            if (method == 'tools/list') {
              return {
                'tools': [
                  {'name': 'read_example'},
                  {'name': 'write_example'},
                ],
              };
            }
            calls++;
            return {'content': []};
          },
        ),
      );
      final extension = ResourceExtensionSession(
        caller: 'test',
        resourceUri: 'resource://test',
        broker: broker,
        contributors: () => [],
        approve: (_, _) async => false,
        isCurrent: () async => true,
        resourceTools: {'read_example'},
      );
      Future<Map<String, Object?>> invoke(String name) =>
          extension.dispatch('tools/call', {
            'name': 'call_resource_tool',
            'arguments': {
              'uri': 'resource://test',
              'name': name,
              'arguments': {},
            },
          });
      await invoke('read_example');
      expect(calls, 1);
      await expectLater(invoke('write_example'), throwsStateError);
      expect(calls, 1);
      await extension.close();
      broker.dispose();
    },
  );
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
