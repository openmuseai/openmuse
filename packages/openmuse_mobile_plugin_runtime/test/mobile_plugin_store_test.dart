import 'dart:io';

import 'dart:convert';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';

void main() {
  test('install, disable and restore only verified Mobile bundle', () async {
    final directory = await Directory.systemTemp.createTemp('openmuse-mobile-store-');
    addTearDown(() => directory.delete(recursive: true));
    final entries = (jsonDecode(await File('assets/market.json').readAsString()) as List)
        .map((item) => MobileMarketEntry.fromJson(Map<String, Object?>.from(item as Map))).toList();
    final entry = entries.first;
    final bytes = await File('assets/${entry.bundle}').readAsBytes();
    final store = MobilePluginStore(directory);
    await expectLater(store.install(entry, [...bytes, 0]), throwsFormatException);
    expect(store.isInstalled(entry.id), isFalse);
    await store.install(entry, bytes);
    expect(store.isEnabled(entry.id), isTrue);
    await store.setEnabled(entry.id, false);
    final restored = MobilePluginStore(directory);
    await restored.restore();
    expect(restored.isInstalled(entry.id), isTrue);
    expect(restored.isEnabled(entry.id), isFalse);
  });
}
