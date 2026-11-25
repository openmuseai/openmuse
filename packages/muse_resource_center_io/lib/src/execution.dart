import 'dart:async';
import 'dart:convert';
import 'package:muse_resource_center/muse_resource_center.dart';
import 'projection.dart';

/// Compatibility adapter for runtimes that currently accept text content only.
/// Exact bytes are projected on the execution device; a resource plugin extracts
/// bounded context. Only canonical links and extracted text enter the transcript.
/// Projection paths remain private and are never treated as resource identities.
final class ResourceExecutionCoordinator {
  ResourceExecutionCoordinator({
    required this.hub,
    required this.projections,
    required this.extractText,
  });
  final ResourceCenterHub hub;
  final ResourceProjectionManager projections;
  final Future<String?> Function(ResourceContent) extractText;
  final _requests = <String, Future<Map<String, Object?>>>{};
  final _fingerprints = <String, String>{};
  final _completed = <String>{};
  bool _closed = false;
  void _checkOpen() {
    if (_closed) throw const ResourceFailure('SOURCE_UNAVAILABLE');
  }

  Future<Map<String, Object?>> prepare(
    List<ResourceReference> refs,
    String key,
  ) {
    _checkOpen();
    final fingerprint = jsonEncode(refs.map((r) => r.toJson()).toList());
    if (_fingerprints[key] != null && _fingerprints[key] != fingerprint)
      throw const ResourceFailure('IDEMPOTENCY_CONFLICT');
    // Preparation is a read-only operation. Keep bounded retry history rather
    // than permanently exhausting a long-running Host after 100 conversations.
    if (_requests.length >= 100 && !_requests.containsKey(key)) {
      if (_completed.isEmpty) throw const ResourceFailure('EXECUTION_LIMIT');
      final oldest = _completed.first;
      _completed.remove(oldest);
      _requests.remove(oldest);
      _fingerprints.remove(oldest);
    }
    _fingerprints[key] = fingerprint;
    return _requests.putIfAbsent(key, () {
      final request = _prepare(refs);
      void completed() {
        if (!_closed) _completed.add(key);
      }

      unawaited(
        request.then<void>(
          (_) => completed(),
          onError: (Object _, StackTrace __) => completed(),
        ),
      );
      return request;
    });
  }

  Future<Map<String, Object?>> _prepare(List<ResourceReference> refs) async {
    await projections.sweep();
    final leases = <ResourceProjection>[];
    try {
      final chunks = <String>[];
      var total = 0;
      for (final ref in refs) {
        _checkOpen();
        if (!ref.pinned) throw const ResourceFailure('VERSION_REQUIRED');
        final opened = await hub.resolve(
          ref,
          hub.access(
            'conversation-execution',
            operations: {ResourceOperation.read},
            resourceKeys: {
              ref.identity.key,
              if (ref.retained != null) ref.retained!.identity.key,
            },
            allowOfflineReplica: true,
          ),
        );
        final lease = await projections.materialize(opened);
        leases.add(lease);
        _checkOpen();
        // Read the execution projection, not the origin again. The plugin sees
        // verified bytes; it never receives the filesystem path or grant.
        final content = ResourceContent(
          opened.content.record,
          await lease.file.readAsBytes(),
        );
        final text = await extractText(content);
        if (text == null) throw const ResourceFailure('CONTEXT_UNAVAILABLE');
        total += utf8.encode(text).length;
        if (total > 32 * 1024) throw const ResourceFailure('CONTEXT_TOO_LARGE');
        final name = opened.content.record.name
            .replaceAll('[', '')
            .replaceAll(']', '')
            .replaceAll('\n', ' ');
        chunks.add(
          '[$name](${ref.toUri()})\n\n<resource-context digest="${ref.version!.digest}">\n$text\n</resource-context>',
        );
      }
      _checkOpen();
      return {
        'content': chunks.join('\n\n'),
        'references': refs.map((r) => r.toJson()).toList(),
      };
    } on Object {
      for (final lease in leases) {
        await lease.dispose();
      }
      rethrow;
    }
  }

  Future<void> close() async {
    _closed = true;
    await projections.clear();
    _requests.clear();
    _fingerprints.clear();
    _completed.clear();
  }
}
