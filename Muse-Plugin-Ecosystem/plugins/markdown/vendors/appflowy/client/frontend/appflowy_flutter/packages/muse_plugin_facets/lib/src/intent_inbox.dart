import 'dart:async';
import 'dart:convert';

import 'contracts.dart';
import 'models.dart';

typedef MuseIntentHandler = Future<MusePresentationIntentResultV1> Function(
  MusePresentationIntentV1 intent,
);

/// Runtime-neutral contract dispatch. Transports never select business handlers.
final class MuseIntentInbox {
  MuseIntentInbox({required this.scopeRef, int Function()? clock})
      : _clock = clock ?? (() => DateTime.now().millisecondsSinceEpoch);

  final String scopeRef;
  final int Function() _clock;
  final _routes = <String, ({String digest, MuseIntentHandler handler})>{};
  final _receipts = <String,
      ({String input, Future<MusePresentationIntentResultV1> result})>{};
  bool _closed = false;

  void register({
    required String pluginId,
    required String intentType,
    required String schemaDigest,
    required MuseIntentHandler handler,
  }) {
    final key = jsonEncode([pluginId, intentType]);
    if (_closed || _routes.containsKey(key)) {
      throw StateError('FACET_ALREADY_REGISTERED_OR_CLOSED');
    }
    _routes[key] = (digest: schemaDigest, handler: handler);
  }

  Future<MusePresentationIntentResultV1> dispatch(
    MusePresentationIntentV1 intent,
  ) {
    validateMuseFacetValue(
      MuseFacetSchemaKind.presentationIntent,
      intent.toJson(),
    );
    final input = museCanonicalJson(intent.toJson());
    if (utf8.encode(input).length > 32 * 1024) {
      return Future.value(
        _result(
          intent,
          MusePresentationIntentStatus.rejected,
          'PAYLOAD_TOO_LARGE',
        ),
      );
    }
    if (_closed) {
      return Future.value(
        _result(
          intent,
          MusePresentationIntentStatus.surfaceClosed,
          'INBOX_CLOSED',
        ),
      );
    }
    if (intent.scopeRef != scopeRef) {
      return Future.value(
        _result(
          intent,
          MusePresentationIntentStatus.rejected,
          'SCOPE_MISMATCH',
        ),
      );
    }
    final previous = _receipts[intent.intentRef];
    if (previous != null) {
      return previous.input == input
          ? previous.result
          : Future.value(
              _result(
                intent,
                MusePresentationIntentStatus.rejected,
                'INTENT_ID_REUSED',
              ),
            );
    }
    if (_receipts.length >= 256) {
      // Fail closed; do not evict a still-replayable receipt and apply it twice.
      return Future.value(
        _result(
          intent,
          MusePresentationIntentStatus.rejected,
          'INBOX_CAPACITY',
        ),
      );
    }
    final result = _dispatch(intent);
    _receipts[intent.intentRef] = (input: input, result: result);
    return result;
  }

  Future<MusePresentationIntentResultV1> _dispatch(
    MusePresentationIntentV1 intent,
  ) async {
    final now = _clock();
    if (intent.expiresAt <= now) {
      return _result(
        intent,
        MusePresentationIntentStatus.stale,
        'INTENT_EXPIRED',
      );
    }
    final route = _routes[jsonEncode([intent.pluginId, intent.intentType])];
    if (route == null) {
      return _result(
        intent,
        MusePresentationIntentStatus.notSupported,
        'FACET_NOT_FOUND',
      );
    }
    if (route.digest != intent.intentSchemaDigest) {
      return _result(
        intent,
        MusePresentationIntentStatus.rejected,
        'SCHEMA_DIGEST_MISMATCH',
      );
    }
    try {
      final result = await route
          .handler(intent)
          .timeout(Duration(milliseconds: intent.expiresAt - now));
      if (_closed) {
        return _result(
          intent,
          MusePresentationIntentStatus.surfaceClosed,
          'INBOX_CLOSED',
        );
      }
      validateMuseFacetValue(
        MuseFacetSchemaKind.presentationIntentResult,
        result.toJson(),
      );
      if (result.intentRef != intent.intentRef) {
        return _result(
          intent,
          MusePresentationIntentStatus.rejected,
          'RECEIPT_MISMATCH',
        );
      }
      return result;
    } on TimeoutException {
      return _result(
        intent,
        MusePresentationIntentStatus.timedOut,
        'HANDLER_TIMEOUT',
      );
    } catch (_) {
      return _result(
        intent,
        MusePresentationIntentStatus.rejected,
        'HANDLER_FAILED',
      );
    }
  }

  void close() {
    _closed = true;
    _routes.clear();
    _receipts.clear();
  }

  MusePresentationIntentResultV1 _result(
    MusePresentationIntentV1 intent,
    MusePresentationIntentStatus status,
    String reason,
  ) =>
      MusePresentationIntentResultV1(
        intentRef: intent.intentRef,
        status: status,
        reasonCode: reason,
        completedAt: _clock(),
      );
}
