import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_registry_provider/openmuse_registry_provider.dart';

void main() {
  test(
    'persists a selected HTTPS provider and rejects unsafe origins',
    () async {
      final directory = await Directory.systemTemp.createTemp('mcp-providers-');
      addTearDown(() => directory.delete(recursive: true));
      final file = File('${directory.path}/providers.json');
      final first = McpRegistryProviders(
        file: file,
        defaultOrigin: Uri.parse('https://plugins.openmuseai.com'),
      );
      await first.load();
      expect(first.selected.toString(), 'https://plugins.openmuseai.com');
      await first.select('https://alternate.example/');
      expect(first.selected.toString(), 'https://alternate.example');
      final restored = McpRegistryProviders(
        file: file,
        defaultOrigin: Uri.parse('https://plugins.openmuseai.com'),
      );
      await restored.load();
      expect(restored.selected.toString(), 'https://alternate.example');
      expect(
        restored.origins,
        contains(Uri.parse('https://plugins.openmuseai.com')),
      );
      expect(
        () => McpRegistryProviders.validate('http://example.com'),
        throwsFormatException,
      );
      expect(
        () => McpRegistryProviders.validate('https://user@example.com'),
        throwsFormatException,
      );
      expect(
        () => McpRegistryProviders.validate('https://example.com/other'),
        throwsFormatException,
      );
    },
  );
}
