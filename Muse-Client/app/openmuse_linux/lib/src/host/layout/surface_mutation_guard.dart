import 'package:flutter/foundation.dart';

enum SurfaceMutationKind { resize, move, swap, close }

enum SurfaceMutationDecision { allow, defer, deny }

@immutable
final class SurfaceMutationResult {
  const SurfaceMutationResult(this.decision, {this.reason});

  const SurfaceMutationResult.allow()
    : decision = SurfaceMutationDecision.allow,
      reason = null;

  final SurfaceMutationDecision decision;
  final String? reason;
}

abstract interface class SurfaceMutationGuard {
  Future<SurfaceMutationResult> prepareMutation(SurfaceMutationKind kind);
}

/// Optional, Host-local protection for transient native interaction state.
///
/// It deliberately does not live in the plugin SDK: this guards placement
/// mutations (for example active IME marked text), not plugin business state.
final class SurfaceMutationGuards {
  final Map<String, SurfaceMutationGuard> _guards = {};

  void register(String instanceRef, SurfaceMutationGuard guard) {
    _guards[instanceRef] = guard;
  }

  void unregister(String instanceRef, SurfaceMutationGuard guard) {
    if (identical(_guards[instanceRef], guard)) {
      _guards.remove(instanceRef);
    }
  }

  Future<SurfaceMutationResult> prepare(
    Iterable<String> instanceRefs,
    SurfaceMutationKind kind,
  ) async {
    var deferred = false;
    String? deferredReason;
    for (final instanceRef in instanceRefs) {
      final guard = _guards[instanceRef];
      if (guard == null) continue;
      final result = await guard.prepareMutation(kind);
      if (result.decision == SurfaceMutationDecision.deny) return result;
      if (result.decision == SurfaceMutationDecision.defer) {
        deferred = true;
        deferredReason ??= result.reason;
      }
    }
    return deferred
        ? SurfaceMutationResult(
            SurfaceMutationDecision.defer,
            reason: deferredReason,
          )
        : const SurfaceMutationResult.allow();
  }
}
