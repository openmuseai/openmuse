import 'package:flutter_test/flutter_test.dart';
import 'package:spreadsheet_editor/src/sheet_axis_geometry.dart';
import 'package:spreadsheet_editor/src/visual_geometry.dart';

void main() {
  test('each resize handle preserves the opposite frame edges', () {
    const original = VisualFrame(104, 56, 200, 100);
    for (final handle in VisualResizeHandle.values) {
      final next = VisualGeometry.resizedFrame(
        original,
        handle,
        deltaX: 30,
        deltaY: 20,
      );
      if (handle.west) {
        expect(next.right, original.right);
        expect(next.left, original.left + 30);
      } else if (handle.east) {
        expect(next.left, original.left);
        expect(next.right, original.right + 30);
      } else {
        expect(next.left, original.left);
        expect(next.right, original.right);
      }
      if (handle.north) {
        expect(next.bottom, original.bottom);
        expect(next.top, original.top + 20);
      } else if (handle.south) {
        expect(next.top, original.top);
        expect(next.bottom, original.bottom + 20);
      } else {
        expect(next.top, original.top);
        expect(next.bottom, original.bottom);
      }
    }
  });

  test('west and north resize clamp to sheet start and minimum frame', () {
    const original = VisualFrame(10, 5, 50, 40);
    final grow = VisualGeometry.resizedFrame(
      original,
      VisualResizeHandle.nw,
      deltaX: -100,
      deltaY: -100,
    );
    expect(grow.left, 0);
    expect(grow.top, 0);
    expect(grow.right, original.right);
    expect(grow.bottom, original.bottom);
    final shrink = VisualGeometry.resizedFrame(
      original,
      VisualResizeHandle.nw,
      deltaX: 100,
      deltaY: 100,
    );
    expect(shrink.width, VisualGeometry.minFramePixels);
    expect(shrink.height, VisualGeometry.minFramePixels);
    expect(shrink.right, original.right);
    expect(shrink.bottom, original.bottom);
  });

  test('northwest resize writes a normalized workbook anchor', () {
    final visual = {
      'kind': 'shape',
      'id': 42,
      'anchor': {
        'row': 7,
        'col': 1,
        'dx': 0,
        'dy': 0,
        'width': 150,
        'height': 75,
      },
    };
    final params = VisualGeometry.resize(
      visual,
      handle: VisualResizeHandle.nw,
      deltaX: -120,
      deltaY: -220,
      rows: SheetAxisGeometry(count: VisualGeometry.maxRows, defaultExtent: 28),
      cols: SheetAxisGeometry(
        count: VisualGeometry.maxCols,
        defaultExtent: 104,
      ),
    );
    expect(params['at'], 'A1');
    expect(params['dx'], 0);
    expect(params['dy'], 0);
    expect(params['width'], closeTo(254, .01));
    expect(params['height'], closeTo(271, .01));
  });

  test(
    'zoom scales object frame and gesture without changing workbook units',
    () {
      final visual = {
        'kind': 'shape',
        'id': 8,
        'anchor': {
          'row': 2,
          'col': 1,
          'dx': 6,
          'dy': 3,
          'width': 150,
          'height': 75,
        },
      };
      final normal = VisualGeometry.frame(
        visual,
        rows: SheetAxisGeometry(
          count: VisualGeometry.maxRows,
          defaultExtent: 28,
        ),
        cols: SheetAxisGeometry(
          count: VisualGeometry.maxCols,
          defaultExtent: 104,
        ),
      );
      final zoomed = VisualGeometry.frame(
        visual,
        rows: SheetAxisGeometry(
          count: VisualGeometry.maxRows,
          defaultExtent: 56,
        ),
        cols: SheetAxisGeometry(
          count: VisualGeometry.maxCols,
          defaultExtent: 208,
        ),
        zoomFactor: 2,
      );
      expect(zoomed.left, normal.left * 2);
      expect(zoomed.top, normal.top * 2);
      expect(zoomed.width, normal.width * 2);
      final moved = VisualGeometry.move(
        visual,
        deltaX: 52,
        deltaY: 28,
        rows: SheetAxisGeometry(
          count: VisualGeometry.maxRows,
          defaultExtent: 56,
        ),
        cols: SheetAxisGeometry(
          count: VisualGeometry.maxCols,
          defaultExtent: 208,
        ),
        zoomFactor: 2,
      );
      expect(moved['at'], 'B3');
      expect(moved['dx'], closeTo(32, .01));
      expect(moved['dy'], closeTo(17, .01));
    },
  );

  test('sparse row and column geometry positions and normalizes objects', () {
    final rows = SheetAxisGeometry(
      count: VisualGeometry.maxRows,
      defaultExtent: 20,
      overrides: {0: 40, 1: 12, 3: 0},
    );
    final cols = SheetAxisGeometry(
      count: VisualGeometry.maxCols,
      defaultExtent: 64,
      overrides: {1: 176, 2: 32},
    );
    expect(rows.leading(4), 72);
    expect(rows.indexAt(72), 4);
    expect(cols.leading(3), 272);
    expect(cols.indexAt(250), 2);
    final visual = {
      'kind': 'shape',
      'id': 9,
      'anchor': {
        'row': 2,
        'col': 2,
        'dx': 4,
        'dy': 3,
        'width': 50,
        'height': 30,
      },
    };
    final frame = VisualGeometry.frame(visual, rows: rows, cols: cols);
    expect(frame.left, 244);
    expect(frame.top, 55);
    final moved = VisualGeometry.move(
      visual,
      deltaX: 40,
      deltaY: 20,
      rows: rows,
      cols: cols,
    );
    expect(moved['at'], 'D5');
    expect(moved['dx'], closeTo(12, .01));
    expect(moved['dy'], closeTo(3, .01));
  });
}
