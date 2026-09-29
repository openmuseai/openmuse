import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';
import 'package:openmuse_mobile/main.dart';

void main() {
  testWidgets('mobile composition shows Cloud Workspace', (tester) async {
    await tester.pumpWidget(
      OpenMuseHostShell(composition: mobileComposition()),
    );
    await tester.pumpAndSettle();
    expect(find.text('OpenMuse Cloud'), findsOneWidget);
    expect(find.text('Cloud Workspace'), findsOneWidget);
  });
}
