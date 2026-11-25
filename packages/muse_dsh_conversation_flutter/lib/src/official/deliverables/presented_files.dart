import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:flutter/material.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';

/// Opens one file declared by `deliverables/presented`.
///
/// [action] is the official `PresentedAction`: `open` or `reveal`.
typedef DshPresentedFileOpener =
    Future<void> Function(int seq, int index, String action);

const _collapsedPresentedCount = 4;

/// Official presented-file cards from
/// `@deepseek-ai/dsh-client-ui-deliverables@0.1.7-rc.1`.
final class DshPresentedFiles extends StatefulWidget {
  const DshPresentedFiles({super.key, required this.files, this.onOpen});

  final List<JsonMap> files;
  final DshPresentedFileOpener? onOpen;

  @override
  State<DshPresentedFiles> createState() => _DshPresentedFilesState();
}

final class _DshPresentedFilesState extends State<DshPresentedFiles> {
  final Map<String, String> _phase = {};
  bool _expanded = false;

  Future<void> _act(JsonMap file, String action) async {
    final open = widget.onOpen;
    final seq = file['seq'];
    final index = file['index'];
    if (open == null || seq is! int || index is! int) return;
    final key = '$seq:$index';
    setState(() => _phase[key] = action == 'reveal' ? 'revealing' : 'opening');
    try {
      await open(seq, index, action);
      if (!mounted) return;
      setState(() => _phase[key] = action == 'reveal' ? 'revealed' : 'opened');
    } on Object {
      if (!mounted) return;
      setState(
        () => _phase[key] = action == 'reveal' ? 'revealError' : 'error',
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    final files = widget.files;
    if (files.isEmpty) return const SizedBox.shrink();
    final collapsible = files.length > _collapsedPresentedCount;
    final shown = collapsible && !_expanded
        ? files.take(_collapsedPresentedCount).toList(growable: false)
        : files;
    return Padding(
      padding: const EdgeInsets.only(top: 12),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          LayoutBuilder(
            builder: (context, constraints) {
              final columns = shown.length == 1 || constraints.maxWidth < 620
                  ? 1
                  : 2;
              return Wrap(
                spacing: 10,
                runSpacing: 10,
                children: [
                  for (final file in shown)
                    SizedBox(
                      width: columns == 1
                          ? constraints.maxWidth
                          : (constraints.maxWidth - 10) / 2,
                      child: _PresentedFileCard(
                        file: file,
                        phase: _phase['${file['seq']}:${file['index']}'],
                        enabled: widget.onOpen != null,
                        onOpen: () => _act(file, 'open'),
                        onReveal: () => _act(file, 'reveal'),
                      ),
                    ),
                ],
              );
            },
          ),
          if (collapsible)
            Align(
              child: TextButton.icon(
                key: const ValueKey('dsh-native.presented.toggle'),
                onPressed: () => setState(() => _expanded = !_expanded),
                icon: Icon(
                  _expanded
                      ? Icons.keyboard_arrow_up_rounded
                      : Icons.keyboard_arrow_down_rounded,
                  size: 16,
                ),
                label: Text(
                  _expanded
                      ? stringsOf(context).dshCollapse()
                      : stringsOf(context).dshAllFiles(count: files.length),
                ),
              ),
            ),
        ],
      ),
    );
  }
}

final class _PresentedFileCard extends StatelessWidget {
  const _PresentedFileCard({
    required this.file,
    required this.phase,
    required this.enabled,
    required this.onOpen,
    required this.onReveal,
  });

  final JsonMap file;
  final String? phase;
  final bool enabled;
  final VoidCallback onOpen;
  final VoidCallback onReveal;

  @override
  Widget build(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    final name = file['name'] as String? ?? '';
    final path = file['path'] as String? ?? name;
    final pending = phase == 'opening' || phase == 'revealing';
    final status = _status(
      context,
      phase,
      file['description'] as String?,
      name,
    );
    final failed = phase == 'error' || phase == 'revealError';
    final mark = _fileMark(name);
    final dark = Theme.of(context).brightness == Brightness.dark;
    final fill = dark
        ? colors.surfaceContainerHighest
        : const Color(0xFFF7F7F8);
    return Material(
      color: fill,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(18),
        side: BorderSide(color: colors.outlineVariant),
      ),
      child: InkWell(
        key: ValueKey('dsh-native.presented.card:$path'),
        borderRadius: BorderRadius.circular(18),
        onTap: !enabled || pending ? null : onOpen,
        child: SizedBox(
          height: 60,
          child: Padding(
            padding: const EdgeInsets.fromLTRB(10, 8, 8, 8),
            child: Row(
              children: [
                DecoratedBox(
                  decoration: BoxDecoration(
                    color: fill,
                    borderRadius: BorderRadius.circular(10),
                    border: Border.all(color: colors.outlineVariant),
                  ),
                  child: SizedBox.square(
                    dimension: 40,
                    child: Icon(mark.icon, size: 20, color: mark.color),
                  ),
                ),
                const SizedBox(width: 10),
                Expanded(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        name,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                        style: const TextStyle(
                          fontSize: 13,
                          fontWeight: FontWeight.w500,
                          height: 20 / 13,
                        ),
                      ),
                      Text(
                        status,
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                        style: TextStyle(
                          fontSize: 10,
                          height: 16 / 10,
                          color: failed
                              ? colors.error
                              : colors.onSurfaceVariant,
                        ),
                      ),
                    ],
                  ),
                ),
                const SizedBox(width: 8),
                DecoratedBox(
                  decoration: BoxDecoration(
                    borderRadius: BorderRadius.circular(10),
                    border: Border.all(color: colors.outlineVariant),
                  ),
                  child: SizedBox(
                    height: 28,
                    child: Row(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        IconButton(
                          key: ValueKey('dsh-native.presented.open:$path'),
                          tooltip: stringsOf(context).dshDefaultOpen(),
                          visualDensity: VisualDensity.compact,
                          padding: const EdgeInsets.symmetric(horizontal: 8),
                          constraints: const BoxConstraints(
                            minWidth: 28,
                            minHeight: 28,
                          ),
                          onPressed: !enabled || pending ? null : onOpen,
                          icon: Icon(
                            Icons.open_in_new_rounded,
                            size: 15,
                            color: colors.primary,
                          ),
                        ),
                        DecoratedBox(
                          decoration: BoxDecoration(
                            border: Border(
                              left: BorderSide(color: colors.outlineVariant),
                            ),
                          ),
                          child: IconButton(
                            key: ValueKey('dsh-native.presented.more:$path'),
                            tooltip: stringsOf(
                              context,
                            ).dshMoreFileActions(name: name),
                            visualDensity: VisualDensity.compact,
                            padding: const EdgeInsets.symmetric(horizontal: 4),
                            constraints: const BoxConstraints(
                              minWidth: 22,
                              minHeight: 28,
                            ),
                            onPressed: !enabled || pending
                                ? null
                                : () async {
                                    final action = await showMenu<String>(
                                      context: context,
                                      position: _menuPosition(context),
                                      items: [
                                        PopupMenuItem(
                                          value: 'open',
                                          child: Text(
                                            stringsOf(context).dshDefaultOpen(),
                                          ),
                                        ),
                                        PopupMenuItem(
                                          value: 'reveal',
                                          child: Text(
                                            stringsOf(context).dshReveal(),
                                          ),
                                        ),
                                      ],
                                    );
                                    if (action == 'reveal') onReveal();
                                    if (action == 'open') onOpen();
                                  },
                            icon: Icon(
                              Icons.keyboard_arrow_down_rounded,
                              size: 16,
                              color: colors.onSurfaceVariant,
                            ),
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

RelativeRect _menuPosition(BuildContext context) {
  final box = context.findRenderObject() as RenderBox?;
  final overlay = Overlay.of(context).context.findRenderObject() as RenderBox?;
  if (box == null || !box.hasSize || overlay == null || !overlay.hasSize) {
    return const RelativeRect.fromLTRB(0, 0, 0, 0);
  }
  final offset = box.localToGlobal(Offset.zero, ancestor: overlay);
  return RelativeRect.fromRect(offset & box.size, Offset.zero & overlay.size);
}

String _status(
  BuildContext context,
  String? phase,
  String? description,
  String name,
) {
  return switch (phase) {
    'opening' => stringsOf(context).dshOpening(),
    'opened' => stringsOf(context).dshOpenedDefault(),
    'error' => stringsOf(context).dshOpenRetry(),
    'revealing' => stringsOf(context).dshRevealing(),
    'revealed' => stringsOf(context).dshRevealed(),
    'revealError' => stringsOf(context).dshRevealFailed(),
    _ => _cardDescription(context, description, name),
  };
}

/// Official cards drop a trailing parenthetical before showing the description.
String _cardDescription(
  BuildContext context,
  String? description,
  String name,
) {
  final trimmed = description
      ?.replaceFirst(RegExp(r'\s*(?:\([^()]*\)|（[^（）]*）)\s*$'), '')
      .trim();
  if (trimmed == null || trimmed.isEmpty) {
    final dot = name.lastIndexOf('.');
    if (dot <= 0 || dot == name.length - 1) return stringsOf(context).dshFile();
    return name.substring(dot + 1).toUpperCase();
  }
  return trimmed;
}

({IconData icon, Color color}) _fileMark(String name) {
  final dot = name.lastIndexOf('.');
  final ext = dot <= 0 ? '' : name.substring(dot + 1).toLowerCase();
  return switch (ext) {
    'sh' ||
    'bash' ||
    'zsh' ||
    'ps1' ||
    'bat' ||
    'cmd' => (icon: Icons.terminal_rounded, color: const Color(0xFF1F8A4C)),
    'md' ||
    'markdown' ||
    'txt' ||
    'rst' => (icon: Icons.description_outlined, color: const Color(0xFF3B6FD8)),
    'png' ||
    'jpg' ||
    'jpeg' ||
    'gif' ||
    'webp' ||
    'svg' => (icon: Icons.image_outlined, color: const Color(0xFF3B6FD8)),
    _ => (
      icon: Icons.insert_drive_file_outlined,
      color: const Color(0xFF3B6FD8),
    ),
  };
}
