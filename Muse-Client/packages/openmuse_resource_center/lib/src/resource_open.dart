import 'dart:async';
import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'controller.dart';
import 'strings.dart';

String resourceErrorText(BuildContext context, Object error) {
  final s = ResourceCenterStrings(context);
  final code = error is ResourceFailure ? error.code : '';
  return switch (code) {
    'SOURCE_UNAVAILABLE' || 'REQUEST_EXPIRED' => s.sourceUnavailable,
    'VERSION_UNAVAILABLE' || 'VERSION_MISMATCH' => s.versionUnavailable,
    'DENIED' => s.denied,
    'CONTEXT_UNAVAILABLE' => s.strings.resourcecenterContextUnavailable(),
    'CONTEXT_TOO_LARGE' => s.strings.resourcecenterContextTooLarge(),
    'RESOURCE_CENTER_UNAVAILABLE' => s.strings.resourcecenterPeerUpgrade(),
    'EXPIRED' || 'STALE_GENERATION' => s.expired,
    'READ_ONLY' => s.sourceReadOnly,
    'NAME_CONFLICT' => s.nameConflict,
    'INVALID_NAME' => s.invalidName,
    'EXTENSION_REQUIRED' => s.keepExtension,
    'INVALID_URL' => s.invalidUrl,
    'CREATOR_UNAVAILABLE' => s.creatorUnavailable,
    'METADATA_CONFLICT' ||
    'REVISION_CONFLICT' ||
    'IDEMPOTENCY_CONFLICT' => s.conflict,
    'TRANSFER_TOO_LARGE' ||
    'RESOURCE_TOO_LARGE' ||
    'RESPONSE_TOO_LARGE' => s.tooLarge,
    'DIGEST_MISMATCH' => s.integrityFailure,
    'INVALID_REFERENCE' ||
    'INVALID_ENVELOPE' ||
    'UNSUPPORTED_PROTOCOL' => s.invalidReference,
    _ => s.failed,
  };
}

void showResourceError(BuildContext context, Object error) {
  if (context.mounted)
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(resourceErrorText(context, error)),
        behavior: SnackBarBehavior.floating,
      ),
    );
}

/// Shared entry for list rows, pasted references and conversation cards.
Future<bool> openResourcePage(
  BuildContext context,
  ResourceCenterController controller,
  ResourceReference reference, {
  bool chooseHandler = false,
}) async {
  final navigator = Navigator.of(context);
  bool cancelled = false;
  late DialogRoute<void> progress;
  progress = DialogRoute<void>(
    context: context,
    barrierDismissible: false,
    builder: (dialog) {
      final s = ResourceCenterStrings(dialog);
      return PopScope(
        canPop: false,
        child: AlertDialog(
          title: Text(s.open),
          content: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const LinearProgressIndicator(),
              const SizedBox(height: 18),
              Text(s.loading),
            ],
          ),
          actions: [
            TextButton(
              key: const ValueKey('resource-open-cancel'),
              onPressed: () {
                cancelled = true;
                navigator.removeRoute(progress);
              },
              child: Text(s.cancel),
            ),
          ],
        ),
      );
    },
  );
  unawaited(navigator.push(progress));
  void dismiss() {
    if (navigator.mounted && progress.isActive) navigator.removeRoute(progress);
  }

  try {
    final opened = await controller.resolve(reference);
    if (cancelled || !context.mounted) {
      dismiss();
      return false;
    }
    final choices = await controller.handlers(opened.content.record);
    if (cancelled || !context.mounted) {
      dismiss();
      return false;
    }
    dismiss();
    final record = opened.content.record, s = ResourceCenterStrings(context);
    ResourceHandlerChoice? choice;
    final preferred = controller.hub.defaultHandler(
      controller.preferenceKey(record),
    );
    if (!chooseHandler) {
      choice = choices.where((x) => x.key == preferred).firstOrNull;
      if (choice == null && choices.length == 1) choice = choices.single;
    }
    if (choices.isEmpty) {
      if (await controller.canOpenLegacy?.call(record) == true &&
          context.mounted) {
        final success =
            await controller.openLegacy?.call(record, opened.content.bytes) ??
            false;
        if (success) await controller.opened(record, 'legacy');
        return success;
      }
      if (context.mounted)
        await showDialog<void>(
          context: context,
          builder: (dialog) => AlertDialog(
            title: Text(s.openWith),
            content: Text(s.noHandler),
            actions: [
              TextButton(
                onPressed: () => Navigator.pop(dialog),
                child: Text(s.close),
              ),
            ],
          ),
        );
      return false;
    }
    if (choice == null) {
      if (!context.mounted) return false;
      choice = await showModalBottomSheet<ResourceHandlerChoice>(
        context: context,
        isScrollControlled: true,
        useSafeArea: true,
        showDragHandle: true,
        builder: (sheet) => _HandlerSheet(
          controller: controller,
          record: record,
          choices: choices,
        ),
      );
    }
    if (choice == null || !context.mounted) return false;
    if (controller.registry.stateOf(choice.plugin.descriptor.id) !=
        OpenMusePluginState.active) {
      throw const ResourceFailure('HANDLER_UNAVAILABLE');
    }
    final provider = controller.hub.provider(record.identity),
        selected = choice;
    final session = ResourceEditorSession(
      opened: opened,
      allowEditing:
          choice.spec.editable &&
          provider.available &&
          provider.source.canEdit &&
          !record.readOnly &&
          !opened.fromSnapshot,
      commitContent: (base, bytes, key) => provider.commit(
        record.identity.resource,
        base,
        bytes,
        key,
        controller.hub.access(
          'editor:${selected.plugin.descriptor.id}',
          operations: {ResourceOperation.edit},
          resourceKeys: {record.identity.key},
        ),
      ),
      onOpened: (record) => unawaited(controller.opened(record, selected.key)),
      onChanged: (record) => unawaited(controller.hub.refreshRecord(record)),
    );
    session.openLinkedContent = (name, content) async {
      final materialize = controller.materializeLinkedContent;
      if (materialize == null || !context.mounted) return false;
      final linked = await materialize(session.record, name, content);
      if (!context.mounted) return false;
      if ((await controller.handlers(linked)).isEmpty &&
          await controller.canOpenLegacy?.call(linked) != true)
        return false;
      if (!context.mounted) return false;
      return openResourcePage(context, controller, linked.reference(pin: true));
    };
    try {
      final present = controller.presentEditor;
      if (present != null) {
        await present(ResourceEditorPresentation(selected, session));
      } else {
        await navigator.push(
          MaterialPageRoute<void>(
            settings: const RouteSettings(name: openMuseResourceRouteName),
            builder: (_) => _ResourceEditorPage(
              controller: controller,
              choice: selected,
              session: session,
            ),
          ),
        );
      }
    } finally {
      session.dispose();
    }
    return true;
  } on Object catch (error) {
    dismiss();
    if (!cancelled && context.mounted) {
      final strings = ResourceCenterStrings(context);
      await showDialog<void>(
        context: context,
        builder: (dialog) => AlertDialog(
          title: Text(strings.open),
          content: Text(resourceErrorText(dialog, error)),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(dialog),
              child: Text(strings.close),
            ),
          ],
        ),
      );
    }
    return false;
  }
}

final class _HandlerSheet extends StatefulWidget {
  const _HandlerSheet({
    required this.controller,
    required this.record,
    required this.choices,
  });
  final ResourceCenterController controller;
  final ResourceRecord record;
  final List<ResourceHandlerChoice> choices;
  @override
  State<_HandlerSheet> createState() => _HandlerSheetState();
}

final class _HandlerSheetState extends State<_HandlerSheet> {
  late ResourceHandlerChoice selected = widget.choices.first;
  bool remember = false;
  @override
  Widget build(BuildContext context) {
    final s = ResourceCenterStrings(context), c = Theme.of(context).colorScheme;
    return Padding(
      padding: EdgeInsets.fromLTRB(
        20,
        4,
        20,
        20 + MediaQuery.viewInsetsOf(context).bottom,
      ),
      child: SingleChildScrollView(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(s.openWith, style: Theme.of(context).textTheme.titleLarge),
            const SizedBox(height: 14),
            Text(
              widget.record.name,
              maxLines: 2,
              overflow: TextOverflow.ellipsis,
            ),
            const SizedBox(height: 12),
            for (final choice in widget.choices)
              ListTile(
                key: ValueKey('resource-handler-${choice.key}'),
                contentPadding: EdgeInsets.zero,
                leading: Icon(Icons.description_outlined, color: c.primary),
                title: Text(choice.spec.title),
                subtitle: Text(choice.plugin.descriptor.name),
                trailing: choice.key == selected.key
                    ? Icon(Icons.check_rounded, color: c.primary)
                    : null,
                onTap: () => setState(() => selected = choice),
              ),
            CheckboxListTile(
              contentPadding: EdgeInsets.zero,
              controlAffinity: ListTileControlAffinity.leading,
              title: Text(
                s.defaultHandler,
                style: const TextStyle(fontSize: 13),
              ),
              value: remember,
              onChanged: (value) => setState(() => remember = value ?? false),
            ),
            const SizedBox(height: 10),
            SizedBox(
              width: double.infinity,
              child: FilledButton(
                key: const ValueKey('resource-handler-open'),
                onPressed: () async {
                  if (remember)
                    await widget.controller.hub.setDefault(
                      widget.controller.preferenceKey(widget.record),
                      selected.key,
                    );
                  if (context.mounted) Navigator.pop(context, selected);
                },
                child: Text(s.open),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

final class _ResourceEditorPage extends StatefulWidget {
  const _ResourceEditorPage({
    required this.controller,
    required this.choice,
    required this.session,
  });
  final ResourceCenterController controller;
  final ResourceHandlerChoice choice;
  final ResourceEditorSession session;
  @override
  State<_ResourceEditorPage> createState() => _ResourceEditorPageState();
}

final class _ResourceEditorPageState extends State<_ResourceEditorPage> {
  ResourceCenterController get controller => widget.controller;
  ResourceHandlerChoice get choice => widget.choice;
  ResourceEditorSession get session => widget.session;
  bool handlingPop = false;
  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: session,
    builder: (context, _) {
      final s = ResourceCenterStrings(context), record = session.record;
      return PopScope(
        canPop:
            !session.dirty &&
            (session.flushPendingChanges == null || session.closePrepared),
        onPopInvokedWithResult: (popped, result) async {
          if (popped || handlingPop) return;
          handlingPop = true;
          try {
            if (session.flushPendingChanges != null && !session.closePrepared) {
              if (await session.prepareToClose()) {
                if (context.mounted) Navigator.pop(context);
                return;
              }
            }
            if (!context.mounted || !session.dirty) return;
            final discard = await showDialog<bool>(
              context: context,
              builder: (dialog) => AlertDialog(
                title: Text(s.unsaved),
                actions: [
                  TextButton(
                    key: const ValueKey('resource-keep-editing'),
                    onPressed: () => Navigator.pop(dialog, false),
                    child: Text(s.keepEditing),
                  ),
                  TextButton(
                    key: const ValueKey('resource-discard'),
                    onPressed: () => Navigator.pop(dialog, true),
                    child: Text(s.discard),
                  ),
                ],
              ),
            );
            if (discard == true && context.mounted) {
              session.discardChangesForClose();
              Navigator.pop(context);
            }
          } finally {
            handlingPop = false;
          }
        },
        child: Scaffold(
          appBar: AppBar(
            toolbarHeight: 80,
            title: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  record.name,
                  key: const ValueKey('resource-editor-title'),
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(
                    fontSize: 17,
                    fontWeight: FontWeight.w600,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  session.allowEditing
                      ? record.sourceLabel
                      : '${record.sourceLabel} · ${s.readOnly}',
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: TextStyle(
                    fontSize: 12,
                    color: Theme.of(context).colorScheme.onSurfaceVariant,
                  ),
                ),
              ],
            ),
            actions: [
              if (session.opened.fromSnapshot)
                IconButton(
                  tooltip: s.openOriginal,
                  icon: const Icon(Icons.source_outlined),
                  onPressed: () => openResourcePage(
                    context,
                    controller,
                    ResourceReference(
                      identity: session.opened.reference.identity,
                    ),
                  ),
                ),
            ],
          ),
          body: Column(
            children: [
              if (session.opened.fromSnapshot)
                Material(
                  color: Theme.of(context).colorScheme.surfaceContainer,
                  child: Padding(
                    padding: const EdgeInsets.all(12),
                    child: Row(
                      children: [
                        const Icon(Icons.history, size: 18),
                        const SizedBox(width: 8),
                        Expanded(
                          child: Text(
                            session.opened.sourceOffline
                                ? s.offlineCopy
                                : s.conversationVersion,
                            style: const TextStyle(fontSize: 12),
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              Expanded(
                child: choice.contributor.buildResourceEditor(
                  context,
                  choice.spec.id,
                  session,
                ),
              ),
            ],
          ),
        ),
      );
    },
  );
}
