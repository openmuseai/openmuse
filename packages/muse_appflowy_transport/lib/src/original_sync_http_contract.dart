/// Maps authenticated original durable HTTP responses into journal receipts.
/// This adapter does not decide authorization, merge content, or persist state.
library;

import 'dart:convert';
import 'package:muse_original_sync/muse_original_sync.dart';

const _limit = 50 * 1024 * 1024;
Never _invalid() =>
    throw const FormatException('INVALID_DURABLE_SYNC_RESPONSE');
Map<String, Object?> _map(Object? value) =>
    value is Map<String, Object?> ? value : _invalid();
Map<String, Object?> _data(Map<String, Object?> response) {
  if (response['code'] != 0) _invalid();
  return _map(response['data']);
}

void _endpoint(Uri uri, PendingOriginalSync operation, {required bool lookup}) {
  final scope = operation.scope;
  final path =
      '/api/workspace/v1/${scope.workspaceId}/collab/${scope.objectId}/';
  final expected = lookup
      ? 'sync-receipt/${operation.operationId}'
      : 'durable-sync';
  if (uri.origin != scope.origin ||
      uri.path != '$path$expected' ||
      uri.userInfo.isNotEmpty ||
      uri.hasFragment)
    _invalid();
  final q = uri.queryParameters;
  if (q['collab_type'] != '${scope.collabType}' ||
      q['generation'] != scope.generation ||
      q['model_profile'] != scope.modelProfile ||
      q['epoch'] != scope.epoch ||
      (lookup && q['request_sha256'] != operation.requestSha256))
    _invalid();
}

CommittedOriginalSyncReceipt _receipt(
  Object? value,
  PendingOriginalSync operation,
  Uri uri,
) {
  final r = _map(value);
  if (r['protocolProfile'] != 'openmuse.original-sync-receipt/1' ||
      r['strength'] != 'original-pg-commit-v1')
    _invalid();
  final receipt = CommittedOriginalSyncReceipt.fromJson({
    'profile': r['protocolProfile'],
    'status': 'committed',
    'scope': {
      'origin': uri.origin,
      'principalId': r['principalId'],
      'workspaceId': r['workspaceId'],
      'objectId': r['objectId'],
      'collabType': r['collabType'],
      'generation': r['generation'],
      'modelProfile': r['modelProfile'],
      'epoch': r['epoch'],
    },
    'operationId': r['operationId'],
    'requestSha256': r['requestSha256'],
    'docStateSha256': r['docStateSha256'],
    'recordedAt': r['recordedAt'],
  });
  if (!receipt.scope.matches(operation.scope) ||
      receipt.operationId != operation.operationId ||
      receipt.requestSha256 != operation.requestSha256)
    _invalid();
  return receipt;
}

/// Caller supplies the actual authenticated response URI after disabling redirects.
/// Only this explicit durable endpoint can produce a commit-strength receipt.
OriginalDurableExchangeResult parseOriginalDurableExchange(
  Map<String, Object?> response, {
  required Uri responseUri,
  required PendingOriginalSync operation,
}) {
  _endpoint(responseUri, operation, lookup: false);
  final data = _data(response);
  final receipt = _receipt(data['receipt'], operation, responseUri);
  final wire = data['missingUpdateBase64'];
  if (wire is! String ||
      wire.isEmpty ||
      wire.length > ((_limit + 2) ~/ 3) * 4 ||
      !['pending', 'delivered'].contains(data['propagation']))
    _invalid();
  final bytes = base64Decode(wire);
  if (bytes.isEmpty || bytes.length > _limit) _invalid();
  return OriginalDurableExchangeResult(
    receipt: receipt,
    missingUpdate: bytes,
    propagation: data['propagation'] as String,
  );
}

CommittedOriginalSyncReceipt? parseOriginalReceiptLookup(
  Map<String, Object?> response, {
  required Uri responseUri,
  required PendingOriginalSync operation,
}) {
  _endpoint(responseUri, operation, lookup: true);
  final data = _data(response);
  if (data['found'] == false && data['receipt'] == null) return null;
  if (data['found'] != true) _invalid();
  return _receipt(data['receipt'], operation, responseUri);
}

/// Discovery describes support, never an authorization lease. Missing discovery
/// remains the legacy profile; an advertised unknown profile is not a fallback.
OriginalSyncCapability? parseOriginalSyncCapability(
  Object? value, {
  required String origin,
  required String workspaceId,
  required String objectId,
  required int collabType,
  required bool canWrite,
}) {
  if (value == null) return null;
  final data = _map(value);
  if (data['protocolProfile'] != 'openmuse.original-sync-receipt/1' ||
      data['send'] is! bool ||
      data['lookup'] is! bool)
    _invalid();
  if (data['send'] == false)
    return OriginalSyncCapability(
      send: false,
      lookup: data['lookup'] as bool,
      canWrite: canWrite,
    );
  bool limit(String key) =>
      data[key] is int &&
      (data[key] as int) > 0 &&
      (data[key] as int) <= _limit;
  if (!canWrite ||
      data['lookup'] != true ||
      data['receiptStrength'] != 'original-pg-commit-v1' ||
      data['operationHeader'] != 'X-OpenMuse-Sync-Operation' ||
      data['digestHeader'] != 'X-OpenMuse-Sync-Digest' ||
      data['principalHeader'] != 'X-OpenMuse-Sync-Principal' ||
      !limit('maxEncodedBytes') ||
      !limit('maxMissingUpdateBytes'))
    _invalid();
  if ([
    'principalId',
    'generation',
    'modelProfile',
    'epoch',
  ].any((key) => data[key] is! String))
    _invalid();
  final scope = OriginalSyncScope(
    origin: origin,
    principalId: data['principalId'] as String,
    workspaceId: workspaceId,
    objectId: objectId,
    collabType: collabType,
    generation: data['generation'] as String,
    modelProfile: data['modelProfile'] as String,
    epoch: data['epoch'] as String,
  );
  return OriginalSyncCapability(
    send: true,
    lookup: true,
    canWrite: true,
    scope: scope,
    maxEncodedBytes: data['maxEncodedBytes'] as int,
    maxMissingUpdateBytes: data['maxMissingUpdateBytes'] as int,
  );
}
