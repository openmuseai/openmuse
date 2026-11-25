import 'dart:convert';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';

void main() {
  late Map<String, dynamic> fixture;
  setUpAll(() async {
    final result = await Process.run('node', [
      '../../services/config-control/test/runtime-fixture.mjs',
    ]);
    if (result.exitCode != 0) throw StateError('${result.stderr}');
    fixture = jsonDecode(result.stdout as String) as Map<String, dynamic>;
  });
  for (final topInset in [0.0, 24.0, 59.0]) {
    testWidgets(
      'runtime banner consumes top inset $topInset once above a document route',
      (tester) async {
        tester.view.devicePixelRatio = 1;
        tester.view.physicalSize = const Size(428, 926);
        tester.view.padding = FakeViewPadding(top: topInset, bottom: 34);
        tester.view.viewPadding = FakeViewPadding(top: topInset, bottom: 34);
        addTearDown(tester.view.resetDevicePixelRatio);
        addTearDown(tester.view.resetPhysicalSize);
        addTearDown(tester.view.resetPadding);
        addTearDown(tester.view.resetViewPadding);
        addTearDown(tester.view.resetViewInsets);
        final locale = OpenMuseLocaleController();
        final runtime = OpenMuseClientRuntime(
          bundle: DeploymentBundle.parse(jsonEncode(fixture['bundle'])),
          transport: _Transport(),
          store: MemoryConfigStore(),
          build: 1,
          locale: locale,
          clock: () => DateTime.parse(fixture['now'] as String),
        );
        final navigator = GlobalKey<NavigatorState>();
        Widget app() => OpenMuseRuntimeHost(
          runtime: runtime,
          disposeRuntime: false,
          child: MaterialApp(
            navigatorKey: navigator,
            builder: (_, child) => OpenMuseRuntimeNotice(child: child!),
            home: const Scaffold(body: Text('home')),
          ),
        );
        await tester.pumpWidget(app());
        navigator.currentState!.push<void>(
          MaterialPageRoute(
            builder: (_) => Scaffold(
              appBar: AppBar(title: const Text('Document')),
              body: SafeArea(
                child: Column(
                  children: [
                    const Text('editor toolbar'),
                    const Expanded(child: TextField()),
                    const Text(
                      'editor footer',
                      key: ValueKey('document-bottom'),
                    ),
                  ],
                ),
              ),
            ),
          ),
        );
        await tester.pumpAndSettle();
        expect(
          tester.getSize(find.byType(AppBar)).height,
          closeTo(kToolbarHeight + topInset, 0.1),
        );

        runtime.pendingRestart = runtime.presentation;
        await tester.pumpWidget(app());
        await tester.pumpAndSettle();
        expect(find.text('Document'), findsOneWidget);
        expect(
          tester.getSize(find.byType(AppBar)).height,
          closeTo(kToolbarHeight, 0.1),
          reason:
              'The title must follow the banner without a second status-bar gap',
        );
        expect(
          tester.getTopLeft(find.text('editor toolbar')).dy,
          closeTo(tester.getBottomLeft(find.byType(AppBar)).dy, 0.1),
        );
        expect(
          tester
              .getBottomLeft(find.byKey(const ValueKey('document-bottom')))
              .dy,
          closeTo(926 - 34, 0.1),
        );

        tester.view.padding = FakeViewPadding(top: topInset);
        tester.view.viewInsets = const FakeViewPadding(bottom: 300);
        await tester.pumpAndSettle();
        expect(
          tester
              .getBottomLeft(find.byKey(const ValueKey('document-bottom')))
              .dy,
          closeTo(926 - 300, 0.1),
        );

        runtime.pendingRestart = null;
        await tester.pumpWidget(app());
        await tester.pumpAndSettle();
        expect(find.text('Document'), findsOneWidget);
        expect(
          tester.getSize(find.byType(AppBar)).height,
          closeTo(kToolbarHeight + topInset, 0.1),
        );
        await tester.pumpWidget(const SizedBox());
        runtime.dispose();
        locale.dispose();
      },
    );
  }
  test(
    'Node and Dart compile all registered ICU messages to the same bounded AST',
    () {
      for (final template in fixture['templates'] as List) {
        final parsed = MessageTemplate(template['source'] as String);
        expect(parsed.toData(), template['nodes']);
        expect(parsed.parameters.toList()..sort(), template['parameters']);
      }
    },
  );
  test(
    'Node signed releases drive language updates, persistent cohorts and safe session staging',
    () async {
      final bundle = DeploymentBundle.parse(jsonEncode(fixture['bundle']));
      final frames = fixture['frames'] as List;
      final store = MemoryConfigStore(), transport = _Transport();
      var now = DateTime.parse(fixture['now'] as String);
      final locale = OpenMuseLocaleController(
        defaultTag: bundle.snapshot.defaultLocale,
      );
      final runtime = OpenMuseClientRuntime(
        bundle: bundle,
        transport: transport,
        store: store,
        build: 1,
        locale: locale,
        cohort: 7,
        clock: () => now,
      );
      transport.use(frames[0] as Map);
      await runtime.restore();
      await runtime.refresh();
      expect(runtime.lastError, isNull);
      expect(locale.strings.commonSave(), 'Save globally');
      expect(locale.tag, bundle.snapshot.enabledLocales.first);
      expect(transport.requests.first.path, endsWith('/active/7.jws'));
      final firstRevision = locale.catalog.revision;
      transport.use(frames[1] as Map);
      await runtime.refresh();
      expect(locale.strings.commonSave(), 'Save worldwide');
      expect(locale.catalog.revision, greaterThan(firstRevision));
      expect(runtime.network.supportEmail, 'updated@example.test');
      transport.use(frames[2] as Map);
      await runtime.refresh();
      expect(runtime.pendingRestart?.sequence, 3);
      expect(runtime.network.sequence, 2);
      expect(runtime.presentation.sequence, 3);
      // A newer pending release cannot renew an old session's endpoint lease.
      now = now.add(const Duration(minutes: 11));
      expect(runtime.client.networkConfigExpired, isFalse);
      expect(runtime.checkNetwork, throwsStateError);
      final saved = await store.read();
      final restoredLocale = OpenMuseLocaleController();
      final restored = OpenMuseClientRuntime(
        bundle: bundle,
        transport: transport,
        store: store,
        build: 1,
        locale: restoredLocale,
        clock: () => now,
      );
      await restored.restore();
      expect(restored.client.cohort, 7);
      expect(restored.network.sequence, 3);
      expect(restored.pendingRestart, isNull);
      transport.use(frames[3] as Map);
      await restored.refresh();
      expect(restored.lastError, isFormatException);
      expect(restored.client.current.sequence, 3);
      expect(await store.read(), saved);
      expect(restoredLocale.strings.commonSave(), 'Save worldwide');
      runtime.dispose();
      restored.dispose();
      locale.dispose();
      restoredLocale.dispose();
    },
  );
  test(
    'an authentic cache with invalid message semantics preserves replay protection and blocks cloud I/O',
    () async {
      final bundle = DeploymentBundle.parse(jsonEncode(fixture['bundle']));
      final frame = (fixture['frames'] as List).last as Map;
      final snapshot = frame['snapshot'] as Map;
      final resource = (snapshot['resources'] as List).single as Map;
      final store = MemoryConfigStore();
      await store.commit(
        jsonEncode({
          'jws': frame['jws'],
          'cohort': 9,
          'resources': {
            resource['id']: (frame['resources'] as Map)[resource['url']],
          },
        }),
      );
      final locale = OpenMuseLocaleController();
      final transport = _Transport()
        ..use((fixture['frames'] as List)[2] as Map);
      final runtime = OpenMuseClientRuntime(
        bundle: bundle,
        transport: transport,
        store: store,
        build: 1,
        locale: locale,
        clock: () => DateTime.parse(fixture['now'] as String),
      );
      await runtime.restore();
      expect(runtime.client.current.sequence, 0);
      expect(runtime.client.networkUnavailable, isTrue);
      expect(runtime.client.restoreError, isFormatException);
      expect(runtime.checkNetwork, throwsStateError);
      await runtime.refresh();
      expect(runtime.lastError, isFormatException);
      expect(runtime.checkNetwork, throwsStateError);
      runtime.dispose();
      locale.dispose();
    },
  );
}

final class _Transport implements ConfigTransport {
  Map frame = {};
  final requests = <Uri>[];
  void use(Map value) => frame = value;
  @override
  Future<List<int>> get(Uri uri, {required int maxBytes}) async {
    requests.add(uri);
    final List<int> bytes = uri.path.endsWith('.jws')
        ? utf8.encode(frame['jws'] as String)
        : base64Decode((frame['resources'] as Map)[uri.toString()] as String);
    if (bytes.length > maxBytes) {
      throw const FormatException('Resource too large');
    }
    return bytes;
  }
}
