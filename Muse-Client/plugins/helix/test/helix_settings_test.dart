import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_helix_plugin/openmuse_helix_plugin.dart';

void main() {
  testWidgets('settings switch between Vim and VS Code input profiles', (
    tester,
  ) async {
    if (!Platform.isMacOS) return;
    final runtime = HelixRuntimePool(
      executable: File('assets/engines/helix/hx').absolute.path,
    );
    addTearDown(runtime.dispose);
    expect(await tester.runAsync(runtime.probeCapabilities), isTrue);
    final plugin = OpenMuseHelixPlugin(runtime: runtime);
    Future<void> selectProfile(HelixInputProfile profile) async {
      final dropdown = tester.widget<DropdownButton<HelixInputProfile>>(
        find.byType(DropdownButton<HelixInputProfile>),
      );
      await tester.runAsync(() async {
        dropdown.onChanged!(profile);
        for (var attempt = 0; attempt < 100; attempt++) {
          if (plugin.currentPreferences.inputProfile == profile) return;
          await Future<void>.delayed(const Duration(milliseconds: 10));
        }
        throw StateError('设置未完成输入模式切换：$profile');
      });
      await tester.pump();
    }

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SingleChildScrollView(
            child: Builder(builder: plugin.buildSettings),
          ),
        ),
      ),
    );
    expect(find.text('Vim 模式'), findsOneWidget);
    final choices = tester.widget<DropdownButton<HelixInputProfile>>(
      find.byType(DropdownButton<HelixInputProfile>),
    );
    expect(choices.items!.map((item) => (item.child as Text).data), [
      'Vim 模式',
      'VS Code 模式',
    ]);
    expect(choices.items!.last.enabled, isTrue);
    await selectProfile(HelixInputProfile.standardNonmodal);
    expect(
      runtime.preferences.inputProfile,
      HelixInputProfile.standardNonmodal,
    );
    expect(find.text('VS Code 模式'), findsOneWidget);
    await selectProfile(HelixInputProfile.helixModal);
    expect(runtime.preferences.inputProfile, HelixInputProfile.helixModal);
  });

  testWidgets('LS install action appears only after selecting a language', (
    tester,
  ) async {
    final plugin = OpenMuseHelixPlugin();
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: SingleChildScrollView(
            child: Builder(builder: plugin.buildSettings),
          ),
        ),
      ),
    );
    expect(find.textContaining('一键安装'), findsNothing);
    await tester.tap(find.text('选择语言并配置 LS…'));
    await tester.pumpAndSettle();
    expect(find.textContaining('一键安装'), findsNothing);
    await tester.tap(find.byType(DropdownButton<String>).last);
    await tester.pumpAndSettle();
    await tester.tap(find.text('Rust').last);
    await tester.pumpAndSettle();
    expect(find.text('一键安装此语言的 LS'), findsOneWidget);
  });
}
