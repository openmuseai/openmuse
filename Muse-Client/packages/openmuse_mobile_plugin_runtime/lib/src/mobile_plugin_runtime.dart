import 'dart:async';
import 'dart:convert';

import 'package:a2ui_core/a2ui_core.dart' as a2ui;
import 'package:fjs/fjs.dart';
import 'package:flutter/material.dart';
import 'package:genui/genui.dart';

/// Host owns capabilities. A plugin only receives the operations granted to it.
typedef MobilePluginCapability =
    Future<Object?> Function(
      String pluginId,
      String service,
      Object? arguments,
    );

/// One JS VM and one A2UI surface controller per installed plugin session.
final class MobilePluginSession {
  static Future<void>? _initializeFjs;

  MobilePluginSession({
    required this.pluginId,
    required this.source,
    required this.capability,
  }) : surface = SurfaceController(
         catalogs: [BasicCatalogItems.asNoAssetCatalog()],
       );

  final String pluginId;
  final String source;
  final MobilePluginCapability capability;
  final SurfaceController surface;
  final ValueNotifier<String?> lastError = ValueNotifier<String?>(null);
  JsEngine? _engine;
  StreamSubscription<ChatMessage>? _actions;
  bool _closed = false;
  Future<void> _pending = Future<void>.value();

  Future<void> start() async {
    if (_closed || _engine != null) throw StateError('invalid session state');
    await (_initializeFjs ??= LibFjs.init());
    final engine = await JsEngine.create(
      builtins: const JsBuiltinOptions(),
      runtimeOptions: JsEngineRuntimeOptions(
        info: pluginId,
        memoryLimit: BigInt.from(16 * 1024 * 1024),
        maxStackSize: BigInt.from(512 * 1024),
      ),
    );
    _engine = engine;
    await engine.init(
      bridge: (request) async {
        final value = request.value;
        if (value is! Map || value['service'] is! String) {
          return JsResult.err(const JsError.type('invalid Host request'));
        }
        try {
          final result = await capability(
            pluginId,
            value['service'] as String,
            value['arguments'],
          );
          return JsResult.ok(JsValue.from(result));
        } catch (error) {
          return JsResult.err(JsError.type('$error'));
        }
      },
    );
    await engine.eval(source: JsCode.code(source));
    _actions = surface.onSubmit.listen(_onSubmit);
    await _evaluate('JSON.stringify(globalThis.openMusePlugin.onStart())');
  }

  void _onSubmit(ChatMessage message) {
    for (final part in message.parts) {
      if (part is! DataPart ||
          part.mimeType != UiPartConstants.interactionMimeType) {
        continue;
      }
      final wire = jsonDecode(UiInteractionPart.fromDataPart(part).interaction);
      if (wire is! Map || wire['action'] is! Map) continue;
      final action = Map<String, Object?>.from(wire['action'] as Map);
      _pending = _pending
          .then(
            (_) => _evaluate(
              'JSON.stringify(globalThis.openMusePlugin.onAction(${jsonEncode(action)}))',
            ),
          )
          .catchError((Object error) {
            if (!_closed) lastError.value = '$error';
          });
    }
  }

  Future<void> _evaluate(String expression) async {
    if (_closed) return;
    final result = await _engine!.eval(source: JsCode.code(expression));
    final raw = result.value;
    if (raw is! String)
      throw const FormatException('plugin must return A2UI JSON');
    if (raw.length > 65536)
      throw const FormatException('A2UI update too large');
    final envelopes = jsonDecode(raw);
    if (envelopes is! List || envelopes.length > 32) {
      throw const FormatException('plugin must return <=32 A2UI envelopes');
    }
    for (final item in envelopes) {
      final envelope = Map<String, Object?>.from(item as Map);
      if (envelope['version'] != 'v0.9') {
        throw const FormatException('unsupported A2UI version');
      }
      if (envelope['createSurface'] case final Map create) {
        if (create['catalogId'] != basicCatalogId ||
            create['surfaceId'] != 'main') {
          throw const FormatException('unknown A2UI catalog or surface');
        }
      }
      if (envelope['updateComponents'] case final Map update) {
        final components = update['components'];
        if (update['surfaceId'] != 'main' ||
            components is! List ||
            components.length > 64) {
          throw const FormatException('invalid A2UI component update');
        }
        const permitted = {
          'Column',
          'Row',
          'Card',
          'Text',
          'Button',
          'TextField',
          'CheckBox',
          'Divider',
        };
        for (final component in components) {
          if (component is! Map ||
              !permitted.contains(component['component'])) {
            throw const FormatException('A2UI component not in Mobile profile');
          }
        }
      }
      surface.handleMessage(a2ui.A2uiMessage.fromJson(envelope));
    }
  }

  Future<void> close() async {
    if (_closed) return;
    _closed = true;
    await _actions?.cancel();
    await _engine?.close();
    surface.dispose();
    lastError.dispose();
  }
}

final class MobilePluginSurface extends StatelessWidget {
  const MobilePluginSurface({super.key, required this.session});
  final MobilePluginSession session;

  @override
  Widget build(BuildContext context) =>
      Surface(surfaceContext: session.surface.contextFor('main'));
}
