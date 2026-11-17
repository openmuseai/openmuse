import 'dart:async';
import 'dart:convert';
import 'cloud_http_io.dart' if (dart.library.js_interop) 'cloud_http_web.dart';

import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:uuid/uuid.dart';

part 'appflowy_workspace_access.dart';
part 'appflowy_row_document.dart';
part 'appflowy_workspace_invitations.dart';
part 'appflowy_access_mutations.dart';
part 'appflowy_owner_invitations.dart';
part 'appflowy_workspace_invite_code.dart';
part 'appflowy_document_creation.dart';
part 'appflowy_page_rename.dart';
part 'appflowy_page_title.dart';
part 'appflowy_document_publication.dart';

typedef RefreshAccessTokenProvider = Future<String?> Function();

final class AppFlowyCloudWorkspaceService
    implements
        CloudWorkspaceService,
        CloudResourceCatalogPort,
        DshRuntimeConnector,
        DshSessionCatalogPort,
        ResourceRangePort,
        OfficeResourceCommitPort {
  AppFlowyCloudWorkspaceService({
    required this.baseUri,
    required this.accessToken,
    required this.deviceId,
    this.refreshAccessToken,
    this.accountSubject,
    HttpClient? client,
    this.allowHttpForTesting = false,
    this.maxJsonBytes = 1024 * 1024,
    this.rowCreationTimeout = const Duration(seconds: 20),
    this.documentCreationTimeout = const Duration(seconds: 20),
    this.pageRenameTimeout = const Duration(seconds: 20),
  }) : _client = client ?? HttpClient() {
    final loopback =
        baseUri.host == 'localhost' ||
        baseUri.host == '127.0.0.1' ||
        baseUri.host == '::1' ||
        baseUri.host == '10.0.2.2';
    if (rowCreationTimeout <= Duration.zero ||
        rowCreationTimeout > const Duration(minutes: 2)) {
      throw ArgumentError('invalid row creation timeout');
    }
    if (documentCreationTimeout <= Duration.zero ||
        documentCreationTimeout > const Duration(minutes: 2)) {
      throw ArgumentError('invalid document creation timeout');
    }
    if (pageRenameTimeout <= Duration.zero ||
        pageRenameTimeout > const Duration(minutes: 2)) {
      throw ArgumentError('invalid page rename timeout');
    }
    if (baseUri.host.isEmpty ||
        baseUri.userInfo.isNotEmpty ||
        baseUri.hasQuery ||
        baseUri.hasFragment ||
        (baseUri.path.isNotEmpty && baseUri.path != '/') ||
        (baseUri.scheme != 'https' &&
            !(allowHttpForTesting && baseUri.scheme == 'http' && loopback)) ||
        deviceId.isEmpty ||
        deviceId.length > 256) {
      throw ArgumentError('invalid AppFlowy Cloud endpoint or device id');
    }
  }

  final Uri baseUri;
  final Future<String?> Function() accessToken;
  final RefreshAccessTokenProvider? refreshAccessToken;

  /// Pins each operation to the originating authenticated account, including
  /// token refresh and delayed HTTP responses. No token is decoded locally.
  final String? Function()? accountSubject;
  final String deviceId;
  final bool allowHttpForTesting;
  final int maxJsonBytes;
  final Duration rowCreationTimeout;
  final Duration documentCreationTimeout;
  final Duration pageRenameTimeout;
  final HttpClient _client;
  int _accountEpoch = 0;
  final _rowDocumentCreations = <(String?, int, String, String, String)>{};
  final _documentIntents = <String, _DocumentCreationIntent>{};
  final _renameIntents = <(String, String), _PageRenameIntent>{};
  int _pageNameReadSerial = 0;
  final _pageNameReads = <String, int>{};
  final _pageNameSnapshots =
      <
        String,
        ({
          int version,
          (String?, int) account,
          List<AppFlowyWorkspacePage> pages,
        })
      >{};
  final _renameChanges = StreamController<void>.broadcast();
  final _pageNameChanges =
      StreamController<AppFlowyPageNameObservation>.broadcast();
  Stream<void> get pageRenameChanges => _renameChanges.stream;
  Stream<AppFlowyPageNameObservation> get pageNameChanges =>
      _pageNameChanges.stream;
  final _documentChanges = StreamController<void>.broadcast();
  Stream<void> get documentCreationChanges => _documentChanges.stream;
  final _accessIntents = <_AccessIntent>[];
  final _codeIntents = <_AccessIntent>[];
  final _codeReads = <String, AppFlowyWorkspaceInviteCode>{};
  final _codePreviews = <String, AppFlowyWorkspaceInviteCodePreview>{};
  int _codeReadVersion = 0;
  final _ownerCancellations =
      <_OwnerCancelKey, Future<AppFlowyOwnerInvitationCancellation>>{};
  final _accessChanges = StreamController<void>.broadcast();
  Stream<void> get accessMutationChanges => _accessChanges.stream;
  void _accessChanged() {
    if (!_accessChanges.isClosed) _accessChanges.add(null);
  }

  /// Called by the owning account plugin on sign-out/subject transitions.
  /// Also fences A -> B -> A transitions during an in-flight operation.
  void invalidateAccountOperations() {
    _accountEpoch++;
    _documentIntents.clear();
    _renameIntents.clear();
    _pageNameReads.clear();
    _pageNameSnapshots.clear();
    if (!_renameChanges.isClosed) _renameChanges.add(null);
    if (!_documentChanges.isClosed) _documentChanges.add(null);
    _accessIntents.clear();
    _codeIntents.clear();
    _codeReads.clear();
    _codePreviews.clear();
    _ownerCancellations.clear();
    _accessChanged();
  }

  @override
  DshPlacement get placement => DshPlacement.cloudRemote;

  void closeClient() {
    _renameIntents.clear();
    _pageNameReads.clear();
    _pageNameSnapshots.clear();
    unawaited(_renameChanges.close());
    unawaited(_pageNameChanges.close());
    _documentIntents.clear();
    unawaited(_documentChanges.close());
    _accessIntents.clear();
    _codeIntents.clear();
    _codeReads.clear();
    _codePreviews.clear();
    _ownerCancellations.clear();
    _client.close(force: true);
    unawaited(_accessChanges.close());
  }

  @override
  Future<List<CloudWorkspaceRecord>> listWorkspaces() async {
    final data = await _envelope('GET', '/api/workspace?include_role=true');
    if (data is! List) throw _invalid('workspace data must be a list');
    return data
        .map((item) {
          final value = _object(item, 'workspace');
          final role = value['role'];
          return CloudWorkspaceRecord(
            workspaceRef: _requiredString(value, 'workspace_id'),
            title: _requiredString(value, 'workspace_name'),
            revision:
                _optionalString(value, 'database_storage_id') ??
                _optionalString(value, 'created_at') ??
                'cloud',
            writable:
                role == 'Owner' || role == 'Member' || role == 1 || role == 2,
            storageState: CloudStorageState.available,
          );
        })
        .toList(growable: false);
  }

  @override
  Future<List<DshSessionSummary>> listSessions() async {
    final data = await _envelope('GET', '/api/muse/dsh/sessions');
    final value = _object(data, 'session list');
    final items = value['items'];
    if (items is! List) throw _invalid('session items must be a list');
    return items
        .map((item) {
          final session = _object(item, 'session');
          return DshSessionSummary(
            sessionRef: _requiredString(session, 'sessionRef'),
            workspaceRef: _requiredString(session, 'workspaceRef'),
            state: _requiredString(session, 'state'),
            instanceRef: _optionalString(session, 'instanceRef'),
            nodeId: _requiredString(session, 'nodeId'),
            createdAtMs: _requiredInt(session, 'createdAt'),
            lastActiveAtMs: _requiredInt(session, 'lastActiveAt'),
            attachedDeviceCount: _requiredInt(session, 'attachedDeviceCount'),
            queuePosition: _optionalInt(session, 'queuePosition'),
          );
        })
        .toList(growable: false);
  }

  @override
  Future<DshSessionDescriptor> open(String workspaceRef, int generation) async {
    final data = await _envelope(
      'POST',
      '/api/muse/dsh/session/open',
      body: {'workspaceId': workspaceRef, 'deviceId': deviceId},
    );
    final value = _object(data, 'session');
    final webUrl = Uri.tryParse(_requiredString(value, 'webUrl'));
    if (webUrl == null || webUrl.host.isEmpty || webUrl.path.isEmpty) {
      throw _invalid('webUrl must be an absolute URL');
    }
    final origin = Uri(
      scheme: webUrl.scheme,
      host: webUrl.host,
      port: webUrl.hasPort ? webUrl.port : null,
    ).toString();
    if (webUrl.hasFragment)
      throw _invalid('webUrl must not contain a fragment');
    final path = webUrl.hasQuery
        ? '${webUrl.path}?${webUrl.query}'
        : webUrl.path;
    return DshSessionDescriptor(
      sessionRef: _requiredString(value, 'sessionRef'),
      origin: origin,
      path: path,
      generation: generation,
      allowInsecureLoopback: allowHttpForTesting,
    );
  }

  @override
  Future<void> close(String sessionRef) async {
    await _envelope(
      'POST',
      '/api/muse/dsh/session/close',
      body: {'sessionRef': sessionRef, 'deviceId': deviceId},
    );
  }

  @override
  Future<List<CloudResourceRecord>> listResources({
    required String workspaceRef,
    required String revision,
    required int generation,
  }) async {
    final data = await _envelope(
      'POST',
      '/api/muse/workspace/tree',
      body: {'workspaceId': workspaceRef, 'depth': 4, 'limit': 64},
    );
    final value = _object(data, 'workspace tree');
    final items = value['items'];
    if (items is! List) throw _invalid('workspace tree items must be a list');
    return items
        .map(_resourceFromTreeItem)
        .whereType<CloudResourceRecord>()
        .toList(growable: false);
  }

  CloudResourceRecord? _resourceFromTreeItem(Object? item) {
    final value = _object(item, 'workspace tree item');
    if (value['isSpace'] == true) return null;
    final layout = _requiredString(value, 'layout');
    return CloudResourceRecord(
      resourceRef: _requiredString(value, 'viewId'),
      title: _requiredString(value, 'title'),
      revision: 'view',
      size: 0,
      mediaType: 'application/x-openmuse-$layout',
      writable: false,
    );
  }

  @override
  Future<ResourceHandle> issueResourceHandle({
    required String workspaceRef,
    required String resourceRef,
    required String revision,
    required String audience,
    required int generation,
  }) => throw _unavailable('resource handles are not available on this server');

  @override
  Future<CloudChangeProposal> propose({
    required String workspaceRef,
    required String expectedRevision,
    required String instruction,
    required int generation,
  }) => throw _unavailable(
    'workspace proposals are not available on this server',
  );

  @override
  Future<CloudApplyReceipt> approve({
    required CloudChangeProposal proposal,
    required int generation,
  }) => throw _unavailable('workspace apply is not available on this server');

  @override
  Future<List<int>> read(ResourceHandle handle, int start, int endExclusive) =>
      throw _unavailable('resource ranges are not available on this server');

  @override
  Future<OfficeResourceCommitReceipt> commit({
    required String resourceRef,
    required String expectedRevision,
    required List<int> bytes,
    required String idempotencyKey,
    required int generation,
  }) => throw _unavailable('Office commits are not available on this server');

  Future<Object?> _envelope(
    String method,
    String path, {
    Object? body,
    int? responseBudget,
    Exception Function()? authorizationRejection,
    Duration? timeout,
    _AccessIntent? accessIntent,
  }) async {
    if (responseBudget != null &&
        (responseBudget < 1 || responseBudget > 32 * 1024 * 1024)) {
      throw ArgumentError('invalid response budget');
    }
    final subject = (accountSubject?.call(), _accountEpoch);
    for (var attempt = 0; ; attempt++) {
      final requestScope = _CloudRequestScope();
      try {
        final task = _envelopeOnce(
          method,
          path,
          subject: subject,
          body: body,
          responseBudget: responseBudget ?? maxJsonBytes,
          authorizationRejection: authorizationRejection,
          requestScope: requestScope,
          accessIntent: accessIntent,
        );
        return timeout == null
            ? await task
            : await task.timeout(
                timeout,
                onTimeout: () {
                  requestScope.cancel();
                  throw TimeoutException(
                    'Cloud creation request deadline exceeded',
                  );
                },
              );
      } on HttpException {
        if (method != 'GET' || attempt != 0) rethrow;
        _checkAccount(subject);
      } on SocketException {
        if (method != 'GET' || attempt != 0) rethrow;
        _checkAccount(subject);
      }
    }
  }

  // A suspended native client may miss a keep-alive connection close. Retry
  // one idempotent read with fresh authorization, never an ambiguous mutation.
  Future<Object?> _envelopeOnce(
    String method,
    String path, {
    required (String?, int) subject,
    Object? body,
    required int responseBudget,
    Exception Function()? authorizationRejection,
    required _CloudRequestScope requestScope,
    _AccessIntent? accessIntent,
  }) async {
    _checkAccount(subject);
    var response = await _request(
      method,
      path,
      subject: subject,
      body: body,
      requestScope: requestScope,
      accessIntent: accessIntent,
    );
    requestScope.check();
    if (response.statusCode == 401 && refreshAccessToken != null) {
      accessIntent?.rejected = true;
      await response.drain<void>();
      final refreshed = await refreshAccessToken!();
      _checkAccount(subject);
      if (refreshed != null && refreshed.isNotEmpty) {
        response = await _request(
          method,
          path,
          subject: subject,
          body: body,
          requestScope: requestScope,
          accessIntent: accessIntent,
        );
      } else {
        if (authorizationRejection != null) throw authorizationRejection();
        throw const CloudServiceException(
          CloudServiceErrorCode.unauthorized,
          'login required',
        );
      }
    }
    if (response.statusCode < 200 || response.statusCode >= 300) {
      final status = response.statusCode;
      if (status == 401 || status == 403) accessIntent?.rejected = true;
      await response.drain<void>();
      if ((status == 401 || status == 403) && authorizationRejection != null)
        throw authorizationRejection();
      throw CloudServiceException(
        status == 401 || status == 403
            ? CloudServiceErrorCode.unauthorized
            : CloudServiceErrorCode.unavailable,
        'Cloud request failed with HTTP $status',
        httpStatus: status,
      );
    }
    final root = _object(
      await _readJson(response, responseBudget),
      'Cloud envelope',
    );
    _checkAccount(subject);
    final code = root['code'];
    if (code is! num || code != 0) {
      if ({1011, 1012, 1024}.contains(code)) accessIntent?.rejected = true;
      if ({1011, 1012, 1024}.contains(code) && authorizationRejection != null)
        throw authorizationRejection();
      throw CloudServiceException(
        {1011, 1012, 1024}.contains(code)
            ? CloudServiceErrorCode.unauthorized
            : CloudServiceErrorCode.unavailable,
        'Cloud returned a business error',
      );
    }
    return root['data'];
  }

  Future<HttpClientResponse> _request(
    String method,
    String path, {
    required (String?, int) subject,
    Object? body,
    required _CloudRequestScope requestScope,
    _AccessIntent? accessIntent,
  }) async {
    requestScope.check();
    final token = await accessToken();
    requestScope.check();
    _checkAccount(subject);
    if (token == null || token.isEmpty || token.length > 8192) {
      throw const CloudServiceException(
        CloudServiceErrorCode.unauthorized,
        'login required',
      );
    }
    final request = await _client.openUrl(method, baseUri.resolve(path));
    requestScope.attach(request);
    try {
      _checkAccount(subject);
    } catch (_) {
      request.abort();
      rethrow;
    }
    request.followRedirects = false;
    request.headers
      ..set(HttpHeaders.authorizationHeader, 'Bearer $token')
      ..set(HttpHeaders.acceptHeader, 'application/json')
      ..set('device_id', deviceId)
      ..set('client-version', '0.16.5');
    if (body != null) {
      request.headers.contentType = ContentType.json;
      request.write(jsonEncode(body));
    }
    if (accessIntent != null) {
      accessIntent.sent = true;
      accessIntent.rejected = false;
    }
    return request.close();
  }

  void _checkAccount((String?, int) expected) {
    if (expected.$2 != _accountEpoch ||
        (accountSubject != null &&
            (expected.$1 == null || expected.$1 != accountSubject!()))) {
      throw const CloudServiceException(
        CloudServiceErrorCode.unauthorized,
        'originating account is no longer active',
      );
    }
  }

  Future<Object?> _readJson(HttpClientResponse response, int limit) async {
    final bytes = <int>[];
    await for (final chunk in response) {
      bytes.addAll(chunk);
      if (bytes.length > limit) throw _invalid('JSON response too large');
    }
    try {
      return jsonDecode(utf8.decode(bytes));
    } on FormatException {
      throw _invalid('invalid JSON response');
    }
  }
}

Map<String, Object?> _object(Object? value, String name) {
  if (value is! Map) throw _invalid('$name must be an object');
  return value.cast<String, Object?>();
}

String _requiredString(Map<String, Object?> value, String key) {
  final result = value[key];
  if (result is! String || result.isEmpty)
    throw _invalid('$key must be a string');
  return result;
}

String? _optionalString(Map<String, Object?> value, String key) {
  final result = value[key];
  return result is String && result.isNotEmpty ? result : null;
}

int _requiredInt(Map<String, Object?> value, String key) {
  final result = value[key];
  if (result is! num || result.toInt() < 0)
    throw _invalid('$key must be an integer');
  return result.toInt();
}

int? _optionalInt(Map<String, Object?> value, String key) {
  final result = value[key];
  return result is num && result.toInt() >= 0 ? result.toInt() : null;
}

CloudServiceException _invalid(String message) =>
    CloudServiceException(CloudServiceErrorCode.invalidResponse, message);

CloudServiceException _unavailable(String message) =>
    CloudServiceException(CloudServiceErrorCode.unavailable, message);

// A deadline retires this attempt before a delayed token/openUrl can send a
// late mutation. Aborting an already-sent POST does not claim server rollback.
final class _CloudRequestScope {
  bool cancelled = false;
  HttpClientRequest? request;
  void check() {
    if (cancelled) throw TimeoutException('Cloud request attempt ended');
  }

  void attach(HttpClientRequest value) {
    request = value;
    if (cancelled) value.abort();
    check();
  }

  void cancel() {
    cancelled = true;
    request?.abort();
  }
}
