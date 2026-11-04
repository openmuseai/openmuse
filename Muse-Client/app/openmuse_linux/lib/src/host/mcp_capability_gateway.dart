import 'dart:convert';
import 'dart:math';
import 'dart:typed_data';

import 'package:crypto/crypto.dart';
import 'package:mcp_dart/mcp_dart.dart';

/// Public capability profile v1, carried by ordinary MCP tools. Each instance
/// belongs to one View, including development Views. No shared Agent token.
final class CapabilityProvider {
  CapabilityProvider({
    required this.id,
    required this.version,
    required this.digest,
    required this.contracts,
    required this.tools,
    required this.invoke,
    required this.isCurrent,
    this.close,
  });
  final String id, version, digest;
  final List<Map<String, Object?>> contracts;
  final List<Map<String, Object?>> tools;
  final Future<Map<String, Object?>> Function(String, Map<String, Object?>)
  invoke;
  final Future<bool> Function() isCurrent;
  final Future<void> Function()? close;
  Map<String, Object?> toJson() => {
    'id': id,
    'version': version,
    'sha256': digest,
    'contracts': contracts,
    'tools': tools,
  };
}

final class GrantedResource {
  GrantedResource({
    required this.title,
    required this.mediaType,
    required this.bytes,
  });
  final String title, mediaType;
  final Uint8List bytes;
}

final class _Binding {
  _Binding(this.provider, this.resource, this.tools);
  final CapabilityProvider provider;
  final GrantedResource resource;
  final Set<String> tools;
}

final class McpCapabilityGateway {
  McpCapabilityGateway({
    required this.describeHost,
    required this.listProviders,
    required this.chooseResource,
    required this.approve,
    required this.openDevelopment,
    required this.readDevelopment,
    required this.scope,
  }) : _scope = scope();
  final Future<Map<String, Object?>> Function() describeHost;
  final Future<List<CapabilityProvider>> Function() listProviders;
  final Future<GrantedResource?> Function(String title, String mediaType)
  chooseResource;
  final Future<bool> Function(String title, String details) approve;
  final Future<Map<String, Object?>> Function(Map<String, Object?>)
  openDevelopment;
  final Future<Map<String, Object?>> Function(Map<String, Object?>)
  readDevelopment;
  final String Function() scope;
  final String _scope;
  final _resources = <String, GrantedResource>{};
  final _bindings = <String, _Binding>{};
  bool _closed = false;
  String _id() => List.generate(
    24,
    (_) => Random.secure().nextInt(256).toRadixString(16).padLeft(2, '0'),
  ).join();
  void _check() {
    if (_closed || scope() != _scope) throw StateError('CALLER_SCOPE_REVOKED');
  }

  static Map<String, Object?> _tool(
    String name,
    String description,
    Map<String, Object?> properties,
    List<String> required,
  ) => {
    'name': name,
    'description': description,
    'inputSchema': {
      'type': 'object',
      'properties': properties,
      'required': required,
      'additionalProperties': false,
    },
    'outputSchema': {'type': 'object'},
  };
  static const _string = {'type': 'string', 'minLength': 1, 'maxLength': 1024};
  static final tools = <Map<String, Object?>>[
    _tool(
      'development_read_resource',
      'Read a resource from a development run owned by this View.',
      {'runRef': _string, 'uri': _string},
      ['runRef', 'uri'],
    ),
    _tool(
      'host_describe',
      'Describe the exact running Host build and public profiles.',
      {},
      [],
    ),
    _tool(
      'capabilities_list',
      'List installed providers and their public contracts. No source access.',
      {},
      [],
    ),
    _tool(
      'resources_choose',
      'Ask the user to authorize a single local resource for this View.',
      {'title': _string, 'mediaType': _string},
      ['title', 'mediaType'],
    ),
    _tool(
      'capabilities_bind',
      'Authorize explicit tools of an installed provider for one chosen resource.',
      {
        'provider': _string,
        'resourceRef': _string,
        'contract': _string,
        'tools': {
          'type': 'array',
          'items': _string,
          'minItems': 1,
          'maxItems': 16,
          'uniqueItems': true,
        },
      },
      ['provider', 'resourceRef', 'contract', 'tools'],
    ),
    _tool(
      'capabilities_call',
      'Invoke an authorized tool. File content is injected by the Host, never by the consumer.',
      {
        'binding': _string,
        'tool': _string,
        'arguments': {'type': 'object'},
      },
      ['binding', 'tool', 'arguments'],
    ),
    _tool(
      'capabilities_release',
      'Release one binding owned by this View.',
      {'binding': _string},
      ['binding'],
    ),
    _tool(
      'development_open_bundle',
      'Open a digest-verified candidate MCPB in an isolated development View after user approval.',
      {
        'content': {'type': 'string', 'contentEncoding': 'base64'},
        'sha256': {'type': 'string', 'pattern': r'^[a-f0-9]{64}$'},
      },
      ['content', 'sha256'],
    ),
  ];
  bool handles(String name) => tools.any((tool) => tool['name'] == name);
  Future<Map<String, Object?>> call(
    String name,
    Map<String, Object?> args,
  ) async {
    _check();
    final definition = tools.where((tool) => tool['name'] == name).single;
    _validator(
      JsonSchema.fromJson(
        Map<String, dynamic>.from(definition['inputSchema'] as Map),
      ),
    )(args);
    switch (name) {
      case 'development_read_resource':
        return readDevelopment(args);
      case 'host_describe':
        return describeHost();
      case 'capabilities_list':
        final providers = await listProviders();
        try {
          _check();
          return {'providers': providers.map((p) => p.toJson()).toList()};
        } finally {
          for (final p in providers) {
            await p.close?.call();
          }
        }
      case 'resources_choose':
        if (_resources.length >= 8) {
          throw StateError('RESOURCE_BUDGET_EXCEEDED');
        }
        final selected = await chooseResource(
          args['title'] as String,
          args['mediaType'] as String,
        );
        _check();
        if (selected == null) throw StateError('USER_CANCELLED');
        if (selected.bytes.length > 64 * 1024 * 1024) {
          throw StateError('RESOURCE_TOO_LARGE');
        }
        final id = _id();
        _resources[id] = selected;
        return {
          'resourceRef': id,
          'title': selected.title,
          'mediaType': selected.mediaType,
          'size': selected.bytes.length,
          'revision': sha256.convert(selected.bytes).toString(),
        };
      case 'capabilities_bind':
        if (_bindings.length >= 8) throw StateError('BINDING_BUDGET_EXCEEDED');
        final resource = _resources[args['resourceRef']];
        if (resource == null) throw StateError('RESOURCE_NOT_GRANTED');
        final providers = await listProviders();
        CapabilityProvider? selected;
        try {
          selected = providers.where((p) => p.id == args['provider']).single;
          final contract = selected.contracts
              .where((c) => c['id'] == args['contract'])
              .single;
          final allowed = (args['tools'] as List).cast<String>().toSet();
          if (!allowed.every(
            (name) => (contract['tools'] as List).contains(name),
          )) {
            throw StateError('TOOL_NOT_IN_CONTRACT');
          }
          for (final name in allowed) {
            final schema =
                selected.tools.singleWhere(
                      (tool) => tool['name'] == name,
                    )['inputSchema']
                    as Map;
            final content = (schema['properties'] as Map?)?['content'];
            if (content is! Map ||
                content['contentEncoding'] != 'base64' ||
                content['contentMediaType'] != resource.mediaType) {
              throw StateError('RESOURCE_CONTRACT_MISMATCH');
            }
          }
          if (!await approve(
            '授权插件能力',
            '${selected.id} ${selected.version}\n${resource.title}\n${allowed.join(', ')}\n仅当前插件页面可调用；关闭或切换帐号即撤销。',
          )) {
            throw StateError('PERMISSION_DENIED');
          }
          _check();
          if (!await selected.isCurrent()) throw StateError('PROVIDER_CHANGED');
          final id = _id();
          _bindings[id] = _Binding(selected, resource, allowed);
          final result = {
            'binding': id,
            'provider': selected.id,
            'version': selected.version,
            'sha256': selected.digest,
            'contract': contract,
            'resourceTitle': resource.title,
          };
          // Ownership transfers to the binding; all other sessions are closed.
          for (final p in providers) {
            if (!identical(p, selected)) await p.close?.call();
          }
          return result;
        } catch (_) {
          for (final p in providers) {
            await p.close?.call();
          }
          rethrow;
        }
      case 'capabilities_call':
        final binding = _bindings[args['binding']];
        final tool = args['tool'] as String;
        if (binding == null || !binding.tools.contains(tool)) {
          throw StateError('BINDING_NOT_GRANTED');
        }
        final input = Map<String, Object?>.from(args['arguments'] as Map);
        if (input.containsKey('content') || input.containsKey('name')) {
          throw StateError('RESOURCE_OVERRIDE_DENIED');
        }
        final definition = binding.provider.tools.singleWhere(
          (value) => value['name'] == tool,
        );
        final schema = Map<String, dynamic>.from(
          definition['inputSchema'] as Map,
        );
        input['content'] = base64Encode(binding.resource.bytes);
        if ((schema['properties'] as Map).containsKey('name')) {
          input['name'] = binding.resource.title;
        }
        _validator(JsonSchema.fromJson(schema))(input);
        if (!await binding.provider.isCurrent()) {
          throw StateError('PROVIDER_CHANGED');
        }
        _check(); // Recheck after every asynchronous authorization lookup.
        final result = await binding.provider
            .invoke(tool, input)
            .timeout(const Duration(seconds: 90));
        _check();
        if (jsonEncode(result).length > 4 * 1024 * 1024) {
          throw StateError('OUTPUT_BUDGET_EXCEEDED');
        }
        final output = definition['outputSchema'];
        if (result['isError'] != true && output is Map) {
          _validator(JsonSchema.fromJson(Map<String, dynamic>.from(output)))(
            result['structuredContent'],
          );
        }
        return result;
      case 'capabilities_release':
        final binding = _bindings.remove(args['binding']);
        if (binding == null) throw StateError('BINDING_NOT_GRANTED');
        await binding.provider.close?.call();
        return {'released': true};
      case 'development_open_bundle':
        return openDevelopment(args);
      default:
        throw StateError('METHOD_UNAVAILABLE');
    }
  }

  Future<void> close() async {
    _closed = true;
    _resources.clear();
    final bindings = _bindings.values.toList();
    _bindings.clear();
    for (final b in bindings) {
      await b.provider.close?.call();
    }
  }
}

void Function(dynamic) _validator(JsonSchema schema) => schema.validate;
