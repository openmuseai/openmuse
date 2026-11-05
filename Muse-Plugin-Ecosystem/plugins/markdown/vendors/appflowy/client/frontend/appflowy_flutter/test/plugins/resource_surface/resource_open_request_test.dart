import 'dart:io';

import 'package:appflowy/plugins/resource_surface/engines/register.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  late Directory workspace;
  late Directory outside;

  setUp(() async {
    workspace = await Directory.systemTemp.createTemp('openmuse-resource-');
    outside = await Directory.systemTemp.createTemp('openmuse-outside-');
  });

  tearDown(() async {
    if (workspace.existsSync()) await workspace.delete(recursive: true);
    if (outside.existsSync()) await outside.delete(recursive: true);
  });

  test('routes docx, text and binary resources to their engines', () async {
    final docx = await File('${workspace.path}/brief.docx').writeAsBytes([1]);
    final source = await File('${workspace.path}/main.dart')
        .writeAsString('void main() {}');
    final image = await File('${workspace.path}/diagram.png').writeAsBytes([1]);
    final router = MuseLocalResourceRouter();

    expect(
      (await router.resolve(_request(docx.path))).engine,
      MuseLocalEngine.ioffice,
    );
    expect(
      (await router.resolve(_request(source.path))).engine,
      MuseLocalEngine.helix,
    );
    expect(
      (await router.resolve(_request(image.path))).engine,
      MuseLocalEngine.openFileViewer,
    );
  });

  test('routes the DSH product format matrix through the shared protocol',
      () async {
    final router = MuseLocalResourceRouter();
    final cases = <MuseLocalEngine, List<String>>{
      MuseLocalEngine.ioffice: ['docx'],
      MuseLocalEngine.helix: [
        'rs',
        'ts',
        'js',
        'java',
        'c',
        'md',
      ],
      MuseLocalEngine.openFileViewer: [
        'pdf',
        'pptx',
        'xlsx',
        'png',
        'html',
        'url',
        'mp4',
        'mp3',
      ],
    };

    for (final MapEntry(key: engine, value: extensions) in cases.entries) {
      for (final extension in extensions) {
        final file =
            await File('${workspace.path}/sample.$extension').writeAsBytes([1]);
        final resolved = await router.resolve(
          MuseResourceOpenRequest(
            path: file.path,
            origin: MuseResourceOpenOrigin.dshConversation,
            sessionCwd: workspace.path,
          ),
        );
        expect(resolved.engine, engine, reason: extension);
      }
    }
  });

  test('allows DSH resources only inside the declared session workspace',
      () async {
    final file = await File('${workspace.path}/notes.md').writeAsString('ok');
    final router = MuseLocalResourceRouter();

    final resolved = await router.resolve(
      MuseResourceOpenRequest(
        path: file.path,
        origin: MuseResourceOpenOrigin.dshConversation,
        sessionCwd: workspace.path,
      ),
    );
    expect(resolved.file.path, file.resolveSymbolicLinksSync());

    final outsideFile =
        await File('${outside.path}/secret.txt').writeAsString('secret');
    expect(
      () => router.resolve(
        MuseResourceOpenRequest(
          path: outsideFile.path,
          origin: MuseResourceOpenOrigin.dshConversation,
          sessionCwd: workspace.path,
        ),
      ),
      throwsA(
        isA<MuseResourceOpenException>().having(
          (error) => error.code,
          'code',
          'OUTSIDE_SESSION_WORKSPACE',
        ),
      ),
    );
  });

  test('resolves symlinks before enforcing DSH containment', () async {
    final outsideFile =
        await File('${outside.path}/secret.md').writeAsString('secret');
    final link = File('${workspace.path}/linked.md');
    await Link(link.path).create(outsideFile.path);

    expect(
      () => MuseLocalResourceRouter().resolve(
        MuseResourceOpenRequest(
          path: link.path,
          origin: MuseResourceOpenOrigin.dshConversation,
          sessionCwd: workspace.path,
        ),
      ),
      throwsA(isA<MuseResourceOpenException>()),
    );
  });
}

MuseResourceOpenRequest _request(String path) => MuseResourceOpenRequest(
      path: path,
      origin: MuseResourceOpenOrigin.hostPicker,
    );
