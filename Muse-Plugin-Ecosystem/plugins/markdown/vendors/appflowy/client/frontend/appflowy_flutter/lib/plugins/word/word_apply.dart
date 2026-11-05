/// P3 apply is blocked until kernel `WordSession.toDocx()` exists.
class WordApplyBlocked implements Exception {
  const WordApplyBlocked();

  @override
  String toString() => 'BLOCKED_BY_KERNEL: WordSession.toDocx is not implemented';
}

/// Never write the open-time `_docx` bytes back. Callers must export via toDocx.
Never refuseStaleDocxSave() {
  throw const WordApplyBlocked();
}
