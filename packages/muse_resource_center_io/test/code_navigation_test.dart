import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_resource_center/muse_resource_center.dart';
import 'package:muse_resource_center_io/muse_resource_center_io.dart';

class CatalogNavigation
    implements ResourceProvider, ResourceCodeNavigationProvider {
  CatalogNavigation(this.delegate);
  final LocalResourceProvider delegate;
  @override
  ResourceSource get source => delegate.source;
  @override
  bool get available => true;
  Future<void> Function()? beforeReturn;
  ResourceIdentity? target;
  @override
  Future<ResourceCodeResult> codeNavigation(
    ResourceReference base,
    ResourceCodeRequest request,
    ResourceAccess access,
  ) async {
    await beforeReturn?.call();
    final version = ResourceVersion(revision: 'v1', digest: 'a' * 64);
    return ResourceCodeResult(
      enabled: true,
      matches: [
        ResourceCodeTarget(
          reference: ResourceReference(
            identity: target ?? source.identity('definition'),
            version: version,
          ),
          label: 'Cloud symbol',
          line: 119,
          character: 4,
          endLine: 119,
          endCharacter: 8,
        ),
      ],
    );
  }

  @override
  dynamic noSuchMethod(Invocation invocation) => throw UnimplementedError();
}

void main() {
  test(
    'navigation RPC is owner/source scoped, supports opaque resources and rejects revoked results',
    () async {
      final dir = await Directory.systemTemp.createTemp('code-navigation-rpc-');
      final local = await LocalResourceProvider.open(
        dir,
        scope: 'account',
        workspace: 'workspace',
        label: 'Cloud',
        deviceLabel: 'Test',
      );
      final owner = CatalogNavigation(local),
          hub = ResourceCenterHub(scope: 'account');
      hub.register(owner);
      final rpc = ResourceRpcService(hub, snapshots: local);
      final peer = ResourcePeer(
        scope: 'account',
        deviceRef: 'mobile',
        workspaceRef: 'workspace',
        expiresAtMs: DateTime.now().millisecondsSinceEpoch + 60000,
      );
      final query = {
        'protocol': ResourceRpcService.protocol,
        'operation': 'codeNavigation',
        'arguments': {
          'base': ResourceReference(
            identity: owner.source.identity('base'),
          ).toJson(),
          'request': ResourceCodeRequest(
            ResourceCodeOperation.definition,
            version: ResourceVersion(revision: 'v1', digest: 'a' * 64),
          ).toJson(),
        },
      };
      final result = ResourceCodeResult.fromJson(
        await rpc.dispatch(query, peer),
      );
      expect(result.matches.single.line, 119);
      expect(result.matches.single.reference.identity.resource, 'definition');
      await expectLater(
        rpc.dispatch(
          query,
          ResourcePeer(
            scope: 'other',
            deviceRef: 'mobile',
            workspaceRef: 'workspace',
            expiresAtMs: peer.expiresAtMs,
          ),
        ),
        throwsA(isA<ResourceFailure>()),
      );
      owner.target = ResourceIdentity(
        scope: 'account',
        source: 'evil',
        workspace: 'workspace',
        mount: 'mount',
        resource: 'secret',
      );
      await expectLater(
        rpc.dispatch(query, peer),
        throwsA(isA<ResourceFailure>()),
      );
      owner.target = null;
      owner.beforeReturn = () async => hub.unregister(owner.source.key);
      await expectLater(
        rpc.dispatch(query, peer),
        throwsA(isA<ResourceFailure>()),
      );
      await rpc.close();
      await hub.dispose();
      await local.close();
      await dir.delete(recursive: true);
    },
  );
  test(
    'navigation contract rejects arbitrary operations and invalid target ranges',
    () {
      expect(
        () => ResourceCodeRequest.fromJson({'operation': 'executeCommand'}),
        throwsA(isA<ResourceFailure>()),
      );
      expect(
        () => ResourceCodeRequest(ResourceCodeOperation.definition),
        throwsA(isA<ResourceFailure>()),
      );
      expect(
        () => ResourceCodeRequest(ResourceCodeOperation.status, line: -1),
        throwsA(isA<ResourceFailure>()),
      );
    },
  );
}
