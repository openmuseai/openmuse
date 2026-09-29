const openMuseContractProtocol = 'openmuse.contract';
const openMuseContractMajor = 1;
const openMuseContractMinor = 0;
const _maxSafeInteger = 9007199254740991;

final class OpenMuseContractException implements FormatException {
  const OpenMuseContractException(this.message);

  @override
  final String message;

  @override
  Object? get source => null;

  @override
  int? get offset => null;

  @override
  String toString() => 'OpenMuseContractException: $message';
}

enum PrincipalKind { user, agent, plugin, service, device }

enum ContractErrorCode {
  denied,
  notFound,
  conflict,
  expired,
  staleGeneration,
  unavailable,
  transient,
  integrityFailed,
}

enum ReceiptState { committed, rejected, cancelled, expired, failed }

enum HandleState { active, revoked, expired }

enum LeaseState { active, released, revoked, expired }

final class ProtocolVersion {
  const ProtocolVersion({required this.major, required this.minor});

  final int major;
  final int minor;

  factory ProtocolVersion.fromJson(Object? value) {
    final map = _object(value, 'protocol');
    _keys(map, required: const {'name', 'major', 'minor'});
    if (map['name'] != openMuseContractProtocol) {
      throw const OpenMuseContractException('unsupported protocol name');
    }
    return ProtocolVersion(
      major: _int(map, 'major', min: 1),
      minor: _int(map, 'minor'),
    );
  }

  void validate() {
    if (major != openMuseContractMajor || minor > openMuseContractMinor) {
      throw OpenMuseContractException(
        'incompatible protocol version $major.$minor',
      );
    }
  }

  Map<String, Object?> toJson() => {
        'name': openMuseContractProtocol,
        'major': major,
        'minor': minor,
      };
}

final class PrincipalRef {
  const PrincipalRef({required this.principalRef, required this.kind});

  final String principalRef;
  final PrincipalKind kind;

  factory PrincipalRef.fromJson(Object? value, String field) {
    final map = _object(value, field);
    _keys(map, required: const {'principalRef', 'kind'});
    return PrincipalRef(
      principalRef: _ref(map, 'principalRef'),
      kind: _enumValue(map, 'kind', PrincipalKind.values),
    );
  }

  Map<String, Object?> toJson() => {
        'principalRef': principalRef,
        'kind': kind.name,
      };
}

final class ContractScope {
  const ContractScope({
    required this.authorityRef,
    this.workspaceRef,
    this.resourceRef,
  });

  final String authorityRef;
  final String? workspaceRef;
  final String? resourceRef;

  factory ContractScope.fromJson(Object? value) {
    final map = _object(value, 'scope');
    _keys(
      map,
      required: const {'authorityRef'},
      optional: const {'workspaceRef', 'resourceRef'},
    );
    return ContractScope(
      authorityRef: _ref(map, 'authorityRef'),
      workspaceRef: _optionalRef(map, 'workspaceRef'),
      resourceRef: _optionalRef(map, 'resourceRef'),
    );
  }

  Map<String, Object?> toJson() => {
        'authorityRef': authorityRef,
        if (workspaceRef != null) 'workspaceRef': workspaceRef,
        if (resourceRef != null) 'resourceRef': resourceRef,
      };
}

sealed class ContractEnvelope {
  const ContractEnvelope({
    required this.protocol,
    required this.requestId,
    required this.actor,
    required this.caller,
    required this.scope,
    required this.generation,
    required this.deadlineAtMs,
    required this.cancellationRef,
  });

  final ProtocolVersion protocol;
  final String requestId;
  final PrincipalRef actor;
  final PrincipalRef caller;
  final ContractScope scope;
  final int generation;
  final int deadlineAtMs;
  final String cancellationRef;

  Map<String, Object?> toJson();

  void validateGeneration(int expectedGeneration) {
    if (generation != expectedGeneration) {
      throw OpenMuseContractException(
        'stale generation: expected $expectedGeneration, got $generation',
      );
    }
  }
}

final class RequestEnvelope extends ContractEnvelope {
  const RequestEnvelope({
    required super.protocol,
    required super.requestId,
    required super.actor,
    required super.caller,
    required super.scope,
    required super.generation,
    required super.deadlineAtMs,
    required super.cancellationRef,
    required this.operation,
    required this.payload,
  });

  final String operation;
  final Object? payload;

  factory RequestEnvelope.fromJson(
    Map<String, Object?> map, {
    int? expectedGeneration,
  }) {
    _keys(map, required: const {
      'protocol',
      'kind',
      'requestId',
      'actor',
      'caller',
      'scope',
      'generation',
      'deadlineAtMs',
      'cancellationRef',
      'operation',
      'payload',
    });
    if (map['kind'] != 'request') {
      throw const OpenMuseContractException('kind must be request');
    }
    final envelope = RequestEnvelope(
      protocol: ProtocolVersion.fromJson(map['protocol']),
      requestId: _ref(map, 'requestId'),
      actor: PrincipalRef.fromJson(map['actor'], 'actor'),
      caller: PrincipalRef.fromJson(map['caller'], 'caller'),
      scope: ContractScope.fromJson(map['scope']),
      generation: _int(map, 'generation', min: 1),
      deadlineAtMs: _int(map, 'deadlineAtMs', min: 1),
      cancellationRef: _ref(map, 'cancellationRef'),
      operation: _ref(map, 'operation'),
      payload: _copy(map['payload']),
    );
    envelope.protocol.validate();
    if (expectedGeneration != null) {
      envelope.validateGeneration(expectedGeneration);
    }
    return envelope;
  }

  void ensureLiveAt(int nowMs) {
    if (nowMs >= deadlineAtMs) {
      throw OpenMuseContractException(
        'deadline $deadlineAtMs has expired at $nowMs',
      );
    }
  }

  @override
  Map<String, Object?> toJson() => {
        'protocol': protocol.toJson(),
        'kind': 'request',
        'requestId': requestId,
        'actor': actor.toJson(),
        'caller': caller.toJson(),
        'scope': scope.toJson(),
        'generation': generation,
        'deadlineAtMs': deadlineAtMs,
        'cancellationRef': cancellationRef,
        'operation': operation,
        'payload': _copy(payload),
      };
}

sealed class ContractOutcome {
  const ContractOutcome(this.receipt);

  final Receipt receipt;
  Map<String, Object?> toJson();
}

final class OkOutcome extends ContractOutcome {
  const OkOutcome(super.receipt, this.value);
  final Object? value;

  @override
  Map<String, Object?> toJson() => {
        'status': 'ok',
        'receipt': receipt.toJson(),
        'value': _copy(value),
      };
}

final class ErrorOutcome extends ContractOutcome {
  const ErrorOutcome(super.receipt, this.error);
  final ContractError error;

  @override
  Map<String, Object?> toJson() => {
        'status': 'error',
        'receipt': receipt.toJson(),
        'error': error.toJson(),
      };
}

final class ResponseEnvelope extends ContractEnvelope {
  const ResponseEnvelope({
    required super.protocol,
    required super.requestId,
    required super.actor,
    required super.caller,
    required super.scope,
    required super.generation,
    required super.deadlineAtMs,
    required super.cancellationRef,
    required this.outcome,
  });

  final ContractOutcome outcome;

  factory ResponseEnvelope.fromJson(
    Map<String, Object?> map, {
    int? expectedGeneration,
  }) {
    _keys(map, required: const {
      'protocol',
      'kind',
      'requestId',
      'actor',
      'caller',
      'scope',
      'generation',
      'deadlineAtMs',
      'cancellationRef',
      'outcome',
    });
    if (map['kind'] != 'response') {
      throw const OpenMuseContractException('kind must be response');
    }
    final requestId = _ref(map, 'requestId');
    final generation = _int(map, 'generation', min: 1);
    final outcome = _outcome(map['outcome']);
    if (outcome.receipt.requestId != requestId ||
        outcome.receipt.generation != generation) {
      throw const OpenMuseContractException(
        'receipt does not match its envelope',
      );
    }
    if (outcome is OkOutcome &&
        outcome.receipt.state != ReceiptState.committed) {
      throw const OpenMuseContractException('ok receipt must be committed');
    }
    if (outcome is ErrorOutcome &&
        outcome.receipt.state == ReceiptState.committed) {
      throw const OpenMuseContractException(
          'error receipt cannot be committed');
    }
    final envelope = ResponseEnvelope(
      protocol: ProtocolVersion.fromJson(map['protocol']),
      requestId: requestId,
      actor: PrincipalRef.fromJson(map['actor'], 'actor'),
      caller: PrincipalRef.fromJson(map['caller'], 'caller'),
      scope: ContractScope.fromJson(map['scope']),
      generation: generation,
      deadlineAtMs: _int(map, 'deadlineAtMs', min: 1),
      cancellationRef: _ref(map, 'cancellationRef'),
      outcome: outcome,
    );
    envelope.protocol.validate();
    if (expectedGeneration != null) {
      envelope.validateGeneration(expectedGeneration);
    }
    return envelope;
  }

  @override
  Map<String, Object?> toJson() => {
        'protocol': protocol.toJson(),
        'kind': 'response',
        'requestId': requestId,
        'actor': actor.toJson(),
        'caller': caller.toJson(),
        'scope': scope.toJson(),
        'generation': generation,
        'deadlineAtMs': deadlineAtMs,
        'cancellationRef': cancellationRef,
        'outcome': outcome.toJson(),
      };
}

final class ContractError {
  const ContractError({
    required this.code,
    required this.message,
    required this.retryable,
    required this.details,
  });

  final ContractErrorCode code;
  final String message;
  final bool retryable;
  final Map<String, Object?> details;

  factory ContractError.fromJson(Object? value) {
    final map = _object(value, 'error');
    _keys(map, required: const {'code', 'message', 'retryable', 'details'});
    return ContractError(
      code: _errorCode(_string(map, 'code')),
      message: _string(map, 'message'),
      retryable: _bool(map, 'retryable'),
      details: _copyMap(_object(map['details'], 'details')),
    );
  }

  Map<String, Object?> toJson() => {
        'code': _errorCodeWire(code),
        'message': message,
        'retryable': retryable,
        'details': _copyMap(details),
      };
}

final class Receipt {
  const Receipt({
    required this.receiptRef,
    required this.requestId,
    required this.generation,
    required this.state,
    required this.issuedAtMs,
    required this.effects,
  });

  final String receiptRef;
  final String requestId;
  final int generation;
  final ReceiptState state;
  final int issuedAtMs;
  final List<String> effects;

  factory Receipt.fromJson(Object? value) {
    final map = _object(value, 'receipt');
    _keys(map, required: const {
      'receiptRef',
      'requestId',
      'generation',
      'state',
      'issuedAtMs',
      'effects',
    });
    return Receipt(
      receiptRef: _ref(map, 'receiptRef'),
      requestId: _ref(map, 'requestId'),
      generation: _int(map, 'generation', min: 1),
      state: _enumValue(map, 'state', ReceiptState.values),
      issuedAtMs: _int(map, 'issuedAtMs', min: 1),
      effects: _stringList(map['effects'], 'effects'),
    );
  }

  Map<String, Object?> toJson() => {
        'receiptRef': receiptRef,
        'requestId': requestId,
        'generation': generation,
        'state': state.name,
        'issuedAtMs': issuedAtMs,
        'effects': [...effects],
      };
}

final class Descriptor {
  const Descriptor({
    required this.descriptorRef,
    required this.generation,
    required this.revision,
    required this.issuedAtMs,
    required this.value,
  });

  final String descriptorRef;
  final int generation;
  final String revision;
  final int issuedAtMs;
  final Object? value;

  factory Descriptor.fromJson(Object? value) {
    final map = _object(value, 'descriptor');
    _keys(map, required: const {
      'descriptorRef',
      'generation',
      'revision',
      'issuedAtMs',
      'value',
    });
    return Descriptor(
      descriptorRef: _ref(map, 'descriptorRef'),
      generation: _int(map, 'generation', min: 1),
      revision: _ref(map, 'revision'),
      issuedAtMs: _int(map, 'issuedAtMs', min: 1),
      value: _copy(map['value']),
    );
  }

  Map<String, Object?> toJson() => {
        'descriptorRef': descriptorRef,
        'generation': generation,
        'revision': revision,
        'issuedAtMs': issuedAtMs,
        'value': _copy(value),
      };
}

final class CapabilityHandle {
  const CapabilityHandle({
    required this.handleRef,
    required this.audience,
    required this.scope,
    required this.access,
    required this.generation,
    required this.issuedAtMs,
    required this.expiresAtMs,
    required this.state,
  });

  final String handleRef;
  final PrincipalRef audience;
  final ContractScope scope;
  final List<String> access;
  final int generation;
  final int issuedAtMs;
  final int expiresAtMs;
  final HandleState state;

  factory CapabilityHandle.fromJson(Object? value) {
    final map = _object(value, 'handle');
    _keys(map, required: const {
      'handleRef',
      'audience',
      'scope',
      'access',
      'generation',
      'issuedAtMs',
      'expiresAtMs',
      'state',
    });
    final issuedAtMs = _int(map, 'issuedAtMs', min: 1);
    final expiresAtMs = _int(map, 'expiresAtMs', min: 1);
    _interval(issuedAtMs, expiresAtMs);
    final access = _stringList(map['access'], 'access');
    if (access.isEmpty) {
      throw const OpenMuseContractException('access must not be empty');
    }
    return CapabilityHandle(
      handleRef: _ref(map, 'handleRef'),
      audience: PrincipalRef.fromJson(map['audience'], 'audience'),
      scope: ContractScope.fromJson(map['scope']),
      access: access,
      generation: _int(map, 'generation', min: 1),
      issuedAtMs: issuedAtMs,
      expiresAtMs: expiresAtMs,
      state: _enumValue(map, 'state', HandleState.values),
    );
  }

  Map<String, Object?> toJson() => {
        'handleRef': handleRef,
        'audience': audience.toJson(),
        'scope': scope.toJson(),
        'access': [...access],
        'generation': generation,
        'issuedAtMs': issuedAtMs,
        'expiresAtMs': expiresAtMs,
        'state': state.name,
      };
}

final class Lease {
  const Lease({
    required this.leaseRef,
    required this.holder,
    required this.scope,
    required this.generation,
    required this.issuedAtMs,
    required this.expiresAtMs,
    required this.state,
  });

  final String leaseRef;
  final PrincipalRef holder;
  final ContractScope scope;
  final int generation;
  final int issuedAtMs;
  final int expiresAtMs;
  final LeaseState state;

  factory Lease.fromJson(Object? value) {
    final map = _object(value, 'lease');
    _keys(map, required: const {
      'leaseRef',
      'holder',
      'scope',
      'generation',
      'issuedAtMs',
      'expiresAtMs',
      'state',
    });
    final issuedAtMs = _int(map, 'issuedAtMs', min: 1);
    final expiresAtMs = _int(map, 'expiresAtMs', min: 1);
    _interval(issuedAtMs, expiresAtMs);
    return Lease(
      leaseRef: _ref(map, 'leaseRef'),
      holder: PrincipalRef.fromJson(map['holder'], 'holder'),
      scope: ContractScope.fromJson(map['scope']),
      generation: _int(map, 'generation', min: 1),
      issuedAtMs: issuedAtMs,
      expiresAtMs: expiresAtMs,
      state: _enumValue(map, 'state', LeaseState.values),
    );
  }

  Map<String, Object?> toJson() => {
        'leaseRef': leaseRef,
        'holder': holder.toJson(),
        'scope': scope.toJson(),
        'generation': generation,
        'issuedAtMs': issuedAtMs,
        'expiresAtMs': expiresAtMs,
        'state': state.name,
      };
}

final class LifecycleSnapshot {
  const LifecycleSnapshot({
    required this.protocol,
    required this.descriptor,
    required this.handle,
    required this.lease,
    required this.receipt,
  });

  final ProtocolVersion protocol;
  final Descriptor descriptor;
  final CapabilityHandle handle;
  final Lease lease;
  final Receipt receipt;

  factory LifecycleSnapshot.fromJson(Object? value) {
    final map = _object(value, 'lifecycle');
    _keys(map, required: const {
      'protocol',
      'descriptor',
      'handle',
      'lease',
      'receipt',
    });
    final protocol = ProtocolVersion.fromJson(map['protocol']);
    protocol.validate();
    return LifecycleSnapshot(
      protocol: protocol,
      descriptor: Descriptor.fromJson(map['descriptor']),
      handle: CapabilityHandle.fromJson(map['handle']),
      lease: Lease.fromJson(map['lease']),
      receipt: Receipt.fromJson(map['receipt']),
    );
  }

  Map<String, Object?> toJson() => {
        'protocol': protocol.toJson(),
        'descriptor': descriptor.toJson(),
        'handle': handle.toJson(),
        'lease': lease.toJson(),
        'receipt': receipt.toJson(),
      };
}

ContractEnvelope parseContractEnvelope(
  Object? value, {
  int? expectedGeneration,
}) {
  final map = _object(value, 'envelope');
  return switch (map['kind']) {
    'request' => RequestEnvelope.fromJson(
        map,
        expectedGeneration: expectedGeneration,
      ),
    'response' => ResponseEnvelope.fromJson(
        map,
        expectedGeneration: expectedGeneration,
      ),
    _ => throw const OpenMuseContractException(
        'kind must be request or response',
      ),
  };
}

ContractOutcome _outcome(Object? value) {
  final map = _object(value, 'outcome');
  final status = map['status'];
  if (status == 'ok') {
    _keys(map, required: const {'status', 'receipt', 'value'});
    return OkOutcome(Receipt.fromJson(map['receipt']), _copy(map['value']));
  }
  if (status == 'error') {
    _keys(map, required: const {'status', 'receipt', 'error'});
    return ErrorOutcome(
      Receipt.fromJson(map['receipt']),
      ContractError.fromJson(map['error']),
    );
  }
  throw const OpenMuseContractException('unsupported outcome status');
}

Map<String, Object?> _object(Object? value, String field) {
  if (value is! Map) {
    throw OpenMuseContractException('$field must be an object');
  }
  try {
    return value.cast<String, Object?>();
  } on TypeError {
    throw OpenMuseContractException('$field must use string keys');
  }
}

void _keys(
  Map<String, Object?> map, {
  required Set<String> required,
  Set<String> optional = const {},
}) {
  final actual = map.keys.toSet();
  final missing = required.difference(actual);
  if (missing.isNotEmpty) {
    throw OpenMuseContractException('missing fields: ${missing.join(', ')}');
  }
  final unknown = actual.difference(required.union(optional));
  if (unknown.isNotEmpty) {
    throw OpenMuseContractException('unknown fields: ${unknown.join(', ')}');
  }
}

String _string(Map<String, Object?> map, String field) {
  final value = map[field];
  if (value is! String || value.isEmpty) {
    throw OpenMuseContractException('$field must be a non-empty string');
  }
  return value;
}

String _ref(Map<String, Object?> map, String field) {
  final value = _string(map, field);
  if (RegExp(r'\s').hasMatch(value)) {
    throw OpenMuseContractException('$field must be an opaque reference');
  }
  return value;
}

String? _optionalRef(Map<String, Object?> map, String field) {
  if (!map.containsKey(field)) return null;
  return _ref(map, field);
}

int _int(Map<String, Object?> map, String field, {int min = 0}) {
  final value = map[field];
  if (value is! int || value < min || value > _maxSafeInteger) {
    throw OpenMuseContractException(
      '$field must be a safe integer >= $min',
    );
  }
  return value;
}

bool _bool(Map<String, Object?> map, String field) {
  final value = map[field];
  if (value is! bool) {
    throw OpenMuseContractException('$field must be a boolean');
  }
  return value;
}

T _enumValue<T extends Enum>(
  Map<String, Object?> map,
  String field,
  List<T> values,
) {
  final wire = _string(map, field);
  for (final value in values) {
    if (value.name == wire) return value;
  }
  throw OpenMuseContractException('$field has unsupported value $wire');
}

List<String> _stringList(Object? value, String field) {
  if (value is! List) {
    throw OpenMuseContractException('$field must be an array');
  }
  final items = value.map((item) {
    if (item is! String || item.isEmpty || RegExp(r'\s').hasMatch(item)) {
      throw OpenMuseContractException(
        '$field must contain opaque references',
      );
    }
    return item;
  }).toList(growable: false);
  if (items.toSet().length != items.length) {
    throw OpenMuseContractException('$field must contain unique values');
  }
  return items;
}

void _interval(int issuedAtMs, int expiresAtMs) {
  if (expiresAtMs <= issuedAtMs) {
    throw const OpenMuseContractException(
      'expiresAtMs must be greater than issuedAtMs',
    );
  }
}

ContractErrorCode _errorCode(String value) => switch (value) {
      'DENIED' => ContractErrorCode.denied,
      'NOT_FOUND' => ContractErrorCode.notFound,
      'CONFLICT' => ContractErrorCode.conflict,
      'EXPIRED' => ContractErrorCode.expired,
      'STALE_GENERATION' => ContractErrorCode.staleGeneration,
      'UNAVAILABLE' => ContractErrorCode.unavailable,
      'TRANSIENT' => ContractErrorCode.transient,
      'INTEGRITY_FAILED' => ContractErrorCode.integrityFailed,
      _ => throw OpenMuseContractException('unsupported error code $value'),
    };

String _errorCodeWire(ContractErrorCode value) => switch (value) {
      ContractErrorCode.denied => 'DENIED',
      ContractErrorCode.notFound => 'NOT_FOUND',
      ContractErrorCode.conflict => 'CONFLICT',
      ContractErrorCode.expired => 'EXPIRED',
      ContractErrorCode.staleGeneration => 'STALE_GENERATION',
      ContractErrorCode.unavailable => 'UNAVAILABLE',
      ContractErrorCode.transient => 'TRANSIENT',
      ContractErrorCode.integrityFailed => 'INTEGRITY_FAILED',
    };

Map<String, Object?> _copyMap(Map<String, Object?> value) => value.map(
      (key, item) => MapEntry(key, _copy(item)),
    );

Object? _copy(Object? value) => switch (value) {
      Map() => _copyMap(value.cast<String, Object?>()),
      List() => value.map(_copy).toList(growable: false),
      _ => value,
    };
