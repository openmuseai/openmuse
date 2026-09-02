import 'dart:convert';
import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:muse_plugin_facets/muse_plugin_facets.dart';

void main() {
  final museRoot =
      Platform.environment['MUSE_WORKSPACE_ROOT'] ?? '../../../../../..';
  final sharedRoot = Directory('$museRoot/middlewares/dsh/core/plugin-facets');

  test('Dart matches the shared v1 fixture verdicts', () async {
    final fixtures = jsonDecode(
      await File('${sharedRoot.path}/fixtures/v1/messages.json').readAsString(),
    ) as List<dynamic>;
    for (final raw in fixtures) {
      final fixture = raw as Map<String, dynamic>;
      final kind = MuseFacetSchemaKind.parse(fixture['kind'] as String);
      final expected = fixture['valid'] as bool;
      try {
        validateMuseFacetValue(kind, fixture['value']);
        expect(expected, isTrue, reason: fixture['name'] as String);
      } on MuseFacetContractException {
        expect(expected, isFalse, reason: fixture['name'] as String);
      }
    }
  });

  test('Dart JCS-compatible encoding matches shared schema digests', () async {
    const files = [
      'plugin-descriptor.schema.json',
      'context-contribution.schema.json',
      'domain-change.schema.json',
      'presentation-intent.schema.json',
      'presentation-intent-result.schema.json',
    ];
    final expected = jsonDecode(
      await File('${sharedRoot.path}/fixtures/v1/schema-digests.json')
          .readAsString(),
    ) as Map<String, dynamic>;
    final digests = <String>{};
    for (final file in files) {
      final schema = jsonDecode(
        await File('${sharedRoot.path}/schemas/v1/$file').readAsString(),
      );
      final digest = museSchemaDigest(schema);
      expect(digest, matches(RegExp(r'^sha256:[0-9a-f]{64}$')));
      expect(digest, expected[file.replaceFirst('.schema.json', '')]);
      expect(digests.add(digest), isTrue, reason: file);
    }
  });

  test('typed context serializes to the public envelope', () {
    final value = MuseContextContributionV1(
      pluginId: 'muse.appflowy.markdown',
      pluginVersion: '1.0.0',
      facetInstanceRef: 'facet.1',
      surfaceInstanceRef: 'surface.1',
      surfaceKind: 'markdown.document',
      scopeRef: 'scope.1',
      contextType: 'markdown.selection.v1',
      contextSchemaDigest: 'sha256:${List.filled(64, '0').join()}',
      contextRevision: '1',
      epochRef: 'epoch.1',
      lane: MuseContextLane.state,
      capturedAt: 1,
      expiresAt: 2,
      payload: const {'opaque': true},
    ).toJson();
    expect(
        () => validateMuseFacetValue(
            MuseFacetSchemaKind.contextContribution, value),
        returnsNormally);
  });

  test('typed domain change round-trips origin and commandRef', () {
    final original = MuseDomainChangeV1(
      pluginId: 'muse.appflowy.markdown',
      providerInstanceRef: 'provider.1',
      scopeRef: 'scope.host.opaque',
      resourceRef: 'resource.1',
      eventType: 'markdown.document.changed',
      eventSchemaDigest: 'sha256:${List.filled(64, '0').join()}',
      domainRevision: '8',
      epochRef: 'epoch.1',
      commandRef: 'command.1',
      origin: MuseMutationOrigin.externalCommand,
      occurredAt: 1100,
      payload: const {'selectionRebaseHint': 'sync-required'},
    );
    final parsed = MuseDomainChangeV1.fromJson(original.toJson());
    expect(parsed.origin, MuseMutationOrigin.externalCommand);
    expect(parsed.commandRef, 'command.1');
    expect(parsed.resourceRef, 'resource.1');
    expect(
        () => validateMuseFacetValue(
            MuseFacetSchemaKind.domainChange, parsed.toJson()),
        returnsNormally);
  });
}
