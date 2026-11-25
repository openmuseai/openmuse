import 'dart:async';
import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'controller.dart';
import 'resource_open.dart';
import 'resource_view.dart';
import 'strings.dart';

/// Generic installed-resource dispatch. Content selectors and their UI remain
/// in contributors; a portable parent locator is never an authorization grant.
final class ResourceEmbeddingScope extends StatefulWidget {
  const ResourceEmbeddingScope({
    super.key,
    required this.controller,
    required this.child,
  });
  final ResourceCenterController? controller;
  final Widget child;
  @override
  State<ResourceEmbeddingScope> createState() => _ResourceEmbeddingScopeState();
}

class _ResourceEmbeddingScopeState extends State<ResourceEmbeddingScope> {
  late ResourceEmbedHost _host;
  @override
  void initState() {
    super.initState();
    _bind();
  }

  @override
  void didUpdateWidget(ResourceEmbeddingScope old) {
    super.didUpdateWidget(old);
    if (!identical(old.controller, widget.controller)) _bind();
  }

  void _bind() {
    _host = ResourceEmbedHost(
      build: (context, target, policy) => _ResolvedEmbed(
        key: ValueKey(
          '${identityHashCode(widget.controller)}:${target.key}:${policy.allowLive}',
        ),
        controller: widget.controller,
        target: target,
        policy: policy,
      ),
      pick: (context, surface) => _pick(context, widget.controller, surface),
    );
  }

  @override
  Widget build(BuildContext context) => ResourceEmbedScope(
    host: widget.controller == null ? null : _host,
    allowEditing: true,
    child: widget.child,
  );
}

Future<ResourceHandlerChoice> _choice(
  ResourceCenterController c,
  ResourceRecord record,
  String surface,
) async {
  for (final choice in await c.handlers(record)) {
    final contributor = choice.plugin;
    if (contributor is OpenMuseResourceEmbedContributor &&
        (contributor as OpenMuseResourceEmbedContributor).resourceEmbedSurfaces
            .contains(surface))
      return choice;
  }
  throw const ResourceFailure('HANDLER_UNAVAILABLE');
}

Future<ResourceEmbedTarget?> _pick(
  BuildContext context,
  ResourceCenterController? c,
  String surface,
) async {
  if (c == null || !c.editorLifetime.active)
    throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
  bool supports(ResourceRecord r) => c.registry.descriptors.any((d) {
    final p = c.registry.plugin(d.id);
    return c.registry.stateOf(d.id) == OpenMusePluginState.active &&
        p is OpenMuseResourceEmbedContributor &&
        (p as OpenMuseResourceEmbedContributor).resourceEmbedSurfaces.contains(
          surface,
        ) &&
        p is OpenMuseResourceHandlerContributor &&
        (p as OpenMuseResourceHandlerContributor).resourceHandlers.any(
          (h) => h.accepts(r),
        );
  });
  return showDialog<ResourceEmbedTarget>(
    context: context,
    builder: (dialog) => Dialog(
      key: const ValueKey('resource-embed-picker'),
      child: SizedBox(
        width: 720,
        height: 620,
        child: Column(
          children: [
            Row(
              children: [
                const SizedBox(width: 20),
                const Expanded(child: Text('选择要插入的资源')),
                IconButton(
                  key: const ValueKey('resource-embed-picker-close'),
                  onPressed: () => Navigator.pop(dialog),
                  icon: const Icon(Icons.close),
                ),
              ],
            ),
            Expanded(
              child: ResourceCenterView(
                controller: c,
                recordFilter: supports,
                onSelectResource: (record) async {
                  try {
                    final opened = await c.resolve(record.reference());
                    final choice = await _choice(
                      c,
                      opened.content.record,
                      surface,
                    );
                    final contributor =
                        choice.plugin as OpenMuseResourceEmbedContributor;
                    final targets = await contributor.resourceEmbedTargets(
                      opened,
                      surface,
                    );
                    if (!c.editorLifetime.active ||
                        c.registry.stateOf(choice.plugin.descriptor.id) !=
                            OpenMusePluginState.active)
                      throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
                    if (!dialog.mounted) return;
                    if (targets.isEmpty)
                      throw const ResourceFailure('EMBED_UNAVAILABLE');
                    ResourceEmbedTarget? selected;
                    if (targets.length == 1) {
                      selected = targets.single.target;
                    } else {
                      selected = await showDialog<ResourceEmbedTarget>(
                        context: dialog,
                        builder: (views) => SimpleDialog(
                          title: const Text('选择视图'),
                          children: [
                            for (final target in targets)
                              SimpleDialogOption(
                                onPressed: () =>
                                    Navigator.pop(views, target.target),
                                child: Text(target.title),
                              ),
                          ],
                        ),
                      );
                    }
                    if (selected != null && dialog.mounted)
                      Navigator.pop(dialog, selected);
                  } on Object catch (e) {
                    if (dialog.mounted) showResourceError(dialog, e);
                  }
                },
              ),
            ),
          ],
        ),
      ),
    ),
  );
}

class _ResolvedEmbed extends StatefulWidget {
  const _ResolvedEmbed({
    super.key,
    required this.controller,
    required this.target,
    required this.policy,
  });
  final ResourceCenterController? controller;
  final ResourceEmbedTarget target;
  final ResourceEmbedPolicy policy;
  @override
  State<_ResolvedEmbed> createState() => _ResolvedEmbedState();
}

class _ResolvedEmbedState extends State<_ResolvedEmbed> {
  static const _maxDepth = 16;
  VoidCallback? removeBarrier;
  ResourceEditorSession? session;
  ResourceHandlerChoice? choice;
  Object? failure;
  Set<ResourceSessionScope> lineage = const {};
  Future<void>? _opening;
  @override
  void initState() {
    super.initState();
    unawaited(_start());
  }

  Future<void> _start() =>
      _opening ??= _resolve().whenComplete(() => _opening = null);

  Future<void> _resolve() async {
    if (!widget.policy.allowLive) return;
    try {
      final inherited = context
          .getInheritedWidgetOfExactType<_ResourceEmbedLineage>();
      final parent = context
          .getInheritedWidgetOfExactType<ResourceEditorSessionScope>()
          ?.session;
      final ancestors = <ResourceSessionScope>{
        ...?inherited?.scopes,
        if (parent != null) parent.ownershipScope,
      };
      if (ancestors.length >= _maxDepth) {
        throw const ResourceFailure('RESOURCE_EMBED_DEPTH_EXCEEDED');
      }
      final c = widget.controller;
      if (c == null) throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
      var opened = await c.resolve(widget.target.reference);
      if (widget.target.reference.pinned && !opened.fromSnapshot) {
        opened = OpenedResource(
          reference: widget.target.reference,
          content: opened.content,
          fromSnapshot: true,
        );
      }
      final scope = ResourceSessionScope(
        opened.content.record.identity,
        snapshotVersion: opened.fromSnapshot
            ? opened.content.record.version
            : null,
      );
      if (ancestors.contains(scope)) {
        throw const ResourceFailure('RESOURCE_DEPENDENCY_CYCLE');
      }
      final selected = await _choice(
        c,
        opened.content.record,
        widget.target.surface,
      );
      final contributor = selected.plugin;
      if (contributor is OpenMuseResourceEmbedValidator) {
        await (contributor as OpenMuseResourceEmbedValidator)
            .validateResourceEmbedTarget(opened, widget.target);
      }
      if (!mounted) return;
      if (!c.editorLifetime.active ||
          (parent != null && !parent.isPresentationActive)) {
        throw const ResourceFailure('RESOURCE_SESSION_CLOSED');
      }
      if (c.registry.stateOf(contributor.descriptor.id) !=
              OpenMusePluginState.active ||
          !identical(
            c.registry.plugin(contributor.descriptor.id),
            contributor,
          )) {
        throw const ResourceFailure('HANDLER_UNAVAILABLE');
      }
      final next = c.createEditorSession(opened, selected);
      try {
        removeBarrier = parent?.addDependentSession(next);
      } on Object {
        next.dispose();
        rethrow;
      }
      setState(() {
        session = next;
        choice = selected;
        lineage = {...ancestors, next.ownershipScope};
      });
    } on Object catch (e) {
      if (mounted) setState(() => failure = e);
    }
  }

  @override
  void dispose() {
    removeBarrier?.call();
    session?.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (!widget.policy.allowLive)
      return const Center(child: Text('历史文档中的资源引用未连接实时内容'));
    final c = widget.controller;
    if (c == null) return const Center(child: Text('资源暂不可用'));
    return ListenableBuilder(
      listenable: c.registry,
      builder: (context, _) {
        if (failure != null)
          return Align(
            alignment: Alignment.topLeft,
            child: SingleChildScrollView(
              key: const ValueKey('resource-embed-failure'),
              padding: const EdgeInsets.all(12),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(resourceErrorText(context, failure!)),
                  TextButton(
                    key: const ValueKey('resource-embed-retry'),
                    onPressed: () {
                      setState(() => failure = null);
                      unawaited(_start());
                    },
                    child: Text(
                      ResourceCenterStrings(
                        context,
                      ).strings.resourcecenterEmbedRetry(),
                    ),
                  ),
                ],
              ),
            ),
          );
        if (session == null)
          return const Center(child: CircularProgressIndicator());
        if (!c.editorLifetime.active ||
            c.registry.stateOf(choice!.plugin.descriptor.id) !=
                OpenMusePluginState.active)
          return const Center(child: Text('资源暂不可用'));
        return _ResourceEmbedLineage(
          scopes: lineage,
          child: ResourceEditorSessionScope(
            session: session!,
            child: Builder(
              builder: (context) =>
                  (choice!.plugin as OpenMuseResourceEmbedContributor)
                      .buildResourceEmbed(
                        context,
                        session!,
                        widget.target,
                        widget.policy,
                      ),
            ),
          ),
        );
      },
    );
  }
}

/// Presentation ancestry only; never serialized into a portable locator or ACL.
final class _ResourceEmbedLineage extends InheritedWidget {
  const _ResourceEmbedLineage({required this.scopes, required super.child});
  final Set<ResourceSessionScope> scopes;
  @override
  bool updateShouldNotify(_ResourceEmbedLineage old) =>
      scopes.length != old.scopes.length || !scopes.containsAll(old.scopes);
}
