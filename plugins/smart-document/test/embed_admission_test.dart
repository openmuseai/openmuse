import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_smart_document_plugin/openmuse_smart_document_plugin.dart';
import 'resource_scope_test.dart' as fixture;

void main() {
  for (final invalid in ['object', 'extra', 'ended']) {
    testWidgets(
      'direct smart-document embed rejects $invalid before all owner side effects',
      (t) async {
        final calls = <String, int>{
          'store': 0,
          'provider': 0,
          'realtime': 0,
          'opened': 0,
          'checkpoint': 0,
        };
        final plugin = OpenMuseSmartDocumentPlugin(
          storeFactory: (_) async {
            calls['store'] = calls['store']! + 1;
            return fixture.Memory();
          },
          providerFactory: (_) {
            calls['provider'] = calls['provider']! + 1;
            return null;
          },
          realtimeFactory: (_, _) async {
            calls['realtime'] = calls['realtime']! + 1;
            return null;
          },
        );
        final seed = await plugin.createResource('smart-document.document', {});
        final template = fixture.session(seed.bytes);
        final lifetime = ResourceSessionLifetime();
        final session = ResourceEditorSession(
          opened: template.opened,
          lifetime: lifetime,
          allowEditing: true,
          commitContent: (_, _, _) async {
            calls['checkpoint'] = calls['checkpoint']! + 1;
            throw StateError('unexpected checkpoint');
          },
          onOpened: (_) {
            calls['opened'] = calls['opened']! + 1;
          },
          onChanged: (_) {},
        );
        final valid = (await plugin.resourceEmbedTargets(
          session.opened,
          'smart-document.document',
        )).single.target;
        final invalidTarget = ResourceEmbedTarget(
          reference: valid.reference,
          surface: valid.surface,
          selector: {
            ...valid.selector,
            if (invalid == 'object')
              'objectId': '00000000-0000-4000-8000-000000000099',

            if (invalid == 'extra') 'unexpected': 'value',
          },
        );
        if (invalid == 'ended') lifetime.invalidate();
        await t.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: Builder(
                builder: (context) => plugin.buildResourceEmbed(
                  context,
                  session,
                  invalidTarget,
                  const ResourceEmbedPolicy(allowEditing: true),
                ),
              ),
            ),
          ),
        );
        await t.pump();
        await t.pump(const Duration(milliseconds: 400));
        expect(find.text('智能文档无法打开，请检查内容与编辑能力。'), findsOneWidget);
        expect(calls.values, everyElement(0));
        expect(plugin.activeController(session.record.identity), isNull);
        expect(session.dirty, isFalse);
        await t.pumpWidget(const SizedBox.shrink());
        await t.pump();
        session.dispose();
        template.dispose();
        await plugin.deactivate();
        expect(calls.values, everyElement(0));
        expect(t.takeException(), isNull);
      },
    );
  }
}
