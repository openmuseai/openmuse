import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:url_launcher/url_launcher.dart';

import 'workspace_controller.dart';
import 'workspace_picker.dart';

/// The fixed document workspace shipped through the Mac App Store. The direct
/// distribution continues to use the complete desktop workbench.
class OpenMuseStoreHome extends StatefulWidget {
  const OpenMuseStoreHome({
    super.key,
    required this.resources,
    required this.workspace,
    required this.registry,
  });
  final NativeResourceCenter? resources;
  final LocalWorkspaceController workspace;
  final OpenMusePluginRegistry registry;

  @override
  State<OpenMuseStoreHome> createState() => _OpenMuseStoreHomeState();
}

class _OpenMuseStoreHomeState extends State<OpenMuseStoreHome> {
  static const _menu = MethodChannel('com.openmuse.host/menu');
  bool _mounting = false;

  @override
  void initState() {
    super.initState();
    _menu.setMethodCallHandler((call) async {
      if (call.method == 'settings') await _account();
      if (call.method == 'help') {
        await launchUrl(Uri.parse('https://us.openmuseai.com/en/guide/'));
      }
    });
  }

  @override
  void dispose() {
    _menu.setMethodCallHandler(null);
    super.dispose();
  }

  Future<void> _account() => showDialog<void>(
    context: context,
    builder: (dialog) => AlertDialog(
      title: const Text('Account'),
      content: SizedBox(
        width: 420,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            for (final plugin
                in widget.registry.descriptors
                    .map((d) => widget.registry.plugin(d.id))
                    .whereType<OpenMuseSettingsContributor>())
              plugin.buildSettings(dialog),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(dialog),
          child: const Text('Done'),
        ),
      ],
    ),
  );

  Future<void> _addFolder() async {
    setState(() => _mounting = true);
    try {
      final path = await const WorkspacePicker().chooseDirectory();
      if (path == null) return;
      await widget.workspace.addWorkspace(path);
      await widget.resources?.mountDirectories({
        for (final mount in widget.workspace.mounts) mount.path: mount.name,
      });
    } catch (error) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text(
              'The folder could not be opened. Please select it again.',
            ),
          ),
        );
      }
    } finally {
      if (mounted) setState(() => _mounting = false);
    }
  }

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(
      title: const Text('OpenMuse Desktop'),
      actions: [
        TextButton.icon(
          onPressed: _mounting ? null : _addFolder,
          icon: const Icon(Icons.create_new_folder_outlined),
          label: const Text('Open folder'),
        ),
        IconButton(
          tooltip: 'Account',
          onPressed: _account,
          icon: const Icon(Icons.account_circle_outlined),
        ),
        IconButton(
          tooltip: 'Privacy policy',
          onPressed: () =>
              launchUrl(Uri.parse('https://us.openmuseai.com/en/privacy/')),
          icon: const Icon(Icons.privacy_tip_outlined),
        ),
      ],
    ),
    body: widget.resources == null
        ? const Center(child: CircularProgressIndicator())
        : ResourceCenterView(controller: widget.resources!.controller),
  );
}
