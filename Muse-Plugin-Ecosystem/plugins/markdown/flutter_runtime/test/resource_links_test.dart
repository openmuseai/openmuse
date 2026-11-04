import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_siyuan_client/src/resource_links.dart';

void main() {
  final origin = Uri.parse('http://127.0.0.1:54321');
  test(
    'attachment, phone file, web and canonical resource links are distinct',
    () {
      expect(
        resolveResourceLink(origin, 'assets/1234.docx').kind,
        ResourceLinkKind.asset,
      );
      expect(
        resolveResourceLink(origin, '$origin/assets/1234.docx?box=note').kind,
        ResourceLinkKind.asset,
      );
      expect(
        resolveResourceLink(
          origin,
          '/storage/emulated/0/Download/1234.docx',
        ).kind,
        ResourceLinkKind.localFile,
      );
      expect(
        resolveResourceLink(
          origin,
          'file:///storage/emulated/0/1234.docx',
        ).kind,
        ResourceLinkKind.localFile,
      );
      expect(
        resolveResourceLink(origin, 'content://documents/1234').kind,
        ResourceLinkKind.external,
      );
      expect(
        resolveResourceLink(origin, 'https://example.org/1234.docx').kind,
        ResourceLinkKind.external,
      );
      expect(
        resolveResourceLink(origin, 'openmuse-resource://reference/x').kind,
        ResourceLinkKind.resource,
      );
      for (final link in [
        'javascript:alert(1)',
        '$origin/api/system/getConf',
        'assets/%2e%2e/private.txt',
        'assets/../private.txt',
        'assets/a\\b.docx',
      ]) {
        expect(
          resolveResourceLink(origin, link).kind,
          ResourceLinkKind.blocked,
          reason: link,
        );
      }
    },
  );
  test(
    'asset downloads carry the local session and never follow redirects',
    () async {
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      addTearDown(() => server.close(force: true));
      final base = Uri.parse('http://127.0.0.1:${server.port}');
      server.listen((request) async {
        expect(
          request.headers.value('X-OpenMuse-Local-Session'),
          'test-session',
        );
        if (request.uri.path.endsWith('redirect.docx')) {
          request.response.statusCode = 302;
          request.response.headers.set('location', 'https://example.org/');
        } else {
          request.response.add([1, 2, 3]);
        }
        await request.response.close();
      });
      expect(
        await readResourceLinkAsset(base, base.resolve('/assets/1234.docx'), {
          'X-OpenMuse-Local-Session': 'test-session',
        }),
        [1, 2, 3],
      );
      await expectLater(
        readResourceLinkAsset(base, base.resolve('/assets/redirect.docx'), {
          'X-OpenMuse-Local-Session': 'test-session',
        }),
        throwsA(isA<FileSystemException>()),
      );
    },
  );
}
