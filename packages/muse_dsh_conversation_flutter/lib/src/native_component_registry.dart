import 'package:flutter/material.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';

typedef DshNativeCommandHandler =
    Future<void> Function(String command, JsonMap arguments);

final class DshNativeBindingContext {
  const DshNativeBindingContext(this.value);
  final JsonMap value;

  Object? resolve(Object? expression) {
    if (expression is! Map || expression['bind'] is! String) return expression;
    Object? current = value;
    for (final segment in (expression['bind'] as String).split('.')) {
      if (current is! Map || !current.containsKey(segment)) {
        return expression['fallback'];
      }
      current = current[segment];
    }
    return current;
  }
}

final class DshNativeComponentRegistry {
  const DshNativeComponentRegistry();

  DshNativeCapabilities get capabilities => DshNativeCapabilities.standard();

  Widget buildContribution(
    BuildContext context,
    DshNativeContribution contribution,
    DshNativeBindingContext binding, {
    DshNativeCommandHandler? onCommand,
  }) {
    final title = _string(binding.resolve(contribution.raw['title']));
    return Card.outlined(
      key: ValueKey('native-ui:${contribution.slot}:${contribution.key}'),
      margin: const EdgeInsets.only(left: 40, bottom: 12),
      child: Padding(
        padding: const EdgeInsets.all(14),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                const Icon(Icons.extension_outlined, size: 18),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    title.isEmpty ? contribution.key : title,
                    style: Theme.of(context).textTheme.titleSmall,
                  ),
                ),
              ],
            ),
            if (contribution.body.isNotEmpty) const SizedBox(height: 10),
            for (final node in contribution.body)
              if (_visible(node, binding))
                Padding(
                  padding: const EdgeInsets.only(bottom: 8),
                  child: buildNode(
                    context,
                    node,
                    binding,
                    onCommand: onCommand,
                  ),
                ),
            if (contribution.actions.isNotEmpty)
              Wrap(
                spacing: 8,
                runSpacing: 8,
                children: [
                  for (final action in contribution.actions)
                    OutlinedButton(
                      onPressed: onCommand == null
                          ? null
                          : () => onCommand(
                              _string(action['command']),
                              _boundArguments(action['arguments'], binding),
                            ),
                      child: Text(_string(action['label'])),
                    ),
                ],
              ),
          ],
        ),
      ),
    );
  }

  Widget buildNode(
    BuildContext context,
    JsonMap node,
    DshNativeBindingContext binding, {
    DshNativeCommandHandler? onCommand,
  }) {
    final component = node['component'];
    final text = _string(binding.resolve(node['text']));
    switch (component) {
      case 'text':
      case 'markdown':
        return SelectableText(text);
      case 'code':
        return DecoratedBox(
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.surfaceContainerHighest,
            borderRadius: BorderRadius.circular(8),
          ),
          child: Padding(
            padding: const EdgeInsets.all(10),
            child: SelectableText(
              text,
              style: const TextStyle(fontFamily: 'monospace'),
            ),
          ),
        );
      case 'badge':
        return Chip(
          visualDensity: VisualDensity.compact,
          label: Text(text),
          backgroundColor: _toneColor(context, node['tone'] as String?),
        );
      case 'status':
        return Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(
              Icons.circle,
              size: 9,
              color: _toneColor(context, node['tone'] as String?),
            ),
            const SizedBox(width: 7),
            Text(text),
          ],
        );
      case 'progress':
        final value = binding.resolve(node['value']);
        return LinearProgressIndicator(
          value: value is num ? value.toDouble().clamp(0, 1) : null,
          semanticsLabel: _string(binding.resolve(node['label'])),
        );
      case 'keyValue':
        return Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            SizedBox(
              width: 112,
              child: Text(
                _string(binding.resolve(node['label'])),
                style: TextStyle(
                  color: Theme.of(context).colorScheme.onSurfaceVariant,
                ),
              ),
            ),
            Expanded(
              child: SelectableText(_string(binding.resolve(node['value']))),
            ),
          ],
        );
      case 'button':
        final command = _string(node['command']);
        return OutlinedButton(
          onPressed: onCommand == null || command.isEmpty
              ? null
              : () => onCommand(
                  command,
                  _boundArguments(node['arguments'], binding),
                ),
          child: Text(_string(binding.resolve(node['label']))),
        );
      case 'disclosure':
        return ExpansionTile(
          tilePadding: EdgeInsets.zero,
          title: Text(_string(binding.resolve(node['label']))),
          children: _children(context, node, binding, onCommand),
        );
      case 'row':
        return Wrap(
          spacing: 8,
          runSpacing: 8,
          children: _children(context, node, binding, onCommand),
        );
      case 'column':
      case 'section':
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: _children(context, node, binding, onCommand),
        );
      default:
        return const SizedBox.shrink();
    }
  }

  List<Widget> _children(
    BuildContext context,
    JsonMap node,
    DshNativeBindingContext binding,
    DshNativeCommandHandler? onCommand,
  ) {
    final children = node['children'];
    if (children is! List) return const [];
    return children
        .whereType<Map>()
        .map((child) => child.cast<String, Object?>())
        .where((child) => _visible(child, binding))
        .map(
          (child) => buildNode(context, child, binding, onCommand: onCommand),
        )
        .toList(growable: false);
  }

  bool _visible(JsonMap node, DshNativeBindingContext binding) {
    final condition = node['visibleWhen'];
    if (condition is! Map) return true;
    return _condition(condition.cast<String, Object?>(), binding);
  }

  bool _condition(JsonMap condition, DshNativeBindingContext binding) {
    final all = condition['all'];
    if (all is List) {
      return all.whereType<Map>().every(
        (item) => _condition(item.cast<String, Object?>(), binding),
      );
    }
    final field = condition['field'];
    return field is String &&
        binding.resolve({'bind': field}) == condition['equals'];
  }

  JsonMap _boundArguments(Object? arguments, DshNativeBindingContext binding) {
    if (arguments is! Map) return const {};
    return arguments.map<String, Object?>((key, value) {
      return MapEntry(key.toString(), binding.resolve(value));
    });
  }

  String _string(Object? value) => value == null
      ? ''
      : value is String
      ? value
      : value.toString();

  Color? _toneColor(BuildContext context, String? tone) {
    final colors = Theme.of(context).colorScheme;
    return switch (tone) {
      'info' => colors.primaryContainer,
      'success' => Colors.green.withValues(alpha: 0.18),
      'warning' => Colors.orange.withValues(alpha: 0.2),
      'danger' => colors.errorContainer,
      _ => colors.surfaceContainerHighest,
    };
  }
}
