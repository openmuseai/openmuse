const workspaceCatalogContextType = 'workspace.catalog';
const workspaceCatalogDigest =
    'sha256:2538c0adb882241b625b48c0013e4a0dd248d8b391cac22ebdc14e154c50548e';
const workspaceCatalogMaxItems = 64;
const workspaceCatalogMaxDepth = 4;

final _forbidden = RegExp('access_token|refresh_token|api[_-]?key', caseSensitive: false);

class DshCatalogViewInput {
  const DshCatalogViewInput({
    required this.viewId,
    required this.title,
    required this.parentViewId,
    required this.layout,
    required this.isSpace,
  });

  final String viewId;
  final String title;
  final String parentViewId;
  final int layout;
  final bool isSpace;
}

class DshCatalogItem {
  const DshCatalogItem({
    required this.viewId,
    required this.title,
    required this.layout,
    required this.isSpace,
    required this.depth,
    this.parentViewId,
  });

  final String viewId;
  final String title;
  final String layout;
  final bool isSpace;
  final int depth;
  final String? parentViewId;

  Map<String, Object?> toJson() => {
        'viewId': viewId,
        'title': title,
        'layout': layout,
        'isSpace': isSpace,
        'depth': depth,
        'parentViewId': parentViewId,
      };
}

class DshWorkspaceCatalog {
  const DshWorkspaceCatalog({
    required this.workspaceId,
    required this.items,
    required this.truncated,
  });

  final String workspaceId;
  final List<DshCatalogItem> items;
  final bool truncated;

  Map<String, Object?> toJson() => {
        'workspaceId': workspaceId,
        'truncated': truncated,
        'items': items.map((item) => item.toJson()).toList(),
      };
}

String dshCatalogLayoutOf(int layout) {
  switch (layout) {
    case 0:
      return 'document';
    case 1:
      return 'grid';
    case 2:
      return 'board';
    case 3:
      return 'calendar';
    case 4:
      return 'chat';
    case 9:
      return 'word';
    default:
      return 'unknown';
  }
}

String _clip(String value, int max) =>
    value.length <= max ? value : value.substring(0, max);

int dshCatalogDepthOf(
  String viewId,
  Map<String, String> parentOf, {
  int maxDepth = workspaceCatalogMaxDepth,
}) {
  var depth = 0;
  var current = viewId;
  final seen = <String>{};
  while (true) {
    final parent = parentOf[current];
    if (parent == null || parent.isEmpty || parent == current) {
      return depth;
    }
    if (!seen.add(current)) {
      return depth;
    }
    depth += 1;
    if (depth >= maxDepth) {
      return maxDepth;
    }
    current = parent;
  }
}

DshWorkspaceCatalog? buildWorkspaceCatalog({
  required String workspaceId,
  required List<DshCatalogViewInput> views,
}) {
  final id = workspaceId.trim();
  if (id.isEmpty || id.length > 128) {
    return null;
  }
  final parentOf = <String, String>{
    for (final view in views)
      if (view.viewId.trim().isNotEmpty)
        view.viewId.trim(): view.parentViewId.trim(),
  };
  final items = <DshCatalogItem>[];
  var truncated = views.length > workspaceCatalogMaxItems;
  for (final view in views) {
    if (items.length >= workspaceCatalogMaxItems) {
      truncated = true;
      break;
    }
    final viewId = view.viewId.trim();
    if (viewId.isEmpty || viewId.length > 128) {
      continue;
    }
    final trimmedTitle = view.title.trim();
    final item = DshCatalogItem(
      viewId: viewId,
      title: trimmedTitle.isEmpty ? 'Untitled' : _clip(trimmedTitle, 256),
      layout: dshCatalogLayoutOf(view.layout),
      isSpace: view.isSpace,
      depth: dshCatalogDepthOf(viewId, parentOf),
      parentViewId: view.parentViewId.trim().isEmpty
          ? null
          : _clip(view.parentViewId.trim(), 128),
    );
    if (_forbidden.hasMatch(item.toJson().toString())) {
      truncated = true;
      continue;
    }
    items.add(item);
  }
  final catalog = DshWorkspaceCatalog(
    workspaceId: id,
    items: items,
    truncated: truncated,
  );
  if (_forbidden.hasMatch(catalog.toJson().toString())) {
    return null;
  }
  return catalog;
}
