/// Proposed durable-sync operation journal. Contains no document model, HTTP,
/// credentials, ACL decision, or independent content store. The owner embeds
/// this metadata in its existing atomic content checkpoint and serializes ALL
/// checkpoint writes. A legacy exchange response cannot retire an operation.
library;

import 'dart:async';
import 'dart:convert';
import 'dart:typed_data';
import 'package:crypto/crypto.dart';

const _maxBytes = 50 * 1024 * 1024;
const _maxEpoch = 9007199254740991;
const _profile = 'openmuse.original-sync-journal/2';
final _uuid = RegExp(
  r'^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$',
);
final _digest = RegExp(r'^[0-9a-f]{64}$');

Never _invalid() => throw const FormatException('INVALID_SYNC_JOURNAL');
void _fields(Map<String, Object?> value, Set<String> expected) {
  if (value.length != expected.length || !value.keys.every(expected.contains))
    _invalid();
}

String _text(Object? value) =>
    value is String && value.isNotEmpty ? value : _invalid();
String _id(Object? value) {
  final v = _text(value);
  if (!_uuid.hasMatch(v) || v == '00000000-0000-0000-0000-000000000000')
    _invalid();
  return v;
}

String _decimal(Object? value, {bool positive = false}) {
  final v = _text(value);
  if (v.length > 19 ||
      !RegExp(positive ? r'^[1-9][0-9]*$' : r'^(0|[1-9][0-9]*)$').hasMatch(v) ||
      BigInt.parse(v) > BigInt.parse('9223372036854775807'))
    _invalid();
  return v;
}

int _epoch(Object? value) =>
    value is int && value >= 0 && value <= _maxEpoch ? value : _invalid();
Map<String, Object?> _map(Object? value) =>
    value is Map<String, Object?> ? value : _invalid();

/// Server-owned principal and controlled writer identity. Decimal strings keep
/// signed-64 identifiers exact on Web. This is a binding, never a write grant.
final class OriginalSyncScope {
  OriginalSyncScope({
    required String origin,
    required String principalId,
    required String workspaceId,
    required String objectId,
    required int collabType,
    required String generation,
    required String modelProfile,
    required String epoch,
  }) : this.fromJson({
         'origin': origin,
         'principalId': principalId,
         'workspaceId': workspaceId,
         'objectId': objectId,
         'collabType': collabType,
         'generation': generation,
         'modelProfile': modelProfile,
         'epoch': epoch,
       });

  OriginalSyncScope.fromJson(Map<String, Object?> value) {
    _fields(value, {
      'origin',
      'principalId',
      'workspaceId',
      'objectId',
      'collabType',
      'generation',
      'modelProfile',
      'epoch',
    });
    origin = _text(value['origin']);
    final uri = Uri.tryParse(origin);
    if (uri == null ||
        !['http', 'https'].contains(uri.scheme) ||
        uri.host.isEmpty ||
        uri.userInfo.isNotEmpty ||
        uri.origin != origin)
      _invalid();
    principalId = _decimal(value['principalId'], positive: true);
    workspaceId = _id(value['workspaceId']);
    objectId = _id(value['objectId']);
    generation = _id(value['generation']);
    if (value['collabType'] is! int || ![0, 1, 4].contains(value['collabType']))
      _invalid();
    collabType = value['collabType'] as int;
    modelProfile = _text(value['modelProfile']);
    if (modelProfile.length > 128 ||
        RegExp(r'[\x00-\x1f\x7f]').hasMatch(modelProfile))
      _invalid();
    epoch = _decimal(value['epoch']);
  }
  late final String origin,
      principalId,
      workspaceId,
      objectId,
      generation,
      modelProfile,
      epoch;
  late final int collabType;
  Map<String, Object> toJson() => {
    'origin': origin,
    'principalId': principalId,
    'workspaceId': workspaceId,
    'objectId': objectId,
    'collabType': collabType,
    'generation': generation,
    'modelProfile': modelProfile,
    'epoch': epoch,
  };
  bool matches(OriginalSyncScope other) =>
      jsonEncode(toJson()) == jsonEncode(other.toJson());
}

final class PendingOriginalSync {
  PendingOriginalSync._(
    this.scope,
    this.operationId,
    this.sentLocalEpoch,
    Uint8List body,
  ) : _body = Uint8List.fromList(body).asUnmodifiableView(),
      requestSha256 = sha256.convert(body).toString();
  final OriginalSyncScope scope;
  final String operationId, requestSha256;
  final int sentLocalEpoch;
  final Uint8List _body;
  Uint8List get request => _body;
  Map<String, Object> toJson() => {
    'scope': scope.toJson(),
    'operationId': operationId,
    'sentLocalEpoch': sentLocalEpoch,
    'request': base64Encode(_body),
    'requestSha256': requestSha256,
  };
  factory PendingOriginalSync.fromJson(Map<String, Object?> value) {
    _fields(value, {
      'scope',
      'operationId',
      'sentLocalEpoch',
      'request',
      'requestSha256',
    });
    final encoded = _text(value['request']);
    if (encoded.length > ((_maxBytes + 2) ~/ 3) * 4) _invalid();
    final body = base64Decode(encoded);
    if (body.isEmpty ||
        body.length > _maxBytes ||
        base64Encode(body) != encoded)
      _invalid();
    final result = PendingOriginalSync._(
      OriginalSyncScope.fromJson(_map(value['scope'])),
      _id(value['operationId']),
      _epoch(value['sentLocalEpoch']),
      body,
    );
    if (value['requestSha256'] != result.requestSha256) _invalid();
    return result;
  }
}

/// Proposed response metadata, deliberately separate from legacy acceptance.
/// Transport may construct this ONLY from the authenticated durable endpoint.
final class CommittedOriginalSyncReceipt {
  CommittedOriginalSyncReceipt.fromJson(Map<String, Object?> value) {
    _fields(value, {
      'profile',
      'status',
      'scope',
      'operationId',
      'requestSha256',
      'docStateSha256',
      'recordedAt',
    });
    if (value['profile'] != 'openmuse.original-sync-receipt/1' ||
        value['status'] != 'committed')
      _invalid();
    scope = OriginalSyncScope.fromJson(_map(value['scope']));
    operationId = _id(value['operationId']);
    requestSha256 = _text(value['requestSha256']);
    docStateSha256 = _text(value['docStateSha256']);
    recordedAt = _text(value['recordedAt']);
    if (!_digest.hasMatch(requestSha256) ||
        !_digest.hasMatch(docStateSha256) ||
        DateTime.tryParse(recordedAt)?.isUtc != true)
      _invalid();
  }
  late final OriginalSyncScope scope;
  late final String operationId, requestSha256, docStateSha256;

  /// Insertion time, not a claim about the time the transaction committed.
  late final String recordedAt;
}

final class OriginalSyncJournalState {
  OriginalSyncJournalState._(
    this.durableLocalEpoch,
    this.pending,
    this.boundScope,
  );
  factory OriginalSyncJournalState.empty() =>
      OriginalSyncJournalState._(0, null, null);
  final int durableLocalEpoch;
  final PendingOriginalSync? pending;

  /// Identity retained after ACK. It grants no permission, but prevents silently
  /// downgrading a previously durable session to an ambiguous legacy write.
  final OriginalSyncScope? boundScope;
  Map<String, Object?> toJson() => {
    'profile': _profile,
    'durableLocalEpoch': durableLocalEpoch,
    'pending': pending?.toJson(),
    'boundScope': boundScope?.toJson(),
  };
  factory OriginalSyncJournalState.fromJson(Map<String, Object?> value) {
    _fields(value, {'profile', 'durableLocalEpoch', 'pending', 'boundScope'});
    if (value['profile'] != _profile) _invalid();
    final durable = _epoch(value['durableLocalEpoch']);
    final pending = value['pending'] == null
        ? null
        : PendingOriginalSync.fromJson(_map(value['pending']));
    final scope = value['boundScope'] == null
        ? null
        : OriginalSyncScope.fromJson(_map(value['boundScope']));
    if ((durable > 0 && scope == null) ||
        (pending != null &&
            (pending.sentLocalEpoch < durable ||
                scope == null ||
                !scope.matches(pending.scope))))
      _invalid();
    return OriginalSyncJournalState._(durable, pending, scope);
  }
}

/// The caller's persistence callback MUST atomically write its current original
/// content together with this state, through the SAME existing checkpoint queue.
/// Returning a send ticket or retiring it happens only after that write succeeds.
final class OriginalSyncJournal {
  OriginalSyncJournal(this._state, this._persist);
  OriginalSyncJournalState _state;
  final Future<void> Function(OriginalSyncJournalState) _persist;
  Future<void> _tail = Future.value();
  OriginalSyncJournalState get state => _state;

  Future<T> _serial<T>(Future<T> Function() action) {
    final result = _tail.then((_) => action());
    _tail = result.then<void>((_) {}, onError: (Object _, StackTrace __) {});
    return result;
  }

  Future<PendingOriginalSync> prepare({
    required OriginalSyncScope scope,
    required String operationId,
    required Uint8List request,
    required int sentLocalEpoch,
  }) {
    // Bound the allocation before capturing caller-owned bytes.
    if (request.isEmpty || request.length > _maxBytes) _invalid();
    // Capture caller-owned bytes before the first await, even if queued.
    final bytes = Uint8List.fromList(request);
    return _serial(() async {
      final bound = _state.boundScope;
      if (bound != null && !bound.matches(scope)) {
        throw StateError('SYNC_SCOPE_REQUIRES_RECOVERY');
      }
      final old = _state.pending;
      if (old != null) {
        if (!old.scope.matches(scope))
          throw StateError('SYNC_SCOPE_REQUIRES_RECOVERY');
        // New local edits never replace an operation with an unknown outcome.
        return old;
      }
      if (bytes.isEmpty ||
          bytes.length > _maxBytes ||
          _epoch(sentLocalEpoch) < _state.durableLocalEpoch)
        _invalid();
      final operation = PendingOriginalSync._(
        scope,
        _id(operationId),
        sentLocalEpoch,
        bytes,
      );
      final next = OriginalSyncJournalState._(
        _state.durableLocalEpoch,
        operation,
        scope,
      );
      await _persist(next);
      _state = next;
      return operation;
    });
  }

  Future<int> acknowledge(
    CommittedOriginalSyncReceipt receipt,
  ) => _serial(() async {
    final pending = _state.pending;
    if (pending == null ||
        !pending.scope.matches(receipt.scope) ||
        pending.operationId != receipt.operationId ||
        pending.requestSha256 != receipt.requestSha256) {
      throw StateError('SYNC_RECEIPT_MISMATCH');
    }
    final next = OriginalSyncJournalState._(
      pending.sentLocalEpoch,
      null,
      pending.scope,
    );
    await _persist(next);
    _state = next;
    // Only the sent epoch is retired. Edits made during the request stay dirty.
    return next.durableLocalEpoch;
  });
}
