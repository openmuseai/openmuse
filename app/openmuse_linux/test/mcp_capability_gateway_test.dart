import 'dart:typed_data';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_linux/src/host/mcp_capability_gateway.dart';

void main() {
  test(
    'account is captured at construction, before the first request',
    () async {
      var account = 'one';
      final gateway = McpCapabilityGateway(
        describeHost: () async => {},
        listProviders: () async => [],
        chooseResource: (_, _) async => null,
        approve: (_, _) async => false,
        openDevelopment: (_) async => {},
        readDevelopment: (_) async => {},
        scope: () => account,
      );
      account = 'two';
      await expectLater(
        gateway.call('capabilities_list', {}),
        throwsStateError,
      );
      await gateway.close();
    },
  );
  test(
    'two bindings stay independent; overrides and other callers are rejected',
    () async {
      var scope = 'account-one';
      var current = true;
      final received = <Map<String, Object?>>[];
      Future<List<CapabilityProvider>> providers() async => [
        CapabilityProvider(
          id: 'example/provider',
          version: '1.0.0',
          digest: 'a' * 64,
          contracts: [
            {
              'id': 'https://example.com/read',
              'tools': ['read'],
            },
          ],
          tools: [
            {
              'name': 'read',
              'inputSchema': {
                'type': 'object',
                'properties': {
                  'content': {
                    'type': 'string',
                    'contentEncoding': 'base64',
                    'contentMediaType': 'example/document',
                  },
                  'name': {'type': 'string'},
                },
                'required': ['content'],
                'additionalProperties': false,
              },
              'outputSchema': {
                'type': 'object',
                'properties': {
                  'ok': {'type': 'boolean'},
                },
                'required': ['ok'],
              },
            },
          ],
          invoke: (name, args) async {
            received.add(args);
            return {
              'content': [],
              'structuredContent': {'ok': true},
            };
          },
          isCurrent: () async => current,
        ),
      ];
      McpCapabilityGateway gateway() => McpCapabilityGateway(
        describeHost: () async => {},
        listProviders: providers,
        chooseResource: (title, mime) async => GrantedResource(
          title: title,
          mediaType: mime,
          bytes: Uint8List.fromList(title.codeUnits),
        ),
        approve: (_, details) async => true,
        openDevelopment: (_) async => {},
        readDevelopment: (_) async => {},
        scope: () => scope,
      );
      final first = gateway(), other = gateway();
      final refs = <String>[];
      for (final title in ['first', 'second']) {
        final resource = await first.call('resources_choose', {
          'title': title,
          'mediaType': 'example/document',
        });
        final binding = await first.call('capabilities_bind', {
          'provider': 'example/provider',
          'resourceRef': resource['resourceRef'],
          'contract': 'https://example.com/read',
          'tools': ['read'],
        });
        refs.add(binding['binding'] as String);
      }
      for (final ref in refs) {
        await first.call('capabilities_call', {
          'binding': ref,
          'tool': 'read',
          'arguments': {},
        });
      }
      expect(received.map((r) => r['name']), ['first', 'second']);
      await expectLater(
        other.call('capabilities_call', {
          'binding': refs.first,
          'tool': 'read',
          'arguments': {},
        }),
        throwsStateError,
      );
      await expectLater(
        first.call('capabilities_call', {
          'binding': refs.first,
          'tool': 'read',
          'arguments': {'content': 'override'},
        }),
        throwsStateError,
      );
      current = false;
      await expectLater(
        first.call('capabilities_call', {
          'binding': refs.first,
          'tool': 'read',
          'arguments': {},
        }),
        throwsStateError,
      );
      current = true;
      scope = 'account-two';
      await expectLater(
        first.call('capabilities_call', {
          'binding': refs.first,
          'tool': 'read',
          'arguments': {},
        }),
        throwsStateError,
      );
      await first.close();
      await other.close();
    },
  );
  test('revocation during resource selection cannot create a grant', () async {
    var scope = 'one';
    final gateway = McpCapabilityGateway(
      describeHost: () async => {},
      listProviders: () async => [],
      chooseResource: (_, mime) async {
        scope = 'two';
        return GrantedResource(
          title: 'x',
          mediaType: mime,
          bytes: Uint8List(0),
        );
      },
      approve: (_, details) async => false,
      openDevelopment: (_) async => {},
      readDevelopment: (_) async => {},
      scope: () => scope,
    );
    await expectLater(
      gateway.call('resources_choose', {
        'title': 'Choose',
        'mediaType': 'text/plain',
      }),
      throwsStateError,
    );
    await gateway.close();
  });
}
