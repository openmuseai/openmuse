import 'dart:convert';

const markdownSnapshotContextType = 'markdown.snapshot';
const markdownSnapshotDigest =
    'sha256:e4ea242dbdbc52565fb474cd042af4da590ae88a9d4715e9694ce031eaee03b3';
const markdownSnapshotMaxBytes = 32 * 1024;

final _forbidden = RegExp('access_token|refresh_token|api[_-]?key', caseSensitive: false);

class DshMarkdownSnapshot {
  const DshMarkdownSnapshot({
    required this.viewId,
    required this.text,
    required this.truncated,
    required this.byteLength,
    this.workspaceId,
  });

  final String viewId;
  final String? workspaceId;
  final String text;
  final bool truncated;
  final int byteLength;

  Map<String, Object?> toJson() => {
        'viewId': viewId,
        if (workspaceId != null && workspaceId!.isNotEmpty) 'workspaceId': workspaceId,
        'text': text,
        'truncated': truncated,
        'byteLength': byteLength,
      };
}

({String text, bool truncated, int byteLength}) boundUtf8Text(
  String raw, [
  int maxBytes = markdownSnapshotMaxBytes,
]) {
  final units = utf8.encode(raw);
  if (units.length <= maxBytes) {
    return (text: raw, truncated: false, byteLength: units.length);
  }
  var end = maxBytes;
  while (end > 0 && (units[end] & 0xc0) == 0x80) {
    end -= 1;
  }
  final sliced = utf8.decode(units.sublist(0, end), allowMalformed: false);
  return (text: sliced, truncated: true, byteLength: utf8.encode(sliced).length);
}

DshMarkdownSnapshot? buildMarkdownSnapshot({
  required String viewId,
  required String text,
  String? workspaceId,
}) {
  final id = viewId.trim();
  if (id.isEmpty || id.length > 128) {
    return null;
  }
  final cleaned = text.replaceAll(RegExp(r'[\u0000-\u0008\u000b\u000c\u000e-\u001f]'), ' ').trim();
  if (cleaned.isEmpty) {
    return null;
  }
  final bounded = boundUtf8Text(cleaned);
  final snapshot = DshMarkdownSnapshot(
    viewId: id,
    workspaceId: workspaceId?.trim(),
    text: bounded.text,
    truncated: bounded.truncated,
    byteLength: bounded.byteLength,
  );
  if (_forbidden.hasMatch(snapshot.toJson().toString())) {
    return null;
  }
  return snapshot;
}
