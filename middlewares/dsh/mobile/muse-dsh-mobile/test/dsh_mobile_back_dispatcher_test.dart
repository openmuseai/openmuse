import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_mobile/src/capabilities/dsh_mobile_back_dispatcher.dart';

void main() {
  testWidgets('AppBar back pops when the page does not consume it',
      (tester) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Builder(
          builder: (context) => TextButton(
            onPressed: () {
              Navigator.of(context).push(
                MaterialPageRoute<void>(
                  builder: (_) => const _LockedPage(),
                ),
              );
            },
            child: const Text('open'),
          ),
        ),
      ),
    );
    await tester.tap(find.text('open'));
    await tester.pumpAndSettle();
    expect(find.text('agent'), findsOneWidget);
    await tester.tap(find.byTooltip('Back'));
    await tester.pumpAndSettle();
    expect(find.text('open'), findsOneWidget);
    expect(find.text('agent'), findsNothing);
  });
}

class _LockedPage extends StatelessWidget {
  const _LockedPage();

  @override
  Widget build(BuildContext context) {
    return PopScope(
      canPop: false,
      onPopInvokedWithResult: (didPop, _) async {
        if (didPop) return;
        await const DshMobileBackDispatcher(null).handle(context);
      },
      child: Scaffold(
        appBar: AppBar(
          automaticallyImplyLeading: false,
          leading: IconButton(
            tooltip: MaterialLocalizations.of(context).backButtonTooltip,
            icon: const BackButtonIcon(),
            onPressed: () {
              const DshMobileBackDispatcher(null).handle(context);
            },
          ),
          title: const Text('agent'),
        ),
        body: const SizedBox.expand(),
      ),
    );
  }
}
