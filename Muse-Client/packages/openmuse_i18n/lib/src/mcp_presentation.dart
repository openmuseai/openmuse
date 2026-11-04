import 'dart:async';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'client_runtime.dart';
import 'controller.dart';

/// Public presentation data only. No service addresses, tokens, user identity,
/// documents, or signing roots are sent into the sandboxed MCP Apps View.
Map<String, Object?> mcpAppPresentation(BuildContext context) {
  final runtime = OpenMuseRuntimeHost.maybeOf(context);
  final config = RuntimeConfigScope.maybeOf(context);
  final locale =
      OpenMuseLocaleScope.maybeOf(context)?.tag ??
      Localizations.maybeLocaleOf(context)?.toLanguageTag();
  if (runtime != null && locale != null) {
    unawaited(
      runtime.preparePluginMessages(locale).catchError((Object _) {
        /* Keep the View's bundled/last verified language. */
      }),
    );
  }
  return {
    'locale': ?locale,
    'theme': Theme.of(context).brightness.name,
    if (config != null)
      'openmuse': {
        'schemaVersion': 1,
        'releaseId': config.releaseId,
        'settings': config.json['settings'] ?? <String, Object?>{},
        'localeCatalog':
            (config.json['localization'] as Map)['localeCatalog'] ?? const [],
        'languagePacks': [
          if (runtime != null)
            for (final resource
                in (runtime.client.current.json['resources'] as List? ??
                    const []))
              if (resource['kind'] == 'pluginMessages' &&
                  runtime.client.resources.containsKey(resource['id']))
                jsonDecode(
                  utf8.decode(runtime.client.resources[resource['id']]!),
                ),
        ],
      },
  };
}
