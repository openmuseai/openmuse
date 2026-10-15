import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import '../workbuddy/workbuddy_models.dart';

final class MobileMyPage extends StatelessWidget {
  const MobileMyPage({
    super.key,
    required this.themeMode,
    required this.onThemeMode,
    required this.workspaces,
    required this.deviceName,
    this.authentication,
    this.onSignIn,
    this.onSignOut,
    this.onDevices,
    this.onPlugins,
  });

  final ThemeMode themeMode;
  final ValueChanged<ThemeMode> onThemeMode;
  final List<WbWorkspace> workspaces;
  final String deviceName;
  final OpenMuseAuthenticationController? authentication;
  final VoidCallback? onSignIn;
  final VoidCallback? onSignOut;
  final VoidCallback? onDevices;
  final VoidCallback? onPlugins;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final colors = theme.colorScheme;
    final identity = authentication?.snapshot.identity;
    return Scaffold(
      backgroundColor: theme.scaffoldBackgroundColor,
      body: ListView(
        padding: const EdgeInsets.fromLTRB(20, 20, 20, 30),
        children: [
          Text(
            '我的',
            style: theme.textTheme.headlineLarge?.copyWith(
              fontWeight: FontWeight.w800,
              letterSpacing: -1.2,
            ),
          ),
          const SizedBox(height: 20),
          _group(context, [
            ListTile(
              key: const ValueKey('my-account'),
              leading: CircleAvatar(
                backgroundColor: colors.primaryContainer,
                child: Icon(
                  Icons.person_outline,
                  color: colors.onPrimaryContainer,
                ),
              ),
              title: Text(identity?.email ?? '未登录'),
              subtitle: Text(identity == null ? '登录以同步设备与插件' : '账户与设备已关联'),
              trailing: identity == null
                  ? const Icon(Icons.chevron_right)
                  : null,
              onTap: identity == null ? onSignIn : null,
            ),
            ListTile(
              key: const ValueKey('my-plugins'),
              leading: const Icon(Icons.extension_outlined),
              title: const Text('我的插件'),
              subtitle: const Text('查看已安装、更新与停用'),
              trailing: const Icon(Icons.chevron_right),
              onTap: onPlugins,
            ),
          ]),
          const SizedBox(height: 14),
          _group(context, [
            Column(
              children: [
                const ListTile(
                  leading: Icon(Icons.brightness_6_outlined),
                  title: Text('主题'),
                ),
                Padding(
                  padding: const EdgeInsets.fromLTRB(16, 0, 16, 16),
                  child: SizedBox(
                    width: double.infinity,
                    child: SegmentedButton<ThemeMode>(
                      key: const ValueKey('my-theme-mode'),
                      segments: const [
                        ButtonSegment(
                          value: ThemeMode.system,
                          label: Text('系统'),
                        ),
                        ButtonSegment(
                          value: ThemeMode.light,
                          label: Text('浅色'),
                        ),
                        ButtonSegment(value: ThemeMode.dark, label: Text('深色')),
                      ],
                      selected: {themeMode},
                      onSelectionChanged: (selection) =>
                          onThemeMode(selection.single),
                      showSelectedIcon: false,
                      style: const ButtonStyle(
                        visualDensity: VisualDensity.compact,
                      ),
                    ),
                  ),
                ),
              ],
            ),
            const Divider(height: 1),
            const ListTile(
              leading: Icon(Icons.language_outlined),
              title: Text('语言'),
              trailing: Text('跟随系统'),
            ),
            const ListTile(
              leading: Icon(Icons.text_fields),
              title: Text('字体大小'),
              trailing: Text('跟随系统'),
            ),
          ]),
          const SizedBox(height: 14),
          _group(context, [
            ListTile(
              key: const ValueKey('my-devices'),
              leading: const Icon(Icons.devices_outlined),
              title: const Text('设备与空间'),
              subtitle: Text('$deviceName · ${workspaces.length} 个 Workspace'),
              trailing: const Icon(Icons.chevron_right),
              onTap: onDevices,
            ),
            const ListTile(
              leading: Icon(Icons.info_outline),
              title: Text('关于 OpenMuse'),
            ),
          ]),
          if (identity != null) ...[
            const SizedBox(height: 20),
            TextButton.icon(
              key: const ValueKey('my-sign-out'),
              onPressed: onSignOut,
              icon: const Icon(Icons.logout),
              label: const Text('退出登录'),
            ),
          ],
        ],
      ),
    );
  }

  Widget _group(BuildContext context, List<Widget> children) => Material(
    color: Theme.of(context).colorScheme.surface,
    borderRadius: BorderRadius.circular(20),
    clipBehavior: Clip.antiAlias,
    child: Column(children: children),
  );
}
