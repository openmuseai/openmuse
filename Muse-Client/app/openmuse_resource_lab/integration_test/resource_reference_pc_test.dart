import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:ui' as ui;

import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:muse_dsh_conversation_flutter/muse_dsh_conversation_flutter.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_lab/main.dart' as lab;

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  final output = Directory(
    Platform.environment['OPENMUSE_RESOURCE_QA_EVIDENCE'] ??
        '/tmp/openmuse-resource-center-qa/pc-ui',
  );
  final errors = <String>[];
  unawaited(
    binding.allTestsPassed.future.then((passed) async {
      await output.create(recursive: true);
      await File('${output.path}/result.json').writeAsString(
        jsonEncode({
          'passed': passed,
          'tests': binding.results.map(
            (key, value) => MapEntry(key, value.toString()),
          ),
          'errors': errors,
          'platform': Platform.operatingSystem,
          'surface':
              'production shared Flutter resource UI and native DSH conversation',
        }),
      );
    }),
  );
  testWidgets(
    'PC opens pinned phone references online and offline after execution cleanup',
    (tester) async {
      await output.create(recursive: true);
      final previous = FlutterError.onError;
      FlutterError.onError = (details) {
        errors.add(details.toString());
        previous?.call(details);
      };
      addTearDown(() => FlutterError.onError = previous);
      await lab.main();
      addTearDown(() async {
        await lab.closeResourceLab?.call();
      });
      Future<void> until(bool Function() ready, {int seconds = 60}) async {
        final end = DateTime.now().add(Duration(seconds: seconds));
        while (!ready()) {
          if (DateTime.now().isAfter(end)) {
            throw TestFailure('PC UI condition timed out');
          }
          await tester.pump(const Duration(milliseconds: 120));
          await Future<void>.delayed(const Duration(milliseconds: 80));
        }
        await tester.pump(const Duration(milliseconds: 400));
      }

      Future<void> shot(String name) async {
        await tester.pump(const Duration(milliseconds: 500));
        final boundary =
            lab.resourceLabCapture.currentContext!.findRenderObject()!
                as RenderRepaintBoundary;
        final image = await boundary.toImage(pixelRatio: 1.5);
        try {
          final data = await image.toByteData(format: ui.ImageByteFormat.png);
          await File(
            '${output.path}/$name.png',
          ).writeAsBytes(data!.buffer.asUint8List());
        } finally {
          image.dispose();
        }
      }

      await until(() => lab.resourceLabReviewReady, seconds: 300);
      await until(
        () => find.byType(DshConversationSurface).evaluate().isNotEmpty,
      );
      await Future<void>.delayed(const Duration(seconds: 2));
      final center = lab.resourceLabCenter!;
      final surface = tester.widget<DshConversationSurface>(
        find.byType(DshConversationSurface),
      );
      final pattern = RegExp(r'openmuse-resource://v1/[A-Za-z0-9_-]+');
      final refs = surface.store.snapshot.rows
          .expand((row) => pattern.allMatches(row.text))
          .map((match) => ResourceReference.fromUri(match.group(0)!))
          .toList();
      final origin = refs.firstWhere((ref) => ref.retained == null);
      final retained = refs.firstWhere((ref) => ref.retained != null);
      expect(origin.identity, retained.identity);
      expect(origin.version, retained.version);
      final current = await center.hub
          .provider(origin.identity)
          .read(
            origin.identity.resource,
            null,
            center.hub.access('qa-current-version'),
          );
      expect(utf8.decode(current.bytes), 'Phone current version V2');
      await center.execution.projections.clear();
      final projections = Directory(
        '${center.root.path}/execution-projections',
      );
      if (await projections.exists()) await projections.delete(recursive: true);
      await shot('01-pc-conversation-links');
      Future<void> click(ResourceReference ref) async {
        final timeline = find.byKey(const ValueKey('dsh-native.timeline'));
        final list = tester.widget<ListView>(timeline);
        list.controller!.jumpTo(0);
        await tester.pump(const Duration(milliseconds: 400));
        final button = find.byKey(
          ValueKey('conversation-resource-${ref.toUri()}'),
        );
        await tester.scrollUntilVisible(
          button,
          250,
          scrollable: find
              .descendant(of: timeline, matching: find.byType(Scrollable))
              .first,
          maxScrolls: 30,
        );
        await tester.tap(button);
        await tester.pump(const Duration(milliseconds: 400));
      }

      Future<void> checkVersion(String name) async {
        final content = find.byKey(const ValueKey('resource-notes-content'));
        await until(() => content.evaluate().isNotEmpty);
        final field = tester.widget<TextField>(content);
        expect(field.readOnly, isTrue);
        expect(field.controller!.text, contains('RESOURCE_QA_V1_'));
        expect(
          field.controller!.text,
          isNot(contains('Phone current version V2')),
        );
        await shot(name);
        await tester.pageBack();
        await tester.pump(const Duration(milliseconds: 500));
      }

      await click(origin);
      await checkVersion('02-online-origin-pinned-v1');
      await click(retained);
      await checkVersion('03-online-retained-reference-pinned-v1');
      // Let the independently running phone finish its remote-directory checks
      // and close its foreground bridge. No test toggles a fake availability bit.
      lab.resourceLabReviewReleased = true;
      await until(
        () => !center.hub.provider(origin.identity).available,
        seconds: 75,
      );
      await click(origin);
      await until(() => find.byType(AlertDialog).evaluate().isNotEmpty);
      final strings = OpenMuseStrings.forLocale('en');
      expect(
        find.text(strings.resourcecenterSourceUnavailable()),
        findsOneWidget,
      );
      await shot('04-offline-origin-unavailable');
      await tester.tap(find.text(strings.resourcecenterClose()));
      await tester.pump(const Duration(milliseconds: 400));
      await click(retained);
      await until(
        () => find
            .text(strings.resourcecenterOfflineCopy())
            .evaluate()
            .isNotEmpty,
      );
      await checkVersion('05-offline-retained-readonly-v1');
      expect(
        await projections.exists() ? await projections.list().length : 0,
        0,
      );
      await lab.closeResourceLab?.call();
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pump(const Duration(milliseconds: 500));
      await Future<void>.delayed(const Duration(seconds: 2));
      expect(tester.takeException(), isNull);
      await File('${output.path}/proof.json').writeAsString(
        jsonEncode({
          'origin': origin.toJson(),
          'retained': retained.toJson(),
          'phoneOriginalVersion': 'V2',
          'allOpenedConversationVersions': 'V1',
          'executionProjectionRemovedBeforeOpen': true,
          'onlineOrigin': true,
          'onlineRetained': true,
          'offlineOriginRefused': true,
          'offlineRetainedReadOnly': true,
        }),
      );
    },
  );
}
