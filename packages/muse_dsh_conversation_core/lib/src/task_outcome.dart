/// A content-free result from a newly received, authoritative turn/end event.
enum DshTaskResult { completed, failed }

final class DshTaskOutcome {
  const DshTaskOutcome({
    required this.sessionId,
    required this.sequence,
    required this.result,
  });

  final String sessionId;
  final int sequence;
  final DshTaskResult result;
}
