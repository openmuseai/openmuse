import 'dart:math';
import 'package:flutter/material.dart';
import 'package:muse_resource_center/muse_resource_center.dart';

export 'package:muse_resource_center/muse_resource_center.dart';

const openMuseResourceRouteName = 'openmuse.resource';

/// Shared transcript/URL adapter. The platform Host resolves a fresh access
/// context for every click; the URI itself is never an authorization grant.
final class ResourceLinkRouting extends InheritedWidget {
  const ResourceLinkRouting({
    super.key,
    required this.open,
    required super.child,
  });
  final Future<void> Function(String uri) open;
  static ResourceLinkRouting? maybeOf(BuildContext context) =>
      context.dependOnInheritedWidgetOfExactType<ResourceLinkRouting>();
  @override
  bool updateShouldNotify(ResourceLinkRouting oldWidget) =>
      open != oldWidget.open;
}

/// Optional shared contribution. This is not a Mobile-specific extension and
/// does not add unnegotiated fields to the frozen manifest v2.
abstract interface class OpenMuseResourceHandlerContributor {
  List<ResourceHandlerSpec> get resourceHandlers;
  Future<bool> probeResource(ResourceRecord record, ResourceOpenIntent intent);
  Widget buildResourceEditor(
    BuildContext context,
    String handlerId,
    ResourceEditorSession session,
  );
}

abstract interface class OpenMuseResourceCreatorContributor {
  List<ResourceCreatorSpec> get resourceCreators;
  Future<ResourceSeed> createResource(
    String creatorId,
    Map<String, Object?> values,
  );
}

/// Business interpretation belongs to the resource plugin, not the Host.
/// Returning null declines extraction. The Host applies an aggregate size bound.
abstract interface class OpenMuseResourceContextContributor {
  Future<String?> resourceConversationText(ResourceContent content);
}

/// A plugin receives verified content and a bounded commit callback, never a
/// Provider, Hub, paired cookie, source path or blanket Host command.
final class ResourceEditorSession extends ChangeNotifier {
  ResourceEditorSession({
    required OpenedResource opened,
    required this.allowEditing,
    required this.commitContent,
    required this.onOpened,
    required this.onChanged,
  }) : _opened = opened;
  OpenedResource _opened;
  OpenedResource get opened => _opened;
  ResourceRecord get record => _opened.content.record;
  List<int> get bytes => _opened.content.bytes;
  final bool allowEditing;
  final Future<ResourceRecord> Function(
    ResourceVersion base,
    List<int> bytes,
    String key,
  )
  commitContent;
  final void Function(ResourceRecord record) onOpened, onChanged;
  bool editing = false, dirty = false, _reported = false, _disposed = false;

  /// Optional editor-owned save barrier. Hosts invoke this before closing an
  /// autosaving editor; format knowledge and persistence remain in the plugin.
  Future<void> Function()? flushPendingChanges;

  /// Open linked bytes through installed resource handlers. A false result
  /// leaves the originating plugin free to use the platform's file viewer.
  /// The Host receives content, never a plugin's private filesystem path.
  Future<bool> Function(String name, ResourceSeed content)? openLinkedContent;
  Future<bool>? _closing;
  bool _closePrepared = false;
  bool get closePrepared => _closePrepared;
  Future<bool> prepareToClose() => _closing ??= (() async {
    try {
      await flushPendingChanges?.call();
      _closePrepared = !dirty;
      return _closePrepared;
    } on Object {
      return false;
    } finally {
      _closing = null;
      if (!_disposed) notifyListeners();
    }
  })();
  void discardChangesForClose() {
    dirty = false;
    _closePrepared = true;
    if (!_disposed) notifyListeners();
  }

  String? _pendingFingerprint, _pendingKey;
  void reportOpened() {
    if (!_reported && !_disposed) {
      _reported = true;
      onOpened(record);
    }
  }

  void enterEditing() {
    if (!allowEditing || opened.fromSnapshot)
      throw const ResourceFailure('READ_ONLY');
    editing = true;
    notifyListeners();
  }

  void markDirty(bool value) {
    if (_disposed) return;
    dirty = value;
    if (value) _closePrepared = false;
    notifyListeners();
  }

  Future<void> commit(List<int> bytes) async {
    if (!allowEditing || !editing || opened.fromSnapshot)
      throw const ResourceFailure('READ_ONLY');
    final fingerprint = '${record.version.revision}:${resourceDigest(bytes)}';
    if (_pendingFingerprint != fingerprint) {
      _pendingFingerprint = fingerprint;
      _pendingKey =
          'edit-${DateTime.now().microsecondsSinceEpoch}-${Random.secure().nextInt(1 << 30)}';
    }
    final updated = await commitContent(record.version, bytes, _pendingKey!);
    if (updated.identity != record.identity)
      throw const ResourceFailure('RESOURCE_MISMATCH');
    _opened = OpenedResource(
      reference: ResourceReference(
        identity: opened.reference.identity,
        version: updated.version,
      ),
      content: ResourceContent(updated, bytes),
    );
    dirty = false;
    _pendingFingerprint = null;
    _pendingKey = null;
    onChanged(updated);
    if (!_disposed) notifyListeners();
  }

  @override
  void dispose() {
    _disposed = true;
    super.dispose();
  }
}
