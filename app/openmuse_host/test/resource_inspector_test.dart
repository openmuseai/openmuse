import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host/src/host/resource_inspector.dart';
import 'package:openmuse_host/src/host/workspace_controller.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_helix_plugin/openmuse_helix_plugin.dart';

void main() {
  test(
    'unknown textual suffix routes to Helix after byte inspection',
    () async {
      final root = await Directory.systemTemp.createTemp('openmuse-inspect-');
      addTearDown(() => root.delete(recursive: true));
      final file = File('${root.path}/BUILD.custom')
        ..writeAsStringSync('target = "hello"\n');
      final workspace = LocalWorkspaceController(
        rootPath: root.path,
        initialResources: const [],
      );
      addTearDown(workspace.dispose);
      workspace.openResource(
        OpenMuseResource(uri: file.uri, displayName: 'BUILD.custom'),
      );
      final deadline = DateTime.now().add(const Duration(seconds: 3));
      while (workspace.activeTab!.inspecting &&
          DateTime.now().isBefore(deadline)) {
        await Future<void>.delayed(const Duration(milliseconds: 10));
      }
      final resource = workspace.activeTab!.resource;
      expect(resource.mediaType, 'text/plain');
      expect(
        OpenMuseHelixPlugin().descriptor.editors.single.accepts(resource),
        isTrue,
      );
    },
  );

  test('mp4 bytes are a video the text editor does not claim', () async {
    final root = await Directory.systemTemp.createTemp('openmuse-inspect-');
    addTearDown(() => root.delete(recursive: true));
    final file = File('${root.path}/openmuse-vs-dsh-20s.mp4')
      ..writeAsBytesSync([
        0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, // ....ftyp
        0x69, 0x73, 0x6f, 0x6d, 0x00, 0x00, 0x00, 0x00,
      ]);
    final resource = await inspectLocalResource(
      OpenMuseResource(
        uri: file.uri,
        displayName: 'openmuse-vs-dsh-20s.mp4',
      ),
    );
    expect(resource.mediaType, 'video/mp4');
    expect(
      OpenMuseHelixPlugin().descriptor.editors.single.accepts(resource),
      isFalse,
    );
  });

  test('binary signature overrides a misleading code suffix', () async {
    final root = await Directory.systemTemp.createTemp('openmuse-inspect-');
    addTearDown(() => root.delete(recursive: true));
    final file = File('${root.path}/not-code.rs')
      ..writeAsBytesSync([0x25, 0x50, 0x44, 0x46, 0x2d, 0x31, 0x2e, 0x37]);
    final resource = await inspectLocalResource(
      OpenMuseResource(uri: file.uri, displayName: 'not-code.rs'),
    );
    expect(resource.mediaType, 'application/pdf');
    expect(
      OpenMuseHelixPlugin().descriptor.editors.single.accepts(resource),
      isFalse,
    );
  });

  test('UTF-8 character split at inspection boundary remains text', () async {
    final root = await Directory.systemTemp.createTemp('openmuse-inspect-');
    addTearDown(() => root.delete(recursive: true));
    final file = File('${root.path}/message.unknown')
      ..writeAsBytesSync([
        ...List<int>.filled(16 * 1024 - 1, 0x61),
        0xe4,
        0xb8,
        0xad,
      ]);
    final resource = await inspectLocalResource(
      OpenMuseResource(uri: file.uri, displayName: 'message.unknown'),
    );
    expect(resource.mediaType, 'text/plain');
  });
}
