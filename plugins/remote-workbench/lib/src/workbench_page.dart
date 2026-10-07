import 'dart:async';

import 'package:flutter/material.dart';
import 'package:muse_remote_surface_core/muse_remote_surface_core.dart';

import 'surface_view.dart';

class RemoteWorkbenchPage extends StatefulWidget {
  const RemoteWorkbenchPage({super.key, required this.controller});

  final RemoteWorkbenchController controller;

  @override
  State<RemoteWorkbenchPage> createState() => _RemoteWorkbenchPageState();
}

class _RemoteWorkbenchPageState extends State<RemoteWorkbenchPage> {
  @override
  void initState() {
    super.initState();
    widget.controller.addListener(_changed);
    unawaited(widget.controller.start());
  }

  @override
  void didUpdateWidget(covariant RemoteWorkbenchPage oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.controller == widget.controller) return;
    oldWidget.controller.removeListener(_changed);
    widget.controller.addListener(_changed);
  }

  @override
  void dispose() {
    widget.controller.removeListener(_changed);
    super.dispose();
  }

  void _changed() {
    if (mounted) setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    final controller = widget.controller;
    final snapshot = controller.snapshot;
    return Scaffold(
      appBar: AppBar(
        title: const Text('远程工作台'),
        actions: [
          if (snapshot != null)
            TextButton(
              key: const ValueKey('remote-surface-list'),
              onPressed: controller.closeSurface,
              child: const Text('全部界面'),
            ),
        ],
      ),
      body: Column(
        children: [
          if (controller.notice != null)
            Padding(
              padding: const EdgeInsets.fromLTRB(16, 12, 16, 0),
              child: Text(
                controller.notice!,
                key: const ValueKey('remote-lab-notice'),
              ),
            ),
          if (controller.error != null)
            Padding(
              padding: const EdgeInsets.all(12),
              child: Text(
                controller.error!,
                key: const ValueKey('remote-error'),
              ),
            ),
          if (controller.jobState != null)
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 12),
              child: Text(
                '任务状态 ${controller.jobState}',
                key: const ValueKey('remote-job'),
              ),
            ),
          Expanded(
            child: snapshot != null
                ? RemoteSurfaceView(
                    snapshot: snapshot,
                    components: controller.hello.components,
                    drafts: controller.drafts,
                    busy: controller.busy,
                    onField: controller.setField,
                    onInvoke: (node) => unawaited(controller.invoke(node)),
                    loadMedia: controller.loadMedia,
                  )
                : _empty(controller),
          ),
        ],
      ),
    );
  }

  Widget _empty(RemoteWorkbenchController controller) {
    final offers = controller.offers;
    if (offers.isEmpty) {
      return const Center(child: Text('当前没有可用的远程界面'));
    }
    return ListView(
      children: [
        for (final command in controller.commands)
          ListTile(
            key: ValueKey('remote-command-${command.id}'),
            title: Text(command.label),
            onTap: () => unawaited(command.invoke()),
          ),
        for (final offer in offers)
          ListTile(
            key: ValueKey(
              'remote-surface-${offer.descriptor.pluginId}-${offer.descriptor.surfaceId}',
            ),
            enabled: offer.compatible,
            title: Text(offer.descriptor.title),
            subtitle: offer.compatible
                ? null
                : Text(offer.unsupportedReason ?? '不支持'),
            onTap: offer.compatible
                ? () => unawaited(controller.openOffer(offer))
                : null,
          ),
      ],
    );
  }
}

class RemoteWorkbenchUnavailable extends StatelessWidget {
  const RemoteWorkbenchUnavailable({super.key});

  @override
  Widget build(BuildContext context) {
    return const Center(child: Text('等待已配对的 Desktop'));
  }
}
