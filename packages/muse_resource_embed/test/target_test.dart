import 'package:flutter_test/flutter_test.dart';
import 'package:muse_resource_embed/muse_resource_embed.dart';

void main() {
  final reference = ResourceReference(
    identity: ResourceIdentity(
      scope: 'test',
      source: 'local',
      workspace: 'test',
      mount: 'root',
      resource: 'dataset',
    ),
  );
  test(
    'portable resource identity and opaque selector round trip; no authority envelope',
    () {
      final t = ResourceEmbedTarget(
        reference: reference,
        surface: 'dataset.grid',
        selector: {'objectId': 'object', 'viewId': 'view'},
      );
      expect(ResourceEmbedTarget.fromJson(t.toJson()).key, t.key);
      expect(() => t.selector['viewId'] = 'other', throwsUnsupportedError);
      expect(
        () =>
            ResourceEmbedTarget.fromJson({...t.toJson(), 'token': 'forbidden'}),
        throwsA(isA<ResourceFailure>()),
      );
      final foreign = {
        ...t.toJson(),
        'reference': {
          ...reference.toJson(),
          'endpoint': 'https://untrusted.invalid',
        },
      };
      expect(
        () => ResourceEmbedTarget.fromJson(foreign),
        throwsA(isA<ResourceFailure>()),
      );
      expect(
        () => ResourceEmbedTarget(
          reference: reference,
          surface: 'dataset.grid',
          selector: {'viewId': '\n'},
        ),
        throwsA(isA<ResourceFailure>()),
      );
    },
  );
}
