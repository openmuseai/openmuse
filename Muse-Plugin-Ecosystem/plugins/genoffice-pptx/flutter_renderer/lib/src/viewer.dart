import 'dart:typed_data';

import 'package:flutter/material.dart';

import 'deck.dart';
import 'messages.g.dart';

/// Read-only native Flutter slide preview. The Host supplies bytes only after
/// resolving a user-selected resource and checking the installation receipt.
final class GenOfficePptxViewer extends StatefulWidget {
  const GenOfficePptxViewer({
    super.key,
    required this.bytes,
    required this.title,
    this.messages = const {},
  });
  final Uint8List bytes;
  final String title;

  /// Optional verified plugin translations, keyed by integration message name.
  final Map<String, String> messages;

  @override
  State<GenOfficePptxViewer> createState() => _GenOfficePptxViewerState();
}

final class _GenOfficePptxViewerState extends State<GenOfficePptxViewer> {
  late Future<PptxDeck> _opening;
  int _page = 0;

  @override
  void initState() {
    super.initState();
    _opening = Future(() => PptxDeck.fromBytes(widget.bytes));
  }

  @override
  void didUpdateWidget(GenOfficePptxViewer oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (!identical(oldWidget.bytes, widget.bytes)) {
      _page = 0;
      _opening = Future(() => PptxDeck.fromBytes(widget.bytes));
    }
  }

  @override
  Widget build(BuildContext context) => FutureBuilder<PptxDeck>(
    future: _opening,
    builder: (context, snapshot) {
      final locale = Localizations.maybeLocaleOf(context)?.toLanguageTag();
      final bundled =
          pptxMessages[pptxLocaleAliases[locale]] ??
          pptxMessages[pptxDefaultLocale]!;
      String text(String key) => widget.messages[key] ?? bundled[key]!;
      if (snapshot.hasError) return Center(child: Text(text('openFailed')));
      final deck = snapshot.data;
      if (deck == null) return const Center(child: CircularProgressIndicator());
      final page = _page.clamp(0, deck.slides.length - 1);
      return Column(
        children: [
          Expanded(
            child: Container(
              color: const Color(0xFF292929),
              child: Center(
                child: Padding(
                  padding: const EdgeInsets.all(12),
                  child: FittedBox(
                    child: SizedBox(
                      width: deck.width,
                      height: deck.height,
                      child: ColoredBox(
                        color: Colors.white,
                        child: Stack(
                          clipBehavior: Clip.hardEdge,
                          children: [
                            for (final shape in deck.slides[page].shapes)
                              Positioned.fromRect(
                                rect: shape.bounds,
                                child: _ShapeView(shape),
                              ),
                          ],
                        ),
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ),
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
            child: Row(
              children: [
                IconButton(
                  onPressed: page == 0 ? null : () => setState(() => _page--),
                  icon: const Icon(Icons.chevron_left),
                  tooltip: text('previousPage'),
                ),
                Text(
                  '${page + 1} / ${deck.slides.length}',
                  key: const ValueKey('pptx-page-counter'),
                ),
                IconButton(
                  onPressed: page + 1 == deck.slides.length
                      ? null
                      : () => setState(() => _page++),
                  icon: const Icon(Icons.chevron_right),
                  tooltip: text('nextPage'),
                ),
                Expanded(
                  child: Text(widget.title, overflow: TextOverflow.ellipsis),
                ),
              ],
            ),
          ),
        ],
      );
    },
  );
}

final class _ShapeView extends StatelessWidget {
  const _ShapeView(this.shape);
  final PptxShape shape;

  @override
  Widget build(BuildContext context) {
    if (shape.image case final image?) {
      return Image.memory(
        image,
        fit: BoxFit.fill,
        errorBuilder: (_, _, _) => const SizedBox.shrink(),
      );
    }
    return DecoratedBox(
      decoration: BoxDecoration(
        color: shape.fill,
        border: shape.stroke == null ? null : Border.all(color: shape.stroke!),
      ),
      child: shape.text.isEmpty
          ? const SizedBox.expand()
          : Padding(
              padding: const EdgeInsets.all(2),
              child: Text(
                shape.text,
                key: const ValueKey('pptx-shape-text'),
                maxLines: 30,
                overflow: TextOverflow.clip,
                style: TextStyle(
                  fontSize: shape.fontSize,
                  color: shape.textColor ?? Colors.black,
                  height: 1.1,
                ),
              ),
            ),
    );
  }
}
