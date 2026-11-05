/// The Muse Host → DSH resource-reference clipboard payload (RCX-01/RCX-03).
///
/// A copy of a selection inside a Muse editor writes three clipboard items:
///
/// 1. `text/plain` — the selected text, unchanged. This is the fallback: pasting
///    into any external application still works.
/// 2. `text/html` — the copied HTML fragment prefixed by a marker element
///    carrying the same payload as base64url text in
///    [museReferenceHtmlAttribute]. Browsers only expose `text/plain` and
///    `text/html` to a `paste` event, so the marker rides the HTML item; the
///    DSH composer reads it and inserts a reference chip instead of raw text.
/// 3. `io.openmuse.MuseResourceReference` ([museReferencePlatformType]) — the
///    raw UTF-8 JSON payload. Host-to-Host only: a browser clipboard read cannot
///    see custom platform formats, so DSH never relies on this item.
///
/// The payload carries **only opaque references** plus bounded display
/// metadata. It never carries a device path, a locator, or file content beyond
/// the bounded [museReferenceExcerptMaxBytes] excerpt.
///
/// The canonical format specification is owned by
/// `middlewares/dsh/plugins/dsh-client-ui-resource-reference/README.md`; this
/// file is the producing half and must stay byte-compatible with that document.
library;

import 'dart:convert';

/// Protocol tag of the payload JSON ([MuseResourceReference.protocol]).
const String museReferenceProtocol = 'muse.clipboard/resource-reference/v1';

/// Platform type of the custom Host-to-Host clipboard item.
///
/// Windows registers this as the clipboard format `io.openmuse.MuseResourceReference`
/// (see `super_clipboard`'s `CustomValueFormat.applicationId`); the same string
/// is written to the HTML marker for the browser-visible half.
const String museReferencePlatformType = 'io.openmuse.MuseResourceReference';

/// HTML attribute carrying the base64url payload inside the copied fragment.
const String museReferenceHtmlAttribute = 'data-muse-resource-reference';

/// HTML attribute carrying the payload format version (currently `1`).
const String museReferenceHtmlVersionAttribute =
    'data-muse-resource-reference-version';

/// Version written into [museReferenceHtmlVersionAttribute].
const String museReferenceHtmlVersion = '1';

/// Excerpt bound: an excerpt longer than this many UTF-8 bytes is truncated
/// Host-side and marked [MuseResourceReference.truncated] (Q-4).
const int museReferenceExcerptMaxBytes = 8192;

/// Display-name bound; longer titles are truncated with [String.trim] safe
/// ellipsis handling.
const int museReferenceDisplayNameMaxChars = 160;

/// Anchor kind of a Host editor selection (top-level block range).
const String museReferenceAnchorKindRange = 'range';

/// Opaque reference grammar accepted by the DSH reference/opening contracts.
///
/// Deliberately excludes `/`, `\`, `:` and whitespace: a reference must never be
/// able to read as a device path.
final RegExp museReferenceOpaqueRefPattern = RegExp(r'^[A-Za-z0-9._~-]{1,128}$');

/// Namespace prefix of a document resource reference (`resource.appflowy.document.<viewId>`).
const String museDocumentResourceRefPrefix = 'resource.appflowy.document.';

/// Namespace prefix of a document view reference (`view.appflowy.document.<viewId>`).
const String museDocumentViewRefPrefix = 'view.appflowy.document.';

/// The AppFlowy view id grammar embedded in a Muse document reference.
final RegExp _viewIdPattern = RegExp(r'^[A-Za-z0-9-]{1,64}$');

/// Opaque `resourceRef` for one AppFlowy document; the Host owns its meaning.
String museDocumentResourceRef(String viewId) {
  _requireViewId(viewId);
  return '$museDocumentResourceRefPrefix$viewId';
}

/// Opaque view reference for one AppFlowy document.
String museDocumentViewRef(String viewId) {
  _requireViewId(viewId);
  return '$museDocumentViewRefPrefix$viewId';
}

void _requireViewId(String viewId) {
  if (!_viewIdPattern.hasMatch(viewId)) {
    throw ArgumentError.value(viewId, 'viewId', 'not an AppFlowy view id');
  }
}

/// One bounded excerpt: the retained text, its UTF-8 byte length, and whether
/// the source was cut.
final class MuseBoundedExcerpt {
  const MuseBoundedExcerpt({
    required this.text,
    required this.bytes,
    required this.truncated,
  });

  /// The retained text; re-encoding it as UTF-8 never exceeds the bound.
  final String text;

  /// UTF-8 byte length of [text].
  final int bytes;

  /// Whether text was dropped.
  final bool truncated;
}

/// Bound [text] to [maxBytes] UTF-8 bytes without splitting a rune.
///
/// The cut is made on a rune boundary, so a multibyte character is either fully
/// retained or fully dropped — the retained text never decodes to a replacement
/// character.
MuseBoundedExcerpt boundMuseExcerpt(
  String text, {
  int maxBytes = museReferenceExcerptMaxBytes,
}) {
  if (maxBytes <= 0) {
    throw ArgumentError.value(maxBytes, 'maxBytes', 'must be positive');
  }
  final buffer = StringBuffer();
  var bytes = 0;
  var truncated = false;
  for (final rune in text.runes) {
    final width = _utf8Width(rune);
    if (bytes + width > maxBytes) {
      truncated = true;
      break;
    }
    buffer.writeCharCode(rune);
    bytes += width;
  }
  return MuseBoundedExcerpt(
    text: buffer.toString(),
    bytes: bytes,
    truncated: truncated,
  );
}

/// UTF-8 byte width of one rune.
///
/// Counted from the code point instead of re-encoding, so bounding one excerpt
/// stays linear in its length. A lone surrogate (invalid UTF-16, which Dart
/// strings can hold) is counted as its 3-byte replacement encoding — the same
/// width `utf8.encode` produces for it.
int _utf8Width(int rune) {
  if (rune <= 0x7F) return 1;
  if (rune <= 0x7FF) return 2;
  if (rune <= 0xFFFF) return 3;
  return 4;
}

/// Anchor of a copied selection inside its source surface.
final class MuseReferenceAnchor {
  const MuseReferenceAnchor({
    required this.startBlock,
    required this.endBlock,
    required this.label,
    this.startBlockRef,
    this.endBlockRef,
    this.kind = museReferenceAnchorKindRange,
  });

  /// Anchor kind; only [museReferenceAnchorKindRange] is produced today.
  final String kind;

  /// 0-based index of the first selected top-level block.
  final int startBlock;

  /// 0-based index of the last selected top-level block (inclusive).
  final int endBlock;

  /// Editor block id of the first selected block, when the surface has one.
  final String? startBlockRef;

  /// Editor block id of the last selected block, when the surface has one.
  final String? endBlockRef;

  /// Display label of the range, e.g. `blocks 4–9`.
  final String label;

  Map<String, Object?> toJson() => <String, Object?>{
        'kind': kind,
        'startBlock': startBlock,
        'endBlock': endBlock,
        if (startBlockRef != null) 'startBlockRef': startBlockRef,
        if (endBlockRef != null) 'endBlockRef': endBlockRef,
        'label': label,
      };

  /// Parse one anchor from a payload, tolerating unknown extra keys.
  static MuseReferenceAnchor? tryParse(Object? value) {
    if (value is! Map) return null;
    final start = value['startBlock'];
    final end = value['endBlock'];
    if (start is! int || end is! int || start < 0 || end < start) return null;
    final label = value['label'];
    return MuseReferenceAnchor(
      kind: value['kind'] is String
          ? value['kind'] as String
          : museReferenceAnchorKindRange,
      startBlock: start,
      endBlock: end,
      startBlockRef: _boundedString(value['startBlockRef'], 128),
      endBlockRef: _boundedString(value['endBlockRef'], 128),
      label: label is String && label.isNotEmpty ? label : 'blocks $start–$end',
    );
  }
}

/// The reference payload written to the clipboard by a Host copy.
///
/// Every field is either an opaque Host-owned reference or bounded display
/// metadata; there is no device path and no unbounded content.
final class MuseResourceReference {
  const MuseResourceReference({
    required this.resourceRef,
    required this.displayName,
    required this.anchor,
    required this.excerpt,
    required this.truncated,
    required this.excerptBytes,
    required this.capturedAt,
    this.viewRef,
    this.mountRef,
  });

  /// Protocol tag; must be [museReferenceProtocol].
  String get protocol => museReferenceProtocol;

  /// Opaque Host-owned resource reference (`resource.appflowy.document.<id>`).
  final String resourceRef;

  /// Opaque Host-owned view reference, when the source has a view.
  final String? viewRef;

  /// Mount reference of the Project Workspace mount this source belongs to,
  /// verbatim as the binding publishes it (`mount:<digest>`). Opaque to DSH.
  final String? mountRef;

  /// Human-readable source name (`Design doc`).
  final String displayName;

  /// Anchor of the copied range inside the source.
  final MuseReferenceAnchor anchor;

  /// The bounded excerpt (≤ [museReferenceExcerptMaxBytes] UTF-8 bytes).
  final String excerpt;

  /// Whether [excerpt] is shorter than the copied selection.
  final bool truncated;

  /// UTF-8 byte length of [excerpt].
  final int excerptBytes;

  /// Capture time, Unix epoch milliseconds.
  final int capturedAt;

  /// Whether the reference carries only fields the DSH payload contract accepts.
  bool get isValid {
    if (!museReferenceOpaqueRefPattern.hasMatch(resourceRef)) return false;
    final view = viewRef;
    if (view != null && !museReferenceOpaqueRefPattern.hasMatch(view)) {
      return false;
    }
    final mount = mountRef;
    if (mount != null && mount.trim().isEmpty) return false;
    return displayName.trim().isNotEmpty;
  }

  /// Canonical JSON map; field order is fixed so the encoded payload is stable.
  Map<String, Object?> toJson() => <String, Object?>{
        'protocol': protocol,
        'source': 'host.selection',
        'resourceRef': resourceRef,
        if (viewRef != null) 'viewRef': viewRef,
        if (mountRef != null) 'mountRef': mountRef,
        'displayName': displayName,
        'anchor': anchor.toJson(),
        'excerpt': excerpt,
        'excerptBytes': excerptBytes,
        'truncated': truncated,
        'capturedAt': capturedAt,
      };

  /// Encode the payload as the clipboard JSON string.
  String encode() {
    if (!isValid) {
      throw StateError('Muse resource reference is not a valid payload');
    }
    return jsonEncode(toJson());
  }

  /// Parse a payload previously produced by [encode] (round-trip / test seam).
  static MuseResourceReference? tryDecode(String jsonText) {
    final Object? decoded;
    try {
      decoded = jsonDecode(jsonText);
    } on FormatException {
      return null;
    }
    return fromJson(decoded);
  }

  /// Rebuild a payload from its JSON map, re-applying every bound.
  static MuseResourceReference? fromJson(Object? value) {
    if (value is! Map) return null;
    if (value['protocol'] != museReferenceProtocol) return null;
    final resourceRef = value['resourceRef'];
    if (resourceRef is! String ||
        !museReferenceOpaqueRefPattern.hasMatch(resourceRef)) {
      return null;
    }
    final anchor = MuseReferenceAnchor.tryParse(value['anchor']);
    if (anchor == null) return null;
    final rawExcerpt = value['excerpt'];
    if (rawExcerpt is! String) return null;
    final bounded = boundMuseExcerpt(rawExcerpt);
    final displayName = _boundedString(value['displayName'], 160);
    if (displayName == null) return null;
    final capturedAt = value['capturedAt'];
    final reference = MuseResourceReference(
      resourceRef: resourceRef,
      viewRef: _boundedString(value['viewRef'], 128),
      mountRef: _boundedString(value['mountRef'], 256),
      displayName: displayName,
      anchor: anchor,
      excerpt: bounded.text,
      truncated: value['truncated'] == true || bounded.truncated,
      excerptBytes: bounded.bytes,
      capturedAt: capturedAt is int ? capturedAt : 0,
    );
    return reference.isValid ? reference : null;
  }
}

String? _boundedString(Object? value, int maxChars) {
  if (value is! String) return null;
  final trimmed = value.trim();
  if (trimmed.isEmpty) return null;
  return trimmed.length <= maxChars ? trimmed : trimmed.substring(0, maxChars);
}

/// Prefix the copied HTML fragment with the payload marker element.
///
/// An empty fragment still gets the marker: the marker is the only carrier a
/// browser paste can see, so it must never depend on HTML having been produced.
String museReferenceHtmlCarrier(String? html, String payloadJson) {
  final encoded = base64Url.encode(utf8.encode(payloadJson));
  final marker = '<div $museReferenceHtmlAttribute="$encoded"'
      ' $museReferenceHtmlVersionAttribute="$museReferenceHtmlVersion"></div>';
  if (html == null || html.trim().isEmpty) return marker;
  return '$marker$html';
}

/// Extract the payload JSON from a copied HTML fragment, or null when the
/// fragment carries no Muse reference marker.
///
/// The Host reads its own payload back here (Host-to-Host paste and tests); the
/// DSH client half implements the same extraction in `lib/client.js`.
String? museReferencePayloadFromHtml(String html) {
  final match = RegExp(
    '$museReferenceHtmlAttribute="([A-Za-z0-9_=-]+)"',
  ).firstMatch(html);
  final encoded = match?.group(1);
  if (encoded == null) return null;
  try {
    return utf8.decode(base64Url.decode(encoded));
  } on FormatException {
    return null;
  }
}

/// Where the copy command reads the identity of the surface being copied from.
///
/// The editor holds text, not source identity: the document (or Word) surface
/// attaches itself here while it is alive, and the copy command asks the
/// attached source to describe its current selection.
abstract interface class MuseSelectionReferenceSource {
  /// Opaque `resourceRef` of the source surface.
  String get resourceRef;

  /// Opaque view reference of the source surface, when it has one.
  String? get viewRef;

  /// Mount reference of the Project Workspace mount this source belongs to.
  String? get mountRef;

  /// Display name of the source surface (the view title).
  String get displayName;

  /// Build the payload for one selection of this source.
  ///
  /// [selectedText] is the raw selection text; the source bounds it. The block
  /// range is described by [startBlock]/[endBlock] (0-based top-level block
  /// indices) and their editor block ids.
  MuseResourceReference describeSelection({
    required String selectedText,
    required int startBlock,
    required int endBlock,
    String? startBlockRef,
    String? endBlockRef,
    int? capturedAt,
  });
}

/// The surface whose selection the next Host copy describes.
///
/// One live editor surface at a time: attaching replaces the previous source, so
/// a copy in a second window describes the window the user is actually in. A
/// detached (disposed) source leaves the registry empty, and the copy then falls
/// back to a plain-text copy.
final class MuseSelectionReferenceRegistry {
  MuseSelectionReferenceRegistry._();

  /// Process-wide registry the copy command reads.
  static final MuseSelectionReferenceRegistry instance =
      MuseSelectionReferenceRegistry._();

  MuseSelectionReferenceSource? _source;

  /// The currently attached source, or null when no Muse surface is live.
  MuseSelectionReferenceSource? get source => _source;

  /// Attach [source] as the surface copies describe.
  void attach(MuseSelectionReferenceSource source) {
    _source = source;
  }

  /// Detach [source]; a no-op when another source already replaced it.
  void detach(MuseSelectionReferenceSource source) {
    if (identical(_source, source)) _source = null;
  }

  /// Describe one selection with the attached source, or null when none is live.
  MuseResourceReference? describe({
    required String selectedText,
    required int startBlock,
    required int endBlock,
    String? startBlockRef,
    String? endBlockRef,
    int? capturedAt,
  }) {
    final source = _source;
    if (source == null) return null;
    if (selectedText.trim().isEmpty) return null;
    final reference = source.describeSelection(
      selectedText: selectedText,
      startBlock: startBlock,
      endBlock: endBlock,
      startBlockRef: startBlockRef,
      endBlockRef: endBlockRef,
      capturedAt: capturedAt,
    );
    return reference.isValid ? reference : null;
  }

  /// Clear the registry (test seam).
  void reset() {
    _source = null;
  }
}

/// Build a bounded reference payload for one Host selection.
MuseResourceReference buildMuseResourceReference({
  required String resourceRef,
  required String displayName,
  required String selectedText,
  required int startBlock,
  required int endBlock,
  String? viewRef,
  String? mountRef,
  String? startBlockRef,
  String? endBlockRef,
  int? capturedAt,
  int maxExcerptBytes = museReferenceExcerptMaxBytes,
}) {
  final bounded = boundMuseExcerpt(selectedText, maxBytes: maxExcerptBytes);
  final boundedName = displayName.trim().length <= museReferenceDisplayNameMaxChars
      ? displayName.trim()
      : displayName.trim().substring(0, museReferenceDisplayNameMaxChars);
  return MuseResourceReference(
    resourceRef: resourceRef,
    viewRef: viewRef,
    mountRef: mountRef,
    displayName: boundedName.isEmpty ? resourceRef : boundedName,
    anchor: MuseReferenceAnchor(
      startBlock: startBlock,
      endBlock: endBlock,
      startBlockRef: startBlockRef,
      endBlockRef: endBlockRef,
      label: startBlock == endBlock
          ? 'block ${startBlock + 1}'
          : 'blocks ${startBlock + 1}–${endBlock + 1}',
    ),
    excerpt: bounded.text,
    truncated: bounded.truncated,
    excerptBytes: bounded.bytes,
    capturedAt: capturedAt ?? DateTime.now().millisecondsSinceEpoch,
  );
}
