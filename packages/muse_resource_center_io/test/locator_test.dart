import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_resource_center/muse_resource_center.dart';
import 'package:muse_resource_center_io/muse_resource_center_io.dart';

// A non-filesystem owner proves that the core does not interpret source hints.
class CloudLocator
    implements
        ResourceProvider,
        ResourceLocatorProvider,
        ResourceRelativeLocatorProvider {
  CloudLocator(this.delegate);
  final LocalResourceProvider delegate;
  @override
  ResourceSource get source => delegate.source;
  @override
  bool get available => true;
  @override
  String get locatorOwner => 'cloud-test';
  ResourceReference? result;
  Future<void> Function()? beforeReturn;
  @override
  Future<ResourceLocatorResult> locateRelative(
    ResourceReference base,
    String target,
    ResourceAccess access,
  ) async {
    expect(base.identity, source.identity('base'));
    expect(target, 'catalog:neighbor');
    await beforeReturn?.call();
    return ResourceLocatorResult([
      ResourceLocatorMatch(result!, 'Cloud neighbor'),
    ]);
  }

  @override
  Future<ResourceLocatorResult> locate(
    ResourceLocator locator,
    ResourceAccess access,
  ) async {
    expect(locator.target, 'opaque-catalog-hint');
    await beforeReturn?.call();
    return ResourceLocatorResult([ResourceLocatorMatch(result!, 'Report')]);
  }

  @override
  dynamic noSuchMethod(Invocation invocation) => throw UnimplementedError();
}

void main() {
  test(
    'locator RPC stays source-owned, workspace/account scoped and revocable',
    () async {
      final root = await Directory.systemTemp.createTemp(
        'resource-locator-rpc-',
      );
      final hub = ResourceCenterHub(scope: 'account');
      final local = await LocalResourceProvider.open(
        root,
        scope: 'account',
        workspace: 'workspace',
        label: 'Cloud test',
        deviceLabel: 'Test',
      );
      final owner = CloudLocator(local);
      owner.result = ResourceReference(
        identity: owner.source.identity('report'),
      );
      hub.register(owner);
      final service = ResourceRpcService(hub, snapshots: local);
      final peer = ResourcePeer(
        scope: 'account',
        deviceRef: 'mobile',
        workspaceRef: 'workspace',
        expiresAtMs: DateTime.now().millisecondsSinceEpoch + 60000,
      );
      final locator = ResourceLocator(
        owner: 'cloud-test',
        kind: 'catalog',
        target: 'opaque-catalog-hint',
      );
      final request = {
        'protocol': ResourceRpcService.protocol,
        'operation': 'locate',
        'arguments': {
          'identity': owner.source.identity('root').toJson(),
          'locator': locator.toJson(),
        },
      };
      final result = ResourceLocatorResult.fromJson(
        await service.dispatch(request, peer),
      );
      expect(result.matches.single.reference.identity, owner.result!.identity);
      final relativeRequest = {
        'protocol': ResourceRpcService.protocol,
        'operation': 'locateRelative',
        'arguments': {
          'base': ResourceReference(
            identity: owner.source.identity('base'),
          ).toJson(),
          'target': 'catalog:neighbor',
        },
      };
      final neighbor = ResourceLocatorResult.fromJson(
        await service.dispatch(relativeRequest, peer),
      );
      expect(
        neighbor.matches.single.reference.identity,
        owner.result!.identity,
      );
      await expectLater(
        service.dispatch(
          relativeRequest,
          ResourcePeer(
            scope: 'other',
            deviceRef: 'mobile',
            workspaceRef: 'workspace',
            expiresAtMs: peer.expiresAtMs,
          ),
        ),
        throwsA(isA<ResourceFailure>()),
      );
      await expectLater(
        service.dispatch(
          request,
          ResourcePeer(
            scope: 'account',
            deviceRef: 'mobile',
            workspaceRef: 'other',
            expiresAtMs: peer.expiresAtMs,
          ),
        ),
        throwsA(isA<ResourceFailure>()),
      );
      await expectLater(
        service.dispatch(
          request,
          ResourcePeer(
            scope: 'other',
            deviceRef: 'mobile',
            workspaceRef: 'workspace',
            expiresAtMs: peer.expiresAtMs,
          ),
        ),
        throwsA(isA<ResourceFailure>()),
      );
      owner.beforeReturn = () async {
        hub.unregister(owner.source.key);
      };
      await expectLater(
        service.dispatch(relativeRequest, peer),
        throwsA(isA<ResourceFailure>()),
      );
      await service.close();
      await hub.dispose();
      await local.close();
      await root.delete(recursive: true);
    },
  );
}
