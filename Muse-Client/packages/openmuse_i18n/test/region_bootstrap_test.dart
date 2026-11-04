import 'dart:io';
import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';

void main() {
  InstalledRelease release(String market) => InstalledRelease.parse(
    File(
      '../openmuse_runtime_config/assets/releases/$market.json',
    ).readAsStringSync(),
  );
  testWidgets(
    'Store launch resolves region without a license page or fabricated agreement',
    (tester) async {
      final installed = release('overseas');
      final saved = MemoryConfigStore();
      final region = installed.regions.firstWhere((r) => r.id == 'us');
      var boots = 0;
      Widget app() => OpenMuseRegionBootstrap(
        requiresLegalConsent: false,
        loadRelease: () async => installed,
        store: (_) async => saved,
        boot: (_, _) async {
          boots++;
          return const MaterialApp(home: Text('local document welcome'));
        },
      );
      await tester.pumpWidget(app());
      await tester.pumpAndSettle();
      expect(boots, 0);
      await tester.tap(find.byKey(const ValueKey('region-us')));
      await tester.pump();
      await tester.tap(find.byKey(const ValueKey('region-continue')));
      await tester.pumpAndSettle();
      expect(boots, 1);
      expect(find.text('local document welcome'), findsOneWidget);
      expect(find.byKey(const ValueKey('legal-checkbox')), findsNothing);
      expect(
        await RegionSelection(installed, saved).hasAcceptedLegal(region),
        isFalse,
      );
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pumpWidget(app());
      await tester.pumpAndSettle();
      expect(boots, 2);
      expect(find.byKey(const ValueKey('region-us')), findsNothing);
      expect(
        await RegionSelection(installed, saved).hasAcceptedLegal(region),
        isFalse,
      );
    },
  );
  testWidgets(
    'returning users never flash region or legal UI during restore and boot',
    (tester) async {
      final installed = release('overseas');
      final saved = MemoryConfigStore();
      final selection = RegionSelection(installed, saved);
      final region = installed.regions.firstWhere((r) => r.id == 'us');
      await selection.acceptLegal(region);
      final record = await saved.read();
      final loaded = Completer<InstalledRelease>();
      final restored = _DelayedReadStore();
      final booted = Completer<Widget>();
      await tester.pumpWidget(
        OpenMuseRegionBootstrap(
          loadRelease: () => loaded.future,
          store: (_) async => restored,
          boot: (_, _) => booted.future,
        ),
      );
      void expectLoadingOnly() {
        expect(find.byKey(const ValueKey('bootstrap-loading')), findsOneWidget);
        expect(find.byKey(const ValueKey('region-us')), findsNothing);
        expect(find.byKey(const ValueKey('region-continue')), findsNothing);
        expect(find.byKey(const ValueKey('legal-checkbox')), findsNothing);
        expect(find.text('Privacy Policy'), findsNothing);
      }

      expectLoadingOnly();
      loaded.complete(installed);
      await tester.pump();
      expectLoadingOnly();
      restored.reads[0].complete(record);
      await tester.pump();
      expectLoadingOnly();
      restored.reads[1].complete(record);
      await tester.pump();
      expectLoadingOnly();
      booted.complete(const MaterialApp(home: Text('restored application')));
      await tester.pumpAndSettle();
      expect(find.text('restored application'), findsOneWidget);
      expect(find.byKey(const ValueKey('legal-checkbox')), findsNothing);
    },
  );
  testWidgets(
    'business boot waits for an explicit durable overseas choice; unavailable regions never boot',
    (tester) async {
      var boots = 0;
      String? languageChoice;
      final store = MemoryConfigStore(), overseas = release('overseas');
      Widget app() => OpenMuseRegionBootstrap(
        loadRelease: () async => overseas,
        store: (_) async => store,
        boot: (bundle, language) async {
          languageChoice = language;
          boots++;
          return const MaterialApp(home: Text('business app'));
        },
      );
      await tester.pumpWidget(app());
      await tester.pumpAndSettle();
      expect(boots, 0);
      expect(
        tester
            .widget<FilledButton>(find.byKey(const ValueKey('region-continue')))
            .onPressed,
        isNull,
      );
      expect(
        tester
            .widget<ListTile>(find.byKey(const ValueKey('region-eu')))
            .enabled,
        isFalse,
      );
      await tester.tap(find.byType(DropdownButton<String>));
      await tester.pumpAndSettle();
      await tester.tap(find.text('中文').last);
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('region-us')));
      await tester.pump();
      expect(boots, 0);
      await tester.tap(find.byKey(const ValueKey('region-continue')));
      await tester.pumpAndSettle();
      expect(boots, 0);
      expect(find.text('Privacy Policy'), findsOneWidget);
      expect(
        tester
            .widget<FilledButton>(find.byKey(const ValueKey('legal-accept')))
            .onPressed,
        isNull,
      );
      await tester.tap(find.byKey(const ValueKey('legal-checkbox')));
      await tester.pump();
      await tester.tap(find.byKey(const ValueKey('legal-decline')));
      await tester.pump();
      expect(boots, 0);
      expect(
        tester
            .widget<CheckboxListTile>(
              find.byKey(const ValueKey('legal-checkbox')),
            )
            .value,
        isFalse,
      );
      await tester.tap(find.byKey(const ValueKey('legal-checkbox')));
      await tester.pump();
      await tester.tap(find.byKey(const ValueKey('legal-accept')));
      await tester.pumpAndSettle();
      expect(boots, 1);
      expect(languageChoice, 'zh-Hans');
      expect(find.text('business app'), findsOneWidget);
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pumpWidget(app());
      await tester.pumpAndSettle();
      expect(boots, 2);
      expect(find.byKey(const ValueKey('region-us')), findsNothing);
    },
  );
  testWidgets(
    'failed consent persistence keeps services stopped and offers a retry',
    (tester) async {
      var boots = 0;
      final store = _ConsentFailureStore();
      await tester.pumpWidget(
        OpenMuseRegionBootstrap(
          loadRelease: () async => release('overseas'),
          store: (_) async => store,
          boot: (_, _) async {
            boots++;
            return const SizedBox();
          },
        ),
      );
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('region-us')));
      await tester.pump();
      await tester.tap(find.byKey(const ValueKey('region-continue')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('legal-checkbox')));
      await tester.pump();
      await tester.tap(find.byKey(const ValueKey('legal-accept')));
      await tester.pumpAndSettle();
      expect(boots, 0);
      expect(
        find.text(
          'Unable to continue. Please check the documents and try again.',
        ),
        findsOneWidget,
      );
      expect(
        tester
            .widget<FilledButton>(find.byKey(const ValueKey('legal-accept')))
            .onPressed,
        isNotNull,
      );
      await tester.tap(find.byKey(const ValueKey('legal-change-region')));
      await tester.pumpAndSettle();
      expect(find.byKey(const ValueKey('region-us')), findsOneWidget);
      expect(boots, 0);
    },
  );
  testWidgets(
    'selection persistence failure cannot start account or network adapters',
    (tester) async {
      var boots = 0;
      await tester.pumpWidget(
        OpenMuseRegionBootstrap(
          loadRelease: () async => release('overseas'),
          store: (_) async => _BrokenStore(),
          boot: (_, _) async {
            boots++;
            return const SizedBox();
          },
        ),
      );
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('region-us')));
      await tester.pump();
      await tester.tap(find.byKey(const ValueKey('region-continue')));
      await tester.pumpAndSettle();
      expect(boots, 0);
      expect(find.byKey(const ValueKey('region-continue')), findsOneWidget);
    },
  );
}

final class _DelayedReadStore implements ConfigStore {
  final reads = <Completer<String?>>[];
  @override
  Future<String?> read() {
    final value = Completer<String?>();
    reads.add(value);
    return value.future;
  }

  @override
  Future<void> commit(String record) async =>
      throw StateError('read-only test store');
}

final class _ConsentFailureStore implements ConfigStore {
  String? value;
  int writes = 0;
  @override
  Future<String?> read() async => value;
  @override
  Future<void> commit(String record) async {
    if (++writes > 1) throw StateError('disk full');
    value = record;
  }
}

final class _BrokenStore implements ConfigStore {
  @override
  Future<String?> read() async => null;
  @override
  Future<void> commit(String record) async =>
      throw StateError('storage unavailable');
}
