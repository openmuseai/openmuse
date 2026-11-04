import 'support/localized_test_app.dart';
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_mobile/host/mobile_resource_extensions.dart';

void main() {
  testWidgets(
    'global extension survives navigation and minimize; close revokes only its caller',
    (tester) async {
      final controller = MobileResourceExtensions(
        market: () async => throw StateError('not used'),
        contributors: () => [],
        approve: (_, _) async => false,
      );
      addTearDown(controller.dispose);
      final window = MobileExtensionWindow({
        'id': 'example/toolbar:test',
        'title': 'Example toolbar',
        'digest': 'a' * 64,
        'html': '<title>toolbar</title>',
        'width': 360,
        'height': 152,
      }, 'urn:resource:test');
      window.session = ResourceExtensionSession(
        caller: 'consumer',
        resourceUri: window.resourceUri,
        broker: controller.broker,
        contributors: () => [],
        approve: (_, _) async => false,
        isCurrent: () async => true,
      );
      controller.windows.add(window);
      final navigator = GlobalKey<NavigatorState>();
      await tester.pumpWidget(
        localizedTestApp(
          navigatorKey: navigator,
          builder: (context, child) => MobileResourceExtensionLayer(
            controller: controller,
            child: child!,
          ),
          home: const Scaffold(body: Text('Home')),
        ),
      );
      final state = tester.state(find.byType(RemoteMcpOfflineView));
      await tester.tap(find.byTooltip('缩小'));
      await tester.pump();
      expect(window.minimized, isTrue);
      expect(
        tester.state(find.byType(RemoteMcpOfflineView, skipOffstage: false)),
        same(state),
      );
      navigator.currentState!.push(
        MaterialPageRoute<void>(
          builder: (_) => const Scaffold(body: Text('Other route')),
        ),
      );
      await tester.pumpAndSettle();
      expect(find.text('Other route'), findsOneWidget);
      expect(find.byTooltip('展开'), findsOneWidget);
      await tester.tap(find.byTooltip('展开'));
      await tester.pump();
      expect(tester.state(find.byType(RemoteMcpOfflineView)), same(state));
      await tester.tap(find.byTooltip('关闭'));
      await tester.pumpAndSettle();
      expect(find.byType(RemoteMcpOfflineView), findsNothing);
      expect(find.text('Other route'), findsOneWidget);
      await expectLater(
        window.session.dispatch('tools/list', {}),
        throwsStateError,
      );
    },
  );
}
