import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';

/// Size is a preference. The Mobile Host owns the final grid constraints.
enum OpenMuseWidgetHeight { small, medium, large }

@immutable
final class OpenMuseWorkbenchCard {
  const OpenMuseWorkbenchCard({
    required this.title,
    required this.summary,
    this.columns = 1,
    this.height = OpenMuseWidgetHeight.medium,
  }) : assert(columns == 1 || columns == 2);

  final String title;
  final String summary;
  final int columns;
  final OpenMuseWidgetHeight height;
}

/// Optional Mobile contribution of an installed [OpenMusePlugin].
///
/// The preview is declarative A2UI. The detail document runs inside a single
/// platform WebView opened by the Host, never inside the scrolling workbench.
/// Requests are scoped to this contributor; no generic Host command is exposed.
abstract interface class OpenMuseMobileWorkbenchContributor {
  OpenMuseWorkbenchCard get workbenchCard;
  Listenable get workbenchChanges;

  /// A2UI v0.9 envelopes for surface `main` and the shared basic catalog.
  List<Map<String, Object?>> buildWorkbenchPreview();

  /// A complete, self-contained HTML document. Remote subresources are denied
  /// by the Host's WebView navigation and the document CSP.
  String buildMobileDetailDocument(Brightness brightness);

  /// Explicit plugin-owned domain actions invoked by its WebView document.
  Future<Object?> handleMobileDetailRequest(String method, Object? arguments);
}

/// Optional detail surface supplied by a plugin compiled into the distribution.
/// Native engine lifecycle, data formats and any private bridges stay inside
/// the plugin. Downloaded HTML contributions retain their existing sandbox.
abstract interface class OpenMuseMobileNativeDetailContributor {
  Widget buildMobileDetail(BuildContext context);
}
