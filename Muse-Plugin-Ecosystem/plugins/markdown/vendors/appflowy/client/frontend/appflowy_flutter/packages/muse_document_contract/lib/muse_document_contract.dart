library muse_document_contract;

final class MuseDocumentContentV2 {
  const MuseDocumentContentV2(
      {required this.mediaType,
      required this.text,
      required this.truncated,
      required this.byteLength});
  final String mediaType;
  final String text;
  final bool truncated;
  final int byteLength;
  factory MuseDocumentContentV2.fromJson(Map<String, Object?> value) =>
      MuseDocumentContentV2(
        mediaType: value['mediaType']! as String,
        text: value['text']! as String,
        truncated: value['truncated']! as bool,
        byteLength: value['byteLength']! as int,
      );
  Map<String, Object?> toJson() => {
        'mediaType': mediaType,
        'text': text,
        'truncated': truncated,
        'byteLength': byteLength
      };
}

final class MuseDocumentSnapshotV2 {
  const MuseDocumentSnapshotV2(
      {required this.resourceRef,
      required this.revision,
      required this.content});
  final String resourceRef;
  final String revision;
  final MuseDocumentContentV2 content;
  factory MuseDocumentSnapshotV2.fromJson(Map<String, Object?> value) =>
      MuseDocumentSnapshotV2(
        resourceRef: value['resourceRef']! as String,
        revision: value['revision']! as String,
        content: MuseDocumentContentV2.fromJson(
            (value['content']! as Map).cast<String, Object?>()),
      );
  Map<String, Object?> toJson() => {
        'protocol': 'muse.document/snapshot/v2',
        'resourceRef': resourceRef,
        'revision': revision,
        'content': content.toJson(),
      };
}

final class MuseDocumentCommitEventV2 {
  const MuseDocumentCommitEventV2({
    required this.eventId,
    required this.cursor,
    required this.resourceRef,
    required this.revision,
    required this.commandRef,
    required this.origin,
    required this.changeKind,
    required this.occurredAt,
  });
  final String eventId;
  final String cursor;
  final String resourceRef;
  final String revision;
  final String commandRef;
  final String origin;
  final String changeKind;
  final int occurredAt;
  factory MuseDocumentCommitEventV2.fromJson(Map<String, Object?> value) =>
      MuseDocumentCommitEventV2(
        eventId: value['eventId']! as String,
        cursor: value['cursor']! as String,
        resourceRef: value['resourceRef']! as String,
        revision: value['revision']! as String,
        commandRef: value['commandRef']! as String,
        origin: value['origin']! as String,
        changeKind: ((value['change']! as Map).cast<String, Object?>()['kind'])!
            as String,
        occurredAt: value['occurredAt']! as int,
      );
  Map<String, Object?> toJson() => {
        'protocol': 'muse.document/event/v2',
        'eventId': eventId,
        'cursor': cursor,
        'resourceRef': resourceRef,
        'revision': revision,
        'commandRef': commandRef,
        'origin': origin,
        'change': {'kind': changeKind},
        'occurredAt': occurredAt,
      };
}
