library;

import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:url_launcher/url_launcher.dart';

/// An ordinary installed client plugin. All format-specific creation, rendering
/// and editing lives here; the shared resource Host has no document whitelist.
final class OpenMuseResourceNotesPlugin
    implements
        OpenMusePlugin,
        OpenMuseResourceHandlerContributor,
        OpenMuseResourceCreatorContributor,
        OpenMuseResourceContextContributor {
  OpenMusePluginLocalizations? _locale;
  String _s(String key) =>
      _locale?.message('resourcecenter.$key') ??
      OpenMuseStrings.forLocale('en').format('resourcecenter.$key');
  @override
  OpenMusePluginDescriptor get descriptor => OpenMusePluginDescriptor(
    id: 'com.openmuse.resource-notes',
    name: _s('notes'),
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: const ['onStartup'],
    permissions: const {'resource.read', 'resource.create', 'resource.edit'},
  );
  @override
  Future<void> activate(OpenMusePluginContext context) async {
    _locale = context.localizations;
  }

  @override
  Future<void> deactivate() async {}
  @override
  Future<String?> resourceConversationText(ResourceContent content) async {
    if (!{
      'text/plain',
      'text/markdown',
      'text/uri-list',
    }.contains(content.record.mediaType))
      return null;
    if (content.bytes.length > 1024 * 1024)
      throw const ResourceFailure('CONTEXT_TOO_LARGE');
    return utf8.decode(content.bytes);
  }

  @override
  List<ResourceHandlerSpec> get resourceHandlers => [
    ResourceHandlerSpec(
      id: 'notes.text',
      title: _s('notes'),
      mediaTypes: const {'text/plain', 'text/markdown'},
      extensions: const {'txt', 'md'},
      editable: true,
      priority: 10,
    ),
    ResourceHandlerSpec(
      id: 'notes.link',
      title: _s('webLink'),
      mediaTypes: const {'text/uri-list'},
      extensions: const {'url'},
    ),
  ];
  @override
  List<ResourceCreatorSpec> get resourceCreators => [
    ResourceCreatorSpec(
      id: 'notes.markdown',
      title: _s('markdown'),
      mediaType: 'text/markdown',
      extension: 'md',
      defaultName: _s('untitledMarkdown'),
    ),
    ResourceCreatorSpec(
      id: 'notes.text',
      title: _s('plainText'),
      mediaType: 'text/plain',
      extension: 'txt',
      defaultName: _s('untitledText'),
    ),
    ResourceCreatorSpec(
      id: 'notes.link',
      title: _s('webLink'),
      mediaType: 'text/uri-list',
      extension: 'url',
      defaultName: 'link.url',
      inputSchema: const {
        'type': 'object',
        'properties': {
          'url': {'type': 'string', 'format': 'uri', 'maxLength': 2048},
        },
        'required': ['url'],
        'additionalProperties': false,
      },
    ),
  ];
  @override
  Future<ResourceSeed> createResource(
    String creatorId,
    Map<String, Object?> values,
  ) async {
    return switch (creatorId) {
      'notes.markdown' => ResourceSeed(
        bytes: utf8.encode('# ${values['title'] ?? ''}\n\n'),
        mediaType: 'text/markdown',
      ),
      'notes.text' => const ResourceSeed(bytes: [], mediaType: 'text/plain'),
      'notes.link' => ResourceSeed(
        bytes: utf8.encode('${_url(values['url'])}\n'),
        mediaType: 'text/uri-list',
      ),
      _ => throw const ResourceFailure('CREATOR_UNAVAILABLE'),
    };
  }

  static Uri _url(Object? value) {
    final uri = value is String ? Uri.tryParse(value) : null;
    if (uri == null ||
        !{'https', 'http'}.contains(uri.scheme) ||
        uri.host.isEmpty ||
        uri.userInfo.isNotEmpty) {
      throw const ResourceFailure('INVALID_URL');
    }
    return uri;
  }

  @override
  Future<bool> probeResource(
    ResourceRecord record,
    ResourceOpenIntent intent,
  ) async => resourceHandlers.any(
    (h) =>
        h.accepts(record) && (intent == ResourceOpenIntent.view || h.editable),
  );
  @override
  Widget buildResourceEditor(
    BuildContext context,
    String handlerId,
    ResourceEditorSession session,
  ) => handlerId == 'notes.link'
      ? _LinkView(session: session)
      : _NotesView(session: session);
  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      const SizedBox.shrink();
  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}

final class _NotesView extends StatefulWidget {
  const _NotesView({required this.session});
  final ResourceEditorSession session;
  @override
  State<_NotesView> createState() => _NotesViewState();
}

final class _NotesViewState extends State<_NotesView> {
  late final TextEditingController text;
  String? failure;
  bool busy = false;
  @override
  void initState() {
    super.initState();
    try {
      text = TextEditingController(text: utf8.decode(widget.session.bytes));
    } on FormatException {
      text = TextEditingController();
      failure = 'INVALID_CONTENT';
    }
    text.addListener(_edited);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted && failure == null) widget.session.reportOpened();
    });
  }

  void _edited() {
    if (widget.session.editing) widget.session.markDirty(true);
  }

  @override
  void dispose() {
    text.removeListener(_edited);
    text.dispose();
    super.dispose();
  }

  Future<void> save() async {
    if (busy) return;
    setState(() => busy = true);
    try {
      await widget.session.commit(utf8.encode(text.text));
      if (mounted) {
        setState(() => failure = null);
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text(stringsOf(context).resourcecenterSaved())),
        );
      }
    } on ResourceFailure catch (e) {
      if (mounted) setState(() => failure = e.code);
    } on Object {
      if (mounted) setState(() => failure = 'FAILED');
    } finally {
      if (mounted) setState(() => busy = false);
    }
  }

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: widget.session,
    builder: (context, _) {
      final s = stringsOf(context), session = widget.session;
      return Column(
        children: [
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
            child: Row(
              children: [
                Expanded(
                  child: Text(
                    session.opened.fromSnapshot
                        ? s.resourcecenterConversationVersion()
                        : session.editing
                        ? s.resourcecenterEdit()
                        : s.resourcecenterView(),
                    style: Theme.of(context).textTheme.labelMedium,
                  ),
                ),
                if (session.allowEditing && !session.editing)
                  TextButton.icon(
                    key: const ValueKey('resource-edit'),
                    onPressed: session.enterEditing,
                    icon: const Icon(Icons.edit_outlined, size: 18),
                    label: Text(s.resourcecenterEdit()),
                  ),
                if (session.editing)
                  FilledButton(
                    key: const ValueKey('resource-save'),
                    onPressed: busy || !session.dirty ? null : save,
                    child: Text(
                      busy ? s.resourcecenterSaving() : s.resourcecenterSave(),
                    ),
                  ),
              ],
            ),
          ),
          if (failure != null)
            Padding(
              padding: const EdgeInsets.all(16),
              child: Text(
                failure == 'REVISION_CONFLICT'
                    ? s.resourcecenterConflict()
                    : s.resourcecenterFailed(),
                key: const ValueKey('resource-editor-error'),
                style: TextStyle(color: Theme.of(context).colorScheme.error),
              ),
            ),
          Expanded(
            child: TextField(
              key: const ValueKey('resource-notes-content'),
              controller: text,
              readOnly: !session.editing || busy,
              expands: true,
              minLines: null,
              maxLines: null,
              textAlignVertical: TextAlignVertical.top,
              style: const TextStyle(fontSize: 16, height: 1.65),
              decoration: const InputDecoration(
                contentPadding: EdgeInsets.all(22),
                border: InputBorder.none,
              ),
            ),
          ),
        ],
      );
    },
  );
}

final class _LinkView extends StatefulWidget {
  const _LinkView({required this.session});
  final ResourceEditorSession session;
  @override
  State<_LinkView> createState() => _LinkViewState();
}

final class _LinkViewState extends State<_LinkView> {
  Uri? uri;
  bool failed = false;
  @override
  void initState() {
    super.initState();
    try {
      uri = OpenMuseResourceNotesPlugin._url(
        utf8.decode(widget.session.bytes).trim(),
      );
    } on Object {
      failed = true;
    }
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted && !failed) widget.session.reportOpened();
    });
  }

  @override
  Widget build(BuildContext context) {
    final s = stringsOf(context);
    return Padding(
      padding: const EdgeInsets.all(24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const SizedBox(height: 24),
          Icon(
            Icons.link_rounded,
            size: 42,
            color: Theme.of(context).colorScheme.primary,
          ),
          const SizedBox(height: 24),
          SelectableText(
            uri?.toString() ?? s.resourcecenterInvalidUrl(),
            style: const TextStyle(fontSize: 18, height: 1.5),
          ),
          const SizedBox(height: 24),
          FilledButton(
            onPressed: uri == null
                ? null
                : () async {
                    final opened = await launchUrl(
                      uri!,
                      mode: LaunchMode.externalApplication,
                    );
                    if (!opened && context.mounted)
                      ScaffoldMessenger.of(context).showSnackBar(
                        SnackBar(content: Text(s.resourcecenterFailed())),
                      );
                  },
            child: Text(s.resourcecenterOpenBrowser()),
          ),
        ],
      ),
    );
  }
}
