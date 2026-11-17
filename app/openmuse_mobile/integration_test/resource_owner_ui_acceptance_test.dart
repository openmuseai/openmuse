import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile/main.dart' as app;
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_models.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

const account = String.fromEnvironment('LIVE_ACCEPTANCE_ACCOUNT');
void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized()
    ..shouldPropagateDevicePointerEvents = true;
  testWidgets(
    'CN real Desktop resource owner to Mobile viewer UI',
    (tester) async {
      await app.main();
      Future<void> until(
        bool Function() ready,
        String name, {
        int seconds = 100,
      }) async {
        final clock = Stopwatch()..start();
        while (!ready()) {
          if (clock.elapsed > Duration(seconds: seconds)) {
            throw TimeoutException(name);
          }
          await tester.pump(const Duration(milliseconds: 100));
          await Future<void>.delayed(const Duration(milliseconds: 50));
        }
      }

      await until(
        () =>
            find.byType(WorkBuddyShell).evaluate().isNotEmpty ||
            find.byKey(const ValueKey('legal-accept')).evaluate().isNotEmpty,
        'application shell or first-run agreement',
      );
      if (find.byKey(const ValueKey('legal-accept')).evaluate().isNotEmpty) {
        expect(
          const bool.fromEnvironment('LIVE_ACCEPTANCE_LEGAL_APPROVED'),
          true,
          reason:
              'A user must explicitly approve this agreement action before the run.',
        );
        await tester.tap(find.byKey(const ValueKey('legal-checkbox')));
        await tester.pump();
        await tester.tap(find.byKey(const ValueKey('legal-accept')));
      }
      await until(
        () => find.byType(WorkBuddyShell).evaluate().isNotEmpty,
        'normal application shell',
      );
      WorkBuddyShell shell() =>
          tester.widget<WorkBuddyShell>(find.byType(WorkBuddyShell));
      expect(account, isNotEmpty);
      if (shell().authentication?.snapshot.isAuthenticated != true) {
        final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
        final login = Completer<Map<String, Object?>>();
        server.listen((request) async {
          try {
            if (request.method != 'POST' || request.contentLength > 4096) {
              throw const FormatException('Invalid login envelope');
            }
            final input = Map<String, Object?>.from(
              jsonDecode(await utf8.decoder.bind(request).join()) as Map,
            );
            if (input['email'] != account ||
                input['password'] is! String ||
                login.isCompleted) {
              throw const FormatException('Invalid login account');
            }
            request.response.statusCode = HttpStatus.noContent;
            await request.response.close();
            login.complete(input);
          } on Object catch (e, stack) {
            if (!login.isCompleted) login.completeError(e, stack);
          }
        });
        debugPrint('RESOURCE_UI_LOGIN_PORT=${server.port}');
        try {
          final input = await login.future.timeout(
            const Duration(seconds: 100),
          );
          await shell().authentication!.signInWithPassword(
            input['email'] as String,
            input['password'] as String,
          );
          input.clear();
        } finally {
          await server.close(force: true);
        }
      }
      await until(
        () => shell().authentication?.snapshot.isAuthenticated == true,
        'account login',
      );
      expect(shell().authentication?.snapshot.identity?.email, account);
      final paired = shell().pairedDesktop!;
      await until(
        () => paired.devices.any(
          (d) =>
              d.kind == AccountDeviceKind.desktop &&
              d.online &&
              d.supportsPairedDesktop,
        ),
        'online same-account Desktop',
      );
      final desktops = paired.devices
          .where(
            (d) =>
                d.kind == AccountDeviceKind.desktop &&
                d.online &&
                d.supportsPairedDesktop,
          )
          .toList();
      expect(desktops, hasLength(1));
      final desktop = desktops.single;
      final bind = find.byKey(
        ValueKey('wb-device-bind-paired.${desktop.deviceRef}'),
      );
      await until(
        () =>
            bind.hitTestable().evaluate().isNotEmpty ||
            paired.snapshot.connection != null,
        'Desktop binding UI or restored connection',
      );
      if (bind.hitTestable().evaluate().isNotEmpty) {
        await tester.tap(bind);
      }
      await until(
        () =>
            paired.snapshot.connection?.deviceRef == desktop.deviceRef &&
            find.byKey(const ValueKey('wb-bind-desktop')).evaluate().isEmpty,
        'Desktop connection and binding sheet completed',
      );
      expect(
        shell().controller.selectedDeviceId,
        'paired.${desktop.deviceRef}',
      );
      expect(
        Uri.parse(paired.snapshot.connection!.session.origin).host,
        'cn.openmuseai.com',
      );
      await until(
        () =>
            shell().resources?.hub.providers.any(
              (p) => p.source.locatorOwner == 'dsh' && p.available,
            ) ==
            true,
        'remote resource source catalog',
      );
      final tab = find.byKey(const ValueKey('wb-tab-任务'));
      await until(() => tab.evaluate().isNotEmpty, 'task tab');
      await tester.pump(const Duration(milliseconds: 300));
      await until(
        () => tab.hitTestable().evaluate().isNotEmpty,
        'task tab hit target',
      );
      await tester.tap(tab);
      await tester.pump(const Duration(milliseconds: 300));
      expect(shell().controller.tab, WbTab.tasks);
      debugPrint('RESOURCE_OWNER_UI_TASKS_TAB');
      await until(
        () => shell().controller.tasks.any(
          (t) => t.title.startsWith('File viewer acceptance.'),
        ),
        'actual Desktop acceptance conversation',
      );
      final task = shell().controller.tasks.firstWhere(
        (t) => t.title.startsWith('File viewer acceptance.'),
      );
      shell().controller.openTaskById(task.id);
      Finder paragraph() => find.byWidgetPredicate(
        (w) =>
            w is RichText &&
            w.text.toPlainText().contains('Exact') &&
            w.text.toPlainText().contains('Incomplete') &&
            !w.text.toPlainText().contains('File viewer acceptance'),
      );
      await until(
        () => paragraph().evaluate().isNotEmpty,
        'real assistant file links rendered',
      );
      Future<void> link(String label) async {
        final finder = paragraph().first;
        await tester.ensureVisible(finder);
        await tester.pump(const Duration(milliseconds: 500));
        final widget = tester.widget<RichText>(finder);
        final text = widget.text.toPlainText();
        final index = text.indexOf(label);
        expect(index, greaterThanOrEqualTo(0));
        final render = tester.renderObject<RenderParagraph>(finder);
        final boxes = render.getBoxesForSelection(
          TextSelection(baseOffset: index, extentOffset: index + label.length),
        );
        expect(boxes, isNotEmpty);
        await tester.tapAt(render.localToGlobal(boxes.first.toRect().center));
        await tester.pump();
      }

      Future<void> visible(String marker) => until(
        () => find
            .textContaining(marker, findRichText: true)
            .evaluate()
            .isNotEmpty,
        marker,
      );
      Future<void> back() async {
        await tester.tap(find.byType(BackButton).first);
        await tester.pump();
        await until(
          () => paragraph().evaluate().isNotEmpty,
          'back to same conversation',
        );
        // The underlying conversation exists while the viewer route is still
        // reversing. Wait for its transition before the next pointer event.
        await tester.pump(const Duration(milliseconds: 500));
      }

      await link('Exact');
      await visible('ALPHA_FILE_OWNER_RESOLUTION_OK');
      debugPrint('RESOURCE_OWNER_UI_EXACT_OK');
      await back();
      await link('Incomplete');
      await visible('UNIQUE_FILE_OWNER_RESOLUTION_OK');
      debugPrint('RESOURCE_OWNER_UI_INCOMPLETE_OK');
      await back();
      await link('Ambiguous');
      const prefix = 'resource-locator-resource-locator-acceptance-20261113/';
      final alpha = find.byKey(const ValueKey('${prefix}alpha/hello.txt'));
      final beta = find.byKey(const ValueKey('${prefix}beta/hello.txt'));
      await until(
        () => alpha.evaluate().isNotEmpty && beta.evaluate().isNotEmpty,
        'ambiguous candidate picker',
      );
      await tester.pump(const Duration(milliseconds: 500));
      await until(
        () =>
            alpha.hitTestable().evaluate().isNotEmpty &&
            beta.hitTestable().evaluate().isNotEmpty,
        'ambiguous candidates visible and tappable',
      );
      debugPrint('RESOURCE_OWNER_UI_AMBIGUOUS_PICKER_OK');
      expect(
        find.textContaining('BETA_FILE_OWNER_RESOLUTION_OK'),
        findsNothing,
      );
      await tester.tap(beta);
      await visible('BETA_FILE_OWNER_RESOLUTION_OK');
      debugPrint(
        'RESOURCE_OWNER_UI_PASS exact=true incomplete=true ambiguous=true',
      );
      binding.reportData = {
        'market': 'china',
        'realDesktop': true,
        'realResourceRpc': true,
        'exact': true,
        'incomplete': true,
        'ambiguousPicker': true,
        'selectedBeta': true,
        'connectionAndSessionSetup': 'production controllers',
        'fileActions': 'UI taps and visible content assertions',
      };
      final hold = Stopwatch()..start();
      while (hold.elapsed < const Duration(seconds: 45)) {
        await tester.pump(const Duration(milliseconds: 200));
        await Future<void>.delayed(const Duration(milliseconds: 100));
      }
    },
    timeout: const Timeout(Duration(minutes: 5)),
  );
}
