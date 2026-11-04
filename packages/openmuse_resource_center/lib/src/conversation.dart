import 'package:flutter/material.dart';
import 'package:openmuse_resource_clipboard/openmuse_resource_clipboard.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'controller.dart';
import 'resource_open.dart';
import 'strings.dart';

final class ResourceAttachment {
  const ResourceAttachment({required this.reference, required this.record});
  final ResourceReference reference;
  final ResourceRecord record;
  Map<String, Object?> toJson() => {
    'reference': reference.toJson(),
    'record': record.toJson(),
  };
  factory ResourceAttachment.fromJson(Map<String, Object?> m) =>
      ResourceAttachment(
        reference: ResourceReference.fromJson(m['reference']),
        record: ResourceRecord.fromJson(m['record']),
      );
}

/// Draft metadata only. No content is uploaded or sent by adding a reference.
final class ResourceConversationDraft extends ChangeNotifier {
  ResourceConversationDraft(this.controller);
  final ResourceCenterController controller;
  final _items = <ResourceAttachment>[];
  bool keepVersions = false;
  List<ResourceAttachment> get items => List.unmodifiable(_items);
  bool get isEmpty => _items.isEmpty;
  void attach(ResourceRecord record, {ResourceReference? reference}) {
    if (record.identity.scope != controller.hub.scope)
      throw const ResourceFailure('DENIED');
    final ref = reference ?? record.reference(pin: true);
    if (_items.any((e) => e.reference.key == ref.key))
      throw const ResourceFailure('ALREADY_ATTACHED');
    if (_items.length >= 10) throw const ResourceFailure('ATTACHMENT_LIMIT');
    _items.add(ResourceAttachment(reference: ref, record: record));
    notifyListeners();
  }

  Future<void> paste() async {
    final ref = await ResourceClipboard.read();
    if (ref.identity.scope != controller.hub.scope)
      throw const ResourceFailure('DENIED');
    final target = ref.retained?.identity ?? ref.identity;
    final record = await controller.hub
        .provider(target)
        .describe(
          target.resource,
          controller.hub.access(
            'resource-paste',
            operations: {ResourceOperation.describe},
          ),
        );
    attach(
      record,
      reference: ref.pinned
          ? ref
          : ResourceReference(identity: ref.identity, version: record.version),
    );
  }

  void remove(ResourceReference ref) {
    _items.removeWhere((r) => r.reference.key == ref.key);
    notifyListeners();
  }

  void setKeepVersions(bool value) {
    keepVersions = value;
    notifyListeners();
  }

  void clear() {
    _items.clear();
    keepVersions = false;
    notifyListeners();
  }
}

final class ResourceReferenceCard extends StatelessWidget {
  const ResourceReferenceCard({
    super.key,
    required this.attachment,
    required this.controller,
    this.onRemove,
  });
  final ResourceAttachment attachment;
  final ResourceCenterController controller;
  final VoidCallback? onRemove;
  @override
  Widget build(BuildContext context) {
    final c = Theme.of(context).colorScheme, s = ResourceCenterStrings(context);
    return Material(
      color: c.primary.withValues(alpha: .09),
      borderRadius: BorderRadius.circular(12),
      clipBehavior: Clip.antiAlias,
      child: Row(
        children: [
          Expanded(
            child: InkWell(
              key: ValueKey(
                'resource-reference-${attachment.reference.identity.resource}',
              ),
              onTap: () =>
                  openResourcePage(context, controller, attachment.reference),
              child: Padding(
                padding: const EdgeInsets.symmetric(
                  horizontal: 12,
                  vertical: 11,
                ),
                child: Row(
                  children: [
                    Icon(Icons.link_rounded, size: 20, color: c.primary),
                    const SizedBox(width: 9),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            attachment.record.name,
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                            style: const TextStyle(
                              fontSize: 13,
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                          const SizedBox(height: 4),
                          Text(
                            attachment.reference.retained != null
                                ? s.conversationVersion
                                : attachment.record.sourceLabel,
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                            style: TextStyle(
                              fontSize: 11,
                              color: c.onSurfaceVariant,
                            ),
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
          if (onRemove != null)
            IconButton(
              tooltip: s.removeReference,
              onPressed: onRemove,
              icon: const Icon(Icons.close, size: 18),
            ),
        ],
      ),
    );
  }
}

final class ResourceDraftStrip extends StatelessWidget {
  const ResourceDraftStrip({super.key, required this.draft});
  final ResourceConversationDraft draft;
  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: draft,
    builder: (context, _) {
      if (draft.isEmpty) return const SizedBox.shrink();
      final s = ResourceCenterStrings(context);
      return Material(
        type: MaterialType.transparency,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            for (final item in draft.items)
              Padding(
                padding: const EdgeInsets.only(bottom: 7),
                child: ResourceReferenceCard(
                  attachment: item,
                  controller: draft.controller,
                  onRemove: () => draft.remove(item.reference),
                ),
              ),
            if (draft.items.isNotEmpty)
              CheckboxListTile(
                key: const ValueKey('resource-retain-toggle'),
                contentPadding: EdgeInsets.zero,
                controlAffinity: ListTileControlAffinity.leading,
                dense: true,
                value: draft.keepVersions,
                onChanged: (value) => draft.setKeepVersions(value ?? false),
                title: Text(s.retain, style: const TextStyle(fontSize: 12)),
                subtitle: Text(
                  draft.keepVersions ? s.retainHint : s.originOnly,
                  style: const TextStyle(fontSize: 10),
                ),
              ),
          ],
        ),
      );
    },
  );
}
