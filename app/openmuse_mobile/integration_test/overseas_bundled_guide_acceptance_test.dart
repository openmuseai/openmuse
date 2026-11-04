import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile/main.dart' as mobile;
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:webview_flutter/webview_flutter.dart';

import 'support/device_result.dart';
import 'support/launch_checkpoint.dart';

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  recordDeviceResult(binding);
  testWidgets(
    'shipped Home guide opens in the native Notes editor',
    (tester) async {
      final release = await loadInstalledRelease();
      await tester.pumpWidget(
        await mobile.bootOpenMuseMobile(
          release.regions.firstWhere((r) => r.available).bundle!,
          'en',
        ),
      );
      Future<void> wait(Finder finder) async {
        for (var i = 0; i < 300; i++) {
          await tester.pump(const Duration(milliseconds: 100));
          if (finder.evaluate().isNotEmpty) return;
          await tester.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 100)),
          );
        }
        fail('Timed out waiting for $finder');
      }

      final home = find.byKey(const ValueKey('wb-tab-Home'));
      await wait(home);
      await tester.tap(home);
      final search = find.byKey(const ValueKey('resource-search'));
      await wait(search);
      tester.testTextInput.register();
      await tester.enterText(search, 'OpenMuse User Guide');
      tester.testTextInput.unregister();
      await tester.pumpAndSettle();
      for (var attempt = 0; attempt < 3; attempt++) {
        final guide = find.text('OpenMuse User Guide.sydoc');
        await wait(guide);
        await tester.ensureVisible(guide.first);
        await tester.tap(guide.first);
        for (var i = 0; i < 600; i++) {
          await tester.pump(const Duration(milliseconds: 100));
          final editor = find.byWidgetPredicate(
            (w) => w.runtimeType.toString() == '_DocumentEditor',
          );
          if (editor.evaluate().isNotEmpty) {
            final dynamic state = tester.state(editor);
            // This test opens only the public shipped guide in the isolated QA app.
            // Keep the underlying cause in QA evidence rather than a false save error.
            if (state.failure != null) {
              fail('Bundled guide open failed: ${state.failure}');
            }
          }
          if (find.byType(WebViewWidget).evaluate().isNotEmpty) break;
          await tester.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 100)),
          );
        }
        expect(find.byType(WebViewWidget), findsOneWidget);
        final web = WebViewController.fromPlatform(
          tester
              .widget<WebViewWidget>(find.byType(WebViewWidget))
              .platform
              .params
              .controller,
        );
        for (var i = 0; i < 300; i++) {
          await tester.pump(const Duration(milliseconds: 100));
          final ready = await tester.runAsync(
            () => web.runJavaScriptReturningResult(
              'document.documentElement.dataset.resourceReady === "true"',
            ),
          );
          if (ready == true || ready == 'true') break;
          await tester.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 100)),
          );
        }
        Object? text;
        for (var i = 0; i < 300; i++) {
          text = await tester.runAsync(
            () => web.runJavaScriptReturningResult('document.body.innerText'),
          );
          if (text.toString().contains('Desktop')) break;
          await tester.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 100)),
          );
          await tester.pump(const Duration(milliseconds: 100));
        }
        expect(text.toString(), contains('OpenMuse'));
        expect(text.toString(), contains('Desktop'));
        const richReadyScript = '''
        (() => {
          const editor = window.openMuseResourceEditor?.protyle?.wysiwyg?.element;
          const image = editor?.querySelector('img');
          if (image) image.scrollIntoView({block: 'center'});
          return !!image && image.complete && image.naturalWidth > 0;
        })()
      ''';
        var imageLoaded = false;
        for (var i = 0; i < 200; i++) {
          final ready = await tester.runAsync(
            () => web.runJavaScriptReturningResult(richReadyScript),
          );
          if (ready == true || ready == 'true') {
            imageLoaded = true;
            break;
          }
          await tester.runAsync(
            () => Future<void>.delayed(const Duration(milliseconds: 100)),
          );
          await tester.pump(const Duration(milliseconds: 100));
        }
        expect(
          imageLoaded,
          isTrue,
          reason:
              'Embedded application screenshot must load in the actual native editor',
        );
        await launchCheckpoint(
          tester,
          binding,
          'illustrated-guide-image-$attempt',
        );
        final hyperlink = await tester.runAsync(
          () => web.runJavaScriptReturningResult('''
          !!document.querySelector('[data-type~="a"][data-href^="https://"], a[href^="https://"]')
        '''),
        );
        expect(
          hyperlink == true || hyperlink == 'true',
          isTrue,
          reason: 'Guide must render a real hyperlink, not just URL text',
        );
        await tester.pageBack();
        await tester.pumpAndSettle();
        await wait(search);
      }
      binding.reportData = {
        'milestones': {
          'bundledGuideOpened': true,
          'actualGuideTextRendered': true,
          'embeddedScreenshotLoaded': true,
          'clickableHyperlinkRendered': true,
          'threeConsecutiveGuideOpens': true,
        },
      };
    },
    timeout: const Timeout(Duration(minutes: 6)),
  );
}
