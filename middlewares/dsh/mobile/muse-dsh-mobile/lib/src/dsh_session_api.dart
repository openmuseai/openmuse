import 'dart:convert';

class DshSessionOpen {
  const DshSessionOpen({
    required this.sessionRef,
    this.instanceRef,
    this.webUrl,
    this.expiresAt,
    this.queuePosition,
    this.retryAfterMs,
    this.nodeId = 'local',
  });

  final String sessionRef;
  final String? instanceRef;
  final String? webUrl;
  final int? expiresAt;
  final int? queuePosition;
  final int? retryAfterMs;
  final String nodeId;

  bool get isQueued => queuePosition != null && webUrl == null;

  factory DshSessionOpen.fromJson(Map<String, dynamic> json) {
    return DshSessionOpen(
      sessionRef: json['sessionRef'] as String? ?? '',
      instanceRef: json['instanceRef'] as String?,
      webUrl: json['webUrl'] as String?,
      expiresAt: json['expiresAt'] as int?,
      queuePosition: json['queuePosition'] as int?,
      retryAfterMs: json['retryAfterMs'] as int?,
      nodeId: json['nodeId'] as String? ?? 'local',
    );
  }
}

/// Cloud BFF client. Inject [post] so tests never hit the network.
class DshSessionApi {
  DshSessionApi({
    required this.cloudOrigin,
    required this.accessToken,
    required this.post,
  });

  final Uri cloudOrigin;
  final String accessToken;
  final Future<Map<String, dynamic>> Function(
    Uri uri,
    Map<String, String> headers,
    String body,
  ) post;

  Future<DshSessionOpen> open({
    required String workspaceRef,
    required String deviceId,
  }) async {
    final json = await post(
      cloudOrigin.resolve('/api/muse/dsh/session/open'),
      {
        'Authorization': 'Bearer $accessToken',
        'Content-Type': 'application/json',
      },
      jsonEncode({'workspaceId': workspaceRef, 'deviceId': deviceId}),
    );
    final data = json['data'] is Map<String, dynamic>
        ? json['data'] as Map<String, dynamic>
        : json;
    return DshSessionOpen.fromJson(data);
  }

  Future<void> close({required String sessionRef, required String deviceId}) async {
    await post(
      cloudOrigin.resolve('/api/muse/dsh/session/close'),
      {
        'Authorization': 'Bearer $accessToken',
        'Content-Type': 'application/json',
      },
      jsonEncode({'sessionRef': sessionRef, 'deviceId': deviceId}),
    );
  }

  Future<void> heartbeat({required String sessionRef}) async {
    await post(
      cloudOrigin.resolve('/api/muse/dsh/session/heartbeat'),
      {
        'Authorization': 'Bearer $accessToken',
        'Content-Type': 'application/json',
      },
      jsonEncode({'sessionRef': sessionRef}),
    );
  }
}
