import 'dart:convert';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_resource_center/muse_resource_center.dart';

ResourceIdentity identity({String source = 'phone', String resource = 'r-1'}) =>
    ResourceIdentity(
      scope: 'account-1',
      source: source,
      workspace: 'personal',
      mount: 'documents',
      resource: resource,
    );
void main() {
  test(
    'qualified identity distinguishes sources without inspecting opaque refs',
    () {
      expect(identity(), isNot(identity(source: 'pc')));
      expect(identity(resource: 'r-1').key, identity(resource: 'r-1').key);
    },
  );
  test(
    'typed live/pinned/retained references roundtrip without paths or credentials',
    () {
      final version = ResourceVersion(
        revision: 'v-1',
        digest: resourceDigest(utf8.encode('old version')),
      );
      final ref = ResourceReference(
        identity: identity(),
        version: version,
        retained: RetainedResource(identity(source: 'snapshots'), version),
      );
      final decoded = ResourceReference.fromUri(ref.toUri().toString());
      expect(decoded.identity, ref.identity);
      expect(decoded.version, version);
      expect(decoded.retained!.identity.source, 'snapshots');
      expect(jsonEncode(ref.toJson()), isNot(contains('path')));
    },
  );
  test(
    'reject unknown protocol, extra fields, physical paths and oversized references',
    () {
      expect(
        () => ResourceReference.fromUri('file:///tmp/input.md'),
        throwsA(isA<ResourceFailure>()),
      );
      expect(
        () => ResourceReference.fromUri('openmuse-resource://v1/${'a' * 8200}'),
        throwsA(isA<ResourceFailure>()),
      );
      expect(
        () => ResourceIdentity.fromJson({
          ...identity().toJson(),
          'path': '/tmp/foo',
        }),
        throwsA(isA<ResourceFailure>()),
      );
      for (final bad in [
        '/tmp/foo',
        r'C:\secret',
        r'a\b',
        'a/b',
        'a b',
        'a\n',
      ]) {
        expect(
          () => identity(resource: bad),
          throwsA(isA<ResourceFailure>()),
          reason: bad,
        );
      }
      expect(
        () => ResourceReference.fromJson({
          'protocol': 'unreviewed/v1',
          'identity': identity().toJson(),
        }),
        throwsA(isA<ResourceFailure>()),
      );
    },
  );
  test(
    'retained version cannot cross accounts or substitute a different digest',
    () {
      final version = ResourceVersion(
        revision: 'v',
        digest: resourceDigest([1]),
      );
      expect(
        () => ResourceReference(
          identity: identity(),
          version: version,
          retained: RetainedResource(
            identity(source: 'snapshots'),
            ResourceVersion(revision: 's', digest: resourceDigest([2])),
          ),
        ),
        throwsA(isA<ResourceFailure>()),
      );
    },
  );
  test(
    'access scopes, resource limits, generation and expiry are enforced',
    () {
      const a = ResourceAccess(
        scope: 'account-1',
        audience: 'editor',
        generation: 3,
        expiresAtMs: 100,
        operations: {ResourceOperation.read},
      );
      expect(
        () => a.check(
          identity(),
          ResourceOperation.read,
          currentGeneration: 3,
          nowMs: 99,
        ),
        returnsNormally,
      );
      for (final fn in [
        () => a.check(
          identity(),
          ResourceOperation.edit,
          currentGeneration: 3,
          nowMs: 99,
        ),
        () => a.check(
          identity(),
          ResourceOperation.read,
          currentGeneration: 4,
          nowMs: 99,
        ),
        () => a.check(
          identity(),
          ResourceOperation.read,
          currentGeneration: 3,
          nowMs: 100,
        ),
      ]) {
        expect(fn, throwsA(isA<ResourceFailure>()));
      }
      const limited = ResourceAccess(
        scope: 'account-1',
        audience: 'agent',
        generation: 3,
        expiresAtMs: 100,
        operations: {ResourceOperation.read},
        resourceKeys: {},
      );
      expect(
        () => limited.check(
          identity(),
          ResourceOperation.read,
          currentGeneration: 3,
          nowMs: 1,
        ),
        throwsA(isA<ResourceFailure>()),
      );
    },
  );
}
