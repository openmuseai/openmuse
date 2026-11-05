import 'dart:convert';
import 'dart:io';

import 'package:appflowy/plugins/dsh_agent/dsh_runtime.dart';
import 'package:appflowy/plugins/resource_surface/muse_presentation_channel.dart';
import 'package:appflowy/workspace_platform/domain/workspace_models.dart';
import 'package:appflowy_backend/protobuf/flowy-user/protobuf.dart';
import 'package:flutter/foundation.dart';
import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

/// Publishes the Host Project Workspace ↔ DSH Workspace binding document
/// (`muse.workspace/binding/v1`) that the DSH workspace plugin applies at boot.
///
/// The document never carries a device path. A `host-path` Mount publishes its
/// directory through the locator file `bindings/materialized/<mountRef>.path`,
/// which is the only place a path crosses the boundary, and the plugin reads it
/// inside DSH_HOME.
class DshWorkspaceBridge {
  DshWorkspaceBridge._();

  static const String protocol = 'muse.workspace/binding/v1';
  static const String bindingMapProtocol = 'muse.workspace/dsh-binding-map/v1';

  /// Protocol tag of the active-Mount intent the DSH panel writes.
  static const String activeIntentProtocol = 'muse.workspace/active-intent/v1';

  /// DSH home override for tests; production uses the running DSH layout.
  @visibleForTesting
  static String? dshHomeOverride;

  /// Host storage root override for tests.
  @visibleForTesting
  static String? stateRootOverride;

  /// Push of the granted Mount directories into the Rust core, so a **partial**
  /// Mount-relative path can be resolved inside its Mount instead of failing.
  /// Overridable in tests, like the path overrides above.
  @visibleForTesting
  static bool Function(Map<String, String> roots) publishMountRoots =
      publishMuseMountRoots;

  static String get _dshHome =>
      dshHomeOverride ?? DshRuntimeLayout.defaultDshHome;

  static Directory get _bindingsDir => Directory(p.join(_dshHome, 'bindings'));

  static Directory get _materializedDir =>
      Directory(p.join(_bindingsDir.path, 'materialized'));

  /// Published binding document, also read by the Host mapping table owner.
  static File get bindingFile =>
      File(p.join(_bindingsDir.path, 'workspace-binding.json'));

  /// Legacy v0.5 hint file; kept only to retire it.
  static File get legacyHintFile =>
      File(p.join(_bindingsDir.path, 'current-appflowy-workspace.json'));

  /// Receipt written by the DSH side after applying a binding revision.
  static File get receiptFile =>
      File(p.join(_bindingsDir.path, 'workspace-binding-receipt.json'));

  /// Active-Mount intent written when the DSH panel switches Mount; the Host
  /// adopts it on its next publish, because the panel cannot write Host state.
  static File get activeIntentFile =>
      File(p.join(_bindingsDir.path, 'workspace-active-intent.json'));

  /// Locator file holding the directory of one `host-path` Mount.
  static File locatorFile(String mountRef) =>
      File(p.join(_materializedDir.path, '${safeSegment(mountRef)}.path'));

  /// File-name safe form of an opaque identity. Mirrors `sanitizeMountRef` in
  /// the DSH plugin, so both sides derive the same locator file name.
  static String safeSegment(String value) {
    final cleaned = value
        .replaceAll(RegExp('[^A-Za-z0-9._-]+'), '_')
        .replaceAll(RegExp('^_+|_+\$'), '');
    if (cleaned.isEmpty) return 'mount';
    return cleaned.length > 128 ? cleaned.substring(0, 128) : cleaned;
  }

  /// Provider family declared to DSH for a Host provider id.
  static String providerKindOf(String providerId) {
    final normalized = providerId.toLowerCase();
    if (normalized.contains('local')) return 'local';
    if (normalized.contains('ssh')) return 'ssh-agent';
    if (normalized.contains('sftp')) return 'sftp';
    if (normalized.contains('cloud')) return 'cloud';
    if (normalized.contains('collab')) return 'muse-collab';
    return 'unknown';
  }

  /// Records the AppFlowy workspace identity. Mounts already published for the
  /// same workspace are preserved so a workspace switch cannot drop a binding.
  static Future<void> publish(UserWorkspacePB workspace) async {
    final previous = await _readJson(bindingFile);
    final previousRef =
        previous?['workspaceRef'] as String? ?? previous?['appflowyWorkspaceId'];
    final sameWorkspace = previousRef == workspace.workspaceId;
    final mounts = sameWorkspace && previous?['mounts'] is List
        ? (previous!['mounts'] as List)
            .whereType<Map<String, dynamic>>()
            .toList(growable: false)
        : const <Map<String, dynamic>>[];
    await _publish(
      workspaceRef: workspace.workspaceId,
      title: workspace.name,
      mounts: mounts,
      activeMountRef: sameWorkspace
          ? (previous?['activeMountRef'] as String?)
          : null,
      locators: const {},
      sweepLocators: mounts.isEmpty,
    );
  }

  /// Publishes one Project Workspace with all of its Mounts.
  static Future<void> publishProjectWorkspace({
    required String appflowyWorkspaceId,
    required String title,
    required List<MuseWorkspaceMount> mounts,
    String? activeMountRef,
  }) async {
    final ordered = [...mounts]..sort((a, b) => a.order.compareTo(b.order));
    final state = await _loadState(appflowyWorkspaceId);
    final effectiveActive = await _effectiveActiveMountRef(
      workspaceRef: appflowyWorkspaceId,
      state: state,
      requested: activeMountRef ??
          (ordered.isEmpty ? null : ordered.first.mountRef),
      mounts: ordered,
    );
    await _publish(
      workspaceRef: appflowyWorkspaceId,
      title: title,
      mounts: [for (final mount in ordered) _mountJson(mount)],
      activeMountRef: effectiveActive.ref,
      activeMountRequestedAt: effectiveActive.requestedAt,
      locators: {
        for (final mount in ordered)
          if (providerKindOf(mount.providerId) == 'local')
            mount.mountRef: mount.rootLocator,
      },
      sweepLocators: true,
    );
  }

  /// The active Mount to publish: a panel switch the DSH side recorded after the
  /// Host last wrote its mapping table wins, because the Host is only the source
  /// of truth for what it has seen.
  static Future<({String? ref, int? requestedAt})> _effectiveActiveMountRef({
    required String workspaceRef,
    required Map<String, dynamic>? state,
    required String? requested,
    required List<MuseWorkspaceMount> mounts,
  }) async {
    final stateUpdatedAt = state?['updatedAt'] as int?;
    final intent = await _readJson(activeIntentFile);
    if (intent == null) return (ref: requested, requestedAt: null);
    if (intent['protocol'] != activeIntentProtocol) {
      return (ref: requested, requestedAt: null);
    }
    if (intent['workspaceRef'] != workspaceRef) {
      return (ref: requested, requestedAt: null);
    }
    final mountRef = intent['mountRef'];
    final requestedAt = intent['requestedAt'];
    if (mountRef is! String || requestedAt is! int) {
      return (ref: requested, requestedAt: null);
    }
    if (!mounts.any((mount) => mount.mountRef == mountRef)) {
      return (ref: requested, requestedAt: null);
    }
    if (stateUpdatedAt != null && requestedAt <= stateUpdatedAt) {
      return (ref: requested, requestedAt: null);
    }
    return (ref: mountRef, requestedAt: requestedAt);
  }

  static Map<String, dynamic> _mountJson(MuseWorkspaceMount mount) {
    final kind = providerKindOf(mount.providerId);
    return {
      'mountRef': mount.mountRef,
      'displayName': mount.displayName,
      'providerId': mount.providerId,
      'providerKind': kind,
      'bindingKey': mount.bindingKey,
      'order': mount.order,
      'requestedMode': mount.readOnly ? 'read-only' : 'read-write',
      'materialization': {
        'mode': kind == 'local' ? 'host-path' : 'read-only-projection',
      },
    };
  }

  static Future<void> _publish({
    required String workspaceRef,
    required String title,
    required List<Map<String, dynamic>> mounts,
    required String? activeMountRef,
    int? activeMountRequestedAt,
    required Map<String, String> locators,
    required bool sweepLocators,
  }) async {
    try {
      final state = await _loadState(workspaceRef);
      final revision = ((state?['bindingRevision'] as int?) ?? 0) + 1;
      final ids = <String, String>{
        for (final entry
            in ((state?['mountWorkspaceIds'] as Map<String, dynamic>?) ??
                    const <String, dynamic>{})
                .entries)
          if (entry.value is String) entry.key: entry.value as String,
        ...await _receiptIds(),
      };
      // The Host core cannot search a Mount it does not know, so the granted
      // directories travel to the Rust provider alongside the DSH locator files.
      // Publishing an empty catalog is deliberate: it retires directories of
      // Mounts this Host no longer grants, and it runs even when the binding
      // document itself carries no Mount.
      publishMountRoots(locators);
      if (locators.isNotEmpty) await _writeLocators(locators, sweepLocators);
      final document = <String, Object?>{
        'protocol': protocol,
        'bindingRevision': revision,
        'workspaceRef': workspaceRef,
        'title': title,
        if (activeMountRef != null) 'activeMountRef': activeMountRef,
        'issuedAt': DateTime.now().millisecondsSinceEpoch,
        'mounts': [
          for (final mount in mounts) _withHint(mount, ids),
        ],
      };
      await _writeAtomic(bindingFile, jsonEncode(document));
      await _sweepTemporaries();
      if (mounts.isEmpty) await _deleteIfExists(legacyHintFile);
      await _saveState(
        workspaceRef,
        revision,
        ids,
        activeMountRef: activeMountRef,
        updatedAt: activeMountRequestedAt,
      );
    } on FileSystemException {
      // Binding publication is best-effort and must not fail Host startup.
    }
  }

  static Map<String, dynamic> _withHint(
    Map<String, dynamic> mount,
    Map<String, String> ids,
  ) {
    final mountRef = mount['mountRef'];
    final hint = mountRef is String ? ids[mountRef] : null;
    return {
      ...mount,
      if (hint != null) 'dshWorkspaceId': hint,
    };
  }

  static Future<void> _writeLocators(
    Map<String, String> locators,
    bool sweep,
  ) async {
    await _materializedDir.create(recursive: true);
    final expected = <String>{};
    for (final entry in locators.entries) {
      final file = locatorFile(entry.key);
      expected.add(p.basename(file.path));
      await _writeAtomic(file, '${entry.value}\n');
    }
    if (!sweep) return;
    await for (final entity in _materializedDir.list()) {
      if (entity is! File) continue;
      final name = p.basename(entity.path);
      if (!name.endsWith('.path')) continue;
      if (expected.contains(name)) continue;
      await _deleteIfExists(entity);
    }
  }

  static Future<void> _sweepTemporaries() async {
    if (!await _bindingsDir.exists()) return;
    final deadline = DateTime.now().subtract(const Duration(hours: 1));
    await for (final entity in _bindingsDir.list()) {
      if (entity is! File || !entity.path.endsWith('.tmp')) continue;
      final stat = await entity.stat();
      if (stat.modified.isAfter(deadline)) continue;
      await _deleteIfExists(entity);
    }
  }

  /// DSH workspace ids the DSH side reported for the Mounts it applied.
  static Future<Map<String, String>> _receiptIds() async {
    final ids = <String, String>{};
    final receipt = await _readJson(receiptFile);
    final mounts = receipt?['mounts'];
    if (mounts is! List) return ids;
    for (final entry in mounts.whereType<Map<String, dynamic>>()) {
      final mountRef = entry['mountRef'];
      final workspaceId = entry['dshWorkspaceId'];
      if (mountRef is String && workspaceId is String) {
        ids[mountRef] = workspaceId;
      }
    }
    return ids;
  }

  static Future<File> _stateFile(String workspaceRef) async {
    final override = stateRootOverride;
    final root = override ??
        p.join(
          (await getApplicationSupportDirectory()).path,
          'OpenMuse',
          'workspace-platform-v1',
        );
    final directory = Directory(p.join(root, 'dsh-bindings'));
    await directory.create(recursive: true);
    return File(p.join(directory.path, '${safeSegment(workspaceRef)}.json'));
  }

  static Future<Map<String, dynamic>?> _loadState(String workspaceRef) async {
    try {
      return await _readJson(await _stateFile(workspaceRef));
    } on Object {
      return null;
    }
  }

  static Future<void> _saveState(
    String workspaceRef,
    int revision,
    Map<String, String> mountWorkspaceIds, {
    String? activeMountRef,
    int? updatedAt,
  }) async {
    try {
      final file = await _stateFile(workspaceRef);
      await _writeAtomic(
        file,
        const JsonEncoder.withIndent('  ').convert({
          'protocol': bindingMapProtocol,
          'workspaceRef': workspaceRef,
          'bindingRevision': revision,
          'mountWorkspaceIds': mountWorkspaceIds,
          if (activeMountRef != null) 'activeMountRef': activeMountRef,
          'updatedAt': updatedAt ?? DateTime.now().millisecondsSinceEpoch,
        }),
      );
    } on Object {
      // The mapping table is a cache of DSH receipts, never a source of truth.
    }
  }

  static Future<Map<String, dynamic>?> _readJson(File file) async {
    try {
      final value = jsonDecode(await file.readAsString());
      return value is Map<String, dynamic> ? value : null;
    } on Object {
      return null;
    }
  }

  static Future<void> _writeAtomic(File file, String body) async {
    await file.parent.create(recursive: true);
    final temporary = File(
      '${file.path}.$pid.${DateTime.now().microsecondsSinceEpoch}.tmp',
    );
    await temporary.writeAsString(body, flush: true);
    try {
      if (await file.exists()) await file.delete();
    } on FileSystemException {
      // A concurrent publisher may already have replaced the file.
    }
    await temporary.rename(file.path);
  }

  static Future<void> _deleteIfExists(File file) async {
    try {
      if (await file.exists()) await file.delete();
    } on FileSystemException {
      // Best effort: a missing or locked file is not a binding failure.
    }
  }
}
