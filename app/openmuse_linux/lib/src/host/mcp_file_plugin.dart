import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'mcp_market.dart';
import 'mcp_resource_extensions.dart';

/// Adapts any MCP tool with a JSON Schema base64 file input to a Host editor.
/// The MIME type is standard JSON Schema contentMediaType, supplied by MCP.
final class OpenMuseMcpFilePlugin implements OpenMusePlugin {
  OpenMuseMcpFilePlugin({
    required McpMarketEntry entry,
    required this.runtime,
    required this.toolName,
    required this.mediaType,
    required this.extension,
    this.nativeFlutterEditor = false,
    this.agentBroker,
    this.extensions,
    this.contributors,
    this.approve,
  }) : descriptor = OpenMusePluginDescriptor(
         id: entry.name,
         name: entry.title,
         version: entry.version,
         runtime: OpenMusePluginRuntime.nativeProcess,
         activationEvents: ['onFileType:$extension'],
         permissions: {'filesystem.workspace.read'},
         editors: [
           OpenMuseEditorContribution(
             id: '${entry.name}.file',
             mediaTypes: {mediaType},
             extensions: {extension},
             priority: 50,
           ),
         ],
       );

  final McpBundleRuntime runtime;
  final String toolName;
  final String mediaType;
  final String extension;
  final bool nativeFlutterEditor;
  final ResourceAgentBroker? agentBroker;
  final McpResourceExtensions? extensions;
  final List<({String id, OpenMuseToolContributor plugin})> Function()?
  contributors;
  final Future<bool> Function(String, String)? approve;

  @override
  final OpenMusePluginDescriptor descriptor;

  @override
  Future<void> activate(OpenMusePluginContext context) async {}

  @override
  Future<void> deactivate() async {}

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) {
    if (!resource.uri.isScheme('file') || resource.extension != extension) {
      return const Center(child: Text('请选择本地文件'));
    }
    return _McpFileEditor(
      key: ValueKey('mcp-file-${resource.uri}'),
      runtime: runtime,
      toolName: toolName,
      resource: resource,
      mediaType: mediaType,
      agentBroker: agentBroker,
      extensions: extensions,
      contributors: contributors,
      approve: approve,
    );
  }

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;
}

String? extensionForMediaType(String mediaType) => switch (mediaType) {
  'application/vnd.openxmlformats-officedocument.presentationml.presentation' =>
    'pptx',
  'application/pdf' => 'pdf',
  'text/plain' => 'txt',
  'text/markdown' => 'md',
  _ => null,
};

/// Derives a Host editor contribution from a verified MCPB server's standard
/// tool input schema. The bundle supplies no Host-specific manifest fields.
Future<OpenMuseMcpFilePlugin?> discoverMcpFilePlugin({
  required McpMarketEntry entry,
  required McpBundleRuntime runtime,
  ResourceAgentBroker? agentBroker,
  McpResourceExtensions? extensions,
  List<({String id, OpenMuseToolContributor plugin})> Function()? contributors,
  Future<bool> Function(String, String)? approve,
}) async {
  for (final tool in await runtime.tools()) {
    final schema = tool['inputSchema'];
    final properties = schema is Map ? schema['properties'] : null;
    final content = properties is Map ? properties['content'] : null;
    if (content is! Map || content['contentEncoding'] != 'base64') continue;
    final mediaType = content['contentMediaType'];
    if (mediaType is! String) continue;
    final extension = extensionForMediaType(mediaType);
    if (extension == null || tool['name'] is! String) continue;
    return OpenMuseMcpFilePlugin(
      entry: entry,
      runtime: runtime,
      toolName: tool['name'] as String,
      mediaType: mediaType,
      extension: extension,
      agentBroker: agentBroker,
      extensions: extensions,
      contributors: contributors,
      approve: approve,
      nativeFlutterEditor: false,
    );
  }
  return null;
}

final class _McpFileEditor extends StatefulWidget {
  const _McpFileEditor({
    super.key,
    required this.runtime,
    required this.toolName,
    required this.resource,
    required this.mediaType,
    this.agentBroker,
    this.extensions,
    this.contributors,
    this.approve,
  });
  final McpBundleRuntime runtime;
  final String toolName;
  final OpenMuseResource resource;
  final String mediaType;
  final ResourceAgentBroker? agentBroker;
  final McpResourceExtensions? extensions;
  final List<({String id, OpenMuseToolContributor plugin})> Function()?
  contributors;
  final Future<bool> Function(String, String)? approve;
  @override
  State<_McpFileEditor> createState() => _McpFileEditorState();
}

final class _McpFileEditorState extends State<_McpFileEditor> {
  MethodChannel? _channel;
  ResourceAppSession? _session;
  int _sequence = 0;
  final _extensions = <String, McpResourceExtension>{};
  final _extensionSessions = <String, ResourceExtensionSession>{};
  bool? _active;
  Map<String, Object?> _presentation = const {};
  bool _viewReady = false;
  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    final next = mcpAppPresentation(context);
    if (jsonEncode(next) != jsonEncode(_presentation)) {
      _presentation = next;
      unawaited(_sendPresentation());
    }
  }

  Future<void> _sendPresentation() async {
    if (!_viewReady || _channel == null) return;
    try {
      await _channel!.invokeMethod<void>('message', {
        '_hostContext': _presentation,
      });
    } catch (_) {}
  }

  @override
  void initState() {
    super.initState();
    widget.agentBroker?.addListener(_activityChanged);
    widget.extensions?.addListener(_extensionsChanged);
  }

  Future<void> _extensionsChanged() async {
    final extensions =
        await widget.extensions?.forType(widget.mediaType) ??
        <McpResourceExtension>[];
    if (!mounted || _channel == null) return;
    _extensions.clear();
    for (final e in extensions) {
      _extensions[e.id] = e;
    }
    await _channel!.invokeMethod<void>('message', {
      '_hostExtensionDocuments': [
        for (final e in extensions)
          {
            'id': e.id,
            'html': base64Encode(utf8.encode(restrictMcpAppDocument(e.html))),
            'arguments': {
              'resourceUri': widget.resource.uri.toString(),
              'name': widget.resource.displayName,
              'mediaType': widget.mediaType,
              'active':
                  widget.agentBroker?.activeUri ==
                  widget.resource.uri.toString(),
            },
            'width': e.width,
            'height': e.height,
          },
      ],
    });
    _active = null;
    _activityChanged();
  }

  void _activityChanged() {
    final active =
        widget.agentBroker?.activeUri == widget.resource.uri.toString();
    if (active == _active) return;
    _active = active;
    for (final session in _extensionSessions.values) {
      unawaited(session.close());
    }
    _extensionSessions.clear();
    if (active) {
      for (final e in _extensions.values) {
        _extensionSessions[e.id] = ResourceExtensionSession(
          caller:
              'extension-${identityHashCode(this)}-${e.id}-${DateTime.now().microsecondsSinceEpoch}',
          resourceUri: widget.resource.uri.toString(),
          broker: widget.agentBroker!,
          contributors: widget.contributors ?? () => [],
          approve: widget.approve ?? (_, _) async => false,
          isCurrent: e.isCurrent,
          resourceTools: e.resourceTools,
          serverTools: e.tools,
          serverCall: e.invoke,
        );
      }
    }
    unawaited(
      _channel?.invokeMethod<void>('message', {
            '_hostResourceActive': active,
          }) ??
          Future.value(),
    );
  }

  final Map<String, Completer<Map<String, Object?>>> _pending = {};

  Future<Map<String, Object?>> _request(
    String method,
    Map<String, Object?> params,
  ) async {
    final channel = _channel;
    if (channel == null || !mounted) throw StateError('Resource view closed');
    final id = 'host:${++_sequence}';
    final result = Completer<Map<String, Object?>>();
    _pending[id] = result;
    try {
      await channel.invokeMethod<void>('message', {
        'jsonrpc': '2.0',
        'id': id,
        'method': method,
        'params': params,
      });
      return await result.future.timeout(const Duration(seconds: 45));
    } finally {
      _pending.remove(id);
    }
  }

  void _created(int id) {
    final channel = MethodChannel('com.openmuse.mcp/app-view/$id');
    _channel = channel;
    _viewReady = false;
    _active = null;
    _activityChanged();
    final session = ResourceAppSession(
      uri: widget.resource.uri.toString(),
      title: widget.resource.displayName,
      request: _request,
    );
    _session = session;
    channel.setMethodCallHandler((call) async {
      if (call.method != 'message' || call.arguments is! Map) return;
      final message = Map<String, Object?>.from(call.arguments as Map);
      final extensionId = message['_hostExtension'];
      if (extensionId != null) {
        final response = <String, Object?>{
          'jsonrpc': '2.0',
          'id': message['id'],
          '_hostExtension': extensionId,
        };
        try {
          final extension = _extensionSessions[extensionId];
          if (extension == null) throw StateError('RESOURCE_SESSION_CLOSED');
          response['result'] = await extension.dispatch(
            message['method'] as String,
            Map<String, Object?>.from(message['params'] as Map? ?? {}),
          );
        } catch (error) {
          final reason =
              error is StateError &&
                  RegExp(r'^[A-Z_]+$').hasMatch(error.message)
              ? error.message
              : 'EXTENSION_REQUEST_FAILED';
          response['error'] = {'code': -32602, 'message': reason};
        }
        if (mounted) await channel.invokeMethod<void>('message', response);
        return;
      }
      final pending = _pending[message['id']];
      if (message['method'] == null && pending != null) {
        if (message['error'] != null) {
          pending.completeError(
            StateError('Resource tool rejected: ${message['error']}'),
          );
        } else {
          pending.complete(
            Map<String, Object?>.from(message['result'] as Map? ?? {}),
          );
        }
        return;
      }
      final params = Map<String, Object?>.from(message['params'] as Map? ?? {});
      try {
        switch (message['method']) {
          case 'ui/notifications/initialized':
            _viewReady = true;
            await _sendPresentation();
            widget.agentBroker?.register(session);
            return;
          case 'ui/update-model-context':
            if (jsonEncode(params).length > 128 * 1024) {
              throw const FormatException('Context exceeds budget');
            }
            session.context = params;
          case 'ui/message':
            if (widget.agentBroker?.activeUri != session.uri) {
              throw StateError('Inactive resource');
            }
            final sender = widget.agentBroker?.sendAgentMessage;
            if (sender == null) throw StateError('No Agent plugin available');
            final content = (params['content'] as List)
                .map((e) => Map<String, Object?>.from(e as Map))
                .toList();
            if (jsonEncode(content).length > 128 * 1024) {
              throw const FormatException('Message exceeds budget');
            }
            await sender([
              {
                'type': 'text',
                'text':
                    'Active resource: ${session.uri}. Discover its live tools using list_open_resources and list_resource_tools; edit and save through call_resource_tool.',
              },
              ...content,
            ]);
          default:
            throw UnsupportedError('Method unavailable');
        }
        await channel.invokeMethod<void>('message', {
          'jsonrpc': '2.0',
          'id': message['id'],
          'result': {},
        });
      } catch (_) {
        await channel.invokeMethod<void>('message', {
          'jsonrpc': '2.0',
          'id': message['id'],
          'error': {'code': -32602, 'message': 'Agent request rejected'},
        });
      }
    });
  }

  @override
  void dispose() {
    widget.agentBroker?.removeListener(_activityChanged);
    widget.extensions?.removeListener(_extensionsChanged);
    for (final extension in _extensionSessions.values) {
      unawaited(extension.close());
    }
    _extensionSessions.clear();
    if (_session case final session?) widget.agentBroker?.unregister(session);
    _channel?.setMethodCallHandler(null);
    for (final pending in _pending.values) {
      pending.completeError(StateError('Resource view closed'));
    }
    _pending.clear();
    super.dispose();
  }

  late final Future<Map<String, Object?>> _view = _load();
  Future<Map<String, Object?>> _load() async {
    final html = await widget.runtime.uiDocument(widget.toolName);
    if (html == null) {
      return widget.runtime.callWithFile(
        toolName: widget.toolName,
        filePath: widget.resource.uri.toFilePath(),
      );
    }
    final file = File(widget.resource.uri.toFilePath());
    if (await file.length() > 64 * 1024 * 1024) {
      throw const FormatException('File exceeds 64 MiB');
    }
    for (final e
        in await widget.extensions?.forType(widget.mediaType) ??
            <McpResourceExtension>[]) {
      _extensions[e.id] = e;
    }
    return {
      'html': buildMcpAppHostDocument(
        appHtml: html,
        arguments: {
          'content': base64Encode(await file.readAsBytes()),
          'name': widget.resource.displayName,
        },
        platform: 'desktop',
        presentation: _presentation,
        extensions: [
          for (final e in _extensions.values)
            McpAppExtensionDocument(
              id: e.id,
              html: e.html,
              width: e.width,
              height: e.height,
              arguments: {
                'resourceUri': widget.resource.uri.toString(),
                'name': widget.resource.displayName,
                'mediaType': widget.mediaType,
                'active':
                    widget.agentBroker?.activeUri ==
                    widget.resource.uri.toString(),
              },
            ),
        ],
        agentAvailable: widget.agentBroker != null,
      ),
    };
  }

  @override
  Widget build(BuildContext context) => FutureBuilder<Map<String, Object?>>(
    future: _view,
    builder: (context, snapshot) {
      if (snapshot.hasError) {
        return Center(child: SelectableText('插件打开失败：${snapshot.error}'));
      }
      if (!snapshot.hasData) {
        return const Center(child: CircularProgressIndicator());
      }
      final data = snapshot.data!;
      if (data['html'] is String || data['uri'] is String) {
        return AppKitView(
          viewType: 'com.openmuse.mcp/app-view',
          creationParams: {
            if (data['html'] is String)
              'html': data['html']
            else
              'url': data['uri'],
            'filePath': widget.resource.uri.toFilePath(),
          },
          creationParamsCodec: const StandardMessageCodec(),
          onPlatformViewCreated: _created,
        );
      }
      return SelectableText(const JsonEncoder.withIndent('  ').convert(data));
    },
  );
}
