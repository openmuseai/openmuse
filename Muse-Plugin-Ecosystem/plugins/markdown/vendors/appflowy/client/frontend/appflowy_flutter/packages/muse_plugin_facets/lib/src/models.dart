enum MuseFacetSchemaKind {
  pluginDescriptor('plugin-descriptor'),
  contextContribution('context-contribution'),
  domainChange('domain-change'),
  presentationIntent('presentation-intent'),
  presentationIntentResult('presentation-intent-result');

  const MuseFacetSchemaKind(this.wireName);
  final String wireName;

  static MuseFacetSchemaKind parse(String value) => values.singleWhere(
        (kind) => kind.wireName == value,
        orElse: () =>
            throw FormatException('Unknown Facet schema kind: $value'),
      );
}

enum MuseContextLane { control, state }

enum MuseMutationOrigin {
  uiOptimistic('ui-optimistic'),
  remoteCollab('remote-collab'),
  externalCommand('external-command'),
  recoveryReplay('recovery-replay'),
  unknown('unknown');

  const MuseMutationOrigin(this.wireName);
  final String wireName;

  static MuseMutationOrigin parse(String value) => values.singleWhere(
        (origin) => origin.wireName == value,
        orElse: () => throw FormatException('Unknown mutation origin: $value'),
      );
}

class MuseContextContributionV1 {
  factory MuseContextContributionV1.fromJson(Object? input) {
    final value = (input as Map).cast<String, Object?>();
    if (value['protocol'] != 'muse.context-contribution/v1')
      throw const FormatException('Invalid context protocol');
    return MuseContextContributionV1(
      pluginId: value['pluginId'] as String,
      pluginVersion: value['pluginVersion'] as String,
      facetInstanceRef: value['facetInstanceRef'] as String,
      surfaceInstanceRef: value['surfaceInstanceRef'] as String,
      surfaceKind: value['surfaceKind'] as String,
      scopeRef: value['scopeRef'] as String,
      contextType: value['contextType'] as String,
      contextSchemaDigest: value['contextSchemaDigest'] as String,
      contextRevision: value['contextRevision'] as String,
      epochRef: value['epochRef'] as String,
      lane: MuseContextLane.values.byName(value['lane'] as String),
      capturedAt: value['capturedAt'] as int,
      expiresAt: value['expiresAt'] as int,
      payload: value['payload'],
    );
  }
  const MuseContextContributionV1({
    required this.pluginId,
    required this.pluginVersion,
    required this.facetInstanceRef,
    required this.surfaceInstanceRef,
    required this.surfaceKind,
    required this.scopeRef,
    required this.contextType,
    required this.contextSchemaDigest,
    required this.contextRevision,
    required this.epochRef,
    required this.lane,
    required this.capturedAt,
    required this.expiresAt,
    required this.payload,
  });

  final String pluginId;
  final String pluginVersion;
  final String facetInstanceRef;
  final String surfaceInstanceRef;
  final String surfaceKind;
  final String scopeRef;
  final String contextType;
  final String contextSchemaDigest;
  final String contextRevision;
  final String epochRef;
  final MuseContextLane lane;
  final int capturedAt;
  final int expiresAt;
  final Object? payload;

  Map<String, Object?> toJson() => {
        'protocol': 'muse.context-contribution/v1',
        'pluginId': pluginId,
        'pluginVersion': pluginVersion,
        'facetInstanceRef': facetInstanceRef,
        'surfaceInstanceRef': surfaceInstanceRef,
        'surfaceKind': surfaceKind,
        'scopeRef': scopeRef,
        'contextType': contextType,
        'contextSchemaDigest': contextSchemaDigest,
        'contextRevision': contextRevision,
        'epochRef': epochRef,
        'lane': lane.name,
        'capturedAt': capturedAt,
        'expiresAt': expiresAt,
        'payload': payload,
      };
}

class MuseDomainChangeV1 {
  const MuseDomainChangeV1({
    required this.pluginId,
    required this.providerInstanceRef,
    required this.scopeRef,
    required this.resourceRef,
    required this.eventType,
    required this.eventSchemaDigest,
    required this.domainRevision,
    required this.epochRef,
    required this.origin,
    required this.occurredAt,
    required this.payload,
    this.commandRef,
  });

  final String pluginId;
  final String providerInstanceRef;
  final String scopeRef;
  final String resourceRef;
  final String eventType;
  final String eventSchemaDigest;
  final String domainRevision;
  final String epochRef;
  final String? commandRef;
  final MuseMutationOrigin origin;
  final int occurredAt;
  final Object? payload;

  Map<String, Object?> toJson() => {
        'protocol': 'muse.domain-change/v1',
        'pluginId': pluginId,
        'providerInstanceRef': providerInstanceRef,
        'scopeRef': scopeRef,
        'resourceRef': resourceRef,
        'eventType': eventType,
        'eventSchemaDigest': eventSchemaDigest,
        'domainRevision': domainRevision,
        'epochRef': epochRef,
        if (commandRef != null) 'commandRef': commandRef,
        'origin': origin.wireName,
        'occurredAt': occurredAt,
        'payload': payload,
      };

  factory MuseDomainChangeV1.fromJson(Object? input) {
    final value = (input as Map).map(
      (key, value) => MapEntry(key.toString(), value),
    );
    return MuseDomainChangeV1(
      pluginId: value['pluginId'] as String,
      providerInstanceRef: value['providerInstanceRef'] as String,
      scopeRef: value['scopeRef'] as String,
      resourceRef: value['resourceRef'] as String,
      eventType: value['eventType'] as String,
      eventSchemaDigest: value['eventSchemaDigest'] as String,
      domainRevision: value['domainRevision'] as String,
      epochRef: value['epochRef'] as String,
      commandRef: value['commandRef'] as String?,
      origin: MuseMutationOrigin.parse(value['origin'] as String),
      occurredAt: value['occurredAt'] as int,
      payload: value['payload'],
    );
  }
}

enum MusePresentationIntentStatus {
  applied('applied'),
  rejected('rejected'),
  stale('stale'),
  notFound('not-found'),
  notSupported('not-supported'),
  surfaceClosed('surface-closed'),
  timedOut('timed-out');

  const MusePresentationIntentStatus(this.wireName);
  final String wireName;
}

class MusePresentationIntentV1 {
  factory MusePresentationIntentV1.fromJson(Object? input) {
    final value = (input as Map).cast<String, Object?>();
    if (value['protocol'] != 'muse.presentation-intent/v1')
      throw const FormatException('Invalid intent protocol');
    return MusePresentationIntentV1(
      pluginId: value['pluginId'] as String,
      scopeRef: value['scopeRef'] as String,
      intentType: value['intentType'] as String,
      intentSchemaDigest: value['intentSchemaDigest'] as String,
      intentRef: value['intentRef'] as String,
      requestedAt: value['requestedAt'] as int,
      expiresAt: value['expiresAt'] as int,
      payload: value['payload'],
      targetSurfaceInstanceRef: value['targetSurfaceInstanceRef'] as String?,
    );
  }
  const MusePresentationIntentV1({
    required this.pluginId,
    required this.scopeRef,
    required this.intentType,
    required this.intentSchemaDigest,
    required this.intentRef,
    required this.requestedAt,
    required this.expiresAt,
    required this.payload,
    this.targetSurfaceInstanceRef,
  });

  final String pluginId;
  final String? targetSurfaceInstanceRef;
  final String scopeRef;
  final String intentType;
  final String intentSchemaDigest;
  final String intentRef;
  final int requestedAt;
  final int expiresAt;
  final Object? payload;

  Map<String, Object?> toJson() => {
        'protocol': 'muse.presentation-intent/v1',
        'pluginId': pluginId,
        if (targetSurfaceInstanceRef != null)
          'targetSurfaceInstanceRef': targetSurfaceInstanceRef,
        'scopeRef': scopeRef,
        'intentType': intentType,
        'intentSchemaDigest': intentSchemaDigest,
        'intentRef': intentRef,
        'requestedAt': requestedAt,
        'expiresAt': expiresAt,
        'payload': payload,
      };
}

class MusePresentationIntentResultV1 {
  const MusePresentationIntentResultV1({
    required this.intentRef,
    required this.status,
    required this.completedAt,
    this.appliedSurfaceInstanceRef,
    this.observedDomainRevision,
    this.reasonCode,
  });

  final String intentRef;
  final MusePresentationIntentStatus status;
  final String? appliedSurfaceInstanceRef;
  final String? observedDomainRevision;
  final String? reasonCode;
  final int completedAt;

  Map<String, Object?> toJson() => {
        'protocol': 'muse.presentation-intent-result/v1',
        'intentRef': intentRef,
        'status': status.wireName,
        if (appliedSurfaceInstanceRef != null)
          'appliedSurfaceInstanceRef': appliedSurfaceInstanceRef,
        if (observedDomainRevision != null)
          'observedDomainRevision': observedDomainRevision,
        if (reasonCode != null) 'reasonCode': reasonCode,
        'completedAt': completedAt,
      };
}
