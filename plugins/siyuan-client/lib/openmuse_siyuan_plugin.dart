library;

import 'dart:async';
import 'src/linked_content.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_siyuan_client/openmuse_siyuan_client.dart';

/// Resource format and domain capabilities stay in the independently packaged
/// plugin. Host workspace/navigation and version authority use the public SDK.
final class OpenMuseSiYuanPlugin
    implements
        OpenMusePlugin,
        OpenMuseResourceHandlerContributor,
        OpenMuseResourceCreatorContributor,
        OpenMuseResourceContextContributor {
  OpenMuseSiYuanPlugin({SiYuanKernel? kernel})
    : _kernel = kernel ?? SiYuanKernel();
  final SiYuanKernel _kernel;
  OpenMusePluginLocalizations? _localizations;
  bool get _chinese => _localizations?.locale.startsWith('zh') ?? false;
  String text(String en, String zh) => _chinese ? zh : en;
  @override
  OpenMusePluginDescriptor get descriptor => const OpenMusePluginDescriptor(
    id: 'io.openmuse.siyuan',
    name: 'OpenMuse Notes',
    version: '0.2.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: ['onStartup'],
    permissions: {'resource.read', 'resource.create', 'resource.edit'},
  );
  @override
  Future<void> activate(OpenMusePluginContext context) async {
    _localizations = context.localizations;
    await _kernel.resume();
  }

  @override
  Future<void> deactivate() => _kernel.suspend();
  @override
  List<ResourceHandlerSpec> get resourceHandlers => [
    ResourceHandlerSpec(
      id: 'siyuan.document',
      title: text('Block note', '块笔记'),
      mediaTypes: const {siYuanDocumentMediaType, 'application/zip'},
      extensions: const {'sydoc', 'zip'},
      editable: true,
      priority: 30,
    ),
    ResourceHandlerSpec(
      id: 'siyuan.source',
      title: text('Native block document', '原生块文档'),
      mediaTypes: const {
        'application/json',
        'application/vnd.siyuan.document+json',
      },
      extensions: const {'sy'},
      priority: 30,
    ),
  ];
  @override
  List<ResourceCreatorSpec> get resourceCreators => [
    ResourceCreatorSpec(
      id: 'siyuan.document',
      title: text('Block note', '块笔记'),
      mediaType: siYuanDocumentMediaType,
      extension: 'sydoc',
      defaultName: text('Untitled.sydoc', '未命名.sydoc'),
    ),
  ];
  @override
  Future<ResourceSeed> createResource(
    String creatorId,
    Map<String, Object?> values,
  ) async {
    if (creatorId != 'siyuan.document')
      throw const ResourceFailure('CREATOR_UNAVAILABLE');
    final title = '${values['title'] ?? text('Untitled', '未命名')}'.replaceFirst(
      RegExp(r'\.sydoc$', caseSensitive: false),
      '',
    );
    await _kernel.start(language: _chinese ? 'zh-CN' : 'en');
    return ResourceSeed(
      bytes: await SiYuanResourceApi(_kernel).seed(title),
      mediaType: siYuanDocumentMediaType,
    );
  }

  @override
  Future<bool> probeResource(
    ResourceRecord record,
    ResourceOpenIntent intent,
  ) async {
    final name = record.name.toLowerCase();
    return record.mediaType == siYuanDocumentMediaType ||
        name.endsWith('.sydoc') ||
        name.endsWith('.sy.zip') ||
        (name.endsWith('.sy') && intent == ResourceOpenIntent.view);
  }

  @override
  Widget buildResourceEditor(
    BuildContext context,
    String handlerId,
    ResourceEditorSession session,
  ) => _DocumentEditor(kernel: _kernel, session: session);
  @override
  Future<String?> resourceConversationText(ResourceContent content) async {
    final name = content.record.name.toLowerCase();
    if (content.record.mediaType != siYuanDocumentMediaType &&
        !name.endsWith('.sydoc') &&
        !name.endsWith('.sy.zip') &&
        !name.endsWith('.sy'))
      return null;
    return SiYuanDocumentArchive.decode(
      content.bytes,
      raw: name.endsWith('.sy'),
    ).conversationText();
  }

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      throw UnsupportedError('This resource requires a ResourceEditorSession');
}

final class _DocumentEditor extends StatefulWidget {
  const _DocumentEditor({required this.kernel, required this.session});
  final SiYuanKernel kernel;
  final ResourceEditorSession session;
  @override
  State<_DocumentEditor> createState() => _DocumentEditorState();
}

final class _DocumentEditorState extends State<_DocumentEditor> {
  SiYuanDocumentLease? lease;
  SiYuanResourceController? controller;
  Object? failure;
  bool ready = false, started = false;
  bool surfaceEditing = false;
  int generation = 0;
  Timer? saveTimer;
  Future<void>? saving;
  bool get raw => widget.session.record.name.toLowerCase().endsWith('.sy');
  bool get zh => Localizations.localeOf(context).languageCode == 'zh';
  String s(String en, String cn) => zh ? cn : en;

  @override
  void initState() {
    super.initState();
    widget.session.flushPendingChanges = flushForClose;
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    if (!started) {
      started = true;
      unawaited(open());
    }
  }

  Future<void> open() async {
    try {
      await widget.kernel.start(
        language: Localizations.localeOf(context).toLanguageTag(),
      );
      final value = await SiYuanResourceApi(
        widget.kernel,
      ).open(widget.session.bytes, raw: raw);
      if (!mounted) {
        await value.close();
        return;
      }
      setState(() => lease = value);
    } catch (error) {
      if (mounted) setState(() => failure = error);
    }
  }

  void changed() {
    if (!widget.session.allowEditing || raw) return;
    if (!widget.session.editing) widget.session.enterEditing();
    generation++;
    widget.session.markDirty(true);
    scheduleSave();
  }

  void scheduleSave() {
    saveTimer?.cancel();
    saveTimer = Timer(const Duration(milliseconds: 650), () {
      if (!mounted || !ready) return;
      unawaited(save().catchError((Object _) {}));
    });
  }

  Future<void> save() {
    if (saving != null) return saving!;
    final operation = persist(closing: false);
    saving = operation;
    return operation.whenComplete(() {
      saving = null;
      if (mounted && widget.session.dirty && failure == null) scheduleSave();
    });
  }

  Future<void> persist({required bool closing}) async {
    if (!ready || !widget.session.editing) return;
    try {
      await controller!.flush(preserveEditing: !closing);
      if (!widget.session.dirty) return;
      final captured = generation;
      final bytes = await lease!.snapshot();
      // Export only a quiescent working copy. A new input during export keeps
      // the resource dirty and schedules a fresh snapshot without moving focus.
      if (!closing && captured != generation) return;
      await widget.session.commit(bytes);
      if (captured != generation) widget.session.markDirty(true);
      if (mounted) setState(() => failure = null);
    } catch (error) {
      widget.session.markDirty(true);
      if (mounted) setState(() => failure = error);
      if (closing) {
        await controller?.web.runJavaScript(
          'window.openMuseResource?.setEditing($surfaceEditing)',
        );
      }
      rethrow;
    }
  }

  Future<void> flushForClose() async {
    saveTimer?.cancel();
    try {
      await saving;
    } on Object {
      /* Retry through the close barrier. */
    }
    await persist(closing: true);
  }

  @override
  void dispose() {
    saveTimer?.cancel();
    widget.session.flushPendingChanges = null;
    unawaited(lease?.close().catchError((Object _) {}) ?? Future.value());
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => Column(
    children: [
      if (failure != null)
        MaterialBanner(
          content: Text(
            lease == null
                ? s(
                    'Unable to open this note. The original file is unchanged.',
                    '无法打开此笔记，原始文件未被修改。',
                  )
                : s('Unable to save. Your edits are kept.', '自动保存失败，编辑内容已保留。'),
          ),
          actions: [
            TextButton(
              onPressed: () => unawaited(save().catchError((Object _) {})),
              child: Text(s('Retry', '重试')),
            ),
          ],
          key: const ValueKey('siyuan-resource-error'),
        ),
      Expanded(
        child: lease == null
            ? failure == null
                  ? const Center(child: CircularProgressIndicator())
                  : Center(
                      child: TextButton(
                        onPressed: () {
                          setState(() => failure = null);
                          unawaited(open());
                        },
                        child: Text(s('Retry', '重试')),
                      ),
                    )
            : SiYuanResourceView(
                lease: lease!,
                editing: surfaceEditing,
                canEdit: widget.session.allowEditing && !raw,
                onOpenLinkedContent: (name, bytes) async =>
                    await widget.session.openLinkedContent?.call(
                      name,
                      linkedDocumentContent(name, bytes),
                    ) ??
                    false,
                onOpenResourceUri: ResourceLinkRouting.maybeOf(context)?.open,
                onEditingChanged: (editing) {
                  if (editing && (!widget.session.allowEditing || raw)) return;
                  setState(() => surfaceEditing = editing);
                  if (editing && !widget.session.editing)
                    widget.session.enterEditing();
                },
                onController: (value) => controller = value,
                onDirty: changed,
                onSave: () => unawaited(save().catchError((Object _) {})),
                onReady: () {
                  setState(() => ready = true);
                  widget.session.reportOpened();
                  widget.session.markDirty(widget.session.dirty);
                },
              ),
      ),
    ],
  );
}
