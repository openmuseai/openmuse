import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:flutter/material.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';

import 'update_feed.dart';

/// Displays the launch prompt once, after the app has a Navigator and theme.
final class OpenMuseStartupUpdatePrompt extends StatefulWidget {
  const OpenMuseStartupUpdatePrompt({
    super.key,
    required this.controller,
    required this.child,
    required this.onUpdate,
  });

  final OpenMuseUpdateController? controller;
  final Widget child;
  final Future<void> Function(OpenMuseRelease release) onUpdate;

  @override
  State<OpenMuseStartupUpdatePrompt> createState() =>
      _OpenMuseStartupUpdatePromptState();
}

final class _OpenMuseStartupUpdatePromptState
    extends State<OpenMuseStartupUpdatePrompt> {
  bool _shown = false;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) => _showIfNeeded());
  }

  void _showIfNeeded() {
    if (!mounted || _shown) return;
    if (RuntimeConfigScope.maybeOf(
          context,
        )?.isEnabled(AppFeature.updatePrompts) ==
        false)
      return;
    final release = widget.controller?.available;
    if (release == null) return;
    _shown = true;
    showOpenMuseUpdateDialog(context, release, onUpdate: widget.onUpdate);
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    WidgetsBinding.instance.addPostFrameCallback((_) => _showIfNeeded());
  }

  @override
  Widget build(BuildContext context) => widget.child;
}

Future<void> showOpenMuseUpdateDialog(
  BuildContext context,
  OpenMuseRelease release, {
  required Future<void> Function(OpenMuseRelease release) onUpdate,
}) async {
  if (RuntimeConfigScope.maybeOf(
        context,
      )?.isEnabled(AppFeature.updatePrompts) ==
      false)
    return;
  final localeStrings = stringsOf(context);
  await showDialog<void>(
    context: context,
    barrierDismissible: !release.mandatory,
    builder: (dialogContext) => PopScope(
      canPop: !release.mandatory,
      child: AlertDialog(
        key: const ValueKey('app-update-dialog'),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(18)),
        title: Text(localeStrings.updatesUpdateAvailable5157b4c7()),
        content: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 470),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'OpenMuse ${release.version} (${release.build})',
                style: Theme.of(dialogContext).textTheme.titleMedium,
              ),
              const SizedBox(height: 16),
              Text(
                stringsOf(dialogContext).updatesWhatsNew(),
                style: Theme.of(dialogContext).textTheme.labelLarge,
              ),
              const SizedBox(height: 7),
              Text(release.notesFor(stringsOf(dialogContext).locale)),
            ],
          ),
        ),
        actions: [
          if (!release.mandatory)
            TextButton(
              key: const ValueKey('app-update-later'),
              onPressed: () => Navigator.pop(dialogContext),
              child: Text(localeStrings.updatesLater78fd2cc2()),
            ),
          FilledButton(
            key: const ValueKey('app-update-install'),
            onPressed: () {
              Navigator.pop(dialogContext);
              onUpdate(release);
            },
            child: Text(localeStrings.updatesGetUpdate987882f6()),
          ),
        ],
      ),
    ),
  );
}
