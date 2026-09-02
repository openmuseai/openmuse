import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_mobile/muse_dsh_mobile.dart';

void main() {
  test('bump invalidates previous generation', () {
    final session = DshWebViewSession();
    final first = session.generation;
    expect(session.isLive(first), isTrue);
    final second = session.bump();
    expect(second, first + 1);
    expect(session.isLive(first), isFalse);
    expect(session.isLive(second), isTrue);
    expect(session.phase, DshWebViewPhase.idle);
  });

  test('fatal phase ignores later enter until bump', () {
    final session = DshWebViewSession();
    session.enter(DshWebViewPhase.fatalError);
    session.enter(DshWebViewPhase.documentReady);
    expect(session.phase, DshWebViewPhase.fatalError);
    session.bump();
    session.enter(DshWebViewPhase.loadingDocument);
    expect(session.phase, DshWebViewPhase.loadingDocument);
  });
}
