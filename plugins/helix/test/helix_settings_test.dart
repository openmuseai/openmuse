import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_helix_plugin/openmuse_helix_plugin.dart';

void main() {
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
