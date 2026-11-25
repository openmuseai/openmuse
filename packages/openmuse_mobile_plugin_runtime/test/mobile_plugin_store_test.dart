import 'dart:io';

import 'dart:convert';
import 'package:crypto/crypto.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';

void main() {
  test(
    'oversized legacy metadata stays unchanged and orphan directories remain visible',
    () async {
      final directory = await Directory.systemTemp.createTemp('legacy-budget-');
      addTearDown(() => directory.delete(recursive: true));
      final state = File('${directory.path}/state.json');
      final bytes = List<int>.filled(1024 * 1024 + 1, 65);
      await state.writeAsBytes(bytes);
      await Directory('${directory.path}/com.example.retained').create();
      final store = MobilePluginStore(directory, allowDownloadedPlugins: false);
      await store.restore();
      expect(store.hasUnreadableLegacyState, isTrue);
      expect(store.quarantinedPluginIds, contains('com.example.retained'));
      expect(store.installedBundles, isEmpty);
      expect(await state.readAsBytes(), bytes);
    },
  );
  test(
    'bundled release preserves but never restores downloaded software',
    () async {
      final directory = await Directory.systemTemp.createTemp(
        'mobile-bundled-only-',
      );
      addTearDown(() => directory.delete(recursive: true));
      final entry = MobileMarketEntry.fromJson(
        Map<String, Object?>.from(
          (jsonDecode(await File('assets/market.json').readAsString()) as List)
                  .first
              as Map,
        ),
      );
      final bytes = await File('assets/${entry.bundle}').readAsBytes();
      final development = MobilePluginStore(directory);
      await development.install(entry, bytes);
      final state = File('${directory.path}/state.json');
      final previous = await state.readAsString();
      final release = MobilePluginStore(
        directory,
        allowDownloadedPlugins: false,
      );
      await release.restore();
      expect(release.installedBundles, isEmpty);
      expect(release.quarantinedPluginIds, contains(entry.id));
      final artifact = File(
        '${directory.path}/${entry.id}/${entry.version}.json',
      );
      expect(await artifact.readAsBytes(), bytes);
      await expectLater(release.install(entry, bytes), throwsStateError);
      expect(await state.readAsString(), previous);
      expect(await artifact.readAsBytes(), bytes);
      final restored = MobilePluginStore(directory);
      await restored.restore();
      expect(restored.isInstalled(entry.id), isTrue);
    },
  );

  test(
    'legacy records and orphan directories stay quarantined across restores',
    () async {
      final directory = await Directory.systemTemp.createTemp('mobile-legacy-');
      addTearDown(() => directory.delete(recursive: true));
      final plugin = Directory('${directory.path}/com.openmuse.pptx');
      await plugin.create();
      final orphan = Directory('${directory.path}/com.example.orphan');
      await orphan.create();
      final files = <File, List<int>>{
        File('${directory.path}/state.json'): utf8.encode(
          jsonEncode({
            'installed': {
              'com.openmuse.pptx': {
                'version': 'legacy',
                'entrypoint': 'index.html',
              },
              'com.example.missing': 'old-record',
              '../outside': {'version': '1.0.0'},
            },
            'disabled': ['com.openmuse.pptx'],
            'unknownLegacyField': true,
          }),
        ),
        File('${plugin.path}/index.html'): utf8.encode(
          '<script>legacy()</script>',
        ),
        File('${plugin.path}/user-deck.pptx'): [0, 1, 255, 42],
        File('${directory.path}/user-notes.txt'): utf8.encode('用户内容'),
        File('${orphan.path}/document.json'): utf8.encode('{"user":"data"}'),
      };
      for (final file in files.entries) {
        await file.key.writeAsBytes(file.value);
      }
      final store = MobilePluginStore(directory, allowDownloadedPlugins: false);
      var notifications = 0;
      store.addListener(() => notifications++);
      for (var attempt = 0; attempt < 2; attempt++) {
        await store.restore();
        expect(store.quarantinedPluginIds, {
          'com.openmuse.pptx',
          'com.example.missing',
          'com.example.orphan',
        });
        expect(store.hasUnreadableLegacyState, isFalse);
        expect(store.installedBundles, isEmpty);
        expect(store.installed('com.openmuse.pptx'), isNull);
        expect(store.isEnabled('com.openmuse.pptx'), isFalse);
        await expectLater(
          store.setEnabled('com.openmuse.pptx', true),
          throwsStateError,
        );
        await store.uninstall('com.openmuse.pptx');
        for (final file in files.entries) {
          expect(await file.key.readAsBytes(), file.value);
        }
      }
      expect(notifications, 2);
      expect(await orphan.exists(), isTrue);
      expect(await plugin.exists(), isTrue);
    },
  );

  test('damaged and old state formats do not block bundled startup', () async {
    final directory = await Directory.systemTemp.createTemp(
      'mobile-old-state-',
    );
    addTearDown(() => directory.delete(recursive: true));
    final state = File('${directory.path}/state.json');
    final store = MobilePluginStore(directory, allowDownloadedPlugins: false);
    for (final raw in ['{broken', '[]', '{"installed": []}']) {
      await state.writeAsString(raw);
      await store.restore();
      await store.restore();
      expect(store.hasUnreadableLegacyState, isTrue);
      expect(store.installedBundles, isEmpty);
      expect(await state.readAsString(), raw);
    }
  });

  test('install, disable and restore only verified Mobile bundle', () async {
    final directory = await Directory.systemTemp.createTemp(
      'openmuse-mobile-store-',
    );
    addTearDown(() => directory.delete(recursive: true));
    final entries =
        (jsonDecode(await File('assets/market.json').readAsString()) as List)
            .map(
              (item) => MobileMarketEntry.fromJson(
                Map<String, Object?>.from(item as Map),
              ),
            )
            .toList();
    final entry = entries.first;
    final bytes = await File('assets/${entry.bundle}').readAsBytes();
    var allowed = false;
    final store = MobilePluginStore(directory, allowInstall: () => allowed);
    await expectLater(store.install(entry, bytes), throwsStateError);
    expect(store.isInstalled(entry.id), isFalse);
    allowed = true;
    await expectLater(
      store.install(entry, [...bytes, 0]),
      throwsFormatException,
    );
    expect(store.isInstalled(entry.id), isFalse);
    await store.install(entry, bytes);
    expect(store.isEnabled(entry.id), isTrue);
    await store.setEnabled(entry.id, false);
    final restored = MobilePluginStore(directory);
    await restored.restore();
    expect(restored.isInstalled(entry.id), isTrue);
    expect(restored.isEnabled(entry.id), isFalse);
  });

  test(
    'WebView bundle installs with a validated A2UI workbench card',
    () async {
      final directory = await Directory.systemTemp.createTemp(
        'openmuse-web-store-',
      );
      addTearDown(() => directory.delete(recursive: true));
      final entries =
          (jsonDecode(await File('assets/market.json').readAsString()) as List)
              .map(
                (item) => MobileMarketEntry.fromJson(
                  Map<String, Object?>.from(item as Map),
                ),
              )
              .toList();
      final entry = entries.singleWhere(
        (item) => item.bundle == 'task-board.json',
      );
      final bytes = await File('assets/${entry.bundle}').readAsBytes();
      final store = MobilePluginStore(directory);
      await store.install(entry, bytes);
      final bundle = store.installed(entry.id)!;
      expect(bundle.isWebView, isTrue);
      expect(bundle.card!.title, '任务看板');
      expect(bundle.preview, hasLength(2));
      expect(bundle.manifest.requestedPermissions, isEmpty);
      await store.setEnabled(entry.id, false);
      final restored = MobilePluginStore(directory);
      await restored.restore();
      expect(restored.installed(entry.id)!.html, bundle.html);
      expect(restored.isEnabled(entry.id), isFalse);
      await restored.uninstall(entry.id);
      expect(restored.isInstalled(entry.id), isFalse);
      final afterRemoval = MobilePluginStore(directory);
      await afterRemoval.restore();
      expect(afterRemoval.isInstalled(entry.id), isFalse);
    },
  );

  test(
    'verified WebView update replaces version and re-enables the plugin',
    () async {
      final directory = await Directory.systemTemp.createTemp(
        'openmuse-web-update-',
      );
      addTearDown(() => directory.delete(recursive: true));
      final entry =
          (jsonDecode(await File('assets/market.json').readAsString()) as List)
              .map(
                (item) => MobileMarketEntry.fromJson(
                  Map<String, Object?>.from(item as Map),
                ),
              )
              .singleWhere((item) => item.bundle == 'usage-budget.json');
      final store = MobilePluginStore(directory);
      await store.install(
        entry,
        await File('assets/${entry.bundle}').readAsBytes(),
      );
      await store.setEnabled(entry.id, false);
      final updated =
          jsonDecode(await File('assets/${entry.bundle}').readAsString())
              as Map;
      updated['version'] = '0.3.0';
      (updated['manifest'] as Map)['version'] = '0.3.0';
      final bytes = utf8.encode(jsonEncode(updated));
      final newEntry = MobileMarketEntry(
        id: entry.id,
        name: entry.name,
        version: '0.3.0',
        description: entry.description,
        author: entry.author,
        platforms: entry.platforms,
        bundle: entry.bundle,
        sha256: sha256.convert(bytes).toString(),
        runtime: 'webview',
        permissions: const [],
      );
      await store.install(newEntry, bytes);
      expect(store.installed(entry.id)!.version, '0.3.0');
      expect(store.isEnabled(entry.id), isTrue);
      final restored = MobilePluginStore(directory);
      await restored.restore();
      expect(restored.installed(entry.id)!.version, '0.3.0');
    },
  );

  test(
    'WebView install rejects undeclared permissions and preview components',
    () async {
      final directory = await Directory.systemTemp.createTemp(
        'openmuse-web-reject-',
      );
      addTearDown(() => directory.delete(recursive: true));
      final entries =
          (jsonDecode(await File('assets/market.json').readAsString()) as List)
              .map(
                (item) => MobileMarketEntry.fromJson(
                  Map<String, Object?>.from(item as Map),
                ),
              )
              .toList();
      final original = entries.singleWhere(
        (item) => item.bundle == 'task-board.json',
      );
      final decoded =
          jsonDecode(await File('assets/${original.bundle}').readAsString())
              as Map;
      Future<void> rejects(Map value) async {
        final bytes = utf8.encode(jsonEncode(value));
        final entry = MobileMarketEntry(
          id: original.id,
          name: original.name,
          version: original.version,
          description: original.description,
          author: original.author,
          platforms: original.platforms,
          bundle: original.bundle,
          sha256: sha256.convert(bytes).toString(),
        );
        await expectLater(
          MobilePluginStore(directory).install(entry, bytes),
          throwsFormatException,
        );
      }

      final permissions = jsonDecode(jsonEncode(decoded)) as Map;
      (permissions['manifest'] as Map)['requested_permissions'] = [
        'workspace.paired.read',
      ];
      await rejects(permissions);
      final entrypoint = jsonDecode(jsonEncode(decoded)) as Map;
      ((entrypoint['manifest'] as Map)['ui_runtime'] as Map)['entrypoint'] =
          'other.html';
      await rejects(entrypoint);
      final preview = jsonDecode(jsonEncode(decoded)) as Map;
      (((preview['preview'] as List)[1] as Map)['updateComponents']
              as Map)['components'][0]['component'] =
          'Button';
      await rejects(preview);
    },
  );
}
