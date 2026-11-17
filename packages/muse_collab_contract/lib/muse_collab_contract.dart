export 'presence_roster.dart';

import 'dart:typed_data';

/// Internal preview; deliberately distinct from existing strict resource contracts.
const collaborationContractVersion = 'muse.collab.transport/0.1';

/// Names are opaque to the transport: no paragraph, cell or editor types enter here.
final class CollaborationBinding {
  const CollaborationBinding({
    required this.protocol,
    required this.engine,
    required this.encoding,
    required this.schema,
    required this.epoch,
  });
  final String protocol, engine, encoding, schema;
  final int epoch;

  Map<String, Object> toJson() => {
    'contract': collaborationContractVersion,
    'protocol': protocol,
    'engine': engine,
    'encoding': encoding,
    'schema': schema,
    'epoch': epoch,
  };

  factory CollaborationBinding.fromJson(Map<String, Object?> value) {
    const fields = {
      'contract',
      'protocol',
      'engine',
      'encoding',
      'schema',
      'epoch',
    };
    if (value.length != fields.length ||
        !value.keys.every(fields.contains) ||
        value['contract'] != collaborationContractVersion ||
        ![
          'protocol',
          'engine',
          'encoding',
          'schema',
        ].every((k) => value[k] is String && (value[k] as String).isNotEmpty) ||
        value['epoch'] is! int ||
        (value['epoch'] as int) < 0) {
      throw const FormatException('INVALID_COLLABORATION_BINDING');
    }
    return CollaborationBinding(
      protocol: value['protocol'] as String,
      engine: value['engine'] as String,
      encoding: value['encoding'] as String,
      schema: value['schema'] as String,
      epoch: value['epoch'] as int,
    );
  }

  bool compatibleWith(CollaborationBinding other) =>
      protocol == other.protocol &&
      engine == other.engine &&
      encoding == other.encoding &&
      schema == other.schema &&
      epoch == other.epoch;
}

/// Provider transports opaque state; the domain engine validates and merges it.
/// Exchange success is acceptance only. This interface makes no durable-ACK claim.
abstract interface class CollaborationProvider {
  CollaborationBinding get binding;
  Future<Uint8List?> readState(String objectId);
  Future<void> initializeState(String objectId, Uint8List encodedState);
  Future<Uint8List> exchange(String objectId, Uint8List request);
  void close();
}

/// Optional host-bound object capability. Portable content cannot substitute a
/// different object ID for an already authorized online editor session.
abstract interface class CollaborationObjectScope {
  String? get boundObjectId;
}
