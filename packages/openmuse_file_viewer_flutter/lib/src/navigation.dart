import 'package:flutter/material.dart';
import 'package:muse_resource_center/muse_resource_center.dart';

typedef CodeNavigationCallback =
    Future<ResourceCodeResult> Function(
      ResourceCodeOperation operation,
      int line,
      int character,
    );

final class CodeNavigationSheet extends StatefulWidget {
  const CodeNavigationSheet({
    super.key,
    required this.callback,
    required this.line,
    required this.character,
    required this.symbol,
  });
  final CodeNavigationCallback callback;
  final int line, character;
  final String symbol;
  @override
  State<CodeNavigationSheet> createState() => _CodeNavigationSheetState();
}

final class _CodeNavigationSheetState extends State<CodeNavigationSheet> {
  ResourceCodeResult? _result;
  ResourceCodeOperation _operation = ResourceCodeOperation.definition;
  bool _busy = false;
  String? _error;
  @override
  void initState() {
    super.initState();
    _load(ResourceCodeOperation.status);
  }

  Future<void> _load(ResourceCodeOperation operation) async {
    if (_busy) return;
    setState(() {
      _busy = true;
      _error = null;
    });
    try {
      var result = await widget.callback(
        operation,
        widget.line,
        widget.character,
      );
      if (operation == ResourceCodeOperation.status ||
          operation == ResourceCodeOperation.enable) {
        if (result.enabled && result.configured && widget.symbol != '代码导航') {
          result = await widget.callback(
            ResourceCodeOperation.definition,
            widget.line,
            widget.character,
          );
          _operation = ResourceCodeOperation.definition;
        }
      } else if (operation.index >= ResourceCodeOperation.definition.index) {
        _operation = operation;
      }
      if (mounted) setState(() => _result = result);
    } on Object catch (error) {
      if (mounted)
        setState(
          () => _error = switch (error is ResourceFailure ? error.code : '') {
            'VERSION_MISMATCH' || 'VERSION_UNAVAILABLE' => '文件已变化，请重新打开后查询。',
            'LANGUAGE_UNCONFIGURED' => '当前宿主尚未配置此语言的导航服务。',
            'DISABLED' => '代码导航已关闭。',
            'TIMEOUT' => '查询超时，可重试；源码仍可继续阅读。',
            'BUSY' || 'STOPPING' => '语言服务繁忙，请稍后重试。',
            'DENIED' => '无权访问此资源或目标。',
            'UNSUPPORTED_OPERATION' => '当前语言服务不支持此操作。',
            _ => '代码导航不可用，请检查资源宿主连接。',
          },
        );
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  @override
  Widget build(BuildContext context) => SizedBox(
    height: MediaQuery.sizeOf(context).height * .60,
    child: Padding(
      padding: const EdgeInsets.symmetric(horizontal: 20),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Text(
            '代码导航 · ${widget.symbol}',
            style: Theme.of(context).textTheme.titleMedium,
          ),
          const SizedBox(height: 8),
          if (_busy) const LinearProgressIndicator(),
          if (_error != null)
            Padding(padding: const EdgeInsets.all(12), child: Text(_error!)),
          if (_result case final result?) ...[
            if (!result.configured) const Text('当前宿主尚未配置此语言的导航服务。'),
            if (!result.enabled) ...[
              const Text('代码导航默认关闭。启用后在资源宿主按需运行语言服务，空闲 5 分钟自动释放；代码高亮不受影响。'),
              FilledButton(
                onPressed: _busy || !result.configured
                    ? null
                    : () => _load(ResourceCodeOperation.enable),
                child: const Text('为此工作区启用代码导航'),
              ),
            ] else ...[
              Row(
                children: [
                  const Expanded(child: Text('已启用 · 按需启动')),
                  TextButton(
                    onPressed: _busy
                        ? null
                        : () => _load(ResourceCodeOperation.disable),
                    child: const Text('关闭并释放'),
                  ),
                ],
              ),
              if (widget.symbol != '代码导航')
                Wrap(
                  spacing: 8,
                  children: [
                    for (final item in [
                      (ResourceCodeOperation.definition, '定义'),
                      (ResourceCodeOperation.references, '引用'),
                      (ResourceCodeOperation.implementation, '实现'),
                    ])
                      ChoiceChip(
                        label: Text(item.$2),
                        selected: _operation == item.$1,
                        onSelected: _busy ? null : (_) => _load(item.$1),
                      ),
                  ],
                ),
              if (!_busy &&
                  result.matches.isEmpty &&
                  widget.symbol != '代码导航' &&
                  _error == null)
                const Padding(
                  padding: EdgeInsets.all(12),
                  child: Text('未找到目标。'),
                ),
              if (result.truncated) const Text('结果已截断，请缩小范围。'),
            ],
          ],
          Expanded(
            child: ListView(
              children: [
                for (final target in _result?.matches ?? <ResourceCodeTarget>[])
                  ListTile(
                    title: Text('${target.label}:${target.line + 1}'),
                    subtitle: Text(
                      '第 ${target.line + 1} 行 · 第 ${target.character + 1} 列',
                    ),
                    trailing: const Icon(Icons.arrow_forward),
                    onTap: _busy ? null : () => Navigator.pop(context, target),
                  ),
              ],
            ),
          ),
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('返回源码'),
          ),
        ],
      ),
    ),
  );
}
