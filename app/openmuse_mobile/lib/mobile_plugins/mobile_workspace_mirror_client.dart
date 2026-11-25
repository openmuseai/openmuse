import '../mobile_build_locale.dart';

import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

import '../remote_dsh_page.dart';

/// Mobile adapter for the Desktop-owned opaque workspace mirror protocol.
/// The paired grant is never passed into plugin HTML or JavaScript.
final class MobileWorkspaceMirrorClient {
  MobileWorkspaceMirrorClient(
    this.connection, {
    HttpClient? client,
    OpenMuseStrings? strings,
    this.idleTimeout = const Duration(seconds: 45),
    this.totalTimeout = const Duration(minutes: 5),
  }) : _client = client ?? HttpClient(),
       _strings = strings ?? OpenMuseStrings.fallback;

  final OpenMuseStrings _strings;

  final PairedDesktopConnection connection;
  final HttpClient _client;
  final Duration idleTimeout;
  final Duration totalTimeout;

  Uri get _origin {
    final target = resolveRemoteDshUri(connection.session);
    if (connection.expiresAtMs <= DateTime.now().millisecondsSinceEpoch) {
      throw StateError(
        (_strings.mobileDesktopAuthorizationHasExpiredCff089e8()),
      );
    }
    return Uri(
      scheme: target.scheme,
      host: target.host,
      port: target.hasPort ? target.port : null,
    );
  }

  Future<Map<String, Object?>> mounts() =>
      _get({'operation': 'mounts', 'workspaceRef': connection.workspaceRef});

  Future<Map<String, Object?>> children({
    required String mountRef,
    required String parentRef,
    String? cursor,
  }) => _get({
    'operation': 'children',
    'workspaceRef': connection.workspaceRef,
    'mountRef': mountRef,
    'parentRef': parentRef,
    'limit': '100',
    'cursor': ?cursor,
  });

  Future<Map<String, Object?>> search(String query) {
    final trimmed = query.trim();
    if (trimmed.isEmpty || trimmed.length > 120) {
      throw FormatException(
        (_strings.mobileInvalidSearchQueryLength9db7d4a7()),
      );
    }
    return _get({
      'operation': 'search',
      'workspaceRef': connection.workspaceRef,
      'limit': '100',
      'q': trimmed,
    });
  }

  Future<Map<String, Object?>> preview(String resourceRef) async {
    final payload = await readResource(resourceRef);
    final mediaType = payload['mediaType'] as String;
    final bytes = payload['bytes'] as Uint8List;
    if (bytes.length > 2 * 1024 * 1024) {
      throw FormatException(
        (_strings.mobileThisFileIsTooLargeForAnInlinePrevi9350b84b()),
      );
    }
    if (mediaType.startsWith('text/') ||
        mediaType.startsWith('application/json')) {
      return {
        'type': 'text',
        'mediaType': mediaType,
        'content': utf8.decode(bytes, allowMalformed: true),
      };
    }
    if (const {
      'image/png',
      'image/jpeg',
      'image/gif',
      'image/webp',
    }.contains(mediaType.split(';').first.trim())) {
      return {
        'type': 'image',
        'mediaType': mediaType,
        'dataUrl': 'data:$mediaType;base64,${base64Encode(bytes)}',
      };
    }
    return {'type': 'unsupported', 'mediaType': mediaType};
  }

  Future<Map<String, Object?>> readResource(
    String resourceRef, {
    void Function(String step, int received, int? total)? onProgress,
  }) async {
    if (resourceRef.isEmpty || resourceRef.length > 128) {
      throw FormatException(
        (_strings.mobileInvalidResourceReferenceD9a208e5()),
      );
    }
    final uri = _origin
        .resolve('/openmuse/workspace-mirror/resource/v1')
        .replace(
          queryParameters: {
            'workspaceRef': connection.workspaceRef,
            'resourceRef': resourceRef,
          },
        );
    final response = await _request(uri, onProgress: onProgress);
    final mediaType = response.$1;
    final bytes = response.$2;
    if (bytes.length > 64 * 1024 * 1024) {
      throw FormatException(
        (_strings.mobileTheFileExceedsThe64MibLimitAd41f580()),
      );
    }
    return {'mediaType': mediaType, 'bytes': bytes};
  }

  Future<Map<String, Object?>> _get(Map<String, String> query) async {
    final uri = _origin
        .resolve('/openmuse/workspace-mirror/v1')
        .replace(queryParameters: query);
    final (_, bytes) = await _request(uri);
    final value = jsonDecode(utf8.decode(bytes));
    if (value is! Map) {
      throw FormatException(
        (_strings.mobileInvalidDirectoryResponseEf6bbe6d()),
      );
    }
    return Map<String, Object?>.from(value);
  }

  Future<(String, Uint8List)> _request(
    Uri uri, {
    void Function(String step, int received, int? total)? onProgress,
  }) async {
    var step = (_strings.mobileConnectingToDesktop6fa59c15());
    HttpClientRequest? activeRequest;
    final deadline = Timer(totalTimeout, () {
      activeRequest?.abort(
        TimeoutException((_strings.mobileTheFileTransferTimedOut3f953668())),
      );
      _client.close(force: true);
    });
    try {
      onProgress?.call(step, 0, null);
      final request = activeRequest = await _client
          .getUrl(uri)
          .timeout(idleTimeout);
      request.headers.set(
        HttpHeaders.cookieHeader,
        'OpenMuse-Paired=${connection.grantRef}',
      );
      step = (_strings.mobileWaitingForDesktopToReadTheFile2407e9d7());
      onProgress?.call(step, 0, null);
      final response = await request.close().timeout(idleTimeout);
      if (response.statusCode != HttpStatus.ok) {
        final message = switch (response.statusCode) {
          401 || 403 =>
            (_strings.mobileDesktopAuthorizationHasExpiredReconnecA267882b()),
          400 || 404 =>
            (_strings.mobileThisFileReferenceHasExpiredRefreshTh19c36658()),
          502 || 503 || 504 =>
            (_strings.mobileTheDesktopRelayIsUnavailableReconnectEbf1165a()),
          _ => (_strings.mobileTheDesktopRequestFailedHttpEb760a86(
            value1: response.statusCode,
          )),
        };
        throw StateError(message);
      }
      step = (_strings.mobileReceivingTheDesktopFileDb4f8afd());
      final total = response.contentLength >= 0 ? response.contentLength : null;
      if (total != null && total > 64 * 1024 * 1024) {
        throw FormatException(
          (_strings.mobileTheDesktopResponseExceedsThe64MibLiDc5ead0a()),
        );
      }
      final bytes = BytesBuilder(copy: false);
      onProgress?.call(step, 0, total);
      await for (final chunk in response.timeout(idleTimeout)) {
        if (bytes.length + chunk.length > 64 * 1024 * 1024) {
          throw FormatException(
            (_strings.mobileTheDesktopResponseExceedsThe64MibLiDc5ead0a()),
          );
        }
        bytes.add(chunk);
        onProgress?.call(step, bytes.length, total);
      }
      return (
        response.headers.contentType?.toString() ?? 'application/octet-stream',
        bytes.takeBytes(),
      );
    } on TimeoutException {
      activeRequest?.abort();
      throw StateError(
        (_strings.mobileTimedOutCheckYourNetworkAndRetry031c4f34(value1: step)),
      );
    } on SocketException {
      throw StateError(
        (_strings.mobileWasInterruptedReconnectDesktopAndRetr38590292(
          value1: step,
        )),
      );
    } on HttpException {
      throw StateError(
        (_strings.mobileWasInterruptedTryAgain217c79d1(value1: step)),
      );
    } finally {
      deadline.cancel();
    }
  }

  void close() => _client.close(force: true);
}
