import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';

class RemoteSurfaceView extends StatelessWidget {
  const RemoteSurfaceView({
    super.key,
    required this.snapshot,
    required this.components,
    required this.drafts,
    required this.busy,
    required this.onField,
    required this.onInvoke,
    this.loadMedia,
  });

  final RemoteSurfaceSnapshot snapshot;
  final Iterable<String> components;
  final Map<String, Object?> drafts;
  final bool busy;
  final void Function(String fieldId, Object? value) onField;
  final void Function(RemoteSurfaceNode node) onInvoke;
  final Future<Uint8List?> Function(String handle)? loadMedia;

  @override
  Widget build(BuildContext context) {
    return ListView(
      padding: const EdgeInsets.all(16),
      children: [for (final node in snapshot.nodes) _block(context, node)],
    );
  }

  Widget _block(BuildContext context, RemoteSurfaceNode node) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 12),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          _leaf(context, node),
          for (final child in node.children)
            Padding(
              padding: const EdgeInsets.only(left: 12, top: 8),
              child: _block(context, child),
            ),
        ],
      ),
    );
  }

  Widget _leaf(BuildContext context, RemoteSurfaceNode node) {
    if (!remoteComponentSupported(node.type, components)) {
      return Text('不支持的组件', key: ValueKey('remote-unsupported-${node.nodeId}'));
    }
    final props = node.props;
    return switch (node.type) {
      'text' || 'markdown' => Text(props['text']! as String),
      'image' => _still(
        props['mediaHandle']! as String,
        props['alt'] as String? ?? '图片预览',
        node.nodeId,
      ),
      'gallery' => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          for (final item in props['items']! as List)
            Text((item as Map)['alt'] as String? ?? '图片预览'),
        ],
      ),
      'video-player' => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          _still(props['posterHandle']! as String, '视频封面', node.nodeId),
          Text('${(props['durationMs']! as int) ~/ 1000} 秒'),
        ],
      ),
      'document' || 'link' => Text(props['label']! as String),
      'list' || 'grid' => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          for (final item in props['items']! as List)
            Text((item as Map)['label']! as String),
        ],
      ),
      'form' => Column(
        children: [
          for (final field in props['fields']! as List)
            _field(context, (field as Map).cast<String, Object?>()),
        ],
      ),
      'stepper' => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          for (final step in props['steps']! as List)
            Text((step as Map)['label']! as String),
        ],
      ),
      'progress' => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(props['label']! as String),
          LinearProgressIndicator(value: (props['value']! as int) / 100),
        ],
      ),
      'status' => Text(props['label']! as String),
      'diff' || 'compare' => Text('${props['before']}\n${props['after']}'),
      'timeline-basic' => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          for (final clip in props['clips']! as List)
            Text((clip as Map)['label']! as String),
        ],
      ),
      'button' || 'confirmation' => _action(node),
      _ => const Text('不支持的组件'),
    };
  }

  Widget _still(String handle, String alt, String nodeId) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(alt),
        _MediaStill(handle: handle, nodeId: nodeId, loadMedia: loadMedia),
      ],
    );
  }

  Widget _field(BuildContext context, Map<String, Object?> field) {
    final id = field['id']! as String;
    final label = field['label']! as String;
    final kind = field['kind']! as String;
    final current = drafts[id] ?? field['value'];
    return switch (kind) {
      'switch' => SwitchListTile(
        key: ValueKey('remote-field-$id'),
        contentPadding: EdgeInsets.zero,
        title: Text(label),
        value: current == true,
        onChanged: busy ? null : (value) => onField(id, value),
      ),
      'choice' => DropdownButtonFormField<String>(
        key: ValueKey('remote-field-$id'),
        initialValue: current as String?,
        decoration: InputDecoration(labelText: label),
        items: [
          for (final option in field['options']! as List)
            DropdownMenuItem(value: option as String, child: Text(option)),
        ],
        onChanged: busy ? null : (value) => onField(id, value),
      ),
      _ => TextFormField(
        key: ValueKey('remote-field-$id'),
        initialValue: current as String? ?? '',
        decoration: InputDecoration(labelText: label),
        enabled: !busy,
        onChanged: (value) => onField(id, value),
      ),
    };
  }

  Widget _action(RemoteSurfaceNode node) {
    final actionId = node.props['actionId']! as String;
    final prompt = node.props['prompt'];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        if (prompt is String) Text(prompt),
        const SizedBox(height: 8),
        FilledButton(
          key: ValueKey('remote-action-$actionId'),
          onPressed: busy ? null : () => onInvoke(node),
          child: Text(node.props['label']! as String),
        ),
      ],
    );
  }
}

class _MediaStill extends StatefulWidget {
  const _MediaStill({
    required this.handle,
    required this.nodeId,
    required this.loadMedia,
  });

  final String handle;
  final String nodeId;
  final Future<Uint8List?> Function(String handle)? loadMedia;

  @override
  State<_MediaStill> createState() => _MediaStillState();
}

class _MediaStillState extends State<_MediaStill> {
  late final Future<Uint8List?> _bytes =
      widget.loadMedia?.call(widget.handle) ?? Future<Uint8List?>.value();

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<Uint8List?>(
      future: _bytes,
      builder: (context, snapshot) {
        final bytes = snapshot.data;
        if (bytes == null || bytes.isEmpty) {
          return Text('媒体不可用', key: ValueKey('remote-media-${widget.nodeId}'));
        }
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Image.memory(
              bytes,
              key: ValueKey('remote-media-${widget.nodeId}'),
              width: 48,
              height: 48,
              gaplessPlayback: true,
            ),
            Text('已读取 ${bytes.length} 字节'),
          ],
        );
      },
    );
  }
}
