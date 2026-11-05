import 'package:appflowy/core/frameless_window.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';

import 'package:appflowy/workspace/application/tabs/tabs_bloc.dart';
import 'package:appflowy/workspace/presentation/home/home_sizes.dart';
import 'package:appflowy/workspace/presentation/home/tabs/flowy_tab.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

class TabsManager extends StatefulWidget {
  const TabsManager({super.key, required this.onIndexChanged});

  final void Function(int) onIndexChanged;

  @override
  State<TabsManager> createState() => _TabsManagerState();
}

class _TabsManagerState extends State<TabsManager> {
  final _scrollController = ScrollController();
  final _tabKeys = <String, GlobalKey>{};

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return BlocConsumer<TabsBloc, TabsState>(
      listenWhen: (prev, curr) =>
          prev.currentIndex != curr.currentIndex || prev.pages != curr.pages,
      listener: (context, state) {
        widget.onIndexChanged(state.currentIndex);
        _centerCurrent(state);
      },
      builder: (context, state) {
        if (state.pages == 1) {
          return const SizedBox.shrink();
        }

        final isAllPinned = state.isAllPinned;

        return Container(
          alignment: Alignment.bottomLeft,
          height: HomeSizes.tabBarHeight,
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.surfaceContainerHighest,
          ),
          child: MoveWindowDetector(
            child: Listener(
              onPointerSignal: (event) {
                if (event is! PointerScrollEvent ||
                    !_scrollController.hasClients) {
                  return;
                }
                final delta = event.scrollDelta.dx != 0
                    ? event.scrollDelta.dx
                    : event.scrollDelta.dy;
                final next = (_scrollController.offset + delta).clamp(
                  0.0,
                  _scrollController.position.maxScrollExtent,
                );
                _scrollController.jumpTo(next);
              },
              child: SingleChildScrollView(
                controller: _scrollController,
                scrollDirection: Axis.horizontal,
                child: Row(
                  children: [
                    for (final pm in state.pageManagers)
                      FlowyTab(
                        key: _tabKeys.putIfAbsent(
                          pm.plugin.id,
                          GlobalKey.new,
                        ),
                        pageManager: pm,
                        isCurrent: state.currentPageManager == pm,
                        isAllPinned: isAllPinned,
                        onTap: () {
                          if (state.currentPageManager != pm) {
                            final index = state.pageManagers.indexOf(pm);
                            widget.onIndexChanged(index);
                          }
                        },
                      ),
                  ],
                ),
              ),
            ),
          ),
        );
      },
    );
  }

  void _centerCurrent(TabsState state) {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      final ctx = _tabKeys[state.currentPageManager.plugin.id]?.currentContext;
      if (ctx == null) return;
      Scrollable.ensureVisible(
        ctx,
        alignment: 0.5,
        duration: const Duration(milliseconds: 180),
        curve: Curves.easeOut,
      );
    });
  }
}
