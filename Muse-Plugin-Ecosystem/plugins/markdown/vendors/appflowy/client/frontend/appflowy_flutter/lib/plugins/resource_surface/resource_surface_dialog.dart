import 'package:appflowy/plugins/resource_surface/resource_file_plugin.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace/application/tabs/tabs_bloc.dart';
import 'package:flowy_infra/file_picker/file_picker_service.dart';
import 'package:flutter/material.dart';

/// How one [`MuseResourceSurfaceOpener.open`] call ended.
enum MuseResourceSurfaceOpenStatus { opened, unavailable, failed }

/// Outcome of one surface open.
///
/// `open` keeps its void-shaped API for the picker, the workspace explorer and
/// the DSH WebView bridge; the dispatch seam observes this value through
/// [`MuseResourceSurfaceOpenObserver`] so it can answer the Host with what
/// actually happened instead of assuming success.
final class MuseResourceSurfaceOpenResult {
  const MuseResourceSurfaceOpenResult.opened({
    required MuseResolvedResource resource,
    required String surfaceKey,
    this.revision,
  })  : status = MuseResourceSurfaceOpenStatus.opened,
        resource = resource,
        surfaceKey = surfaceKey,
        errorCode = null;

  /// No Host surface could be reached at all (no window, no tab registry).
  const MuseResourceSurfaceOpenResult.unavailable()
      : status = MuseResourceSurfaceOpenStatus.unavailable,
        resource = null,
        surfaceKey = null,
        revision = null,
        errorCode = 'SURFACE_UNAVAILABLE';

  /// The shared resource router refused the request; `errorCode` is its code
  /// (`NOT_A_FILE`, `OUTSIDE_SESSION_WORKSPACE`, ...).
  const MuseResourceSurfaceOpenResult.failed(String this.errorCode)
      : status = MuseResourceSurfaceOpenStatus.failed,
        resource = null,
        surfaceKey = null,
        revision = null;

  final MuseResourceSurfaceOpenStatus status;

  /// The file the router resolved, when one was opened.
  final MuseResolvedResource? resource;

  /// Identity of the opened tab (`MuseResourceFilePlugin.id`). It carries a
  /// device path, so it must never cross the bridge unhashed.
  final String? surfaceKey;

  /// Revision the surface itself reports, when it has one. It wins over the
  /// revision the Host carried on the dispatch.
  final String? revision;

  /// Router failure code, or `SURFACE_UNAVAILABLE`.
  final String? errorCode;

  bool get isOpened => status == MuseResourceSurfaceOpenStatus.opened;
}

/// Receives the result of one `open` call.
typedef MuseResourceSurfaceOpenObserver = void Function(
  MuseResourceSurfaceOpenResult result,
);

final class MuseResourceSurfaceOpener {
  static Future<void> pickAndOpen(BuildContext context) async {
    final result = await getIt<FilePickerService>().pickFiles(
      dialogTitle: 'Open with OpenMuse',
    );
    if (!context.mounted || result == null || result.files.isEmpty) return;
    final path = result.files.first.path;
    if (path == null || path.isEmpty) return;
    await open(
      context,
      MuseResourceOpenRequest(
        path: path,
        origin: MuseResourceOpenOrigin.hostPicker,
      ),
    );
  }

  static Future<void> open(
    BuildContext context,
    MuseResourceOpenRequest request, {
    MuseResourceSurfaceOpenObserver? observer,
  }) async {
    try {
      final resource = await MuseLocalResourceRouter().resolve(request);
      if (!context.mounted) {
        observer?.call(const MuseResourceSurfaceOpenResult.unavailable());
        return;
      }
      final plugin = MuseResourceFilePlugin(resource);
      getIt<TabsBloc>().openExternalPlugin(plugin);
      observer?.call(
        MuseResourceSurfaceOpenResult.opened(
          resource: resource,
          surfaceKey: plugin.id,
        ),
      );
    } on MuseResourceOpenException catch (error) {
      observer?.call(MuseResourceSurfaceOpenResult.failed(error.code));
      if (!context.mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Unable to open resource: $error')),
      );
    } on Object catch (error) {
      // No tab registry, no window, or a plugin builder that refused: the Host
      // surface is unavailable rather than the resource missing.
      observer?.call(const MuseResourceSurfaceOpenResult.unavailable());
      if (!context.mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Unable to open resource: $error')),
      );
    }
  }
}
