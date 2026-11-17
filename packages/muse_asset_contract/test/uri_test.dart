import 'dart:convert';
import 'package:muse_asset_contract/muse_asset_contract.dart';
import 'package:test/test.dart';

void main() {
  final ref = AssetReference(
    provider: 'appflowy.private-assets/1',
    scope: 'workspace',
    owner: 'row',
    key: 'a' * 64,
    mediaType: 'image/png',
    byteLength: 128,
    sha256: 'a' * 64,
  );
  test(
    'portable original-media URL carries only the existing asset identity',
    () {
      final uri = ref.toUri();
      expect(uri.scheme, 'muse-asset');
      expect(uri.host, 'v1');
      expect(uri.hasQuery, false);
      expect(AssetReference.fromUri(uri).toJson(), ref.toJson());
      expect(uri.toString(), isNot(contains('http')));
    },
  );
  test(
    'URI rejects alternate authorities, extra grants and noncanonical identity payloads',
    () {
      final uri = ref.toUri();
      for (final candidate in [
        uri.replace(scheme: 'https'),
        uri.replace(host: 'other'),
        uri.replace(port: 443),
        uri.replace(userInfo: 'user'),
        uri.replace(query: 'token=synthetic'),
        uri.replace(fragment: 'x'),
        uri.replace(path: uri.path + '/extra'),
        Uri.parse('muse-asset://v1/invalid'),
      ]) {
        expect(() => AssetReference.fromUri(candidate), throwsFormatException);
      }
      for (final raw in [
        {...ref.toJson(), 'authorization': 'synthetic'},
        {...ref.toJson(), 'byteLength': 0},
      ]) {
        final encoded = base64Url
            .encode(utf8.encode(jsonEncode(raw)))
            .replaceAll('=', '');
        expect(
          () => AssetReference.fromUri(Uri.parse('muse-asset://v1/$encoded')),
          throwsFormatException,
        );
      }
      final pretty = base64Url
          .encode(
            utf8.encode(
              const JsonEncoder.withIndent(' ').convert(ref.toJson()),
            ),
          )
          .replaceAll('=', '');
      expect(
        () => AssetReference.fromUri(Uri.parse('muse-asset://v1/$pretty')),
        throwsFormatException,
      );
    },
  );
}
