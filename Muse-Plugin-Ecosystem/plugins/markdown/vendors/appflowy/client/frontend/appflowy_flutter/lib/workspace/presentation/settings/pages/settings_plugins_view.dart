import 'dart:async';
import 'dart:io';

import 'package:appflowy/plugins/resource_surface/helix/helix_language_servers.dart';
import 'package:appflowy/plugins/resource_surface/helix/helix_settings.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace/presentation/settings/shared/af_dropdown_menu_entry.dart';
import 'package:appflowy/workspace/presentation/settings/shared/document_color_setting_button.dart';
import 'package:appflowy/workspace/presentation/settings/shared/setting_list_tile.dart';
import 'package:appflowy/workspace/presentation/settings/shared/settings_body.dart';
import 'package:appflowy/workspace/presentation/settings/shared/settings_category.dart';
import 'package:appflowy/workspace/presentation/settings/shared/settings_dropdown.dart';
import 'package:appflowy/workspace/presentation/widgets/toggle/toggle.dart';
import 'package:flowy_infra/file_picker/file_picker_service.dart';
import 'package:flowy_infra_ui/flowy_infra_ui.dart';
import 'package:flutter/material.dart';

class SettingsPluginsView extends StatefulWidget {
  const SettingsPluginsView({super.key});

  @override
  State<SettingsPluginsView> createState() => _SettingsPluginsViewState();
}

class _SettingsPluginsViewState extends State<SettingsPluginsView> {
  late final HelixSettingsController _controller =
      getIt<HelixSettingsController>();

  @override
  void initState() {
    super.initState();
    unawaited(_controller.ensureLoaded());
  }

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: _controller,
      builder: (context, _) {
        final settings = _controller.settings;
        return SettingsBody(
          title: 'Plugin',
          description: '配置 Host 内嵌的编辑器插件。修改会在下次打开文件时生效。',
          children: [
            SettingsCategory(
              title: 'Helix',
              description:
                  '语法高亮来自 tree-sitter grammars，不是 Language Server。LS 只提供诊断、转到定义和查找引用。',
              children: [
                SettingListTile(
                  label: '语法高亮',
                  hint: '编译 Dart / Rust 等常用语言的 tree-sitter grammar。首次需要 git 与 C 编译器。',
                  trailing: [
                    FlowyButton(
                      useIntrinsicWidth: true,
                      text: const FlowyText('安装语法高亮', fontSize: 13),
                      disable: _controller.installer.busyId != null,
                      onTap: () async {
                        try {
                          await _controller.installHighlightGrammars();
                        } on Object catch (error) {
                          if (!context.mounted) return;
                          ScaffoldMessenger.of(context).showSnackBar(
                            SnackBar(content: Text('$error')),
                          );
                        }
                      },
                    ),
                  ],
                ),
                SettingListTile(
                  label: '颜色主题',
                  hint: 'Helix runtime 主题，同时决定默认背景',
                  trailing: [
                    SizedBox(
                      width: 220,
                      child: SettingsDropdown<String>(
                        expandWidth: false,
                        selectedOption: settings.theme,
                        onChanged: (theme) => _controller.update(
                          settings.copyWith(
                            theme: theme,
                            backgroundColor:
                                HelixSettings.themeBackgrounds[theme] ??
                                    settings.backgroundColor,
                          ),
                        ),
                        options: [
                          for (final theme in HelixSettings.themes)
                            buildDropdownMenuEntry<String>(
                              context,
                              value: theme,
                              label: theme,
                              selectedValue: settings.theme,
                            ),
                        ],
                      ),
                    ),
                  ],
                ),
                SettingListTile(
                  label: '背景颜色',
                  hint: '覆盖主题背景，立即作用于终端画布',
                  trailing: [
                    DocumentColorSettingButton(
                      currentColor: settings.backgroundColor,
                      dialogTitle: 'Helix 背景颜色',
                      previewWidgetBuilder: (color) => Container(
                        width: 18,
                        height: 18,
                        decoration: BoxDecoration(
                          color: color ?? settings.backgroundColor,
                          borderRadius: BorderRadius.circular(4),
                          border: Border.all(
                            color: Theme.of(context).dividerColor,
                          ),
                        ),
                      ),
                      onApply: (color) => _controller.update(
                        settings.copyWith(backgroundColor: color),
                      ),
                    ),
                  ],
                ),
                SettingListTile(
                  label: 'Keymap',
                  hint: 'VS Code 模式提供 F12 / Shift+F12 / Alt+←→，无需 Helix 命令',
                  trailing: [
                    SizedBox(
                      width: 220,
                      child: SettingsDropdown<HelixKeymapPreset>(
                        expandWidth: false,
                        selectedOption: settings.keymap,
                        onChanged: (keymap) => _controller.update(
                          settings.copyWith(keymap: keymap),
                        ),
                        options: [
                          buildDropdownMenuEntry<HelixKeymapPreset>(
                            context,
                            value: HelixKeymapPreset.vscode,
                            label: 'VS Code',
                            selectedValue: settings.keymap,
                          ),
                          buildDropdownMenuEntry<HelixKeymapPreset>(
                            context,
                            value: HelixKeymapPreset.helix,
                            label: 'Helix（模态）',
                            selectedValue: settings.keymap,
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
                SettingListTile(
                  label: '字体',
                  trailing: [
                    SizedBox(
                      width: 220,
                      child: SettingsDropdown<String>(
                        expandWidth: false,
                        selectedOption: settings.fontFamily,
                        onChanged: (font) => _controller.update(
                          settings.copyWith(fontFamily: font),
                        ),
                        options: [
                          for (final font in HelixSettings.fonts)
                            buildDropdownMenuEntry<String>(
                              context,
                              value: font,
                              label: font,
                              selectedValue: settings.fontFamily,
                              fontFamily: font,
                            ),
                        ],
                      ),
                    ),
                  ],
                ),
                SettingListTile(
                  label: '字号  ${settings.fontSize.toStringAsFixed(0)}',
                  trailing: [
                    SizedBox(
                      width: 180,
                      child: Slider(
                        min: 11,
                        max: 20,
                        divisions: 9,
                        value: settings.fontSize.clamp(11, 20),
                        onChanged: (size) => _controller.update(
                          settings.copyWith(fontSize: size),
                        ),
                      ),
                    ),
                  ],
                ),
                SettingListTile(
                  label: 'Language Server',
                  hint: '转到定义、查找引用、补全。不负责关键字着色。点击配置以安装或指定本地二进制。',
                  trailing: [
                    Toggle(
                      value: settings.enableLsp,
                      onChanged: (enabled) => _controller.update(
                        settings.copyWith(enableLsp: enabled),
                      ),
                    ),
                  ],
                ),
                Align(
                  alignment: Alignment.centerLeft,
                  child: FlowyButton(
                    useIntrinsicWidth: true,
                    text: const FlowyText('配置 Language Server', fontSize: 13),
                    disable: _controller.installer.busyId != null,
                    onTap: () => _openLanguageServerDialog(context),
                  ),
                ),
                if (_controller.installer.message != null)
                  FlowyText.regular(
                    _controller.installer.message!,
                    fontSize: 12,
                    maxLines: 4,
                    color: Theme.of(context).hintColor,
                  ),
                if (_controller.grammarStatus != null)
                  FlowyText.regular(
                    _controller.grammarStatus!,
                    fontSize: 12,
                    maxLines: 4,
                    color: Theme.of(context).hintColor,
                  ),
              ],
            ),
          ],
        );
      },
    );
  }

  Future<void> _openLanguageServerDialog(BuildContext context) async {
    await showDialog<void>(
      context: context,
      builder: (_) => _LanguageServerSetupDialog(controller: _controller),
    );
  }
}

class _LanguageServerSetupDialog extends StatefulWidget {
  const _LanguageServerSetupDialog({required this.controller});

  final HelixSettingsController controller;

  @override
  State<_LanguageServerSetupDialog> createState() =>
      _LanguageServerSetupDialogState();
}

class _LanguageServerSetupDialogState extends State<_LanguageServerSetupDialog> {
  late String _selectedId = helixLspCatalog.first.id;
  late final TextEditingController _commandController;
  late final TextEditingController _configController;
  String? _pathError;

  HelixLanguageServerInstaller get _installer => widget.controller.installer;

  HelixLspPackage get _package =>
      helixLspCatalog.firstWhere((item) => item.id == _selectedId);

  HelixLspStatus? get _status {
    for (final status in _installer.statuses) {
      if (status.package.id == _selectedId) return status;
    }
    return null;
  }

  @override
  void initState() {
    super.initState();
    _commandController = TextEditingController();
    _configController = TextEditingController();
    _installer.addListener(_syncFromInstaller);
    unawaited(
      _installer.refresh().then((_) {
        if (mounted) _syncFromInstaller();
      }),
    );
  }

  @override
  void dispose() {
    _installer.removeListener(_syncFromInstaller);
    _commandController.dispose();
    _configController.dispose();
    super.dispose();
  }

  void _syncFromInstaller() {
    final override = _installer.overrides[_selectedId];
    final command = override?.commandPath ?? _status?.resolvedPath ?? '';
    final config = override?.configPath ?? '';
    if (_commandController.text != command) {
      _commandController.text = command;
    }
    if (_configController.text != config) {
      _configController.text = config;
    }
    if (mounted) setState(() {});
  }

  void _selectPackage(String id) {
    setState(() {
      _selectedId = id;
      _pathError = null;
    });
    _syncFromInstaller();
  }

  Future<void> _pickFile({required bool config}) async {
    final result = await getIt<FilePickerService>().pickFiles(
      dialogTitle: config ? '选择 Language Server 配置文件' : '选择 Language Server 可执行文件',
    );
    final path = result?.files.isNotEmpty == true
        ? result!.files.first.path
        : null;
    if (path == null || path.isEmpty) return;
    if (config) {
      _configController.text = path;
    } else {
      _commandController.text = path;
    }
    setState(() => _pathError = null);
  }

  Future<void> _saveOverride() async {
    final command = _commandController.text.trim();
    final config = _configController.text.trim();
    if (command.isNotEmpty && !helixLspIsExecutableFile(command)) {
      setState(() {
        _pathError = Directory(command).existsSync()
            ? '请填写 Language Server 可执行文件路径（二进制本身），不是它所在的目录'
            : '找不到该可执行文件';
      });
      return;
    }
    if (config.isNotEmpty && !File(config).existsSync()) {
      setState(() => _pathError = '找不到该配置文件');
      return;
    }
    await _installer.setOverride(
      _selectedId,
      HelixLspOverride(
        commandPath: command.isEmpty ? null : command,
        configPath: config.isEmpty ? null : config,
      ),
    );
    if (command.isNotEmpty && widget.controller.settings.enableLsp == false) {
      await widget.controller.update(
        widget.controller.settings.copyWith(enableLsp: true),
      );
    }
    if (mounted) {
      setState(() => _pathError = null);
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('已保存 Language Server 路径')),
      );
    }
  }

  Future<void> _install() async {
    try {
      await _installer.install(_selectedId);
      if (widget.controller.settings.enableLsp == false) {
        await widget.controller.update(
          widget.controller.settings.copyWith(enableLsp: true),
        );
      }
    } on Object catch (error) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('$error')),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    final status = _status;
    final installing = _installer.busyId == _selectedId;
    final readyLabel = switch (status?.presence) {
      HelixLspPresence.custom => '已就绪（本地路径）',
      HelixLspPresence.bundled => '已就绪（OpenMuse）',
      HelixLspPresence.system => '已就绪（系统）',
      HelixLspPresence.missing || null => '未安装',
    };
    String defaultConfigDir;
    try {
      defaultConfigDir = _installer.defaultConfigDirectory;
    } on Object {
      defaultConfigDir = '';
    }
    return Dialog(
      backgroundColor: Colors.transparent,
      elevation: 0,
      insetPadding: const EdgeInsets.symmetric(horizontal: 28, vertical: 24),
      child: Material(
        type: MaterialType.transparency,
        child: Container(
          constraints: const BoxConstraints(maxWidth: 520),
          padding: const EdgeInsets.fromLTRB(20, 16, 16, 20),
          decoration: context.getPopoverDecoration(),
          child: AnimatedBuilder(
            animation: _installer,
            builder: (context, _) {
              return SingleChildScrollView(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Row(
                      children: [
                        const Expanded(
                          child: FlowyText(
                            'Language Server',
                            fontSize: 16,
                            fontWeight: FontWeight.w500,
                          ),
                        ),
                        FlowyIconButton(
                          width: 24,
                          icon: const Icon(Icons.close, size: 16),
                          onPressed: () => Navigator.of(context).pop(),
                        ),
                      ],
                    ),
                    const VSpace(8),
                    FlowyText.regular(
                      '选择编程语言后可一键下载对应 Language Server，或填入本机已有的可执行文件路径。',
                      fontSize: 12,
                      maxLines: 3,
                      color: Theme.of(context).hintColor,
                    ),
                    const VSpace(16),
                    const FlowyText.medium('编程语言', fontSize: 13),
                    const VSpace(6),
                    SettingsDropdown<String>(
                      selectedOption: _selectedId,
                      onChanged: _selectPackage,
                      options: [
                        for (final package in helixLspCatalog)
                          buildDropdownMenuEntry<String>(
                            context,
                            value: package.id,
                            label: '${package.languages} · ${package.label}',
                            selectedValue: _selectedId,
                          ),
                      ],
                    ),
                    const VSpace(8),
                    FlowyText.regular(
                      '$readyLabel${status?.resolvedPath == null ? '' : ' · ${status!.resolvedPath}'}',
                      fontSize: 12,
                      maxLines: 2,
                      color: status?.ready == true
                          ? Theme.of(context).colorScheme.primary
                          : Theme.of(context).hintColor,
                    ),
                    const VSpace(16),
                    FlowyButton(
                      useIntrinsicWidth: true,
                      disable: installing || _installer.busyId != null,
                      text: FlowyText(
                        installing
                            ? '正在下载 ${_package.label}…'
                            : '一键下载 ${_package.label}',
                        fontSize: 13,
                      ),
                      onTap: _install,
                    ),
                    if (installing && _installer.progress > 0) ...[
                      const VSpace(8),
                      LinearProgressIndicator(value: _installer.progress),
                    ],
                    const VSpace(16),
                    _HintLabel(
                      label: '本地可执行文件',
                      hint: _package.binaryHint,
                    ),
                    const VSpace(6),
                    Row(
                      children: [
                        Expanded(
                          child: FlowyTextField(
                            controller: _commandController,
                            hintText:
                                '/usr/local/bin/${_package.helixCommand}',
                          ),
                        ),
                        const HSpace(8),
                        FlowyButton(
                          useIntrinsicWidth: true,
                          text: const FlowyText('浏览', fontSize: 13),
                          onTap: () => _pickFile(config: false),
                        ),
                      ],
                    ),
                    const VSpace(16),
                    _HintLabel(
                      label: '配置文件',
                      hint: [
                        if (defaultConfigDir.isNotEmpty)
                          '默认配置目录：$defaultConfigDir',
                        if (_package.defaultConfigFiles.isNotEmpty)
                          '常见文件：${_package.defaultConfigFiles.join('、')}',
                      ].join('\n'),
                    ),
                    const VSpace(6),
                    Row(
                      children: [
                        Expanded(
                          child: FlowyTextField(
                            controller: _configController,
                            hintText: '$defaultConfigDir/languages.toml',
                          ),
                        ),
                        const HSpace(8),
                        FlowyButton(
                          useIntrinsicWidth: true,
                          text: const FlowyText('浏览', fontSize: 13),
                          onTap: () => _pickFile(config: true),
                        ),
                      ],
                    ),
                    const VSpace(16),
                    FlowyButton(
                      useIntrinsicWidth: true,
                      text: const FlowyText('保存本地路径', fontSize: 13),
                      onTap: _saveOverride,
                    ),
                    if (_pathError != null) ...[
                      const VSpace(8),
                      FlowyText.regular(
                        _pathError!,
                        fontSize: 12,
                        maxLines: 3,
                        color: Theme.of(context).colorScheme.error,
                      ),
                    ],
                    if (_installer.message != null) ...[
                      const VSpace(8),
                      FlowyText.regular(
                        _installer.message!,
                        fontSize: 12,
                        maxLines: 4,
                        color: Theme.of(context).hintColor,
                      ),
                    ],
                  ],
                ),
              );
            },
          ),
        ),
      ),
    );
  }
}

class _HintLabel extends StatelessWidget {
  const _HintLabel({required this.label, required this.hint});

  final String label;
  final String hint;

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        FlowyText.medium(label, fontSize: 13),
        if (hint.trim().isNotEmpty)
          Transform.translate(
            offset: const Offset(1, -4),
            child: AppFlowyPopover(
              direction: PopoverDirection.bottomWithLeftAligned,
              offset: const Offset(0, 6),
              constraints: const BoxConstraints(maxWidth: 320),
              popupBuilder: (_) => Padding(
                padding: const EdgeInsets.all(10),
                child: FlowyText.regular(
                  hint,
                  fontSize: 12,
                  maxLines: 8,
                ),
              ),
              child: Padding(
                padding: const EdgeInsets.all(2),
                child: Icon(
                  Icons.info_outline,
                  size: 13,
                  color: Theme.of(context).hintColor,
                ),
              ),
            ),
          ),
      ],
    );
  }
}