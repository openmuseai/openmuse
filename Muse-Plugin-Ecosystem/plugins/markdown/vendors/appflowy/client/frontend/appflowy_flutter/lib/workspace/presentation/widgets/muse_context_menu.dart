import 'dart:async';

import 'package:flowy_infra_ui/flowy_infra_ui.dart';
import 'package:flutter/material.dart';

/// Shared Host context-menu chrome matching [ViewMoreActionPopover]:
/// 10px radius, light/dark popover shadow, 34px icon-text rows, 14px type.
const museContextMenuConstraints = BoxConstraints(
  minWidth: 260,
  maxWidth: 280,
  maxHeight: 480,
);

const EdgeInsets museContextMenuPadding = EdgeInsets.all(6);

sealed class MuseContextMenuEntry {
  const MuseContextMenuEntry();
}

final class MuseContextMenuDivider extends MuseContextMenuEntry {
  const MuseContextMenuDivider();
}

final class MuseContextMenuAction extends MuseContextMenuEntry {
  const MuseContextMenuAction({
    required this.id,
    required this.label,
    this.icon,
    this.trailing,
    this.destructive = false,
    this.enabled = true,
    this.submenuBuilder,
  });

  final String id;
  final String label;
  final IconData? icon;
  final Widget? trailing;
  final bool destructive;
  final bool enabled;
  final Widget Function(BuildContext context, VoidCallback close)?
      submenuBuilder;
}

class MuseContextMenuBody extends StatelessWidget {
  const MuseContextMenuBody({
    super.key,
    required this.entries,
    required this.onSelected,
    this.onDismiss,
  });

  final List<MuseContextMenuEntry> entries;
  final void Function(String id) onSelected;
  final VoidCallback? onDismiss;

  @override
  Widget build(BuildContext context) {
    final column = Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        for (final entry in entries)
          switch (entry) {
            MuseContextMenuDivider() => const Padding(
                padding: EdgeInsets.all(8),
                child: FlowyDivider(),
              ),
            MuseContextMenuAction() => MuseContextMenuRow(
                action: entry,
                onSelected: onSelected,
                onDismiss: onDismiss,
              ),
          },
      ],
    );
    return LayoutBuilder(
      builder: (context, constraints) {
        if (!constraints.hasBoundedHeight) return column;
        return ConstrainedBox(
          constraints: BoxConstraints(maxHeight: constraints.maxHeight),
          child: SingleChildScrollView(child: column),
        );
      },
    );
  }
}

class MuseContextMenuRow extends StatelessWidget {
  const MuseContextMenuRow({
    super.key,
    required this.action,
    required this.onSelected,
    this.onDismiss,
  });

  final MuseContextMenuAction action;
  final void Function(String id) onSelected;
  final VoidCallback? onDismiss;

  @override
  Widget build(BuildContext context) {
    final row = _row(context);
    final submenu = action.submenuBuilder;
    if (submenu == null) return row;
    return AppFlowyPopover(
      triggerActions: PopoverTriggerFlags.hover | PopoverTriggerFlags.click,
      offset: const Offset(6, 0),
      constraints: const BoxConstraints(
        minWidth: 240,
        maxWidth: 320,
        maxHeight: 420,
      ),
      popupBuilder: (ctx) => submenu(ctx, () => onDismiss?.call()),
      child: row,
    );
  }

  Widget _row(BuildContext context) {
    return Container(
      height: 34,
      padding: const EdgeInsets.symmetric(vertical: 2),
      child: FlowyIconTextButton(
        disable: !action.enabled,
        margin: const EdgeInsets.symmetric(horizontal: 6),
        iconPadding: 10,
        mainAxisAlignment: MainAxisAlignment.start,
        onTap: action.enabled && action.submenuBuilder == null
            ? () => onSelected(action.id)
            : null,
        leftIconBuilder: action.icon == null
            ? null
            : (onHover) => Icon(
                  action.icon,
                  size: 16,
                  color: _foreground(context, onHover),
                ),
        rightIconBuilder: action.trailing == null &&
                action.submenuBuilder == null
            ? null
            : (_) =>
                action.trailing ?? const Icon(Icons.chevron_right, size: 16),
        textBuilder: (onHover) => FlowyText.regular(
          action.label,
          fontSize: 14,
          lineHeight: 1,
          figmaLineHeight: 18,
          color: _foreground(context, onHover),
        ),
      ),
    );
  }

  Color? _foreground(BuildContext context, bool onHover) {
    if (action.destructive && onHover) {
      return Theme.of(context).colorScheme.error;
    }
    return null;
  }
}

Future<String?> showMuseContextMenu({
  required BuildContext context,
  required Offset globalPosition,
  required List<MuseContextMenuEntry> entries,
}) {
  final overlayState = Overlay.of(context);
  final overlayBox = overlayState.context.findRenderObject()! as RenderBox;
  final completer = Completer<String?>();
  late OverlayEntry overlay;

  void finish(String? id) {
    if (completer.isCompleted) return;
    overlay.remove();
    completer.complete(id);
  }

  overlay = OverlayEntry(
    builder: (ctx) {
      final local = overlayBox.globalToLocal(globalPosition);
      return Stack(
        children: [
          Positioned.fill(
            child: GestureDetector(
              behavior: HitTestBehavior.opaque,
              onTap: () => finish(null),
              onSecondaryTap: () => finish(null),
            ),
          ),
          CustomSingleChildLayout(
            delegate: _MuseContextMenuLayoutDelegate(position: local),
            child: Material(
              type: MaterialType.transparency,
              child: Container(
                constraints: museContextMenuConstraints,
                padding: museContextMenuPadding,
                decoration: ctx.getPopoverDecoration(),
                child: MuseContextMenuBody(
                  entries: entries,
                  onSelected: finish,
                  onDismiss: () => finish(null),
                ),
              ),
            ),
          ),
        ],
      );
    },
  );
  overlayState.insert(overlay);
  return completer.future;
}

class _MuseContextMenuLayoutDelegate extends SingleChildLayoutDelegate {
  _MuseContextMenuLayoutDelegate({required this.position});

  final Offset position;
  static const _padding = 8.0;

  @override
  BoxConstraints getConstraintsForChild(BoxConstraints constraints) {
    return BoxConstraints(
      maxWidth:
          (constraints.maxWidth - _padding * 2).clamp(0.0, double.infinity),
      maxHeight:
          (constraints.maxHeight - _padding * 2).clamp(0.0, double.infinity),
    );
  }

  @override
  Offset getPositionForChild(Size size, Size childSize) {
    var x = position.dx;
    var y = position.dy;
    if (x + childSize.width > size.width - _padding) {
      x = size.width - _padding - childSize.width;
    }
    if (x < _padding) x = _padding;
    if (y + childSize.height > size.height - _padding) {
      y = position.dy - childSize.height;
    }
    if (y < _padding) y = _padding;
    return Offset(x, y);
  }

  @override
  bool shouldRelayout(covariant _MuseContextMenuLayoutDelegate oldDelegate) =>
      oldDelegate.position != position;
}
