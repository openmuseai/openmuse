import 'package:appflowy/shared/muse_reference_clipboard.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace_platform/application/workspace_controller.dart';

/// The Markdown/document surface as a Muse reference source (RCX-01).
///
/// A document editor knows its text but not its view identity, and the copy
/// command holds only the editor. This source is the bridge: the document
/// surface attaches one while it is alive, and `handleCopyCommand` asks it to
/// describe the selection being copied.
///
/// The payload carries opaque references only:
///
/// - `resourceRef` = `resource.appflowy.document.<viewId>`
/// - `viewRef`     = `view.appflowy.document.<viewId>`
/// - `mountRef`    = the active Project Workspace mount ref, read live so the
///   reference follows a Mount switch made after the document was opened.
///
/// The document's own AppFlowy view id is the Host-side identity. It is not a
/// device path and never becomes one at this boundary.
final class MuseDocumentSelectionReference
    implements MuseSelectionReferenceSource {
  MuseDocumentSelectionReference({
    required this.viewId,
    required String displayName,
    String? Function()? activeMountRef,
  })  : _displayName = displayName,
        _activeMountRef = activeMountRef ?? _workspaceActiveMountRef;

  /// AppFlowy view id of the document being edited.
  final String viewId;

  final String? Function() _activeMountRef;
  String _displayName;

  /// Update the displayed source name (the view title changed).
  set displayName(String value) {
    final trimmed = value.trim();
    if (trimmed.isNotEmpty) _displayName = trimmed;
  }

  @override
  String get resourceRef => museDocumentResourceRef(viewId);

  @override
  String? get viewRef => museDocumentViewRef(viewId);

  @override
  String? get mountRef => _activeMountRef();

  @override
  String get displayName => _displayName;

  @override
  MuseResourceReference describeSelection({
    required String selectedText,
    required int startBlock,
    required int endBlock,
    String? startBlockRef,
    String? endBlockRef,
    int? capturedAt,
  }) =>
      buildMuseResourceReference(
        resourceRef: resourceRef,
        viewRef: viewRef,
        mountRef: mountRef,
        displayName: _displayName,
        selectedText: selectedText,
        startBlock: startBlock,
        endBlock: endBlock,
        startBlockRef: startBlockRef,
        endBlockRef: endBlockRef,
        capturedAt: capturedAt,
      );
}

/// Active Mount of the current Project Workspace, or null when no workspace
/// controller exists (mobile/Web surfaces without the workspace platform).
String? _workspaceActiveMountRef() {
  if (!getIt.isRegistered<MuseWorkspaceController>()) return null;
  return getIt<MuseWorkspaceController>().activeMountRef;
}
