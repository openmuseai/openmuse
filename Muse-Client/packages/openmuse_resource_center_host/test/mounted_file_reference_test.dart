import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';

void main() {
  test(
    'mounted file resolves an opaque read-only resource and rejects outside files',
    () async {
      final root = await Directory.systemTemp.createTemp('mounted-reference-');
      final mount = await Directory('${root.path}/mount').create();
      final nested = await Directory('${mount.path}/nested').create();
      final file = await File(
        '${nested.path}/Note.sydoc',
      ).writeAsString('note');
      final outside = await File(
        '${root.path}/Outside.sydoc',
      ).writeAsString('private');
      for (var i = 0; i < 205; i++) {
        await File('${nested.path}/A-$i.txt').writeAsString('x');
      }
      final registry = OpenMusePluginRegistry(
        context: OpenMusePluginContext(
          executeHostCommand: (_, _) async => null,
        ),
      );
      final center = await NativeResourceCenter.open(
        scope: 'test',
        deviceRef: 'mac',
        deviceLabel: 'Mac',
        registry: registry,
        storageRoot: Directory('${root.path}/state'),
      );
      try {
        await center.mountDirectories({mount.path: 'Project'});
        final ref = await center.referenceForMountedFile(file);
        expect(ref, isNotNull);
        expect(ref!.identity.resource, isNot(contains('Note.sydoc')));
        final record = await center.controller.describe(ref.identity);
        expect(record.name, 'Note.sydoc');
        expect(record.readOnly, isTrue);
        expect(await center.referenceForMountedFile(outside), isNull);
        final link = Link('${mount.path}/Escape.sydoc');
        await link.create(outside.path);
        expect(await center.referenceForMountedFile(File(link.path)), isNull);
        await center.mountDirectories({});
        expect(await center.referenceForMountedFile(file), isNull);
      } finally {
        await center.close();
        await root.delete(recursive: true);
      }
    },
  );
}
