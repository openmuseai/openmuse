import 'dart:convert';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mcp_app_runtime/src/mcp_resource_source.dart';

void main() {
  final hash = List.filled(64, 'a').join(), uri = 'artifact://project/node/1/0';
  final key = '${base64Url.encode(utf8.encode(uri)).replaceAll('=', '')}.$hash';
  test(
    'pinned MCP references bind the installed App, current account and exact digest',
    () {
      final ref = checkedMcpResourceLink(
        'mcp-resource://example.app/$key',
        'account-a',
        'example.app',
      );
      expect(ref.identity.scope, 'account-a');
      expect(ref.identity.source, 'mcp-example.app');
      expect(ref.version!.digest, hash);
      expect(decodePinnedMcpResource(key).uri, uri);
      for (final value in [
        'mcp-resource://other.app/$key',
        'mcp-resource://example.app/$key?token=anything',
        'mcp-resource://example.app/$key#fragment',
        'mcp-resource://example.app/not-pinned',
      ])
        expect(
          () => checkedMcpResourceLink(value, 'account-a', 'example.app'),
          throwsA(anything),
        );
      expect(
        () => checkedMcpResourceLink(
          'mcp-resource://example.app/$key',
          'signed-out-local',
          'example.app',
        ),
        throwsA(anything),
      );
    },
  );
  test(
    'MCP data references reject UI resources and oversized native identities',
    () {
      final ui =
          '${base64Url.encode(utf8.encode('ui://example/main')).replaceAll('=', '')}.$hash';
      expect(() => decodePinnedMcpResource(ui), throwsA(anything));
      expect(
        () => decodePinnedMcpResource(List.filled(300, 'a').join()),
        throwsA(anything),
      );
    },
  );
}
