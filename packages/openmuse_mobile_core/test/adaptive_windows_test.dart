import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:test/test.dart';

void main() {
  const projector = AdaptiveWindowProjector();
  const windows = [LogicalWindow('a'), LogicalWindow('b'), LogicalWindow('c')];
  test('360 600 960dp project one two three stable windows', () {
    expect(
      projector
          .project(width: 360, height: 800, windows: windows)
          .visible
          .length,
      1,
    );
    expect(
      projector
          .project(width: 600, height: 800, windows: windows)
          .visible
          .length,
      2,
    );
    expect(
      projector
          .project(width: 960, height: 800, windows: windows)
          .visible
          .length,
      3,
    );
  });
  test('fold segments and constrained height stay non-negative', () {
    final fold = projector.project(
      width: 900,
      height: 700,
      windows: windows,
      segments: const [DisplaySegment(430, 700), DisplaySegment(430, 700)],
    );
    expect(fold.visible.length, 2);
    expect(
      projector
          .project(
            width: 960,
            height: 300,
            windows: windows,
            imeHeight: 299,
            fontScale: 2,
          )
          .visible
          .length,
      1,
    );
  });
  test('200 rotations preserve instance refs and never duplicate surfaces', () {
    for (var index = 0; index < 200; index++) {
      final result = projector.project(
        width: index.isEven ? 360 : 960,
        height: 700,
        windows: windows,
        activeIndex: index % 3,
      );
      expect(result.lifecycle.keys, {'a', 'b', 'c'});
      expect(
        result.visible.map((item) => item.instanceRef).toSet().length,
        result.visible.length,
      );
    }
  });
}
