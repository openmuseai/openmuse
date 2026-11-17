part of 'appflowy_cloud_workspace_service.dart';

/// Publication remains the original Cloud authority, independent of membership
/// and of the document CRDT. A browser link carries identity, never a token.
extension AppFlowyDocumentPublicationApi on AppFlowyCloudWorkspaceService {
  Future<bool> isDocumentPublished(String workspaceId, String viewId) async {
    final wid = _uuid(workspaceId), vid = _uuid(viewId);
    final result = await _envelope('GET', '/api/workspace/$wid/published-info');
    if (result is! List) throw _invalid('publication list required');
    return result.any((item) {
      final row = _object(item, 'publication');
      final info = _object(row['info'], 'publication info');
      return info['view_id'] == vid && info['unpublished_timestamp'] == null;
    });
  }

  Future<void> publishDocument(String workspaceId, String viewId) async {
    final wid = _uuid(workspaceId), vid = _uuid(viewId);
    await _envelope(
      'POST',
      '/api/workspace/$wid/page-view/$vid/publish',
      body: {
        'publish_name': 'smartdoc-$vid',
        'visible_database_view_ids': <String>[],
        'comments_enabled': false,
        'duplicate_enabled': false,
      },
      timeout: const Duration(seconds: 30),
    );
  }

  Future<void> unpublishDocument(String workspaceId, String viewId) async {
    await _envelope(
      'POST',
      '/api/workspace/${_uuid(workspaceId)}/page-view/${_uuid(viewId)}/unpublish',
      body: const {},
      timeout: const Duration(seconds: 30),
    );
  }
}
