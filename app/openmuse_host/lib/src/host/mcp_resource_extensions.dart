import 'dart:convert';

import 'package:mcp_dart/mcp_dart.dart';
import 'package:flutter/foundation.dart';

import 'mcp_market.dart';
export 'package:openmuse_plugin_sdk/src/resource_extension_session.dart';

/// Public Resource Extensions profile; no document or Agent business schema.
final class McpResourceExtension {
  const McpResourceExtension(
    this.id,
    this.html,
    this.width,
    this.height,
    this.isCurrent,
    this.tools,
    this.invoke,
    this.resourceTools,
  );
  final Set<String> resourceTools;
  final List<Map<String, Object?>> tools;
  final Future<Map<String, Object?>> Function(String, Map<String, Object?>)
  invoke;
  final String id, html;
  final int width, height;
  final Future<bool> Function() isCurrent;
}

final class McpResourceExtensions extends ChangeNotifier {
  McpResourceExtensions({required this.market, required this.nodeExecutable});
  final McpRegistryMarket market;
  final String nodeExecutable;
  Future<List<({McpResourceExtension extension, Set<String> types})>>? _cache;
  final _development =
      <
        String,
        ({
          McpMarketEntry entry,
          McpBundleRuntime runtime,
          Future<bool> Function() valid,
        })
      >{};
  void invalidate() {
    _cache = null;
    notifyListeners();
  }

  Future<bool> addDevelopment(
    String caller,
    McpMarketEntry entry,
    McpBundleRuntime runtime,
    Future<bool> Function() valid,
  ) async {
    final session = await runtime.connect();
    try {
      if (!(await session.client.listResources()).resources.any(
        (r) =>
            r.mimeType == 'application/vnd.openmuse.resource-extensions+json',
      )) {
        return false;
      }
    } finally {
      await session.close();
    }
    _development[caller] = (entry: entry, runtime: runtime, valid: valid);
    invalidate();
    return true;
  }

  void removeDevelopment(String caller) {
    if (_development.remove(caller) != null) invalidate();
  }

  Future<List<McpResourceExtension>> forType(String type) async => [
    for (final entry in await (_cache ??= _discover()))
      if (entry.types.contains(type)) entry.extension,
  ];
  Future<List<({McpResourceExtension extension, Set<String> types})>>
  _discover() async {
    final found = <({McpResourceExtension extension, Set<String> types})>[];
    final sources = [
      for (final entry in await market.installedEntries())
        (
          entry: entry,
          runtime: McpBundleRuntime(
            bundleDirectory: market.directoryFor(entry),
            nodeExecutable: nodeExecutable,
          ),
          valid: () async => (await market.installedEntries()).any(
            (e) => e.name == entry.name && e.sha256 == entry.sha256,
          ),
        ),
      ..._development.values,
    ];
    for (final source in sources) {
      final entry = source.entry, runtime = source.runtime;
      McpBundleSession? session;
      try {
        session = await runtime.connect();
        final resources = await session.client.listResources();
        for (final resource in resources.resources.where(
          (r) =>
              r.mimeType == 'application/vnd.openmuse.resource-extensions+json',
        )) {
          final read = await session.client.readResource(
            ReadResourceRequest(uri: resource.uri),
          );
          final catalog =
              jsonDecode(
                    read.contents.whereType<TextResourceContents>().single.text,
                  )
                  as Map;
          if (catalog['profile'] != 'openmuse-resource-extensions/1') continue;
          final items = catalog['extensions'] as List;
          if (items.length > 4) throw const FormatException('Extension budget');
          for (final item in items.whereType<Map>()) {
            final id = item['id'], tool = item['tool'];
            if (id is! String ||
                !RegExp(r'^[a-zA-Z0-9._-]{1,80}$').hasMatch(id) ||
                tool is! String) {
              throw const FormatException('Invalid extension');
            }
            final width = item['width'], height = item['height'];
            if (width is! int ||
                width < 200 ||
                width > 480 ||
                height is! int ||
                height < 48 ||
                height > 180) {
              throw const FormatException('Invalid extension bounds');
            }
            final types = (item['mediaTypes'] as List).cast<String>().toSet();
            final html = await runtime.uiDocument(tool);
            if (html == null || utf8.encode(html).length > 4 * 1024 * 1024) {
              continue;
            }
            found.add((
              extension: McpResourceExtension(
                '${entry.name}:$id',
                html,
                width,
                height,
                source.valid,
                (await session.client.listTools()).tools
                    .map((t) => t.toJson())
                    .toList(),
                runtime.callTool,
                (item['resourceTools'] as List? ?? []).cast<String>().toSet(),
              ),
              types: types,
            ));
          }
        }
      } catch (_) {
        // An incompatible optional contribution must not prevent file opening.
      } finally {
        await session?.close();
      }
    }
    return found.take(4).toList();
  }
}
