import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:path/path.dart' as p;

/// Generic presentation envelope emitted by an installed CLI plugin.
/// No social-platform knowledge lives in the DSH integration.
final class PluginInteraction {
  const PluginInteraction({
    required this.pluginId,
    required this.title,
    required this.imagePath,
    required this.statusPath,
  });

  final String pluginId;
  final String title;
  final String imagePath;
  final String statusPath;

  static Future<PluginInteraction> read(
    File event,
    String inbox, {
    String? installRoot,
  }) async {
    final value = jsonDecode(await event.readAsString());
    if (value is! Map ||
        value['protocol'] != 'openmuse.plugin-interaction/v1' ||
        value['type'] != 'image.challenge' ||
        value['pluginId'] is! String ||
        value['title'] is! String ||
        value['imagePath'] is! String ||
        value['statusPath'] is! String ||
        value['issuedAt'] is! int) {
      throw const FormatException('Invalid plugin interaction');
    }
    final issuedAt = DateTime.fromMillisecondsSinceEpoch(
      (value['issuedAt'] as int) * 1000,
    );
    if (DateTime.now().difference(issuedAt).abs() >
        const Duration(minutes: 10)) {
      throw const FormatException('Expired plugin interaction');
    }
    final pluginId = value['pluginId'] as String;
    if (!RegExp(
      r'^com\.openmuse\.[A-Za-z0-9][A-Za-z0-9._-]{0,100}$',
    ).hasMatch(pluginId)) {
      throw const FormatException('Invalid plugin identity');
    }
    final plugins =
        installRoot ?? p.join(Directory(inbox).parent.path, 'plugins');
    final receipt = File(p.join(plugins, pluginId, 'receipt.json'));
    final installed = jsonDecode(await receipt.readAsString());
    if (installed is! Map ||
        installed['pluginId'] != pluginId ||
        installed['workspacePath'] is! String) {
      throw const FormatException('Plugin is not installed');
    }
    final workspace = await Directory(
      installed['workspacePath'] as String,
    ).resolveSymbolicLinks();
    Future<String> checkedPath(String path) async {
      if (!p.isAbsolute(path)) throw const FormatException('Relative path');
      final canonical = await File(path).resolveSymbolicLinks();
      if (!p.isWithin(workspace, canonical)) {
        throw const FormatException(
          'Interaction file outside plugin workspace',
        );
      }
      return canonical;
    }

    final title = value['title'] as String;
    if (title.isEmpty || title.length > 120) {
      throw const FormatException('Invalid interaction title');
    }
    final image = await checkedPath(value['imagePath'] as String);
    final status = await checkedPath(value['statusPath'] as String);
    if (await File(image).length() > 2 * 1024 * 1024) {
      throw const FormatException('Interaction image too large');
    }
    return PluginInteraction(
      pluginId: pluginId,
      title: title,
      imagePath: image,
      statusPath: status,
    );
  }
}

final class PluginInteractionListener {
  PluginInteractionListener(this.directory, this.onInteraction);

  final String directory;
  final void Function(PluginInteraction) onInteraction;
  Timer? _timer;
  bool _scanning = false;

  void start() {
    _timer ??= Timer.periodic(const Duration(milliseconds: 500), (_) => scan());
    unawaited(scan());
  }

  void stop() {
    _timer?.cancel();
    _timer = null;
  }

  Future<void> scan() async {
    if (_scanning) return;
    _scanning = true;
    try {
      final dataRoot = Directory(directory).parent;
      final pluginsRoot = Directory(p.join(dataRoot.path, 'plugins'));
      final inboxes = <Directory>[Directory(directory)];
      if (await pluginsRoot.exists()) {
        await for (final plugin in pluginsRoot.list(followLinks: false)) {
          if (plugin is! Directory) continue;
          try {
            final receipt = jsonDecode(
              await File(p.join(plugin.path, 'receipt.json')).readAsString(),
            );
            if (receipt is Map &&
                receipt['pluginId'] == p.basename(plugin.path) &&
                receipt['workspacePath'] is String) {
              inboxes.add(
                Directory(
                  p.join(
                    receipt['workspacePath'] as String,
                    'state',
                    'interactions',
                  ),
                ),
              );
            }
          } catch (_) {}
        }
      }
      for (final inbox in inboxes) {
        if (!await inbox.exists()) continue;
        await _scanInbox(inbox, pluginsRoot.path);
      }
    } finally {
      _scanning = false;
    }
  }

  Future<void> _scanInbox(Directory inbox, String installRoot) async {
    await for (final entity in inbox.list(followLinks: false)) {
      if (entity is! File || !entity.path.endsWith('.json')) continue;
      try {
        final interaction = await PluginInteraction.read(
          entity,
          inbox.path,
          installRoot: installRoot,
        );
        await entity.rename('${entity.path}.seen');
        onInteraction(interaction);
      } catch (_) {
        // A malformed or stale event must never repeatedly block the inbox.
        try {
          await entity.rename('${entity.path}.rejected');
        } catch (_) {}
      }
    }
  }
}

Future<void> showPluginInteraction(
  BuildContext context,
  PluginInteraction interaction,
) => showDialog<void>(
  context: context,
  barrierDismissible: true,
  builder: (_) => _ImageChallengeDialog(interaction: interaction),
);

final class _ImageChallengeDialog extends StatefulWidget {
  const _ImageChallengeDialog({required this.interaction});
  final PluginInteraction interaction;

  @override
  State<_ImageChallengeDialog> createState() => _ImageChallengeDialogState();
}

final class _ImageChallengeDialogState extends State<_ImageChallengeDialog> {
  Timer? _timer;
  String _status = '等待扫码';
  Uint8List? _imageBytes;
  DateTime? _imageModified;

  @override
  void initState() {
    super.initState();
    _timer = Timer.periodic(const Duration(seconds: 1), (_) => _refresh());
    unawaited(_refresh());
  }

  @override
  void dispose() {
    _timer?.cancel();
    super.dispose();
  }

  Future<void> _refresh() async {
    try {
      final image = File(widget.interaction.imagePath);
      final modified = await image.lastModified();
      if (_imageModified != modified) {
        final bytes = await image.readAsBytes();
        if (mounted) {
          setState(() {
            _imageModified = modified;
            _imageBytes = bytes;
          });
        }
      }
      final data = jsonDecode(
        await File(widget.interaction.statusPath).readAsString(),
      );
      if (data is! Map || !mounted) return;
      final state = data['state'];
      if (state == 'success') {
        Navigator.of(context).pop();
        return;
      }
      setState(() {
        _status =
            data['message'] is String && (data['message'] as String).isNotEmpty
            ? data['message'] as String
            : '$state';
      });
    } catch (_) {}
  }

  @override
  Widget build(BuildContext context) => AlertDialog(
    title: Text(widget.interaction.title),
    content: ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 360),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          if (_imageBytes case final bytes?)
            Image.memory(bytes, width: 280, height: 280, fit: BoxFit.contain)
          else
            const SizedBox(
              width: 280,
              height: 280,
              child: Center(child: CircularProgressIndicator()),
            ),
          const SizedBox(height: 12),
          Text(_status, key: const Key('plugin-interaction-status')),
        ],
      ),
    ),
    actions: [
      TextButton(
        onPressed: () => Navigator.of(context).pop(),
        child: const Text('关闭'),
      ),
    ],
  );
}
