enum OpenMuseUiRuntimeKind { none, flutter, webView }

enum OpenMuseExecutionConnectorKind {
  none,
  hostProcess,
  remoteDsh,
  sandboxProvider,
}

enum OpenMuseTargetOs { macos, windows, linux, android, ios, web }

enum OpenMuseTargetArch { aarch64, x86_64, wasm32 }

enum OpenMuseTargetLibc { darwin, msvc, gnu, musl, bionic, none }

enum OpenMuseTargetStatus { supported, unsupported }

enum OpenMuseArtifactKind {
  hostBundle,
  webBundle,
  nativeExecutable,
  runtimeClosure,
  sandboxWorker,
}

enum OpenMusePresentationSurface {
  editor,
  leftSidebar,
  rightSidebar,
  bottomPanel,
}

final class OpenMuseManifestFormatException implements FormatException {
  const OpenMuseManifestFormatException(this.message);

  @override
  final String message;

  @override
  Object? get source => null;

  @override
  int? get offset => null;

  @override
  String toString() => 'OpenMuseManifestFormatException: $message';
}

final class OpenMuseManifestProtocol {
  const OpenMuseManifestProtocol({required this.major, required this.minor});

  final int major;
  final int minor;

  factory OpenMuseManifestProtocol.fromJson(Object? value) {
    final map = _object(value, 'protocol');
    _keys(map, required: const {'major', 'minor'});
    final protocol = OpenMuseManifestProtocol(
      major: _integer(map, 'major', min: 1),
      minor: _integer(map, 'minor'),
    );
    if (protocol.major != 1 || protocol.minor > 0) {
      throw OpenMuseManifestFormatException(
        'incompatible plugin protocol ${protocol.major}.${protocol.minor}',
      );
    }
    return protocol;
  }

  Map<String, Object?> toJson() => {'major': major, 'minor': minor};
}

final class OpenMuseUiRuntimeV2 {
  const OpenMuseUiRuntimeV2({required this.kind, this.entrypoint});

  final OpenMuseUiRuntimeKind kind;
  final String? entrypoint;

  factory OpenMuseUiRuntimeV2.fromJson(Object? value) {
    final map = _object(value, 'ui_runtime');
    _keys(map, required: const {'kind'}, optional: const {'entrypoint'});
    final kind = _wireEnum(
      map,
      'kind',
      OpenMuseUiRuntimeKind.values,
      _uiRuntimeWire,
    );
    final entrypoint = _optionalString(map, 'entrypoint');
    if (kind == OpenMuseUiRuntimeKind.none && entrypoint != null) {
      throw const OpenMuseManifestFormatException(
        'none UI runtime cannot declare an entrypoint',
      );
    }
    return OpenMuseUiRuntimeV2(kind: kind, entrypoint: entrypoint);
  }

  Map<String, Object?> toJson() => {
    'kind': _uiRuntimeWire(kind),
    if (entrypoint != null) 'entrypoint': entrypoint,
  };
}

final class OpenMuseExecutionConnectorV2 {
  const OpenMuseExecutionConnectorV2({required this.kind, this.protocol});

  final OpenMuseExecutionConnectorKind kind;
  final String? protocol;

  factory OpenMuseExecutionConnectorV2.fromJson(Object? value) {
    final map = _object(value, 'execution_connector');
    _keys(map, required: const {'kind'}, optional: const {'protocol'});
    final kind = _wireEnum(
      map,
      'kind',
      OpenMuseExecutionConnectorKind.values,
      _executionConnectorWire,
    );
    final protocol = _optionalString(map, 'protocol');
    if (kind == OpenMuseExecutionConnectorKind.none && protocol != null) {
      throw const OpenMuseManifestFormatException(
        'none execution connector cannot declare a protocol',
      );
    }
    return OpenMuseExecutionConnectorV2(kind: kind, protocol: protocol);
  }

  Map<String, Object?> toJson() => {
    'kind': _executionConnectorWire(kind),
    if (protocol != null) 'protocol': protocol,
  };
}

final class OpenMuseTarget {
  const OpenMuseTarget({
    required this.os,
    required this.arch,
    required this.libc,
  });

  final OpenMuseTargetOs os;
  final OpenMuseTargetArch arch;
  final OpenMuseTargetLibc libc;

  factory OpenMuseTarget.fromJson(Object? value) {
    final map = _object(value, 'target');
    _keys(map, required: const {'os', 'arch', 'libc'});
    final target = OpenMuseTarget(
      os: _wireEnum(map, 'os', OpenMuseTargetOs.values, _targetOsWire),
      arch: _wireEnum(map, 'arch', OpenMuseTargetArch.values, _targetArchWire),
      libc: _wireEnum(map, 'libc', OpenMuseTargetLibc.values, _targetLibcWire),
    );
    target._validate();
    return target;
  }

  void _validate() {
    final valid = switch (os) {
      OpenMuseTargetOs.macos ||
      OpenMuseTargetOs.ios => libc == OpenMuseTargetLibc.darwin,
      OpenMuseTargetOs.windows => libc == OpenMuseTargetLibc.msvc,
      OpenMuseTargetOs.android => libc == OpenMuseTargetLibc.bionic,
      OpenMuseTargetOs.linux =>
        libc == OpenMuseTargetLibc.gnu || libc == OpenMuseTargetLibc.musl,
      OpenMuseTargetOs.web =>
        arch == OpenMuseTargetArch.wasm32 && libc == OpenMuseTargetLibc.none,
    };
    if (!valid) {
      throw const OpenMuseManifestFormatException('invalid target triple');
    }
  }

  Map<String, Object?> toJson() => {
    'os': _targetOsWire(os),
    'arch': _targetArchWire(arch),
    'libc': _targetLibcWire(libc),
  };

  @override
  bool operator ==(Object other) =>
      other is OpenMuseTarget &&
      os == other.os &&
      arch == other.arch &&
      libc == other.libc;

  @override
  int get hashCode => Object.hash(os, arch, libc);

  @override
  String toString() =>
      '${_targetOsWire(os)}/${_targetArchWire(arch)}/${_targetLibcWire(libc)}';
}

final class OpenMuseTargetDecision {
  const OpenMuseTargetDecision({
    required this.target,
    required this.status,
    this.reason,
  });

  final OpenMuseTarget target;
  final OpenMuseTargetStatus status;
  final String? reason;

  factory OpenMuseTargetDecision.fromJson(Object? value) {
    final map = _object(value, 'target decision');
    _keys(
      map,
      required: const {'target', 'status'},
      optional: const {'reason'},
    );
    final status = _wireEnum(
      map,
      'status',
      OpenMuseTargetStatus.values,
      (value) => value.name,
    );
    final reason = _optionalString(map, 'reason');
    if (status == OpenMuseTargetStatus.unsupported && reason == null) {
      throw const OpenMuseManifestFormatException(
        'unsupported target requires a reason',
      );
    }
    if (status == OpenMuseTargetStatus.supported && reason != null) {
      throw const OpenMuseManifestFormatException(
        'supported target cannot carry a reason',
      );
    }
    return OpenMuseTargetDecision(
      target: OpenMuseTarget.fromJson(map['target']),
      status: status,
      reason: reason,
    );
  }

  Map<String, Object?> toJson() => {
    'target': target.toJson(),
    'status': status.name,
    if (reason != null) 'reason': reason,
  };
}

final class OpenMuseArtifactDigest {
  const OpenMuseArtifactDigest(this.value);

  final String value;

  factory OpenMuseArtifactDigest.fromJson(Object? value) {
    final map = _object(value, 'digest');
    _keys(map, required: const {'algorithm', 'value'});
    if (map['algorithm'] != 'sha256') {
      throw const OpenMuseManifestFormatException(
        'digest algorithm must be sha256',
      );
    }
    final digest = _string(map, 'value');
    if (!RegExp(r'^[0-9a-f]{64}$').hasMatch(digest)) {
      throw const OpenMuseManifestFormatException('invalid sha256 digest');
    }
    return OpenMuseArtifactDigest(digest);
  }

  Map<String, Object?> toJson() => {'algorithm': 'sha256', 'value': value};
}

final class OpenMuseArtifactSignature {
  const OpenMuseArtifactSignature({required this.keyId, required this.value});

  final String keyId;
  final String value;

  factory OpenMuseArtifactSignature.fromJson(Object? value) {
    final map = _object(value, 'signature');
    _keys(map, required: const {'algorithm', 'key_id', 'value'});
    if (map['algorithm'] != 'ed25519') {
      throw const OpenMuseManifestFormatException(
        'signature algorithm must be ed25519',
      );
    }
    return OpenMuseArtifactSignature(
      keyId: _string(map, 'key_id'),
      value: _string(map, 'value'),
    );
  }

  Map<String, Object?> toJson() => {
    'algorithm': 'ed25519',
    'key_id': keyId,
    'value': value,
  };
}

final class OpenMusePluginArtifactV2 {
  const OpenMusePluginArtifactV2({
    required this.id,
    required this.kind,
    required this.target,
    required this.digest,
    required this.license,
    required this.abi,
    this.signature,
  });

  final String id;
  final OpenMuseArtifactKind kind;
  final OpenMuseTarget target;
  final OpenMuseArtifactDigest digest;
  final OpenMuseArtifactSignature? signature;
  final String license;
  final String abi;

  factory OpenMusePluginArtifactV2.fromJson(Object? value) {
    final map = _object(value, 'artifact');
    _keys(
      map,
      required: const {'id', 'kind', 'target', 'digest', 'license', 'abi'},
      optional: const {'signature'},
    );
    return OpenMusePluginArtifactV2(
      id: _string(map, 'id'),
      kind: _wireEnum(
        map,
        'kind',
        OpenMuseArtifactKind.values,
        _artifactKindWire,
      ),
      target: OpenMuseTarget.fromJson(map['target']),
      digest: OpenMuseArtifactDigest.fromJson(map['digest']),
      signature: map.containsKey('signature')
          ? OpenMuseArtifactSignature.fromJson(map['signature'])
          : null,
      license: _string(map, 'license'),
      abi: _string(map, 'abi'),
    );
  }

  Map<String, Object?> toJson() => {
    'id': id,
    'kind': _artifactKindWire(kind),
    'target': target.toJson(),
    'digest': digest.toJson(),
    if (signature != null) 'signature': signature!.toJson(),
    'license': license,
    'abi': abi,
  };
}

final class OpenMusePresentationV2 {
  const OpenMusePresentationV2({
    required this.surfaces,
    required this.remoteCapable,
  });

  final Set<OpenMusePresentationSurface> surfaces;
  final bool remoteCapable;

  factory OpenMusePresentationV2.fromJson(Object? value) {
    final map = _object(value, 'presentation');
    _keys(map, required: const {'surfaces', 'remote_capable'});
    final surfaces = _enumList(
      map['surfaces'],
      'surfaces',
      OpenMusePresentationSurface.values,
      _presentationSurfaceWire,
    );
    final remoteCapable = _boolean(map, 'remote_capable');
    if (remoteCapable && surfaces.isEmpty) {
      throw const OpenMuseManifestFormatException(
        'remote presentation requires a surface',
      );
    }
    return OpenMusePresentationV2(
      surfaces: surfaces.toSet(),
      remoteCapable: remoteCapable,
    );
  }

  Map<String, Object?> toJson() => {
    'surfaces': surfaces.map(_presentationSurfaceWire).toList(),
    'remote_capable': remoteCapable,
  };
}

final class OpenMuseAgentCliContributionV2 {
  const OpenMuseAgentCliContributionV2({
    required this.group,
    required this.namespace,
    required this.command,
    required this.schema,
    required this.requiredPermissions,
    required this.effects,
  });

  final String group;
  final String namespace;
  final String command;
  final Map<String, Object?> schema;
  final Set<String> requiredPermissions;
  final Set<String> effects;

  String get identity => '$group/$namespace/$command';

  factory OpenMuseAgentCliContributionV2.fromJson(Object? value) {
    final map = _object(value, 'agent_cli');
    _keys(
      map,
      required: const {
        'group',
        'namespace',
        'command',
        'schema',
        'required_permissions',
        'effects',
      },
    );
    return OpenMuseAgentCliContributionV2(
      group: _string(map, 'group'),
      namespace: _string(map, 'namespace'),
      command: _string(map, 'command'),
      schema: _copyMap(_object(map['schema'], 'agent_cli.schema')),
      requiredPermissions: _stringSet(
        map['required_permissions'],
        'required_permissions',
      ),
      effects: _stringSet(map['effects'], 'effects'),
    );
  }

  Map<String, Object?> toJson() => {
    'group': group,
    'namespace': namespace,
    'command': command,
    'schema': _copyMap(schema),
    'required_permissions': requiredPermissions.toList(),
    'effects': effects.toList(),
  };
}

final class OpenMuseContributionsV2 {
  const OpenMuseContributionsV2({
    required this.commands,
    required this.services,
    required this.editors,
    required this.panels,
    required this.agentCli,
  });

  final List<Map<String, Object?>> commands;
  final List<Map<String, Object?>> services;
  final List<Map<String, Object?>> editors;
  final List<Map<String, Object?>> panels;
  final List<OpenMuseAgentCliContributionV2> agentCli;

  factory OpenMuseContributionsV2.fromJson(Object? value) {
    final map = _object(value, 'contributes');
    _keys(
      map,
      required: const {},
      optional: const {
        'commands',
        'services',
        'editors',
        'panels',
        'agent_cli',
      },
    );
    final commands = _contributions(
      map['commands'],
      'commands',
      required: const {'id', 'title'},
      optional: const {'required_permissions'},
    );
    final services = _contributions(
      map['services'],
      'services',
      required: const {'id', 'version'},
      optional: const {'priority', 'required_permissions'},
    );
    final editors = _contributions(
      map['editors'],
      'editors',
      required: const {'id', 'extensions'},
      optional: const {'priority'},
    );
    final panels = _contributions(
      map['panels'],
      'panels',
      required: const {'id', 'region'},
      optional: const {'priority'},
    );
    final agentCli = _list(
      map['agent_cli'],
      'agent_cli',
    ).map(OpenMuseAgentCliContributionV2.fromJson).toList(growable: false);
    final identities = agentCli.map((item) => item.identity).toSet();
    if (identities.length != agentCli.length) {
      throw const OpenMuseManifestFormatException(
        'duplicate Agent CLI identity',
      );
    }
    return OpenMuseContributionsV2(
      commands: commands,
      services: services,
      editors: editors,
      panels: panels,
      agentCli: agentCli,
    );
  }

  Map<String, Object?> toJson() => {
    'commands': commands.map(_copyMap).toList(),
    'services': services.map(_copyMap).toList(),
    'editors': editors.map(_copyMap).toList(),
    'panels': panels.map(_copyMap).toList(),
    'agent_cli': agentCli.map((item) => item.toJson()).toList(),
  };
}

final class OpenMusePluginManifestV2 {
  OpenMusePluginManifestV2({
    required this.id,
    required this.name,
    required this.version,
    required this.protocol,
    required this.uiRuntime,
    required this.executionConnector,
    required this.compatibility,
    required this.artifacts,
    required this.activationEvents,
    required this.requestedPermissions,
    required this.presentation,
    required this.contributes,
  });

  final String id;
  final String name;
  final String version;
  final OpenMuseManifestProtocol protocol;
  final OpenMuseUiRuntimeV2 uiRuntime;
  final OpenMuseExecutionConnectorV2 executionConnector;
  final List<OpenMuseTargetDecision> compatibility;
  final List<OpenMusePluginArtifactV2> artifacts;
  final List<String> activationEvents;
  final Set<String> requestedPermissions;
  final OpenMusePresentationV2 presentation;
  final OpenMuseContributionsV2 contributes;

  factory OpenMusePluginManifestV2.fromJson(Object? value) {
    final map = _object(value, 'manifest');
    _keys(
      map,
      required: const {
        'manifest_version',
        'id',
        'name',
        'version',
        'protocol',
        'ui_runtime',
        'execution_connector',
        'compatibility',
        'artifacts',
        'activation_events',
        'requested_permissions',
        'presentation',
        'contributes',
      },
    );
    if (_integer(map, 'manifest_version') != 2) {
      throw const OpenMuseManifestFormatException('manifest_version must be 2');
    }
    final compatibilityMap = _object(map['compatibility'], 'compatibility');
    _keys(compatibilityMap, required: const {'targets'});
    final compatibility = _list(
      compatibilityMap['targets'],
      'targets',
    ).map(OpenMuseTargetDecision.fromJson).toList(growable: false);
    if (compatibility.isEmpty) {
      throw const OpenMuseManifestFormatException(
        'compatibility.targets must not be empty',
      );
    }
    final artifacts = _list(
      map['artifacts'],
      'artifacts',
    ).map(OpenMusePluginArtifactV2.fromJson).toList(growable: false);
    final manifest = OpenMusePluginManifestV2(
      id: _string(map, 'id'),
      name: _string(map, 'name'),
      version: _string(map, 'version'),
      protocol: OpenMuseManifestProtocol.fromJson(map['protocol']),
      uiRuntime: OpenMuseUiRuntimeV2.fromJson(map['ui_runtime']),
      executionConnector: OpenMuseExecutionConnectorV2.fromJson(
        map['execution_connector'],
      ),
      compatibility: compatibility,
      artifacts: artifacts,
      activationEvents: _stringList(
        map['activation_events'],
        'activation_events',
      ),
      requestedPermissions: _stringSet(
        map['requested_permissions'],
        'requested_permissions',
      ),
      presentation: OpenMusePresentationV2.fromJson(map['presentation']),
      contributes: OpenMuseContributionsV2.fromJson(map['contributes']),
    );
    manifest._validateResolution();
    return manifest;
  }

  void _validateResolution() {
    final decisions = <OpenMuseTarget, OpenMuseTargetDecision>{};
    for (final decision in compatibility) {
      if (decisions.containsKey(decision.target)) {
        throw OpenMuseManifestFormatException(
          'duplicate target decision ${decision.target}',
        );
      }
      decisions[decision.target] = decision;
    }
    final ids = <String>{};
    final counts = <OpenMuseTarget, int>{};
    for (final artifact in artifacts) {
      if (!ids.add(artifact.id)) {
        throw OpenMuseManifestFormatException(
          'duplicate artifact ${artifact.id}',
        );
      }
      final decision = decisions[artifact.target];
      if (decision?.status != OpenMuseTargetStatus.supported) {
        throw OpenMuseManifestFormatException(
          'artifact ${artifact.id} targets an unsupported platform',
        );
      }
      counts.update(artifact.target, (value) => value + 1, ifAbsent: () => 1);
    }
    for (final decision in compatibility) {
      if (decision.status == OpenMuseTargetStatus.supported &&
          !counts.containsKey(decision.target)) {
        throw OpenMuseManifestFormatException(
          'supported target ${decision.target} has no artifact',
        );
      }
    }
  }

  List<OpenMusePluginArtifactV2> resolveArtifacts(OpenMuseTarget target) {
    final decision = compatibility
        .where((item) => item.target == target)
        .firstOrNull;
    if (decision?.status != OpenMuseTargetStatus.supported) {
      throw OpenMuseManifestFormatException('unsupported target $target');
    }
    final selected = artifacts
        .where((artifact) => artifact.target == target)
        .toList(growable: false);
    if (selected.isEmpty) {
      throw OpenMuseManifestFormatException(
        'supported target $target has no artifact',
      );
    }
    return selected;
  }

  Map<String, Object?> toJson() => {
    'manifest_version': 2,
    'id': id,
    'name': name,
    'version': version,
    'protocol': protocol.toJson(),
    'ui_runtime': uiRuntime.toJson(),
    'execution_connector': executionConnector.toJson(),
    'compatibility': {
      'targets': compatibility.map((item) => item.toJson()).toList(),
    },
    'artifacts': artifacts.map((item) => item.toJson()).toList(),
    'activation_events': [...activationEvents],
    'requested_permissions': requestedPermissions.toList(),
    'presentation': presentation.toJson(),
    'contributes': contributes.toJson(),
  };
}

List<Map<String, Object?>> _contributions(
  Object? value,
  String field, {
  required Set<String> required,
  required Set<String> optional,
}) => _list(value, field)
    .map((item) {
      final map = _object(item, field);
      _keys(map, required: required, optional: optional);
      return _copyMap(map);
    })
    .toList(growable: false);

Map<String, Object?> _object(Object? value, String field) {
  if (value is! Map) {
    throw OpenMuseManifestFormatException('$field must be an object');
  }
  try {
    return value.cast<String, Object?>();
  } on TypeError {
    throw OpenMuseManifestFormatException('$field must use string keys');
  }
}

List<Object?> _list(Object? value, String field) {
  if (value == null) return const [];
  if (value is! List) {
    throw OpenMuseManifestFormatException('$field must be an array');
  }
  return value.cast<Object?>();
}

void _keys(
  Map<String, Object?> map, {
  required Set<String> required,
  Set<String> optional = const {},
}) {
  final actual = map.keys.toSet();
  final missing = required.difference(actual);
  if (missing.isNotEmpty) {
    throw OpenMuseManifestFormatException(
      'missing fields: ${missing.join(', ')}',
    );
  }
  final unknown = actual.difference(required.union(optional));
  if (unknown.isNotEmpty) {
    throw OpenMuseManifestFormatException(
      'unknown fields: ${unknown.join(', ')}',
    );
  }
}

String _string(Map<String, Object?> map, String field) {
  final value = map[field];
  if (value is! String || value.trim().isEmpty) {
    throw OpenMuseManifestFormatException('$field must be a non-empty string');
  }
  return value;
}

String? _optionalString(Map<String, Object?> map, String field) {
  if (!map.containsKey(field)) return null;
  return _string(map, field);
}

int _integer(Map<String, Object?> map, String field, {int min = 0}) {
  final value = map[field];
  if (value is! int || value < min) {
    throw OpenMuseManifestFormatException('$field must be an integer >= $min');
  }
  return value;
}

bool _boolean(Map<String, Object?> map, String field) {
  final value = map[field];
  if (value is! bool) {
    throw OpenMuseManifestFormatException('$field must be a boolean');
  }
  return value;
}

T _wireEnum<T extends Enum>(
  Map<String, Object?> map,
  String field,
  List<T> values,
  String Function(T) wire,
) {
  final value = _string(map, field);
  for (final item in values) {
    if (wire(item) == value) return item;
  }
  throw OpenMuseManifestFormatException('$field has unsupported value $value');
}

List<T> _enumList<T extends Enum>(
  Object? value,
  String field,
  List<T> values,
  String Function(T) wire,
) {
  final result = _list(value, field)
      .map((item) {
        if (item is! String) {
          throw OpenMuseManifestFormatException('$field must contain strings');
        }
        for (final candidate in values) {
          if (wire(candidate) == item) return candidate;
        }
        throw OpenMuseManifestFormatException(
          '$field has unsupported value $item',
        );
      })
      .toList(growable: false);
  if (result.toSet().length != result.length) {
    throw OpenMuseManifestFormatException('$field must contain unique values');
  }
  return result;
}

List<String> _stringList(Object? value, String field) {
  final result = _list(value, field)
      .map((item) {
        if (item is! String || item.isEmpty) {
          throw OpenMuseManifestFormatException(
            '$field must contain non-empty strings',
          );
        }
        return item;
      })
      .toList(growable: false);
  if (result.toSet().length != result.length) {
    throw OpenMuseManifestFormatException('$field must contain unique values');
  }
  return result;
}

Set<String> _stringSet(Object? value, String field) =>
    _stringList(value, field).toSet();

Map<String, Object?> _copyMap(Map<String, Object?> value) =>
    value.map((key, item) => MapEntry(key, _copy(item)));

Object? _copy(Object? value) => switch (value) {
  Map() => _copyMap(value.cast<String, Object?>()),
  List() => value.map(_copy).toList(growable: false),
  _ => value,
};

String _uiRuntimeWire(OpenMuseUiRuntimeKind value) => switch (value) {
  OpenMuseUiRuntimeKind.none => 'none',
  OpenMuseUiRuntimeKind.flutter => 'flutter',
  OpenMuseUiRuntimeKind.webView => 'web-view',
};

String _executionConnectorWire(OpenMuseExecutionConnectorKind value) =>
    switch (value) {
      OpenMuseExecutionConnectorKind.none => 'none',
      OpenMuseExecutionConnectorKind.hostProcess => 'host-process',
      OpenMuseExecutionConnectorKind.remoteDsh => 'remote-dsh',
      OpenMuseExecutionConnectorKind.sandboxProvider => 'sandbox-provider',
    };

String _targetOsWire(OpenMuseTargetOs value) => value.name;

String _targetArchWire(OpenMuseTargetArch value) => value.name;

String _targetLibcWire(OpenMuseTargetLibc value) => value.name;

String _artifactKindWire(OpenMuseArtifactKind value) => switch (value) {
  OpenMuseArtifactKind.hostBundle => 'host-bundle',
  OpenMuseArtifactKind.webBundle => 'web-bundle',
  OpenMuseArtifactKind.nativeExecutable => 'native-executable',
  OpenMuseArtifactKind.runtimeClosure => 'runtime-closure',
  OpenMuseArtifactKind.sandboxWorker => 'sandbox-worker',
};

String _presentationSurfaceWire(OpenMusePresentationSurface value) =>
    switch (value) {
      OpenMusePresentationSurface.editor => 'editor',
      OpenMusePresentationSurface.leftSidebar => 'left-sidebar',
      OpenMusePresentationSurface.rightSidebar => 'right-sidebar',
      OpenMusePresentationSurface.bottomPanel => 'bottom-panel',
    };
