part of 'appflowy_cloud_workspace_service.dart';

/// Original server-resolved Root/Row/Document relationship. These booleans are
/// current discovery only; every content read and create reauthorizes remotely.
final class AppFlowyRowDocumentInfo {
  const AppFlowyRowDocumentInfo._(
    this.workspaceId,
    this.rootId,
    this.rowId,
    this.documentId,
    this.exists,
    this.canCreate,
    this.creationPending,
  );
  final String workspaceId, rootId, rowId, documentId;
  final bool exists, canCreate, creationPending;
  static AppFlowyRowDocumentInfo _read(
    Object? value,
    String workspace,
    String root,
    String row, {
    bool pending = false,
  }) {
    final data = _object(value, 'row document');
    if (data['profile'] != 'appflowy.database-row-document/1' ||
        _uuid(_requiredString(data, 'workspaceId')) != workspace ||
        _uuid(_requiredString(data, 'rootId')) != root ||
        _uuid(_requiredString(data, 'rowId')) != row ||
        data['exists'] is! bool ||
        data['canCreate'] is! bool ||
        (data['exists'] == true && data['canCreate'] == true)) {
      throw _invalid('original row document binding required');
    }
    final document = _uuid(_requiredString(data, 'documentId'));
    if ({workspace, root, row}.contains(document))
      throw _invalid('row document identity alias');
    return AppFlowyRowDocumentInfo._(
      workspace,
      root,
      row,
      document,
      data['exists'] as bool,
      data['canCreate'] as bool,
      pending && data['exists'] != true,
    );
  }
}

extension AppFlowyRowDocumentApi on AppFlowyCloudWorkspaceService {
  Future<AppFlowyRowDocumentInfo> inspectRowDocument(
    String workspaceId,
    String rootId,
    String rowId,
  ) async {
    final workspace = _uuid(workspaceId),
        root = _uuid(rootId),
        row = _uuid(rowId);
    final subject = (accountSubject?.call(), _accountEpoch);
    _checkAccount(subject);
    final data = await _envelope(
      'GET',
      '/api/workspace/$workspace/database/$root/row/$row/document',
      timeout: rowCreationTimeout,
    );
    _checkAccount(subject);
    final key = (subject.$1, subject.$2, workspace, root, row);
    final result = AppFlowyRowDocumentInfo._read(
      data,
      workspace,
      root,
      row,
      pending: _rowDocumentCreations.contains(key),
    );
    if (result.exists) _rowDocumentCreations.remove(key);
    return result;
  }

  /// One explicit create-only request. A timeout/server/parse failure is an
  /// unknown outcome: callers may inspect, never automatically replay this.
  Future<AppFlowyRowDocumentInfo> createEmptyRowDocument(
    String workspaceId,
    String rootId,
    String rowId,
  ) async {
    final workspace = _uuid(workspaceId),
        root = _uuid(rootId),
        row = _uuid(rowId);
    final subject = (accountSubject?.call(), _accountEpoch);
    _checkAccount(subject);
    final key = (subject.$1, subject.$2, workspace, root, row);
    if (!_rowDocumentCreations.add(key))
      throw const AppFlowyRowDocumentCreationPending();
    final data = await _envelope(
      'POST',
      '/api/workspace/$workspace/database/$root/row/$row/document',
      body: const <String, Object?>{},
      timeout: rowCreationTimeout,
    );
    _checkAccount(subject);
    final result = AppFlowyRowDocumentInfo._read(data, workspace, root, row);
    if (!result.exists) throw _invalid('created row document required');
    _rowDocumentCreations.remove(key);
    return result;
  }
}

/// The prior create result is unconfirmed. Query the same original relation;
/// never replay its POST automatically, including after reopening a dialog.
final class AppFlowyRowDocumentCreationPending implements Exception {
  const AppFlowyRowDocumentCreationPending();
}
