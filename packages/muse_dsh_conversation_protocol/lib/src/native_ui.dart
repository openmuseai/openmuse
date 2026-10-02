import 'models.dart';

enum DshNativeUiMode { native, generic, web }

final class DshNativeCapabilities {
  const DshNativeCapabilities({
    this.nativeUiApi = 1,
    required this.components,
    required this.slots,
  });

  factory DshNativeCapabilities.standard() => const DshNativeCapabilities(
    components: {
      'text@1',
      'code@1',
      'badge@1',
      'status@1',
      'progress@1',
      'keyValue@1',
      'disclosure@1',
      'toolCard@1',
      'column@1',
      'row@1',
      'section@1',
    },
    slots: {'tool.call.toolview'},
  );

  final int nativeUiApi;
  final Set<String> components;
  final Set<String> slots;

  JsonMap toJson() => {
    'nativeUiApi': nativeUiApi,
    'components': components.toList(growable: false)..sort(),
    'slots': slots.toList(growable: false)..sort(),
  };
}

final class DshNativeContribution {
  const DshNativeContribution({
    required this.slot,
    required this.key,
    required this.template,
    required this.body,
    required this.actions,
    required this.raw,
  });

  final String slot;
  final String key;
  final String template;
  final List<JsonMap> body;
  final List<JsonMap> actions;
  final JsonMap raw;

  factory DshNativeContribution.fromJson(JsonMap json) {
    final body = json['body'];
    final actions = json['actions'];
    return DshNativeContribution(
      slot: _requiredString(json, 'slot'),
      key: _requiredString(json, 'key'),
      template: _requiredString(json, 'template'),
      body: _objectList(body),
      actions: _objectList(actions),
      raw: Map.unmodifiable(json),
    );
  }
}

final class DshNativeNegotiation {
  const DshNativeNegotiation({
    required this.schemaVersion,
    required this.mode,
    required this.contributions,
    required this.plugins,
  });

  final int schemaVersion;
  final DshNativeUiMode mode;
  final List<DshNativeContribution> contributions;
  final List<JsonMap> plugins;

  bool get requiresWebFallback => mode == DshNativeUiMode.web;

  factory DshNativeNegotiation.fromJson(JsonMap json) {
    final version = json['schemaVersion'];
    if (version != 1) {
      throw FormatException('unsupported native UI schema: $version');
    }
    final mode = switch (json['mode']) {
      'native' => DshNativeUiMode.native,
      'generic' => DshNativeUiMode.generic,
      'web' => DshNativeUiMode.web,
      final value => throw FormatException('invalid native UI mode: $value'),
    };
    return DshNativeNegotiation(
      schemaVersion: version as int,
      mode: mode,
      contributions: _objectList(
        json['contributions'],
      ).map(DshNativeContribution.fromJson).toList(growable: false),
      plugins: _objectList(json['plugins']),
    );
  }
}

List<JsonMap> _objectList(Object? value) {
  if (value == null) return const [];
  if (value is! List || value.any((item) => item is! Map)) {
    throw const FormatException('native UI collection must contain objects');
  }
  return value
      .map((item) => (item as Map).cast<String, Object?>())
      .toList(growable: false);
}

String _requiredString(JsonMap json, String key) {
  final value = json[key];
  if (value is! String || value.isEmpty) {
    throw FormatException('$key must be a non-empty string');
  }
  return value;
}
