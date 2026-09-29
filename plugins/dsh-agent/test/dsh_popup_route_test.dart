import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_dsh_plugin/src/dsh_web_view.dart';

final class _TestPopupRoute extends PopupRoute<void> {
  _TestPopupRoute(this.barrierColor);

  @override
  final Color? barrierColor;
  @override
  bool get barrierDismissible => true;
  @override
  String? get barrierLabel => null;
  @override
  Duration get transitionDuration => Duration.zero;
  @override
  Widget buildPage(
    BuildContext context,
    Animation<double> animation,
    Animation<double> secondaryAnimation,
  ) => const SizedBox.shrink();
}

void main() {
  test('only modal popups hide the Windows DSH WebView', () {
    final observer = DshPopupRouteObserver.instance;
    DshNativeOverlay.popupRoutes.value = 0;
    final menu = _TestPopupRoute(null);
    observer.didPush(menu, null);
    expect(DshNativeOverlay.obscured, isFalse);
    final dialog = _TestPopupRoute(Colors.black54);
    observer.didPush(dialog, menu);
    expect(DshNativeOverlay.obscured, isTrue);
    observer.didPop(menu, dialog);
    expect(DshNativeOverlay.obscured, isTrue);
    observer.didPop(dialog, null);
    expect(DshNativeOverlay.obscured, isFalse);
  });
}
