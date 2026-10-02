typedef JsonMap = Map<String, Object?>;

final class DshGatewayCompatibility {
  const DshGatewayCompatibility({
    required this.nativeConversationCompatible,
    required this.fallbackRequired,
    required this.unsupportedPlugins,
    this.reason,
  });

  final bool nativeConversationCompatible;
  final bool fallbackRequired;
  final String? reason;
  final List<String> unsupportedPlugins;

  factory DshGatewayCompatibility.fromJson(JsonMap json) {
    final plugins = json['unsupportedPlugins'];
    return DshGatewayCompatibility(
      nativeConversationCompatible:
          json['nativeConversationCompatible'] == true,
      fallbackRequired: json['fallbackRequired'] == true,
      reason: json['reason'] as String?,
      unsupportedPlugins: plugins is List
          ? plugins
                .whereType<Map>()
                .map((value) => value['moduleName'])
                .whereType<String>()
                .toList(growable: false)
          : const [],
    );
  }
}

final class DshGatewayHello {
  const DshGatewayHello({
    required this.protocolVersion,
    required this.dshVersion,
    required this.capabilities,
    required this.compatibility,
  });

  static const supportedProtocolVersion = 1;
  static const supportedDshVersion = '0.1.7-rc.1';

  final int protocolVersion;
  final String dshVersion;
  final List<String> capabilities;
  final DshGatewayCompatibility compatibility;

  bool get supported =>
      contractSupported &&
      compatibility.nativeConversationCompatible &&
      !compatibility.fallbackRequired;

  bool get contractSupported =>
      protocolVersion == supportedProtocolVersion &&
      dshVersion == supportedDshVersion;

  factory DshGatewayHello.fromJson(JsonMap json) => DshGatewayHello(
    protocolVersion: _integer(json, 'protocolVersion'),
    dshVersion: _string(json, 'dshVersion'),
    capabilities: _stringList(json, 'capabilities'),
    compatibility: DshGatewayCompatibility.fromJson(
      _map(json, 'compatibility'),
    ),
  );
}

final class DshNativeSessionSummary {
  const DshNativeSessionSummary({
    required this.sessionId,
    required this.updatedAt,
    required this.running,
    required this.blank,
    this.cwd,
    this.title,
    this.headSeq = 0,
    this.projections = const {},
  });

  final String sessionId;
  final int updatedAt;
  final bool running;
  final bool blank;
  final String? cwd;
  final String? title;

  /// Latest durable journal sequence advertised by DSH.
  final int headSeq;

  /// Versioned DSH projections at [headSeq].
  final JsonMap projections;

  factory DshNativeSessionSummary.fromJson(JsonMap json) {
    final rawProjections = json['projections'];
    final projections = rawProjections is Map
        ? rawProjections.cast<String, Object?>()
        : const <String, Object?>{};
    final values = projections['values'];
    final title = values is Map ? values['title'] : null;
    return DshNativeSessionSummary(
      sessionId: _string(json, 'sessionId'),
      updatedAt: _integer(json, 'updatedAt'),
      running: json['running'] == true,
      blank: json['blank'] == true,
      cwd: json['cwd'] as String?,
      title: title is String && title.trim().isNotEmpty ? title.trim() : null,
      headSeq: projections['asOfSeq'] is int
          ? projections['asOfSeq']! as int
          : 0,
      projections: Map.unmodifiable(projections),
    );
  }
}

final class DshNativeWorkspaceSummary {
  const DshNativeWorkspaceSummary({
    required this.workspaceId,
    required this.title,
    required this.sessionIds,
    required this.createdAt,
    required this.updatedAt,
  });

  final String workspaceId;
  final String title;
  final List<String> sessionIds;
  final DateTime createdAt;
  final DateTime updatedAt;

  factory DshNativeWorkspaceSummary.fromJson(JsonMap json) =>
      DshNativeWorkspaceSummary(
        workspaceId: _string(json, 'workspaceId'),
        title: _string(json, 'title'),
        sessionIds: _stringList(json, 'sessionIds'),
        createdAt: DateTime.parse(_string(json, 'createdAt')),
        updatedAt: DateTime.parse(_string(json, 'updatedAt')),
      );
}

final class DshNativeCreatedSession {
  const DshNativeCreatedSession({required this.sessionId, this.agentPreset});

  final String sessionId;
  final String? agentPreset;

  factory DshNativeCreatedSession.fromJson(JsonMap json) =>
      DshNativeCreatedSession(
        sessionId: _string(json, 'sessionId'),
        agentPreset: json['agentPreset'] as String?,
      );
}

final class DshNativeChangedFile {
  const DshNativeChangedFile({
    required this.display,
    required this.added,
    required this.deleted,
    required this.binary,
    required this.oversized,
  });

  final String display;
  final int added;
  final int deleted;
  final bool binary;
  final bool oversized;

  factory DshNativeChangedFile.fromJson(JsonMap json) => DshNativeChangedFile(
    display: _string(json, 'display'),
    added: _integer(json, 'added'),
    deleted: _integer(json, 'deleted'),
    binary: json['binary'] == true,
    oversized: json['oversized'] == true,
  );
}

final class DshNativeWorkspaceChanges {
  const DshNativeWorkspaceChanges({
    required this.turn,
    required this.total,
    required this.added,
    required this.deleted,
    required this.files,
  });

  final int turn;
  final int total;
  final int added;
  final int deleted;
  final List<DshNativeChangedFile> files;

  factory DshNativeWorkspaceChanges.fromJson(JsonMap json) {
    final values = json['files'];
    if (values is! List) throw const FormatException('files must be a list');
    return DshNativeWorkspaceChanges(
      turn: _integer(json, 'turn'),
      total: _integer(json, 'total'),
      added: _integer(json, 'added'),
      deleted: _integer(json, 'deleted'),
      files: values
          .whereType<Map>()
          .map(
            (value) =>
                DshNativeChangedFile.fromJson(value.cast<String, Object?>()),
          )
          .toList(growable: false),
    );
  }
}

enum DshNativeArtifactPreviewKind { text, binary, oversized }

final class DshNativeDiffHunk {
  const DshNativeDiffHunk({
    required this.oldStart,
    required this.oldLines,
    required this.newStart,
    required this.newLines,
    required this.lines,
  });

  final int oldStart;
  final int oldLines;
  final int newStart;
  final int newLines;
  final List<String> lines;

  factory DshNativeDiffHunk.fromJson(JsonMap json) {
    final values = json['lines'];
    if (values is! List || values.any((value) => value is! String)) {
      throw const FormatException('diff hunk lines must be strings');
    }
    return DshNativeDiffHunk(
      oldStart: _integer(json, 'oldStart'),
      oldLines: _integer(json, 'oldLines'),
      newStart: _integer(json, 'newStart'),
      newLines: _integer(json, 'newLines'),
      lines: values.cast<String>().toList(growable: false),
    );
  }
}

/// A path-redacted comparison captured by the Desktop DSH turn recorder.
///
/// The Mobile client addresses this resource only by Session, event sequence,
/// and summary index, so it cannot use the preview API as an arbitrary Desktop
/// file reader.
final class DshNativeArtifactPreview {
  const DshNativeArtifactPreview({
    required this.kind,
    required this.display,
    required this.before,
    required this.after,
    required this.coarse,
    required this.hunks,
  });

  final DshNativeArtifactPreviewKind kind;
  final String display;
  final bool before;
  final bool after;
  final bool coarse;
  final List<DshNativeDiffHunk> hunks;

  bool get isCreated =>
      kind == DshNativeArtifactPreviewKind.text && !before && after;
  bool get isDeleted =>
      kind == DshNativeArtifactPreviewKind.text && before && !after;

  factory DshNativeArtifactPreview.fromJson(JsonMap json) {
    final kind = switch (_string(json, 'kind')) {
      'text' => DshNativeArtifactPreviewKind.text,
      'binary' => DshNativeArtifactPreviewKind.binary,
      'oversized' => DshNativeArtifactPreviewKind.oversized,
      _ => throw const FormatException('unsupported artifact preview kind'),
    };
    final values = json['hunks'];
    if (kind != DshNativeArtifactPreviewKind.text) {
      return DshNativeArtifactPreview(
        kind: kind,
        display: _string(json, 'display'),
        before: false,
        after: false,
        coarse: false,
        hunks: const [],
      );
    }
    if (values is! List) throw const FormatException('preview hunks missing');
    return DshNativeArtifactPreview(
      kind: kind,
      display: _string(json, 'display'),
      before: json['before'] == true,
      after: json['after'] == true,
      coarse: json['coarse'] == true,
      hunks: values
          .whereType<Map>()
          .map(
            (value) =>
                DshNativeDiffHunk.fromJson(value.cast<String, Object?>()),
          )
          .toList(growable: false),
    );
  }
}

final class DshNativeModelSelection {
  const DshNativeModelSelection({
    required this.provider,
    required this.model,
    this.reasoningEffort,
  });

  final String provider;
  final String model;
  final String? reasoningEffort;

  factory DshNativeModelSelection.fromJson(JsonMap json) =>
      DshNativeModelSelection(
        provider: _string(json, 'provider'),
        model: _string(json, 'model'),
        reasoningEffort: json['reasoningEffort'] as String?,
      );
}

final class DshNativeReasoningEffort {
  const DshNativeReasoningEffort({
    required this.id,
    required this.name,
    this.description,
  });

  final String id;
  final String name;
  final String? description;

  factory DshNativeReasoningEffort.fromJson(JsonMap json) =>
      DshNativeReasoningEffort(
        id: _string(json, 'id'),
        name: _string(json, 'name'),
        description: json['description'] as String?,
      );
}

final class DshNativeModelOption {
  const DshNativeModelOption({
    required this.provider,
    required this.providerName,
    required this.id,
    required this.name,
    required this.efforts,
    this.description,
    this.defaultEffort,
  });

  final String provider;
  final String providerName;
  final String id;
  final String name;
  final String? description;
  final List<DshNativeReasoningEffort> efforts;
  final String? defaultEffort;
}

final class DshNativePermissionOption {
  const DshNativePermissionOption({
    required this.value,
    required this.name,
    this.description,
  });

  final String value;
  final String name;
  final String? description;

  factory DshNativePermissionOption.fromJson(JsonMap json) =>
      DshNativePermissionOption(
        value: _string(json, 'value'),
        name: _string(json, 'name'),
        description: json['description'] as String?,
      );
}

final class DshNativeSessionOptions {
  const DshNativeSessionOptions({
    required this.defaultModel,
    required this.models,
    required this.permissions,
    this.defaultPermission,
  });

  final DshNativeModelSelection defaultModel;
  final List<DshNativeModelOption> models;
  final List<DshNativePermissionOption> permissions;
  final String? defaultPermission;

  factory DshNativeSessionOptions.fromJson(JsonMap json) {
    final rawModels = _map(json, 'models');
    final groups = rawModels['groups'];
    final models = <DshNativeModelOption>[];
    if (groups is List) {
      for (final rawGroup in groups.whereType<Map>()) {
        final group = rawGroup.cast<String, Object?>();
        final provider = _string(group, 'id');
        final providerName = _string(group, 'name');
        final rawOptions = group['models'];
        if (rawOptions is! List) continue;
        for (final rawOption in rawOptions.whereType<Map>()) {
          final option = rawOption.cast<String, Object?>();
          final reasoning = option['reasoning'];
          final reasoningMap = reasoning is Map
              ? reasoning.cast<String, Object?>()
              : const <String, Object?>{};
          final rawEfforts = reasoningMap['efforts'];
          models.add(
            DshNativeModelOption(
              provider: provider,
              providerName: providerName,
              id: _string(option, 'id'),
              name: _string(option, 'name'),
              description: option['description'] as String?,
              efforts: rawEfforts is List
                  ? rawEfforts
                        .whereType<Map>()
                        .map(
                          (value) => DshNativeReasoningEffort.fromJson(
                            value.cast<String, Object?>(),
                          ),
                        )
                        .toList(growable: false)
                  : const [],
              defaultEffort: reasoningMap['defaultEffort'] as String?,
            ),
          );
        }
      }
    }
    final rawPermissions = json['permissions'];
    final permissionMap = rawPermissions is Map
        ? rawPermissions.cast<String, Object?>()
        : const <String, Object?>{};
    final permissionItems = permissionMap['options'];
    return DshNativeSessionOptions(
      defaultModel: DshNativeModelSelection.fromJson(
        _map(rawModels, 'default'),
      ),
      models: List.unmodifiable(models),
      permissions: permissionItems is List
          ? permissionItems
                .whereType<Map>()
                .map(
                  (value) => DshNativePermissionOption.fromJson(
                    value.cast<String, Object?>(),
                  ),
                )
                .toList(growable: false)
          : const [],
      defaultPermission: permissionMap['defaultPreset'] as String?,
    );
  }
}

final class DshWireEvent {
  const DshWireEvent({
    required this.type,
    required this.seq,
    required this.time,
    required this.data,
    required this.raw,
    this.ignorable = false,
    this.surfaceOp,
  });

  final String type;
  final int seq;
  final int time;
  final JsonMap data;
  final JsonMap raw;
  final bool ignorable;
  final Object? surfaceOp;

  factory DshWireEvent.fromJson(JsonMap json) => DshWireEvent(
    type: _string(json, 'type'),
    seq: _integer(json, 'seq'),
    time: _integer(json, 'time'),
    data: _map(json, 'data'),
    raw: Map.unmodifiable(json),
    ignorable: json['ignorable'] == true,
    surfaceOp: json['surfaceOp'],
  );
}

sealed class DshFollowFrame {
  const DshFollowFrame();

  factory DshFollowFrame.fromJson(JsonMap json) {
    switch (json['type']) {
      case 'snapshot':
        final records = json['records'];
        return DshSnapshotFrame(
          header: _map(json, 'header'),
          cursor: _integer(json, 'cursor'),
          records: records is List
              ? records
                    .whereType<Map>()
                    .map((record) => record.cast<String, Object?>())
                    .where((record) => record['type'] == 'event')
                    .map(
                      (record) => DshWireEvent.fromJson(_map(record, 'event')),
                    )
                    .toList(growable: false)
              : throw const FormatException('snapshot records must be a list'),
          hasMore: json['hasMore'] == true,
          projections: _map(json, 'projections'),
          assistantStream: json['assistantStream'] is Map
              ? (json['assistantStream'] as Map).cast<String, Object?>()
              : null,
        );
      case 'event':
        return DshEventFrame(DshWireEvent.fromJson(_map(json, 'event')));
      case 'assistant-stream':
        return DshAssistantStreamFrame(_map(json, 'frame'));
      default:
        throw FormatException('unsupported follow frame: ${json['type']}');
    }
  }
}

final class DshSnapshotFrame extends DshFollowFrame {
  const DshSnapshotFrame({
    required this.header,
    required this.cursor,
    required this.records,
    required this.hasMore,
    required this.projections,
    this.assistantStream,
  });

  final JsonMap header;
  final int cursor;
  final List<DshWireEvent> records;
  final bool hasMore;
  final JsonMap projections;
  final JsonMap? assistantStream;
}

final class DshEventFrame extends DshFollowFrame {
  const DshEventFrame(this.event);
  final DshWireEvent event;
}

final class DshAssistantStreamFrame extends DshFollowFrame {
  const DshAssistantStreamFrame(this.frame);
  final JsonMap frame;
}

JsonMap jsonMap(Object? value, [String label = 'value']) {
  if (value is! Map) throw FormatException('$label must be an object');
  return value.cast<String, Object?>();
}

JsonMap _map(JsonMap source, String key) => jsonMap(source[key], key);

String _string(JsonMap source, String key) {
  final value = source[key];
  if (value is! String || value.isEmpty) {
    throw FormatException('$key must be a non-empty string');
  }
  return value;
}

int _integer(JsonMap source, String key) {
  final value = source[key];
  if (value is! int) throw FormatException('$key must be an integer');
  return value;
}

List<String> _stringList(JsonMap source, String key) {
  final value = source[key];
  if (value is! List || value.any((item) => item is! String)) {
    throw FormatException('$key must be a string list');
  }
  return value.cast<String>();
}
