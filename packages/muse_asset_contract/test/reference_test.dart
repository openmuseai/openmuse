import 'package:muse_asset_contract/muse_asset_contract.dart';
import 'package:test/test.dart';

void main() {
  test(
    'construction and JSON ingress enforce the same content-neutral value invariants',
    () {
      AssetReference create(int bytes) => AssetReference(
        provider: 'provider/1',
        scope: 'private-scope',
        owner: 'owner',
        key: 'opaque-key',
        mediaType: 'image/png',
        byteLength: bytes,
        sha256: 'a' * 64,
      );
      final ref = create(10);
      expect(AssetReference.fromJson(ref.toJson()).toJson(), ref.toJson());
      expect(() => create(0), throwsFormatException);
      expect(
        () => AssetReference.fromJson({
          ...ref.toJson(),
          'authorization': 'Bearer secret',
        }),
        throwsFormatException,
      );
      expect(
        () => AssetReference.fromJson({...ref.toJson(), 'owner': 'bad\nowner'}),
        throwsFormatException,
      );
    },
  );
}
