import 'package:flutter/material.dart';
import 'package:flutter/foundation.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import '../workbuddy/workbuddy_controller.dart';
import '../workbuddy/workbuddy_models.dart';
import 'mobile_workbench_page.dart';

/// Owns the app-center chrome. Children retain their independent scroll/search
/// state, while plugin behavior and installation remain in their existing hosts.
final class MobileAppsPage extends StatefulWidget {
  const MobileAppsPage({
    super.key,
    required this.controller,
    required this.overview,
    required this.discovery,
    required this.workbenchKey,
    required this.editing,
    this.managementReturn,
  });
  final WorkBuddyController controller;
  final Widget overview, discovery;
  final GlobalKey<MobileWorkbenchPageState> workbenchKey;
  final ValueListenable<bool> editing;
  final ValueListenable<VoidCallback?>? managementReturn;
  @override
  State<MobileAppsPage> createState() => _MobileAppsPageState();
}

final class _MobileAppsPageState extends State<MobileAppsPage> {
  bool _saving = false;
  int _reselection = 0;
  bool _discoveryVisited = false;
  @override
  void initState() {
    super.initState();
    _reselection = widget.controller.appsReselection;
  }

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: Listenable.merge([
      widget.controller,
      widget.editing,
      ?widget.managementReturn,
    ]),
    builder: (context, _) {
      final c = widget.controller, editing = widget.editing.value;
      final s = stringsOf(context), colors = Theme.of(context).colorScheme;
      if (c.appsSection == AppsSection.discover) _discoveryVisited = true;
      if (_reselection != c.appsReselection) {
        _reselection = c.appsReselection;
        WidgetsBinding.instance.addPostFrameCallback(
          (_) => widget.workbenchKey.currentState?.scrollToTop(),
        );
      }
      return PopScope(
        canPop: !editing,
        onPopInvokedWithResult: (popped, _) async {
          if (!popped) await widget.workbenchKey.currentState?.requestLeave();
        },
        child: Column(
          children: [
            if (editing)
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 16),
                child: Row(
                  children: [
                    TextButton(
                      key: const ValueKey('apps-edit-cancel'),
                      onPressed: _saving
                          ? null
                          : () => widget.workbenchKey.currentState
                                ?.requestLeave(),
                      child: Text(s.commonCancel()),
                    ),
                    Expanded(
                      child: Text(
                        s.mobileEditOverview(),
                        textAlign: TextAlign.center,
                        style: const TextStyle(
                          fontSize: 18,
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                    ),
                    TextButton(
                      key: const ValueKey('apps-edit-done'),
                      onPressed: _saving
                          ? null
                          : () async {
                              setState(() => _saving = true);
                              await widget.workbenchKey.currentState
                                  ?.finishEdit(save: true);
                              if (mounted) setState(() => _saving = false);
                            },
                      child: Text(s.mobileDone92e3eec4()),
                    ),
                  ],
                ),
              )
            else
              Padding(
                padding: const EdgeInsets.fromLTRB(20, 12, 20, 7),
                child: Row(
                  children: [
                    if (widget.managementReturn?.value case final callback?)
                      IconButton(
                        key: const ValueKey('apps-return-to-plugins'),
                        tooltip: s.marketMyPlugins(),
                        onPressed: callback,
                        icon: const Icon(Icons.arrow_back),
                      ),
                    Expanded(
                      child: LayoutBuilder(
                        builder: (context, constraints) {
                          final labels = [
                            s.mobileOverview(),
                            s.marketCategoryDiscover(),
                          ];
                          final textScale =
                              MediaQuery.textScalerOf(context).scale(27) / 27;
                          var width = 0.0;
                          for (final label in labels) {
                            final painter = TextPainter(
                              text: TextSpan(
                                text: label,
                                style: const TextStyle(
                                  fontSize: 27,
                                  fontWeight: FontWeight.w600,
                                ),
                              ),
                              textDirection: Directionality.of(context),
                            )..layout();
                            width += painter.width + 22;
                            painter.dispose();
                          }
                          final scale = textScale > 1
                              ? 1.0
                              : (constraints.maxWidth / width).clamp(.89, 1.0);
                          return SingleChildScrollView(
                            scrollDirection: Axis.horizontal,
                            child: Row(
                              children: [
                                for (final section in AppsSection.values)
                                  Padding(
                                    padding: const EdgeInsets.only(right: 22),
                                    child: Semantics(
                                      selected: c.appsSection == section,
                                      button: true,
                                      child: InkWell(
                                        key: ValueKey(
                                          'apps-section-${section.name}',
                                        ),
                                        onTap: () =>
                                            c.selectAppsSection(section),
                                        borderRadius: BorderRadius.circular(8),
                                        child: Padding(
                                          padding: const EdgeInsets.symmetric(
                                            vertical: 9,
                                          ),
                                          child: Column(
                                            crossAxisAlignment:
                                                CrossAxisAlignment.start,
                                            children: [
                                              Text(
                                                labels[section.index],
                                                maxLines: 1,
                                                softWrap: false,
                                                style: TextStyle(
                                                  fontSize:
                                                      (c.appsSection == section
                                                          ? 27
                                                          : 24) *
                                                      scale,
                                                  fontWeight:
                                                      c.appsSection == section
                                                      ? FontWeight.w600
                                                      : FontWeight.w500,
                                                  letterSpacing: -.6,
                                                  color:
                                                      c.appsSection == section
                                                      ? colors.onSurface
                                                      : colors.onSurfaceVariant,
                                                ),
                                              ),
                                              const SizedBox(height: 9),
                                              Container(
                                                width: 22,
                                                height: 3,
                                                decoration: BoxDecoration(
                                                  color:
                                                      c.appsSection == section
                                                      ? colors.primary
                                                      : Colors.transparent,
                                                  borderRadius:
                                                      BorderRadius.circular(4),
                                                ),
                                              ),
                                            ],
                                          ),
                                        ),
                                      ),
                                    ),
                                  ),
                              ],
                            ),
                          );
                        },
                      ),
                    ),
                    if (c.appsSection == AppsSection.overview &&
                        widget.overview is MobileWorkbenchPage)
                      TextButton(
                        key: const ValueKey('apps-edit'),
                        onPressed: () =>
                            widget.workbenchKey.currentState?.beginEdit(),
                        child: Text(s.mobileEditApps()),
                      ),
                  ],
                ),
              ),
            Expanded(
              child: IndexedStack(
                index: c.appsSection.index,
                children: [
                  TickerMode(
                    enabled: c.appsSection == AppsSection.overview,
                    child: widget.overview,
                  ),
                  TickerMode(
                    enabled: c.appsSection == AppsSection.discover,
                    child: _discoveryVisited
                        ? widget.discovery
                        : const SizedBox.shrink(),
                  ),
                ],
              ),
            ),
          ],
        ),
      );
    },
  );
}

/// Keeps an Account inventory mounted while browsing the single Apps surface
/// below a non-opaque route. System Back and the Apps back affordance restore
/// that inventory; explicit bottom navigation exits the management flow.
final class AppsReturnController extends ValueNotifier<VoidCallback?> {
  AppsReturnController() : super(null);
  bool _closed = false;
  void clear() {
    if (!_closed) value = null;
  }

  @override
  void dispose() {
    _closed = true;
    super.dispose();
  }
}

/// One route switches between the opaque Account inventory and an unobstructed
/// view of the existing Apps destination. No second catalog/search is created.
final class InstalledPluginsRoute extends PageRoute<void> {
  InstalledPluginsRoute({
    required this.buddy,
    required this.returnToInventory,
    required this.inventory,
  }) {
    browsing.addListener(_modeChanged);
  }
  final WorkBuddyController buddy;
  final AppsReturnController returnToInventory;
  final Widget Function(VoidCallback) inventory;
  final browsing = ValueNotifier(false);
  void _modeChanged() {
    if (overlayEntries.isNotEmpty) overlayEntries.first.opaque = opaque;
    changedInternalState();
  }

  @override
  bool get opaque => !browsing.value;
  @override
  bool get barrierDismissible => false;
  @override
  Color? get barrierColor => null;
  @override
  String? get barrierLabel => null;
  @override
  bool get maintainState => true;
  @override
  Duration get transitionDuration => const Duration(milliseconds: 180);
  @override
  Widget buildModalBarrier() =>
      browsing.value ? const SizedBox.shrink() : super.buildModalBarrier();
  @override
  Widget buildPage(
    BuildContext context,
    Animation<double> animation,
    Animation<double> secondaryAnimation,
  ) => InstalledPluginsFlow(
    controller: buddy,
    returnToInventory: returnToInventory,
    inventory: inventory,
    browsing: browsing,
  );
  @override
  Widget buildTransitions(
    BuildContext context,
    Animation<double> animation,
    Animation<double> secondaryAnimation,
    Widget child,
  ) => FadeTransition(opacity: animation, child: child);
  @override
  void dispose() {
    browsing.removeListener(_modeChanged);
    browsing.dispose();
    super.dispose();
  }
}

final class InstalledPluginsFlow extends StatefulWidget {
  const InstalledPluginsFlow({
    super.key,
    required this.controller,
    required this.returnToInventory,
    required this.inventory,
    required this.browsing,
  });
  final ValueNotifier<bool> browsing;
  final WorkBuddyController controller;
  final AppsReturnController returnToInventory;
  final Widget Function(VoidCallback) inventory;
  @override
  State<InstalledPluginsFlow> createState() => _InstalledPluginsFlowState();
}

final class _InstalledPluginsFlowState extends State<InstalledPluginsFlow> {
  bool exploring = false, returning = false;
  double _backDrag = 0;
  @override
  void initState() {
    super.initState();
    widget.controller.addListener(_navigationChanged);
  }

  void _navigationChanged() {
    if (exploring &&
        !returning &&
        widget.controller.tab != WbTab.apps &&
        mounted) {
      exploring = false;
      widget.returnToInventory.value = null;
      Navigator.of(context).pop();
    }
  }

  Future<void> _resume() async {
    returning = true;
    await widget.controller.selectTab(WbTab.mine);
    returning = false;
    if (mounted && widget.controller.tab == WbTab.mine) {
      setState(() => exploring = false);
      widget.browsing.value = false;
      widget.returnToInventory.value = null;
    }
  }

  Future<void> _discover() async {
    await widget.controller.selectAppsSection(AppsSection.discover);
    if (!mounted) return;
    setState(() => exploring = true);
    widget.browsing.value = true;
    widget.returnToInventory.value = () => _resume();
  }

  @override
  void dispose() {
    widget.controller.removeListener(_navigationChanged);
    widget.returnToInventory.clear();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => PopScope(
    canPop: !exploring,
    onPopInvokedWithResult: (popped, _) {
      if (!popped) _resume();
    },
    child: Stack(
      fit: StackFit.expand,
      children: [
        Offstage(
          offstage: exploring,
          child: widget.inventory(() => _discover()),
        ),
        // The transparent management route remains in the stack so Back restores
        // its filter. Supply its iOS leading-edge return gesture without placing
        // a full-screen gesture/barrier above the actual Apps page.
        if (defaultTargetPlatform == TargetPlatform.iOS)
          PositionedDirectional(
            start: 0,
            top: 0,
            bottom: 0,
            width: 20,
            child: ExcludeSemantics(
              child: GestureDetector(
                key: const ValueKey('apps-return-swipe-edge'),
                behavior: HitTestBehavior.translucent,
                onHorizontalDragStart: (_) => _backDrag = 0,
                onHorizontalDragUpdate: (details) =>
                    _backDrag += details.delta.dx,
                onHorizontalDragEnd: (details) {
                  if (_backDrag > 54 || (details.primaryVelocity ?? 0) > 400) {
                    if (exploring) {
                      _resume();
                    } else {
                      Navigator.of(context).maybePop();
                    }
                  }
                },
              ),
            ),
          ),
      ],
    ),
  );
}
