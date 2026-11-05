import 'package:appflowy/workspace/presentation/home/menu/sidebar/shared/sidebar_stack_layout.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  const ids = ['project', 'personal'];

  test('fits: both stay expanded and the active space fills leftover height',
      () {
    final decision = resolveSidebarStackLayout(
      paneIds: ids,
      expandedIds: {'project', 'personal'},
      activeId: 'project',
      bodyHeights: {'project': 120, 'personal': 80},
      availableHeight: 400,
    );
    expect(decision.expandedIds, {'project', 'personal'});
    expect(decision.fillingId, 'project');
  });

  test('overflow with two expanded collapses others and fills the active space',
      () {
    final decision = resolveSidebarStackLayout(
      paneIds: ids,
      expandedIds: {'project', 'personal'},
      activeId: 'project',
      bodyHeights: {'project': 400, 'personal': 120},
      availableHeight: 300,
    );
    expect(decision.expandedIds, {'project'});
    expect(decision.fillingId, 'project');
  });

  test('overflow without an active id keeps the taller space', () {
    final decision = resolveSidebarStackLayout(
      paneIds: ids,
      expandedIds: {'project', 'personal'},
      activeId: null,
      bodyHeights: {'project': 80, 'personal': 400},
      availableHeight: 300,
    );
    expect(decision.expandedIds, {'personal'});
    expect(decision.fillingId, 'personal');
  });

  test('a single overflowing space fills remaining height and stays expanded',
      () {
    final decision = resolveSidebarStackLayout(
      paneIds: ids,
      expandedIds: {'project'},
      activeId: 'project',
      bodyHeights: {'project': 800},
      availableHeight: 300,
    );
    expect(decision.expandedIds, {'project'});
    expect(decision.fillingId, 'project');
  });

  test('unmeasured bodies fill the active space without collapsing yet', () {
    final decision = resolveSidebarStackLayout(
      paneIds: ids,
      expandedIds: {'project', 'personal'},
      activeId: 'project',
      bodyHeights: const {},
      availableHeight: 300,
    );
    expect(decision.expandedIds, {'project', 'personal'});
    expect(decision.fillingId, 'project');
  });

  test('a third space can be kept when it is the active overflowing pane', () {
    final decision = resolveSidebarStackLayout(
      paneIds: ['project', 'personal', 'shared'],
      expandedIds: {'project', 'personal', 'shared'},
      activeId: 'shared',
      bodyHeights: {'project': 200, 'personal': 200, 'shared': 200},
      availableHeight: 280,
    );
    expect(decision.expandedIds, {'shared'});
    expect(decision.fillingId, 'shared');
  });
}
