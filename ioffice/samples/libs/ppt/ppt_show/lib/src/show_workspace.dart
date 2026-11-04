import 'package:flutter/material.dart';
import 'package:ppt_render/ppt_render.dart';

import 'show_controller.dart';

const _kPptOrange = Color(0xFFC43E1C);
const _kRibbonBg = Color(0xFFF3F3F3);
const _kCanvasBg = Color(0xFF525252);

/// PowerPoint-inspired workspace: title + ribbon + fitted slide + status.
class PptShowWorkspace extends StatefulWidget {
  const PptShowWorkspace({
    super.key,
    required this.controller,
    this.onOpenDocument,
    this.onReloadBundled,
    this.onHelloFixture,
  });

  final PptShowController controller;
  final VoidCallback? onOpenDocument;
  final VoidCallback? onReloadBundled;
  final VoidCallback? onHelloFixture;

  @override
  State<PptShowWorkspace> createState() => _PptShowWorkspaceState();
}

class _PptShowWorkspaceState extends State<PptShowWorkspace> {
  PptShowController get c => widget.controller;

  @override
  void initState() {
    super.initState();
    c.addListener(_onCtl);
  }

  @override
  void dispose() {
    c.removeListener(_onCtl);
    super.dispose();
  }

  void _onCtl() {
    if (mounted) setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    return ColoredBox(
      color: Colors.white,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          _TitleBar(
            name: c.documentName,
            onOpen: widget.onOpenDocument,
            onReload: widget.onReloadBundled,
          ),
          _HomeRibbon(
            controller: c,
            onOpen: widget.onOpenDocument,
            onHello: widget.onHelloFixture,
          ),
          const Divider(height: 1, color: Color(0xFFD0D0D0)),
          Expanded(child: _SlideStage(controller: c)),
          _StatusBar(controller: c),
        ],
      ),
    );
  }
}

class _TitleBar extends StatelessWidget {
  const _TitleBar({
    required this.name,
    this.onOpen,
    this.onReload,
  });

  final String name;
  final VoidCallback? onOpen;
  final VoidCallback? onReload;

  @override
  Widget build(BuildContext context) {
    return ColoredBox(
      color: _kPptOrange,
      child: SizedBox(
        height: 36,
        child: Row(
          children: [
            const SizedBox(width: 12),
            const Text(
              'PowerPoint',
              style:
                  TextStyle(color: Colors.white, fontWeight: FontWeight.w600),
            ),
            const SizedBox(width: 16),
            Expanded(
              child: Text(
                name,
                overflow: TextOverflow.ellipsis,
                style: const TextStyle(color: Colors.white70, fontSize: 13),
              ),
            ),
            IconButton(
              tooltip: 'Open',
              onPressed: onOpen,
              icon:
                  const Icon(Icons.folder_open, color: Colors.white, size: 18),
            ),
            IconButton(
              tooltip: 'Reload bundled',
              onPressed: onReload,
              icon: const Icon(Icons.refresh, color: Colors.white, size: 18),
            ),
          ],
        ),
      ),
    );
  }
}

class _HomeRibbon extends StatelessWidget {
  const _HomeRibbon({
    required this.controller,
    this.onOpen,
    this.onHello,
  });

  final PptShowController controller;
  final VoidCallback? onOpen;
  final VoidCallback? onHello;

  @override
  Widget build(BuildContext context) {
    return ColoredBox(
      color: _kRibbonBg,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 6),
        child: Row(
          children: [
            TextButton(onPressed: onOpen, child: const Text('Open')),
            TextButton(onPressed: onHello, child: const Text('Hello fixture')),
            const SizedBox(width: 16),
            const Text('Zoom'),
            Slider(
              value: controller.zoom,
              min: 0.25,
              max: 2,
              onChanged: controller.setZoom,
            ),
          ],
        ),
      ),
    );
  }
}

class _SlideStage extends StatelessWidget {
  const _SlideStage({required this.controller});

  final PptShowController controller;

  @override
  Widget build(BuildContext context) {
    final env = controller.envelope;
    return ColoredBox(
      color: _kCanvasBg,
      child: env == null
          ? const Center(
              child: Text('No slide', style: TextStyle(color: Colors.white70)),
            )
          : Row(
              children: [
                SizedBox(
                  width: 112,
                  child: ColoredBox(
                    color: const Color(0xFFEEEEEE),
                    child: ListView.builder(
                      itemCount: env.pages.length,
                      itemBuilder: (context, index) => ListTile(
                        dense: true,
                        selected: index == controller.pageIndex,
                        leading: Text('${index + 1}'),
                        title: const Icon(Icons.slideshow, size: 28),
                        onTap: () => controller.setPage(index),
                      ),
                    ),
                  ),
                ),
                Expanded(
                  child: Center(
                    child: Builder(builder: (context) {
                      final page = controller.currentPage;
                      return FittedBox(
                        child: Transform.scale(
                          scale: controller.zoom,
                          child: SizedBox(
                            width: page.width,
                            height: page.height,
                            child: CustomPaint(
                              painter: RenderPainter(page.commands),
                              size: Size(page.width, page.height),
                            ),
                          ),
                        ),
                      );
                    }),
                  ),
                ),
              ],
            ),
    );
  }
}

class _StatusBar extends StatelessWidget {
  const _StatusBar({required this.controller});

  final PptShowController controller;

  @override
  Widget build(BuildContext context) {
    return ColoredBox(
      color: const Color(0xFFE8E8E8),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
        child: Row(
          children: [
            IconButton(
              visualDensity: VisualDensity.compact,
              onPressed:
                  controller.pageIndex > 0 ? controller.previousPage : null,
              icon: const Icon(Icons.chevron_left, size: 18),
            ),
            Text(
                'Page ${controller.envelope == null ? 0 : controller.pageIndex + 1}/${controller.envelope?.pageCount ?? 0}'),
            IconButton(
              visualDensity: VisualDensity.compact,
              onPressed: controller.envelope != null &&
                      controller.pageIndex + 1 <
                          controller.envelope!.pages.length
                  ? controller.nextPage
                  : null,
              icon: const Icon(Icons.chevron_right, size: 18),
            ),
            const SizedBox(width: 12),
            Expanded(
              child: Text(
                controller.status,
                overflow: TextOverflow.ellipsis,
                style: const TextStyle(fontSize: 12),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
