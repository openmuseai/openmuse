import 'dart:io';

import 'dart:convert';
import 'package:crypto/crypto.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';

void main() {
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
      await expectLater(release.install(entry, bytes), throwsStateError);
      expect(await state.readAsString(), previous);
      final restored = MobilePluginStore(directory);
      await restored.restore();
      expect(restored.isInstalled(entry.id), isTrue);
    },
  );

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
