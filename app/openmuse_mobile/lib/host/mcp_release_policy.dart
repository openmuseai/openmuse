import 'dart:convert';
import 'bundled_mcp_packages.dart';

/// Internal full-client builds may add exact candidate pins. Store builds
/// continue to use their fixed release closure and ignore these overrides.
({Map<String, String> packages, Map<String, String> remotes}) mcpReviewPolicy({
  required bool debugBuild,
  String packagesJson = '{}',
  String remotesJson = '{}',
}) {
  if (!debugBuild) return (packages: bundledMcpPackagePins, remotes: const {});
  Map<String, String> parse(String raw, bool remote) {
    final value = jsonDecode(raw);
    if (value is! Map) throw const FormatException('Invalid MCP pins');
    final pins = <String, String>{};
    for (final entry in value.entries) {
      if (entry.key is! String ||
          entry.value is! String ||
          !(entry.key as String).contains('@')) {
        throw const FormatException('Invalid MCP pin identity');
      }
      final target = entry.value as String;
      if (remote) {
        final uri = Uri.tryParse(target);
        if (uri == null ||
            uri.userInfo.isNotEmpty ||
            uri.hasFragment ||
            !(uri.scheme == 'https' ||
                uri.scheme == 'http' &&
                    {'127.0.0.1', 'localhost'}.contains(uri.host))) {
          throw const FormatException('Invalid trusted MCP endpoint');
        }
      } else if (!RegExp(r'^[a-f0-9]{64}$').hasMatch(target)) {
        throw const FormatException('Invalid MCP package digest');
      }
      pins[entry.key as String] = target;
    }
    return Map.unmodifiable(pins);
  }

  return (
    packages: Map.unmodifiable({
      ...bundledMcpPackagePins,
      ...parse(packagesJson, false),
    }),
    remotes: parse(remotesJson, true),
  );
}
