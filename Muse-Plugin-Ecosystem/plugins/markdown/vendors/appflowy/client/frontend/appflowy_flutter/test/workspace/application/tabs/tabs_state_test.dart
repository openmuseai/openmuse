import 'package:appflowy/plugins/blank/blank.dart';
import 'package:appflowy/startup/plugin/plugin.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace/application/tabs/tabs_bloc.dart';
import 'package:appflowy/workspace/presentation/home/home_stack.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  setUpAll(() {
    if (!getIt.isRegistered<PluginSandbox>()) {
      getIt.registerSingleton<PluginSandbox>(PluginSandbox());
    }
  });

  tearDownAll(() async {
    await getIt.reset();
  });

  test('closing a tab before the selection preserves selected identity', () {
    final managers = _managers('a', 'b', 'c');
    final removed = managers[0];
    final state = TabsState(currentIndex: 2, pageManagers: managers);

    final next = state.closeView('a');

    expect(next.currentIndex, 1);
    expect(next.currentPageManager.plugin.id, 'c');
    next.dispose();
    removed.dispose();
  });

  test('closing the selected middle tab selects its right neighbour', () {
    final managers = _managers('a', 'b', 'c');
    final removed = managers[1];
    final state = TabsState(currentIndex: 1, pageManagers: managers);

    final next = state.closeView('b');

    expect(next.currentIndex, 1);
    expect(next.currentPageManager.plugin.id, 'c');
    next.dispose();
    removed.dispose();
  });

  test('closing the selected last tab selects the previous tab', () {
    final managers = _managers('a', 'b', 'c');
    final removed = managers[2];
    final state = TabsState(currentIndex: 2, pageManagers: managers);

    final next = state.closeView('c');

    expect(next.currentIndex, 1);
    expect(next.currentPageManager.plugin.id, 'b');
    next.dispose();
    removed.dispose();
  });

  test('PageManager disposal releases its plugin exactly once', () {
    final plugin = _TestPlugin('resource:file.dart');
    final manager = PageManager()..setPlugin(plugin, false);

    manager.dispose();

    expect(plugin.disposeCount, 1);
  });
}

List<PageManager> _managers(String first, String second, String third) => [
      for (final id in [first, second, third])
        PageManager()..setPlugin(_TestPlugin(id), false),
    ];

final class _TestPlugin extends Plugin {
  _TestPlugin(this.id);

  @override
  final PluginId id;

  int disposeCount = 0;

  @override
  PluginType get pluginType => PluginType.blank;

  @override
  PluginWidgetBuilder get widgetBuilder => BlankPagePluginWidgetBuilder();

  @override
  void dispose() {
    disposeCount += 1;
    super.dispose();
  }
}
