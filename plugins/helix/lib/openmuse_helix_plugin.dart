library;

import 'dart:io';

import 'package:flutter/material.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:path/path.dart' as p;

import 'src/helix_editor_surface.dart';
import 'src/helix_language_servers.dart';
import 'src/helix_preferences.dart';
import 'src/helix_runtime.dart';
import 'src/helix_open_trace.dart';

export 'src/helix_runtime.dart';
export 'src/helix_preferences.dart';
export 'src/helix_language_servers.dart';

final class OpenMuseHelixPlugin
    implements
        OpenMusePlugin,
        OpenMuseSettingsContributor,
        OpenMuseBufferFlushContributor,
        OpenMuseEditorBannerContributor {
  OpenMuseHelixPlugin({HelixRuntimePool? runtime}) : _runtime = runtime;

  HelixRuntimePool? _runtime;
  OpenMusePluginContext? _context;
  final ValueNotifier<HelixPreferences> _preferences = ValueNotifier(
    const HelixPreferences(),
  );

  @override
  final descriptor = const OpenMusePluginDescriptor(
    id: 'com.openmuse.helix',
    name: 'Helix Editor',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.nativeProcess,
    activationEvents: ['onFileType:text'],
    permissions: {
      'filesystem.workspace.read',
      'filesystem.workspace.write',
      'process.pty',
    },
    editors: [
      OpenMuseEditorContribution(
        id: 'helix.editor',
        mediaTypes: {'text/*'},
        extensions: {
          'txt',
          'md',
          'rs',
          'c',
          'cc',
          'cpp',
          'cs',
          'csx',
          'css',
          'diff',
          'go',
          'html',
          'h',
          'hpp',
          'ini',
          'java',
          'kts',
          'jsx',
          'kt',
          'log',
          'lua',
          'vue',
          'svelte',
          'dockerfile',
          'tf',
          'ex',
          'exs',
          'php',
          'graphql',
          'proto',
          'mjs',
          'py',
          'rb',
          'scss',
          'sql',
          'swift',
          'tsx',
          'xml',
          'dart',
          'ts',
          'js',
          'json',
          'yaml',
          'yml',
          'toml',
          'sh',
          'bash',
          'zsh',
        },
        priority: 50,
      ),
    ],
  );

  @override
  Future<void> activate(OpenMusePluginContext context) async {
    final activationWatch = Stopwatch()..start();
    HelixOpenTrace.mark('activate_begin');
    _context = context;
    _runtime ??= HelixRuntimePool();
    _runtime!.onActiveResourceChanged = (path) {
      // Host performs canonical mount authorization; PTY events cannot open
      // arbitrary local files through the plugin.
      _context
          ?.executeHostCommand('workspace.openResource', {
            'path': path,
            'editorId': 'helix.editor',
          })
          .catchError((Object error) {
            debugPrint('Helix resource event rejected: $error');
            return null;
          });
    };
    final saved = await context.executeHostCommand(
      'settings.plugin.read',
      descriptor.id,
    );
    if (saved is Map) {
      _preferences.value = HelixPreferences.fromJson(saved);
    }
    await _runtime!.probeCapabilities();
    HelixOpenTrace.mark(
      'probe_done',
      elapsedMs: activationWatch.elapsedMilliseconds,
    );
    await _runtime!.configure(_preferences.value);
    HelixOpenTrace.mark(
      'activate_done',
      elapsedMs: activationWatch.elapsedMilliseconds,
    );
  }

  @override
  Future<void> deactivate() async {
    await _runtime?.stop();
    _runtime?.onActiveResourceChanged = null;
    _runtime?.dispose();
    _runtime = null;
    _context = null;
  }

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) {
    final runtime = _runtime;
    if (runtime == null) {
      throw StateError('Helix plugin must be active before building a surface');
    }
    return HelixEditorSurface(runtime: runtime, resource: resource);
  }

  @override
  Widget? buildEditorBanner(
    BuildContext context,
    OpenMuseResource resource, {
    required bool Function(String editorId) canOpenWith,
    required void Function(String editorId) openWith,
  }) => OpenMuseEditorBanner(
    message: 'Helix 编辑 · Ctrl+S 保存 · Esc 进入命令模式',
    actionLabel:
        resource.mediaType == 'text/markdown' && canOpenWith('viewer.markdown')
        ? 'Preview'
        : null,
    onAction: resource.mediaType == 'text/markdown'
        ? () => openWith('viewer.markdown')
        : null,
  );

  @override
  Widget? buildPanel(BuildContext context, String panelId) => null;

  @override
  Future<void> flushResource(OpenMuseResource resource) async {
    if (!resource.uri.isScheme('file')) return;
    await _runtime?.flushResource(resource.uri.toFilePath());
  }

  Future<void> updatePreferences(HelixPreferences next) async {
    await _runtime?.configure(next);
    await _context?.executeHostCommand('settings.plugin.write', {
      'pluginId': descriptor.id,
      'values': next.toJson(),
    });
    _preferences.value = next;
  }

  Future<void> _updateFromSettings(
    BuildContext context,
    HelixPreferences next,
  ) async {
    try {
      await updatePreferences(next);
    } catch (error) {
      if (context.mounted) {
        ScaffoldMessenger.maybeOf(
          context,
        )?.showSnackBar(SnackBar(content: Text('$error')));
      }
    }
  }

  @override
  Widget buildSettings(
    BuildContext context,
  ) => ValueListenableBuilder<HelixPreferences>(
    valueListenable: _preferences,
    builder: (context, value, _) => Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Text(
          'Helix',
          style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
        ),
        const SizedBox(height: 5),
        Text(
          '语法高亮来自 tree-sitter；Language Server 提供诊断、定义和引用。',
          style: TextStyle(
            fontSize: 12,
            color: Theme.of(context).colorScheme.onSurfaceVariant,
          ),
        ),
        const SizedBox(height: 13),
        _HelixSettingRow(
          label: '语法高亮',
          hint:
              '内置 ${HelixPreferences.bundledGrammarCount(_runtime?.executable ?? resolveHelixExecutable())} 个 tree-sitter grammars',
          trailing: TextButton(
            onPressed: () => _showGrammarStatus(context),
            child: const Text('查看状态'),
          ),
        ),
        _HelixSettingRow(
          label: '颜色主题',
          hint: '同时影响 Helix 运行时与终端画布',
          trailing: DropdownButton<String>(
            value: value.theme,
            items: const [
              DropdownMenuItem(
                value: 'onelight',
                child: Text('OpenMuse Light'),
              ),
              DropdownMenuItem(
                value: 'openmuse_dark',
                child: Text('OpenMuse Dark'),
              ),
            ],
            onChanged: (theme) {
              if (theme != null)
                updatePreferences(value.copyWith(theme: theme));
            },
          ),
        ),
        _HelixSettingRow(
          label: '输入模式',
          hint: _runtime?.nonmodalSelectable == true
              ? '引擎级实验模式；切换前须关闭编辑会话'
              : '非模态引擎仍在验证，发布构建暂不开放',
          trailing: DropdownButton<HelixInputProfile>(
            value: value.inputProfile,
            items: [
              const DropdownMenuItem(
                value: HelixInputProfile.helixModal,
                child: Text('Helix（模态）'),
              ),
              DropdownMenuItem(
                value: HelixInputProfile.standardNonmodal,
                enabled: _runtime?.nonmodalSelectable == true,
                child: const Text('标准非模态（实验）'),
              ),
            ],
            onChanged: (profile) {
              if (profile != null) {
                _updateFromSettings(
                  context,
                  value.copyWith(inputProfile: profile, vscodeKeymap: false),
                );
              }
            },
          ),
        ),
        _HelixSettingRow(
          label: '字体',
          trailing: DropdownButton<String>(
            value: value.fontFamily,
            items: [
              for (final font in HelixPreferences.fonts)
                DropdownMenuItem(value: font, child: Text(font)),
            ],
            onChanged: (font) {
              if (font != null)
                updatePreferences(value.copyWith(fontFamily: font));
            },
          ),
        ),
        _HelixSettingRow(
          label: '字号 ${value.fontSize.round()}',
          trailing: SizedBox(
            width: 190,
            child: Slider(
              min: 11,
              max: 20,
              divisions: 9,
              value: value.fontSize,
              onChanged: (size) =>
                  updatePreferences(value.copyWith(fontSize: size)),
            ),
          ),
        ),
        _HelixSettingRow(
          label: 'Language Server',
          hint: '仅控制 LSP；不影响 tree-sitter 语法高亮',
          trailing: Switch.adaptive(
            value: value.enableLsp,
            onChanged: (enabled) =>
                updatePreferences(value.copyWith(enableLsp: enabled)),
          ),
        ),
        Align(
          alignment: Alignment.centerLeft,
          child: Wrap(
            spacing: 8,
            children: [
              TextButton(
                onPressed: () => _configureLanguageServer(context, value),
                child: const Text('选择语言并配置 LS…'),
              ),
              TextButton(
                onPressed: () => _showLanguageServerStatus(context, value),
                child: const Text('查看 LS 状态'),
              ),
            ],
          ),
        ),
      ],
    ),
  );

  void _showGrammarStatus(BuildContext context) {
    final count = HelixPreferences.bundledGrammarCount(
      _runtime?.executable ?? resolveHelixExecutable(),
    );
    showDialog<void>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('tree-sitter 语法高亮'),
        content: Text(
          count > 0
              ? '当前内置 $count 个语法库。Helix 会按文件类型自动选择；Language Server 单独配置。'
              : '当前未检测到语法库。请使用包含 runtime/grammars 的发行包。',
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('关闭'),
          ),
        ],
      ),
    );
  }

  void _showLanguageServerStatus(
    BuildContext context,
    HelixPreferences preferences,
  ) {
    showDialog<void>(
      context: context,
      builder: (dialogContext) => StatefulBuilder(
        builder: (context, _) {
          final statuses = inspectHelixLanguageServers(
            preferences.languageServerPaths,
          );
          return AlertDialog(
            title: const Text('Language Server 状态'),
            content: SizedBox(
              width: 540,
              child: ListView(
                shrinkWrap: true,
                children: [
                  for (final status in statuses)
                    ListTile(
                      dense: true,
                      title: Text(status.spec.label),
                      subtitle: Text(status.path ?? '未找到；可在本机安装后指定可执行文件'),
                      trailing: Text(switch (status.presence) {
                        HelixServerPresence.custom => '自定义',
                        HelixServerPresence.system => '系统 PATH',
                        HelixServerPresence.missing => '手动配置',
                      }),
                    ),
                ],
              ),
            ),
            actions: [
              TextButton(
                onPressed: () => Navigator.pop(context),
                child: const Text('关闭'),
              ),
            ],
          );
        },
      ),
    );
  }

  Future<void> _configureLanguageServer(
    BuildContext context,
    HelixPreferences current,
  ) async {
    String? selected;
    var installing = false;
    final pathController = TextEditingController();
    final configController = TextEditingController();
    String? error;
    try {
      await showDialog<void>(
        context: context,
        builder: (dialogContext) => StatefulBuilder(
          builder: (context, setDialogState) => AlertDialog(
            title: const Text('配置本地 Language Server'),
            content: SizedBox(
              width: 440,
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  Text(
                    '选择语言服务，再指定本机已有的可执行文件。',
                    style: TextStyle(
                      fontSize: 12,
                      color: Theme.of(context).colorScheme.onSurfaceVariant,
                    ),
                  ),
                  const SizedBox(height: 14),
                  DropdownButton<String>(
                    value: selected,
                    hint: const Text('选择语言服务'),
                    items: [
                      for (final server in helixLanguageServers)
                        DropdownMenuItem(
                          value: server.command,
                          child: Text(server.label),
                        ),
                    ],
                    onChanged: installing
                        ? null
                        : (value) {
                            if (value == null) return;
                            setDialogState(() {
                              selected = value;
                              error = null;
                              pathController.text =
                                  current.languageServerPaths[value] ?? '';
                              configController.text =
                                  current.languageServerConfigPaths[value] ??
                                  '';
                            });
                          },
                  ),
                  if (selected != null) ...[
                    TextField(
                      controller: pathController,
                      decoration: InputDecoration(
                        labelText: '可执行文件的绝对路径',
                        errorText: error,
                      ),
                    ),
                    const SizedBox(height: 10),
                    TextField(
                      controller: configController,
                      decoration: const InputDecoration(
                        labelText: 'LS 配置文件绝对路径（JSON / TOML，可选）',
                      ),
                    ),
                    const SizedBox(height: 7),
                    const Text(
                      '留空并保存可移除覆盖，改用系统 PATH 中的 Language Server。',
                      style: TextStyle(fontSize: 11),
                    ),
                    if (helixLanguageServers
                        .where((item) => item.command == selected)
                        .single
                        .oneClickInstallable) ...[
                      const SizedBox(height: 12),
                      FilledButton.tonal(
                        onPressed: installing
                            ? null
                            : () async {
                                final command = selected!;
                                setDialogState(() {
                                  installing = true;
                                  error = null;
                                });
                                try {
                                  final spec = helixLanguageServers
                                      .where((item) => item.command == command)
                                      .single;
                                  final path = await installHelixLanguageServer(
                                    spec,
                                  );
                                  final paths = Map<String, String>.of(
                                    current.languageServerPaths,
                                  )..[command] = path;
                                  current = current.copyWith(
                                    languageServerPaths: paths,
                                  );
                                  await updatePreferences(current);
                                  if (dialogContext.mounted) {
                                    setDialogState(() {
                                      if (selected == command)
                                        pathController.text = path;
                                      installing = false;
                                    });
                                  }
                                } catch (failure) {
                                  if (dialogContext.mounted) {
                                    setDialogState(() {
                                      error = '$failure';
                                      installing = false;
                                    });
                                  }
                                }
                              },
                        child: Text(installing ? '正在安装…' : '一键安装此语言的 LS'),
                      ),
                    ],
                  ],
                ],
              ),
            ),
            actions: [
              TextButton(
                onPressed: installing
                    ? null
                    : () => Navigator.pop(dialogContext),
                child: const Text('取消'),
              ),
              FilledButton(
                onPressed: selected == null || installing
                    ? null
                    : () async {
                        final path = pathController.text.trim();
                        final configPath = configController.text.trim();
                        if (path.isNotEmpty &&
                            (!p.isAbsolute(path) || !File(path).existsSync())) {
                          setDialogState(() => error = '请选择存在的绝对文件路径');
                          return;
                        }
                        if (configPath.isNotEmpty &&
                            (!p.isAbsolute(configPath) ||
                                !File(configPath).existsSync())) {
                          setDialogState(() => error = '请选择存在的 LS 配置文件绝对路径');
                          return;
                        }
                        final paths = Map<String, String>.of(
                          current.languageServerPaths,
                        );
                        final configPaths = Map<String, String>.of(
                          current.languageServerConfigPaths,
                        );
                        if (path.isEmpty) {
                          paths.remove(selected);
                        } else {
                          paths[selected!] = path;
                        }
                        if (configPath.isEmpty) {
                          configPaths.remove(selected);
                        } else {
                          configPaths[selected!] = configPath;
                        }
                        await updatePreferences(
                          current.copyWith(
                            languageServerPaths: paths,
                            languageServerConfigPaths: configPaths,
                          ),
                        );
                        if (dialogContext.mounted) Navigator.pop(dialogContext);
                      },
                child: const Text('保存'),
              ),
            ],
          ),
        ),
      );
    } finally {
      pathController.dispose();
      configController.dispose();
    }
  }
}

final class _HelixSettingRow extends StatelessWidget {
  const _HelixSettingRow({
    required this.label,
    this.hint,
    required this.trailing,
  });
  final String label;
  final String? hint;
  final Widget trailing;

  @override
  Widget build(BuildContext context) => SizedBox(
    height: hint == null ? 58 : 65,
    child: Row(
      children: [
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text(label, style: const TextStyle(fontSize: 13)),
              if (hint != null)
                Text(
                  hint!,
                  style: TextStyle(
                    fontSize: 11,
                    color: Theme.of(context).colorScheme.onSurfaceVariant,
                  ),
                ),
            ],
          ),
        ),
        trailing,
      ],
    ),
  );
}
