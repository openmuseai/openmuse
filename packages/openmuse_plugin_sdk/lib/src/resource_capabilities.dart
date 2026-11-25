import 'dart:async';
import 'dart:math';
import 'package:flutter/material.dart';
import 'package:muse_resource_center/muse_resource_center.dart';
import 'package:muse_resource_embed/muse_resource_embed.dart';
export 'package:muse_resource_embed/muse_resource_embed.dart';

export 'package:muse_resource_center/muse_resource_center.dart';

part 'resource_close_transaction.dart';

const openMuseResourceRouteName = 'openmuse.resource';

/// Optional account/workspace actions, rendered from active plugins without
/// coupling the Host to a particular Cloud vendor or content implementation.
abstract interface class OpenMuseWorkspaceActionsContributor {
  List<Widget> buildWorkspaceActions(BuildContext context);
}

/// Shared transcript/URL adapter. The platform Host resolves a fresh access
/// context for every click; the URI itself is never an authorization grant.
final class ResourceLinkRouting extends InheritedWidget {
  const ResourceLinkRouting({
    super.key,
    required this.open,
    this.locate,
    required super.child,
  });
  final Future<void> Function(String uri) open;
  final Future<void> Function(ResourceLocator locator)? locate;
  static ResourceLinkRouting? maybeOf(BuildContext context) =>
      context.dependOnInheritedWidgetOfExactType<ResourceLinkRouting>();

  /// Navigator routes are siblings of their launching page. Carry its resource
  /// routing port explicitly; this does not carry or create an access grant.
  static Widget capture(BuildContext context, Widget child) {
    final routing = maybeOf(context);
    return routing == null
        ? child
        : ResourceLinkRouting(
            open: routing.open,
            locate: routing.locate,
            child: child,
          );
  }

  @override
  bool updateShouldNotify(ResourceLinkRouting oldWidget) =>
      open != oldWidget.open || locate != oldWidget.locate;
}

/// Resource source creation belongs to active plugins; Hosts supply only identity.
final class ResourceSourceContext {
  const ResourceSourceContext({
    required this.scope,
    required this.deviceRef,
    required this.deviceLabel,
    required this.workspace,
    required this.workspaceLabel,
    required this.generation,
    this.state,
  });
  final ResourceStateStore? state;
  final String scope, deviceRef, deviceLabel, workspace, workspaceLabel;
  final int Function() generation;
}

abstract interface class OpenMuseResourceSourceContributor {
  List<ResourceProvider> createResourceSources(ResourceSourceContext context);
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

/// A separately authorized child resource contributes its own presentation and
/// opaque selectors. Neither the parent document nor the Host interprets them.
abstract interface class OpenMuseResourceEmbedContributor {
  Set<String> get resourceEmbedSurfaces;
  Future<List<({String title, ResourceEmbedTarget target})>>
  resourceEmbedTargets(OpenedResource opened, String surface);
  Widget buildResourceEmbed(
    BuildContext context,
    ResourceEditorSession session,
    ResourceEmbedTarget target,
    ResourceEmbedPolicy policy,
  );
}

/// Optional pure admission of a selector against already authorized bytes.
/// Implementations must not acquire an owner, report opened, save, or access the
/// network. Source authorization and the presentation's authority stay separate.
abstract interface class OpenMuseResourceEmbedValidator {
  Future<void> validateResourceEmbedTarget(
    OpenedResource opened,
    ResourceEmbedTarget target,
  );
}

/// An account-scoped live owner or a specific immutable historical view.
/// Current resource revisions do not split the live owner as checkpoints change.
final class ResourceSessionScope {
  const ResourceSessionScope(this.identity, {this.snapshotVersion});
  final ResourceIdentity identity;
  final ResourceVersion? snapshotVersion;
  @override
  bool operator ==(Object other) =>
      other is ResourceSessionScope &&
      identity == other.identity &&
      snapshotVersion == other.snapshotVersion;
  @override
  int get hashCode => Object.hash(identity, snapshotVersion);
}

/// Revoked by the account/resource-center owner, independently of any Widget.
/// Retaining a presentation session must never retain a departed account grant.
final class ResourceSessionLifetime extends ChangeNotifier {
  bool _active = true;
  bool get active => _active;
  void invalidate() {
    if (!_active) return;
    _active = false;
    notifyListeners();
  }
}

final class ResourceSessionRetention {
  ResourceSessionRetention._(this._release);
  void Function()? _release;
  void release() {
    final release = _release;
    _release = null;
    release?.call();
  }
}

/// Presentation context only. Embedded contributors receive separate bounded
/// sessions; this scope connects their save barriers to the containing page.
final class ResourceEditorSessionScope extends InheritedWidget {
  const ResourceEditorSessionScope({
    super.key,
    required this.session,
    required super.child,
  });
  final ResourceEditorSession session;
  static ResourceEditorSession? maybeOf(BuildContext context) => context
      .dependOnInheritedWidgetOfExactType<ResourceEditorSessionScope>()
      ?.session;
  @override
  bool updateShouldNotify(ResourceEditorSessionScope old) =>
      !identical(session, old.session);
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
    this.lifetime,
  }) : _opened = opened;
  OpenedResource _opened;
  OpenedResource get opened => _opened;
  ResourceRecord get record => _opened.content.record;
  final _released = Completer<void>();

  /// Completes when the last presentation/owner retention releases this session.
  Future<void> get released => _released.future;
  ({String name, List<String>? location})? _displayMetadata;
  ResourceRecord _projectDisplay(ResourceRecord value) {
    final display = _displayMetadata;
    return display == null
        ? value
        : value.withDisplayMetadata(
            name: display.name,
            location: display.location,
          );
  }

  /// Applies an already verified source label to a live presentation. Content
  /// checkpoints retain their original bytes/version and permission policy.
  void observeDisplayMetadata({
    required ResourceIdentity identity,
    required String name,
    List<String>? location,
  }) {
    _checkActive();
    if (identity != record.identity)
      throw const ResourceFailure('RESOURCE_SESSION_MISMATCH');
    if (opened.fromSnapshot) throw const ResourceFailure('READ_ONLY');
    final display = (
      name: name,
      location: location == null ? null : List<String>.unmodifiable(location),
    );
    final projected = record.withDisplayMetadata(
      name: display.name,
      location: display.location,
    );
    _displayMetadata = display;
    _opened = OpenedResource(
      reference: opened.reference,
      content: ResourceContent(projected, bytes),
      sourceOffline: opened.sourceOffline,
    );
    notifyListeners();
  }

  List<int> get bytes => _opened.content.bytes;

  /// Presentation-only navigation supplied by the resource owner.
  int? initialLine, initialColumn;

  /// The Host routes a document link through this session's resource owner.
  /// Plugins supply the original href and never synthesize a host file path.
  Future<bool> Function(String target)? openResourceLink;
  Future<ResourceCodeResult> Function(ResourceCodeRequest request)?
  navigateCode;
  Future<bool> Function(ResourceCodeTarget target)? openCodeTarget;
  ResourceSessionScope get ownershipScope => ResourceSessionScope(
    record.identity,
    snapshotVersion: opened.fromSnapshot ? record.version : null,
  );
  final bool allowEditing;
  final ResourceSessionLifetime? lifetime;
  bool get isActive => !_disposed && (lifetime?.active ?? true);
  bool get isPresentationActive => !_presentationReleased && isActive;
  int _references = 1;
  bool _presentationReleased = false;
  ResourceSessionRetention retain() {
    _checkActive();
    _references++;
    return ResourceSessionRetention._(_releaseReference);
  }

  void _checkActive() {
    if (!isActive) throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
  }

  /// Observe a verified checkpoint from the same resource owner without a
  /// second provider commit or a different version baseline for this surface.
  void observeSharedCheckpoint(ResourceEditorSession owner) {
    _checkActive();
    if (owner.ownershipScope != ownershipScope)
      throw const ResourceFailure('RESOURCE_SESSION_MISMATCH');
    _opened = OpenedResource(
      reference: owner.opened.reference,
      content: ResourceContent(_projectDisplay(owner.record), owner.bytes),
      fromSnapshot: owner.opened.fromSnapshot,
      sourceOffline: owner.opened.sourceOffline,
    );
  }

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

  /// Optional local presentation restoration after placement changes. This
  /// never saves, changes permissions, or reconstructs a content owner.
  Future<void> Function()? onPlacementRestored;
  Future<void> restorePresentationAfterPlacement() async {
    for (final session in _resourceCloseGraph(this).keys) {
      if (session.isPresentationActive && !session.closeFinalized)
        await session.onPlacementRestored?.call();
    }
  }

  /// Optional final owner release. The Host awaits this while the presentation
  /// is still mounted, after its ordinary and dependent save barriers pass.
  /// A failed final checkpoint must keep the tab and its retryable lease alive.
  Future<void> Function()? releaseForClose;

  /// Optional two-phase owner port. All sessions of one content owner share
  /// this participant; a tree coordinator prepares/checkpoints it only once.
  ResourceCloseParticipant? closeParticipant;
  _PreparedResourceTree? _preparedTree;
  int _closePreparationEpoch = 0;
  void cancelClosePreparation() {
    if (_finalizing != null)
      throw const ResourceFailure('CLOSE_ALREADY_COMMITTING');
    _cancelClosePreparation();
  }

  void _cancelClosePreparation() {
    _closePreparationEpoch++;
    final prepared = _preparedTree;
    _preparedTree = null;
    prepared?.cancel();
    _closePrepared = false;
  }

  bool _closeFinalized = false;
  bool get closeFinalized => _closeFinalized;
  Future<bool>? _finalizing;
  Future<bool> finalizeToClose() =>
      _finalizing ??= _finalizeToClose().whenComplete(() => _finalizing = null);
  Future<bool> _finalizeToClose() async {
    if (_closeFinalized) return true;
    if (!_closePrepared) {
      _cancelClosePreparation();
      return false;
    }
    try {
      final tree = _preparedTree;
      if (tree != null) {
        if (!tree.isCurrent)
          throw const ResourceFailure('RESOURCE_DEPENDENCIES_CHANGED');
        await tree.commit();
        _preparedTree = null;
      } else {
        await releaseForClose?.call();
      }
      _closeFinalized = true;
      return true;
    } on Object {
      _cancelClosePreparation();
      return false;
    } finally {
      if (!_disposed) notifyListeners();
    }
  }

  /// Open linked bytes through installed resource handlers. A false result
  /// leaves the originating plugin free to use the platform's file viewer.
  /// The Host receives content, never a plugin's private filesystem path.
  Future<bool> Function(String name, ResourceSeed content)? openLinkedContent;
  Future<bool>? _closing;
  bool _closePrepared = false;
  final _dependents = <ResourceEditorSession>{};
  final _dependentRemovers = <ResourceEditorSession, VoidCallback>{};
  VoidCallback addDependentSession(ResourceEditorSession child) {
    _checkActive();
    if (record.identity.scope != child.record.identity.scope)
      throw const ResourceFailure('RESOURCE_SESSION_MISMATCH');
    final seen = <ResourceEditorSession>{};
    void check(ResourceEditorSession next) {
      if (next.ownershipScope == ownershipScope || seen.length >= 64)
        throw const ResourceFailure('RESOURCE_DEPENDENCY_CYCLE');
      if (!seen.add(next)) return;
      for (final nested in next._dependents) {
        check(nested);
      }
    }

    check(child);
    if (!_dependents.add(child))
      throw const ResourceFailure('RESOURCE_DEPENDENCY_EXISTS');
    void changed() {
      if (child.dirty) {
        _closePrepared = false;
        if (!_disposed) notifyListeners();
      }
    }

    child.addListener(changed);
    changed();
    var removed = false;
    void remove() {
      if (removed) return;
      removed = true;
      child.removeListener(changed);
      _dependents.remove(child);
      _dependentRemovers.remove(child);
    }

    _dependentRemovers[child] = remove;
    return remove;
  }

  bool get closePrepared => _closePrepared;
  bool get canPreserveForClose {
    try {
      return _resourceCloseGraph(
        this,
      ).keys.every((s) => s.closeParticipant?.supportsPrivateRecovery == true);
    } on Object {
      return false;
    }
  }

  Future<bool> prepareToClose({bool privateRecoveryOnly = false}) {
    if (_finalizing != null) return _finalizing!;
    return _closing ??= (() async {
      if (_preparedTree?.isCurrent == true &&
          _closePrepared &&
          _preparedTree!.privateRecoveryOnly == privateRecoveryOnly)
        return true;
      cancelClosePreparation();
      final epoch = _closePreparationEpoch;
      _closePrepared = false;
      try {
        final graph = _resourceCloseGraph(this);
        if (privateRecoveryOnly ||
            graph.keys.any((s) => s.closeParticipant != null)) {
          _preparedTree = await _prepareResourceTree(
            this,
            graph,
            epoch,
            privateRecoveryOnly: privateRecoveryOnly,
          );
          _closePrepared = true;
          return true;
        }
        await flushPendingChanges?.call();
        for (final child in _dependents.toList()) {
          if (!await child.prepareToClose())
            throw const ResourceFailure('UNSAVED_CHILD_CHANGES');
        }
        _closePrepared =
            !dirty &&
            _dependents.every((child) => child.closePrepared && !child.dirty);
        return _closePrepared;
      } on Object {
        return false;
      } finally {
        if (!_disposed) notifyListeners();
      }
    })().whenComplete(() => _closing = null);
  }

  void discardChangesForClose() {
    cancelClosePreparation();
    _closeFinalized = true;
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
    _checkActive();
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
    _checkActive();
    if (!allowEditing || !editing || opened.fromSnapshot)
      throw const ResourceFailure('READ_ONLY');
    final fingerprint = '${record.version.revision}:${resourceDigest(bytes)}';
    if (_pendingFingerprint != fingerprint) {
      _pendingFingerprint = fingerprint;
      _pendingKey =
          'edit-${DateTime.now().microsecondsSinceEpoch}-${Random.secure().nextInt(1 << 30)}';
    }
    var updated = await commitContent(record.version, bytes, _pendingKey!);
    _checkActive();
    if (updated.identity != record.identity)
      throw const ResourceFailure('RESOURCE_MISMATCH');
    updated = _projectDisplay(updated);
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

  /// A private checkpoint was already saved by the content owner. Ending its
  /// account while awaiting the provider must not publish a stale resource
  /// revision or turn normal lifetime teardown into a save failure. All other
  /// provider failures still reach the explicit close/retry barrier.
  Future<bool> commitCheckpointIfActive(List<int> bytes) async {
    if (!isActive) return false;
    try {
      await commit(bytes);
      return true;
    } on ResourceFailure catch (error) {
      if (!isActive && error.code == 'RESOURCE_SESSION_CLOSED') return false;
      rethrow;
    }
  }

  @override
  // The notifier is disposed through _releaseReference when the last owner
  // releases it; disposing one presentation must retain the shared checkpoint.
  // ignore: must_call_super
  void dispose() {
    if (_presentationReleased) return;
    if (!_closeFinalized && _finalizing == null) cancelClosePreparation();
    _presentationReleased = true;
    _releaseReference();
  }

  void _releaseReference() {
    if (--_references != 0) return;
    _disposed = true;
    _released.complete();
    for (final remove in _dependentRemovers.values.toList()) {
      remove();
    }
    super.dispose();
  }
}
