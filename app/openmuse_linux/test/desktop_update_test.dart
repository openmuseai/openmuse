import 'dart:io';

import 'package:crypto/crypto.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_app_update/openmuse_app_update.dart';
import 'package:pub_semver/pub_semver.dart';

import 'package:openmuse_linux/src/host/desktop_update.dart';

void main() {
  test(
    'desktop update verifies the download and reuses only valid bytes',
    () async {
      if (!Platform.isMacOS) return;
      final directory = await Directory.systemTemp.createTemp(
        'openmuse-update-',
      );
      final server = await HttpServer.bind(InternetAddress.loopbackIPv4, 0);
      final bytes = List<int>.generate(4096, (index) => index % 251);
      var requests = 0;
      String? observedRange;
      server.listen((request) {
        requests++;
        final range = request.headers.value(HttpHeaders.rangeHeader);
        if (range != null) {
          observedRange = range;
          final offset = int.parse(
            range.substring('bytes='.length, range.length - 1),
          );
          request.response.statusCode = HttpStatus.partialContent;
          request.response.headers.set(
            HttpHeaders.contentRangeHeader,
            'bytes $offset-${bytes.length - 1}/${bytes.length}',
          );
          request.response.contentLength = bytes.length - offset;
          request.response.add(bytes.sublist(offset));
        } else {
          request.response.add(bytes);
        }
        request.response.close();
      });
      final updater = OpenMuseDesktopUpdate(stagingDirectory: directory);
      try {
        OpenMuseRelease release(String hash, int build) => OpenMuseRelease(
          channel: 'canary',
          target: 'macos-universal',
          version: Version.parse('1.0.3'),
          build: build,
          publishedAt: DateTime.utc(2026, 10, 13),
          downloadUri: Uri.parse('http://127.0.0.1:${server.port}/update.zip'),
          sha256: hash,
          sizeBytes: bytes.length,
          notes: const {'en': 'Test update'},
          installKind: 'macos-zip',
        );

        final good = release(sha256.convert(bytes).toString(), 4);
        final archive = await updater.stage(good);
        expect(await archive.readAsBytes(), bytes);
        expect(updater.progress.value, 1);
        expect(requests, 1);
        expect((await updater.stage(good)).path, archive.path);
        expect(requests, 1);

        final bad = release('0' * 64, 5);
        await expectLater(updater.stage(bad), throwsFormatException);
        expect(requests, 2);
        expect(
          File(
            '${directory.path}/updates/OpenMuse-1.0.3-5-${'0' * 64}.zip',
          ).existsSync(),
          isFalse,
        );

        final resumed = release(sha256.convert(bytes).toString(), 6);
        final partial = File(
          '${directory.path}/updates/OpenMuse-1.0.3-6-${resumed.sha256}.zip.partial',
        );
        await partial.writeAsBytes(bytes.sublist(0, 1024));
        final completed = await updater.stage(resumed);
        expect(await completed.readAsBytes(), bytes);
        expect(observedRange, 'bytes=1024-');
        expect(requests, 3);
      } finally {
        updater.dispose();
        await server.close(force: true);
        await directory.delete(recursive: true);
      }
    },
  );
}
