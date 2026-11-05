import 'dart:async';
import 'dart:io';

import 'package:appflowy/plugins/resource_surface/resource_surface_session.dart';
import 'package:appflowy/plugins/version_diff/application/text_version_diff_service.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:appflowy/plugins/version_diff/presentation/text_diff_plugin.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace/application/tabs/tabs_bloc.dart';
import 'package:flowy_infra_ui/flowy_infra_ui.dart';
import 'package:flowy_infra_ui/style_widget/hover.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:path/path.dart' as p;

/// Per-file session for the resource-tab version pane.
final class MuseResourceVersionPaneController extends ChangeNotifier {
  MuseResourceVersionPaneController(this.file) {
    unawaited(refreshCanSave());
  }

  final File file;

  bool open = false;
  bool loading = false;
  bool canSave = true;
  String? error;
  List<MuseVersion> versions = const [];
  MuseVersion? selected;

  bool get showingCurrent => selected == null;

  MuseTextVersionDiffService get _service =>
      getIt<MuseTextVersionDiffService>();

  Future<void> prepareMenu() async {
    await _stashUncommitted(message: '自动保存');
    await reload();
  }

  Future<void> show() => prepareMenu();

  void hide() {
    open = false;
    selected = null;
    notifyListeners();
  }

  Future<void> reload() async {
    loading = true;
    error = null;
    notifyListeners();
    try {
      versions = await _service.repository.listCommitted(file);
      await refreshCanSave();
    } on Object catch (err) {
      error = err.toString();
    }
    loading = false;
    notifyListeners();
  }

  Future<void> refreshCanSave() async {
    final bufferDirty = MuseResourceSurfaceSession.instance.isBufferDirty(file);
    final diskDirty = await _service.hasUncommittedChanges(file);
    canSave = bufferDirty || diskDirty;
    notifyListeners();
  }

  Future<void> _stashUncommitted({required String message}) async {
    await MuseResourceSurfaceSession.instance.flush(file);
    try {
      await _service.saveIfDirty(file, message: message);
      MuseResourceSurfaceSession.instance.clearDirty(file);
      error = null;
    } on Object catch (err) {
      error = err.toString();
    }
    await refreshCanSave();
  }

  Future<void> selectCurrent() async {
    selected = null;
    notifyListeners();
  }

  Future<void> selectVersion(MuseVersion version) async {
    await _openDiffTab(version: version, compare: false);
  }

  Future<void> compareWithCurrent(MuseVersion version) async {
    await _openDiffTab(version: version, compare: true);
  }

  Future<void> _openDiffTab({
    required MuseVersion version,
    required bool compare,
  }) async {
    loading = true;
    selected = version;
    notifyListeners();
    try {
      await _stashUncommitted(message: '自动保存');
      versions = await _service.repository.listCommitted(file);
      final document = compare
          ? await _compareDocument(version)
          : await _service.viewVersionChanges(file, version);
      error = null;
      getIt<TabsBloc>().openExternalPlugin(
        compare
            ? MuseTextDiffPlugin.compare(
                document: document,
                fileName: p.basename(file.path),
              )
            : MuseTextDiffPlugin.view(
                document: document,
                fileName: p.basename(file.path),
              ),
      );
    } on Object catch (err) {
      error = err.toString();
    }
    loading = false;
    notifyListeners();
  }

  Future<MuseTextComparisonDocument> _compareDocument(
    MuseVersion version,
  ) async {
    final working = await _service.snapshotWorking(file);
    final latest = versions.isEmpty ? null : versions.last;
    final current =
        latest != null && latest.contentDigest == working.contentDigest
            ? latest
            : working;
    return _service.compareVersions(file, version, current);
  }
}

final class MuseResourceVersionPaneRegistry {
  final Map<String, MuseResourceVersionPaneController> _controllers = {};

  MuseResourceVersionPaneController of(File file) {
    final key = file.absolute.path;
    return _controllers.putIfAbsent(
      key,
      () => MuseResourceVersionPaneController(file),
    );
  }
}

/// Right-hand version list. Current version is highlighted; other rows reveal
/// a Compare action on hover.
final class MuseResourceVersionPane extends StatelessWidget {
  const MuseResourceVersionPane({
    super.key,
    required this.controller,
    this.onDone,
  });

  final MuseResourceVersionPaneController controller;
  final VoidCallback? onDone;

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: controller,
      builder: (context, _) {
        final scheme = Theme.of(context).colorScheme;
        return Material(
          color: scheme.surfaceContainerLow,
          child: ConstrainedBox(
            constraints: const BoxConstraints(
              minWidth: 240,
              maxWidth: 320,
              maxHeight: 400,
            ),
            child: ListView(
              shrinkWrap: true,
              padding: const EdgeInsets.symmetric(vertical: 4),
              children: [
                if (controller.loading)
                  const Padding(
                    padding: EdgeInsets.symmetric(vertical: 8),
                    child: Center(
                      child: SizedBox.square(
                        dimension: 16,
                        child: CircularProgressIndicator(strokeWidth: 2),
                      ),
                    ),
                  ),
                if (controller.error case final error?)
                  Padding(
                    padding: const EdgeInsets.all(10),
                    child: FlowyText(
                      error,
                      fontSize: 12,
                      color: scheme.error,
                      maxLines: 3,
                    ),
                  ),
                _VersionRow(
                  title: '当前',
                  subtitle: '工作副本',
                  selected: controller.showingCurrent,
                  showCompare: false,
                  onSelect: () async {
                    await controller.selectCurrent();
                    onDone?.call();
                  },
                ),
                for (final version in controller.versions.reversed)
                  _VersionRow(
                    title: _label(version),
                    subtitle: _subtitle(version),
                    selected: controller.selected?.ref.id == version.ref.id,
                    showCompare: true,
                    onSelect: () async {
                      await controller.selectVersion(version);
                      onDone?.call();
                    },
                    onCompare: () async {
                      await controller.compareWithCurrent(version);
                      onDone?.call();
                    },
                  ),
                if (controller.versions.isEmpty)
                  const Padding(
                    padding: EdgeInsets.all(12),
                    child: FlowyText(
                      '还没有保存的版本，请先「保存当前版本」。',
                      fontSize: 12,
                      maxLines: 3,
                    ),
                  ),
              ],
            ),
          ),
        );
      },
    );
  }

  static String _label(MuseVersion version) {
    if (version.message != null &&
        version.message!.isNotEmpty &&
        version.message != 'Manual snapshot') {
      return version.message!;
    }
    return DateFormat('MMM d, HH:mm').format(version.createdAt.toLocal());
  }

  static String _subtitle(MuseVersion version) {
    final digest = version.contentDigest.length <= 8
        ? version.contentDigest
        : version.contentDigest.substring(0, 8);
    final time = DateFormat('HH:mm:ss').format(version.createdAt.toLocal());
    return '$digest · $time';
  }
}

final class _VersionRow extends StatefulWidget {
  const _VersionRow({
    required this.title,
    required this.subtitle,
    required this.selected,
    required this.showCompare,
    required this.onSelect,
    this.onCompare,
  });

  final String title;
  final String subtitle;
  final bool selected;
  final bool showCompare;
  final VoidCallback onSelect;
  final VoidCallback? onCompare;

  @override
  State<_VersionRow> createState() => _VersionRowState();
}

final class _VersionRowState extends State<_VersionRow> {
  bool _hovering = false;

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    return MouseRegion(
      onEnter: (_) => setState(() => _hovering = true),
      onExit: (_) => setState(() => _hovering = false),
      child: FlowyHover(
        isSelected: () => widget.selected,
        style: HoverStyle(
          hoverColor: scheme.primaryContainer.withValues(alpha: 0.55),
          borderRadius: BorderRadius.circular(6),
        ),
        child: GestureDetector(
          behavior: HitTestBehavior.opaque,
          onTap: widget.onSelect,
          child: SizedBox(
            height: 44,
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 10),
              child: Row(
                children: [
                  Icon(
                    widget.selected
                        ? Icons.radio_button_checked
                        : Icons.history,
                    size: 15,
                    color: widget.selected
                        ? scheme.primary
                        : scheme.onSurfaceVariant,
                  ),
                  const HSpace(8),
                  Expanded(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        FlowyText(
                          widget.title,
                          fontSize: 13,
                          fontWeight: widget.selected
                              ? FontWeight.w600
                              : FontWeight.w400,
                          overflow: TextOverflow.ellipsis,
                        ),
                        FlowyText(
                          widget.subtitle,
                          fontSize: 11,
                          color: scheme.onSurfaceVariant,
                          overflow: TextOverflow.ellipsis,
                        ),
                      ],
                    ),
                  ),
                  if (_hovering &&
                      widget.showCompare &&
                      widget.onCompare != null)
                    FlowyButton(
                      useIntrinsicWidth: true,
                      margin: const EdgeInsets.symmetric(
                        horizontal: 8,
                        vertical: 4,
                      ),
                      text: const FlowyText('比较', fontSize: 12),
                      onTap: widget.onCompare,
                    ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
