import 'dart:typed_data';
import 'journal.dart';

/// Original protocol support observed at one point in time, never a write grant.
final class OriginalSyncCapability {
  const OriginalSyncCapability({
    required this.send,
    required this.lookup,
    required this.canWrite,
    this.scope,
    this.maxEncodedBytes = 0,
    this.maxMissingUpdateBytes = 0,
  });
  final bool send, lookup, canWrite;
  final OriginalSyncScope? scope;
  final int maxEncodedBytes, maxMissingUpdateBytes;
}

final class OriginalDurableExchangeResult {
  OriginalDurableExchangeResult({
    required this.receipt,
    required Uint8List missingUpdate,
    required this.propagation,
  }) : missingUpdate = Uint8List.fromList(missingUpdate).asUnmodifiableView();
  final CommittedOriginalSyncReceipt receipt;
  final Uint8List missingUpdate;
  final String propagation;
}

/// Optional original-model port. Application owners persist exact operations
/// with their existing content checkpoint; the transport owns no checkpoint.
abstract interface class OriginalDurableSyncProvider {
  Future<OriginalSyncCapability?> readOriginalSyncCapability(String objectId);
  Future<OriginalDurableExchangeResult> exchangeOriginal(
    PendingOriginalSync operation,
  );
  Future<CommittedOriginalSyncReceipt?> readOriginalSyncReceipt(
    PendingOriginalSync operation,
  );
}
