import 'dart:convert';
import 'package:flutter/material.dart';

typedef CodeNavigationManagement =
    Future<Map<String, Object?>> Function(
      String operation,
      String pluginId,
      Map<String, Object?> options,
    );

/// Shared plugin-owned settings surface; no process or filesystem capability.
class CodeNavigationSettingsPage extends StatefulWidget {
  const CodeNavigationSettingsPage({
    super.key,
    required this.pluginId,
    required this.title,
    required this.request,
    required this.targetLabel,
  });
  final String pluginId, title, targetLabel;
  final CodeNavigationManagement request;
  @override
  State<CodeNavigationSettingsPage> createState() =>
      _CodeNavigationSettingsPageState();
}

class _CodeNavigationSettingsPageState
    extends State<CodeNavigationSettingsPage> {
  static const service = 'com.openmuse.code-navigation';
  final sdk = TextEditingController(),
      initialization = TextEditingController(),
      configuration = TextEditingController();
  final idle = TextEditingController(), instances = TextEditingController();
  Map<String, Object?>? plan;
  bool busy = false;
  String? error, message;
  bool get isService => widget.pluginId == service;
  bool get isDart => widget.pluginId.endsWith('.dart');
  @override
  void initState() {
    super.initState();
    _load();
  }

  @override
  void dispose() {
    for (final x in [sdk, initialization, configuration, idle, instances]) {
      x.dispose();
    }
    super.dispose();
  }

  void _apply(Map<String, Object?> value) {
    plan = value;
    final config = value['configuration'] as Map? ?? const {};
    sdk.text = '${config['sdkPath'] ?? ''}';
    initialization.text = const JsonEncoder.withIndent(
      '  ',
    ).convert(config['initializationOptions'] ?? {});
    configuration.text = const JsonEncoder.withIndent(
      '  ',
    ).convert(config['configuration'] ?? {});
    idle.text = '${config['idleSeconds'] ?? 300}';
    instances.text = '${config['maxInstances'] ?? 2}';
  }

  Future<void> _load() => _run(() async {
    _apply(await widget.request('plan', widget.pluginId, {}));
  });
  String _error(Object value) {
    final text = '$value';
    for (final entry in const {
      'DEPENDENCY_REQUIRED': '请先安装代码导航服务，再安装语言扩展。',
      'DEPENDENTS_INSTALLED': '请先卸载 Dart 和 TypeScript/JavaScript 语言扩展，再卸载服务。',
      'DETECTION_REQUIRED': '请先保存配置并完成语言服务检测。',
      'SERVICE_DISABLED': '请先在代码导航服务中启用服务。',
      'CONFIGURATION_CONFLICT': '配置已在另一端更新，请刷新后重试。',
      'SDK_REQUIRED': '请填写这台资源宿主上的 Dart 可执行程序绝对路径。',
      'ENOENT': '未找到配置的程序，请检查资源宿主上的路径。',
      'INVALID_CONFIGURATION': '配置格式无效，请检查路径、JSON 和数值范围。',
      'TIMEOUT': '检测超时，请检查 SDK 和宿主状态。',
      'ARTIFACT': '插件交付物校验失败，请更新正常版应用后重试。',
      'DISABLED': '操作已取消或服务已关闭。',
      'STOP_FAILED': '语言进程尚未完全停止，请重试关闭；当前状态保留。',
    }.entries) {
      if (text.contains(entry.key)) return entry.value;
    }
    if (value is FormatException) return '请输入有效的 JSON 对象或整数。';
    return '操作失败：${text.split('\n').first}';
  }

  Future<void> _run(Future<void> Function() action) async {
    if (busy) return;
    setState(() {
      busy = true;
      error = null;
      message = null;
    });
    try {
      await action();
    } catch (e) {
      if (mounted) error = _error(e);
    } finally {
      if (mounted) setState(() => busy = false);
    }
  }

  Map<String, Object?> _values() {
    if (isService)
      return {
        'idleSeconds': int.parse(idle.text),
        'maxInstances': int.parse(instances.text),
      };
    final init = jsonDecode(initialization.text),
        config = jsonDecode(configuration.text);
    if (init is! Map || config is! Map) throw const FormatException('object');
    return {
      'sdkPath': isDart ? sdk.text.trim() : '',
      'initializationOptions': init,
      'configuration': config,
    };
  }

  Future<void> _act(String operation) => _run(() async {
    final result = await widget.request(operation, widget.pluginId, {});
    _apply(result);
    message = operation == 'detect'
        ? '检测通过 · 临时语言进程已释放'
        : operation == 'enable'
        ? '已启用，首次代码查询时启动'
        : operation == 'disable'
        ? '已关闭，语言进程已释放'
        : operation == 'uninstall'
        ? '已卸载'
        : '安装完成，默认关闭';
  });
  Future<void> _save() => _run(() async {
    final result = await widget.request('configure', widget.pluginId, {
      'expectedRevision': plan!['revision'],
      'values': _values(),
    });
    _apply(result);
    message = isService ? '配置已保存，插件已关闭；可重新启用。' : '配置已保存，语言扩展已关闭；请重新检测后启用。';
  });
  Widget _field(
    String label,
    TextEditingController value, {
    int lines = 1,
    String? hint,
  }) => Padding(
    padding: const EdgeInsets.only(bottom: 16),
    child: TextField(
      controller: value,
      enabled: !busy,
      minLines: lines,
      maxLines: lines == 1 ? 1 : 8,
      decoration: InputDecoration(
        labelText: label,
        helperText: hint,
        border: const OutlineInputBorder(),
      ),
    ),
  );
  @override
  Widget build(BuildContext context) {
    final installed = plan?['installedVersion'] != null,
        enabled = plan?['enabled'] == true;
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
        actions: [
          IconButton(
            tooltip: '刷新状态',
            onPressed: busy ? null : _load,
            icon: const Icon(Icons.refresh),
          ),
        ],
      ),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 760),
          child: ListView(
            padding: const EdgeInsets.all(24),
            children: [
              Text(
                '安装与配置目标：${widget.targetLabel}',
                style: Theme.of(context).textTheme.titleMedium,
              ),
              const SizedBox(height: 8),
              const Text('代码在资源所属宿主解析。安装、配置和启用不会启动常驻语言进程。'),
              const SizedBox(height: 16),
              if (busy) const LinearProgressIndicator(),
              if (error != null)
                Padding(
                  padding: const EdgeInsets.symmetric(vertical: 12),
                  child: Text(
                    error!,
                    style: TextStyle(
                      color: Theme.of(context).colorScheme.error,
                    ),
                  ),
                ),
              if (message != null)
                Padding(
                  padding: const EdgeInsets.symmetric(vertical: 12),
                  child: Text(message!),
                ),
              if (plan != null) ...[
                Text(
                  !installed
                      ? '未安装'
                      : '${enabled ? '已启用 · 按需启动' : '已安装 · 已关闭'} · ${plan!['detected'] == true ? '配置可用' : '待检测'} · 运行实例 ${plan!['instances'] ?? 0}',
                ),
                const SizedBox(height: 20),
                if (!installed) ...[
                  if (!isService)
                    OutlinedButton(
                      onPressed: busy
                          ? null
                          : () => _run(() async {
                              await widget.request('install', service, {});
                              _apply(
                                await widget.request(
                                  'plan',
                                  widget.pluginId,
                                  {},
                                ),
                              );
                              message = '依赖服务已安装，默认关闭';
                            }),
                      child: const Text('安装依赖：代码导航服务'),
                    ),
                  FilledButton(
                    onPressed: busy ? null : () => _act('install'),
                    child: const Text('安装插件'),
                  ),
                ] else ...[
                  if (isService) ...[
                    _field('空闲释放（秒，30–3600）', idle),
                    _field('最多实例（1–4）', instances),
                  ] else ...[
                    if (isDart)
                      _field(
                        '宿主 Dart 可执行程序路径',
                        sdk,
                        hint: '例如 /path/to/dart-sdk/bin/dart',
                      ),
                    if (!isDart)
                      const Padding(
                        padding: EdgeInsets.only(bottom: 16),
                        child: Text(
                          'TypeScript 5.9.3 / typescript-language-server 6.1.0 · 随插件离线安装',
                        ),
                      ),
                    _field('初始化选项（JSON）', initialization, lines: 3),
                    _field('语言服务设置（JSON）', configuration, lines: 3),
                  ],
                  Wrap(
                    spacing: 12,
                    runSpacing: 12,
                    children: [
                      OutlinedButton(
                        onPressed: busy ? null : _save,
                        child: const Text('保存配置'),
                      ),
                      OutlinedButton(
                        onPressed: busy ? null : () => _act('detect'),
                        child: const Text('检测配置'),
                      ),
                      FilledButton(
                        onPressed: busy
                            ? null
                            : () => _act(enabled ? 'disable' : 'enable'),
                        child: Text(enabled ? '关闭并释放' : '启用插件'),
                      ),
                      TextButton(
                        onPressed: busy ? null : () => _act('uninstall'),
                        child: const Text('卸载插件'),
                      ),
                    ],
                  ),
                ],
              ],
            ],
          ),
        ),
      ),
    );
  }
}
