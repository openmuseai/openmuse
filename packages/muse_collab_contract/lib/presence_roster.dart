/// Transient, resource-neutral online identity. Authentication, expiry and
/// location validation belong to the provider and resource domain respectively.
final class CollaborationPresenceSlot {
  const CollaborationPresenceSlot({
    required this.actorId,
    required this.sessionId,
  });
  final String actorId, sessionId;
}

final class CollaborationPresenceUser {
  CollaborationPresenceUser._(this.actorId, Iterable<String> sessions)
    : sessionIds = List.unmodifiable(sessions);
  final String actorId;
  final List<String> sessionIds;
}

/// Stable user aggregation of already authenticated, live slots. No identity is
/// inferred from names, location payloads, client count or local token claims.
List<CollaborationPresenceUser> groupPresenceSlots(
  Iterable<CollaborationPresenceSlot> slots,
) {
  final ownership = <String, String>{}, users = <String, Set<String>>{};
  var count = 0;
  bool valid(String value) =>
      value.isNotEmpty &&
      value.length <= 128 &&
      !value.codeUnits.any((v) => v < 32 || v == 127);
  for (final slot in slots) {
    if (++count > 1024) throw const FormatException('PRESENCE_ROSTER_QUOTA');
    if (!valid(slot.actorId) || !valid(slot.sessionId))
      throw const FormatException('INVALID_PRESENCE_IDENTITY');
    final owner = ownership[slot.sessionId];
    if (owner != null && owner != slot.actorId)
      throw const FormatException('PRESENCE_IDENTITY_CONFLICT');
    ownership[slot.sessionId] = slot.actorId;
    (users[slot.actorId] ??= <String>{}).add(slot.sessionId);
  }
  final ids = users.keys.toList()..sort();
  return List.unmodifiable([
    for (final id in ids)
      CollaborationPresenceUser._(id, users[id]!.toList()..sort()),
  ]);
}
