import 'dart:convert';
import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';

void main() {
  test(
    'runtime defaults and aliases update independently of explicit preference',
    () {
      final catalog = OpenMuseCatalog.builtIn;
      final locale = OpenMuseLocaleController(
        defaultTag: 'en',
        systemLocales: ['zh-MO'],
        aliases: {'zh-MO': 'zh-Hans'},
      );
      expect(locale.tag, 'zh-Hans');
      locale.applyConfiguration(
        catalog: catalog,
        defaultTag: 'en',
        enabledLocales: ['en', 'zh-Hans'],
      );
      expect(locale.tag, 'en');
      locale.selectTag('en');
      locale.applyConfiguration(
        catalog: catalog,
        defaultTag: 'zh-Hans',
        enabledLocales: ['en', 'zh-Hans'],
      );
      expect(locale.tag, 'en');
      locale.followSystem();
      expect(locale.tag, 'zh-Hans');
      locale.dispose();
    },
  );
  test(
    'slow preference restoration cannot override a newer selection or a closed window',
    () async {
      final pending = Completer<String?>();
      final locale = OpenMuseLocaleController(
        preferences: CallbackLocalePreferences(
          readValue: () => pending.future,
          writeValue: (_) async {},
        ),
      );
      final restored = locale.restore();
      locale.selectTag('en');
      pending.complete(
        jsonEncode({'version': 1, 'mode': 'selected', 'locale': 'zh-Hans'}),
      );
      await restored;
      expect(locale.tag, 'en');
      await locale.saved;
      locale.dispose();
      final closed = Completer<String?>();
      final disposed = OpenMuseLocaleController(
        preferences: CallbackLocalePreferences(
          readValue: () => closed.future,
          writeValue: (_) async {},
        ),
      );
      final finishing = disposed.restore();
      disposed.dispose();
      closed.complete(jsonEncode({'version': 1, 'mode': 'system'}));
      await finishing;
    },
  );
  test('reviewed catalog parses and plural rules follow locale', () {
    final catalog = OpenMuseCatalog.builtIn;
    expect(catalog.format('en', 'files.count', {'count': 0}), 'No files');
    expect(catalog.format('en', 'files.count', {'count': 1}), '1 file');
    expect(catalog.format('en', 'files.count', {'count': 2}), '2 files');
    expect(catalog.format('zh-Hans', 'files.count', {'count': 2}), '2 个文件');
  });
  test('nested select/plural and quoted braces never evaluate code', () {
    final template = MessageTemplate(
      "{role, select, admin {{count, plural, one {# file} other {# files}}} other {'{guest}'}} can't",
    );
    expect(
      template.format({'role': 'admin', 'count': 2}, locale: 'en'),
      "2 files can't",
    );
    expect(
      template.format({'role': 'guest', 'count': 0}, locale: 'en'),
      "{guest} can't",
    );
    expect(
      () => MessageTemplate('{n, plural, one {only}}'),
      throwsFormatException,
    );
    expect(
      () => MessageTemplate('{value, execute, other {x}}'),
      throwsFormatException,
    );
  });
  test(
    'remote overlay is immutable and rejects unknown IDs and changed arguments',
    () {
      final original = OpenMuseCatalog.builtIn;
      List<int> pack(Map<String, Object?> messages) => utf8.encode(
        jsonEncode({
          'schemaVersion': 1,
          'sourceRevision': OpenMuseCatalog.builtIn.sourceRevision,
          'locale': 'en',
          'namespace': 'common',
          'messages': messages,
        }),
      );
      final updated = original.overlay([
        pack({'common.save': 'Save now'}),
      ]);
      expect(updated.format('en', 'common.save'), 'Save now');
      expect(original.format('en', 'common.save'), 'Save');
      expect(
        () => original.overlay([
          pack({'common.unknown': 'x'}),
        ]),
        throwsFormatException,
      );
      expect(
        () => original.overlay([
          pack({'common.save': '{secret}'}),
        ]),
        throwsFormatException,
      );
    },
  );
  test(
    'controllers remain independent and preferences restore explicit choice',
    () async {
      final preferences = MemoryLocalePreferences();
      final first = OpenMuseLocaleController(
        initialTag: 'en',
        preferences: preferences,
      );
      final second = OpenMuseLocaleController(initialTag: 'zh-Hans');
      first.selectTag('zh');
      await first.saved;
      expect(first.tag, 'zh-Hans');
      second.selectTag('en');
      expect(first.tag, 'zh-Hans');
      final restored = OpenMuseLocaleController(preferences: preferences);
      await restored.restore();
      expect(restored.tag, 'zh-Hans');
      restored.followSystem();
      restored.updateSystemLocales(['en-GB']);
      expect(restored.tag, 'en');
      expect(restored.followsSystem, isTrue);
      expect(() => restored.selectTag('unknown'), throwsArgumentError);
    },
  );
  testWidgets(
    'language switching rebuilds a mounted surface without losing state',
    (tester) async {
      final controller = OpenMuseLocaleController(initialTag: 'en');
      await tester.pumpWidget(
        OpenMuseLocaleScope(
          controller: controller,
          child: MaterialApp(
            home: Builder(
              builder: (context) => Text(stringsOf(context).commonSave()),
            ),
          ),
        ),
      );
      expect(find.text('Save'), findsOneWidget);
      controller.selectTag('zh-Hans');
      await tester.pump();
      expect(find.text('保存'), findsOneWidget);
      controller.replaceCatalog(
        controller.catalog.overlay([
          utf8.encode(
            jsonEncode({
              'schemaVersion': 1,
              'sourceRevision': OpenMuseCatalog.builtIn.sourceRevision,
              'locale': 'zh-Hans',
              'namespace': 'common',
              'messages': {'common.save': '立即保存'},
            }),
          ),
        ]),
      );
      await tester.pump();
      expect(find.text('立即保存'), findsOneWidget);
    },
  );
}
