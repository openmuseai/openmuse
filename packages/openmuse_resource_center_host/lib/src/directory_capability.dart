import 'package:openmuse_resource_center/openmuse_resource_center.dart';

/// A Host-issued association between a Home directory and an already granted
/// provider repository. Neither paths nor repository grants come from App input.
final class HostRepositoryBinding {
  const HostRepositoryBinding({
    required this.sourceKey,
    required this.nodeRef,
    required this.repositoryId,
  });
  final String sourceKey, nodeRef, repositoryId;
}

/// Read-only access to the very same directory providers used by Home. Local
/// and remote providers keep their original authorization and generation checks.
final class HostDirectoryCapability {
  HostDirectoryCapability({
    required this.hub,
    required this.sourceKeys,
    required this.bindings,
    required this.authorized,
  });
  final ResourceCenterHub? Function() hub;
  final Set<String> sourceKeys;
  final List<HostRepositoryBinding> bindings;
  final bool Function() authorized;
  static const names = {
    'host_directory_sources',
    'host_directory_list',
    'host_directory_repository',
  };
  List<Map<String, Object?>> get tools => [
    for (final name in names)
      {
        'name': name,
        'description':
            'Read granted Home directories and their existing repository binding.',
        'inputSchema': {
          'type': 'object',
          'additionalProperties': false,
          'properties': name == 'host_directory_sources'
              ? <String, Object?>{}
              : {
                  'sourceKey': {'type': 'string'},
                  'nodeRef': {'type': 'string'},
                  if (name == 'host_directory_list')
                    'cursor': {'type': 'string'},
                },
          if (name != 'host_directory_sources')
            'required': ['sourceKey', 'nodeRef'],
        },
        'annotations': {'readOnlyHint': true},
        '_meta': {
          'ui': {
            'visibility': ['app'],
          },
        },
      },
  ];
  ResourceCenterHub _guard() {
    final current = hub();
    if (!authorized() || current == null) throw const ResourceFailure('DENIED');
    return current;
  }

  Future<Map<String, Object?>> call(
    String name,
    Map<String, Object?> args,
  ) async {
    try {
      final current = _guard(), generation = current.generation;
      if (!names.contains(name)) throw const ResourceFailure('DENIED');
      final allowed = name == 'host_directory_sources'
          ? <String>{}
          : {
              'sourceKey',
              'nodeRef',
              if (name == 'host_directory_list') 'cursor',
            };
      if (args.keys.any((key) => !allowed.contains(key))) {
        throw const ResourceFailure('INVALID_REFERENCE');
      }
      final providers = current.visibleProviders.where(
        (p) =>
            sourceKeys.contains(p.source.key) &&
            p.source.scope == current.scope,
      );
      Map<String, Object?> data;
      if (name == 'host_directory_sources') {
        data = {
          'contract': 'openmuse.host-directories/1',
          'sources': [
            for (final p in providers)
              {
                'sourceKey': p.source.key,
                'name': p.source.mountLabel,
                'device': p.source.deviceLabel,
                'deviceRef': p.source.deviceRef,
                'local': p.source.local,
                'available': p.available,
                'nodeRef': 'root',
              },
          ],
        };
      } else {
        final matching = providers.where(
          (p) => p.source.key == args['sourceKey'],
        );
        if (matching.length != 1) throw const ResourceFailure('DENIED');
        final provider = matching.single;
        if (!provider.available) throw const ResourceFailure('SOURCE_OFFLINE');
        final node = checkedRef(args['nodeRef']);
        // Listing also verifies the directory still exists and is authorized.
        final page = await provider.list(
          node,
          current.access(
            'mcp-app-directory',
            operations: {ResourceOperation.read, ResourceOperation.describe},
          ),
          cursor: args['cursor'] as String?,
          limit: 100,
        );
        if (name == 'host_directory_list') {
          data = {
            'directories': [
              for (final e in page.entries)
                if (e.isDirectory) {'nodeRef': e.nodeRef, 'name': e.name},
            ],
            if (page.nextCursor != null) 'nextCursor': page.nextCursor,
          };
        } else {
          final matches = bindings.where(
            (b) => b.sourceKey == provider.source.key && b.nodeRef == node,
          );
          data = {
            'repositoryId': matches.length == 1
                ? matches.single.repositoryId
                : null,
          };
        }
        if (!provider.available ||
            !current.visibleProviders.contains(provider)) {
          throw const ResourceFailure('SOURCE_OFFLINE');
        }
      }
      if (!identical(_guard(), current) || current.generation != generation) {
        throw const ResourceFailure('DENIED');
      }
      return {
        'structuredContent': data,
        'content': [
          {'type': 'text', 'text': 'Directory request completed'},
        ],
      };
    } catch (error) {
      final code = error is ResourceFailure
          ? error.code
          : 'DIRECTORY_REQUEST_FAILED';
      return {
        'isError': true,
        'content': [
          {'type': 'text', 'text': code},
        ],
      };
    }
  }
}
