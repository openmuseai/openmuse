import 'package:flutter/material.dart';

import 'word_product_definition.dart';

/// Resolves GenOffice's absolute and relative page syntax to a one-based page.
/// Invalid syntax returns null; valid out-of-range numbers clamp to the book.
int? resolveWordGoToPage(String input, int current, int total, {int dir = 1}) {
  if (total <= 0) return null;
  final value = input.trim();
  if (value.isEmpty) return (current + dir).clamp(1, total);
  final relative = RegExp(r'^([+-])(\d+)$').firstMatch(value);
  if (relative != null) {
    final magnitude = int.tryParse(relative.group(2)!);
    if (magnitude == null) return null;
    final delta = relative.group(1) == '+' ? magnitude : -magnitude;
    return (current + delta).clamp(1, total);
  }
  if (!RegExp(r'^\d+$').hasMatch(value)) return null;
  final absolute = int.tryParse(value);
  return absolute?.clamp(1, total);
}

/// A native Flutter Go To page panel. Its copy comes from the signed product,
/// while page movement is a view request on the editor controller.
class WordGoToPanel extends StatefulWidget {
  const WordGoToPanel({
    super.key,
    required this.definition,
    required this.currentPage,
    required this.pageCount,
    required this.onNavigate,
    required this.onClose,
  });

  final WordGoToDefinition definition;
  final int currentPage;
  final int pageCount;
  final ValueChanged<int> onNavigate;
  final VoidCallback onClose;

  @override
  State<WordGoToPanel> createState() => _WordGoToPanelState();
}

class _WordGoToPanelState extends State<WordGoToPanel> {
  final _input = TextEditingController();
  bool _miss = false;

  @override
  void dispose() {
    _input.dispose();
    super.dispose();
  }

  void _run(int direction, {bool explicit = false}) {
    final page = resolveWordGoToPage(
      explicit ? _input.text : '',
      widget.currentPage,
      widget.pageCount,
      dir: direction,
    );
    if (page == null) {
      setState(() => _miss = true);
      return;
    }
    setState(() => _miss = false);
    widget.onNavigate(page);
  }

  @override
  Widget build(BuildContext context) {
    final d = widget.definition;
    final border = Colors.grey.shade400;
    return Material(
      color: Colors.white,
      elevation: 0,
      borderRadius: BorderRadius.circular(6),
      child: Container(
        key: const Key('word-go-to-panel'),
        width: 360,
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
        decoration: BoxDecoration(
          color: Colors.white,
          border: Border.all(color: border),
          borderRadius: BorderRadius.circular(6),
          boxShadow: const [
            BoxShadow(
              color: Color(0x26000000),
              blurRadius: 8,
              offset: Offset(0, 3),
            ),
          ],
        ),
        child: Column(mainAxisSize: MainAxisSize.min, children: [
          Row(children: [
            _tab(d.find, enabled: false),
            _tab(d.replace, enabled: false),
            _tab(d.title, enabled: true),
            const Spacer(),
            IconButton(
              key: const Key('word-go-to-close'),
              tooltip: d.close,
              icon: const Icon(Icons.close, size: 14),
              visualDensity: VisualDensity.compact,
              onPressed: widget.onClose,
            ),
          ]),
          const SizedBox(height: 6),
          Row(children: [
            Container(
              height: 28,
              width: 96,
              alignment: Alignment.centerLeft,
              padding: const EdgeInsets.symmetric(horizontal: 8),
              decoration: BoxDecoration(
                border: Border.all(color: border),
                borderRadius: BorderRadius.circular(4),
              ),
              child: Text('${d.page}  ▾', style: const TextStyle(fontSize: 12)),
            ),
            const SizedBox(width: 6),
            Expanded(
              child: SizedBox(
                height: 28,
                child: TextField(
                  key: const Key('word-go-to-input'),
                  controller: _input,
                  autofocus: true,
                  style: const TextStyle(fontSize: 13),
                  decoration: InputDecoration(
                    hintText: d.numberHint,
                    hintStyle: const TextStyle(fontSize: 12),
                    contentPadding: const EdgeInsets.symmetric(horizontal: 8),
                    border: OutlineInputBorder(
                        borderRadius: BorderRadius.circular(4)),
                    focusedBorder: const OutlineInputBorder(
                        borderSide: BorderSide(color: Color(0xFF185ABD))),
                  ),
                  onChanged: (_) {
                    if (_miss) setState(() => _miss = false);
                  },
                  onSubmitted: (_) => _run(1, explicit: true),
                ),
              ),
            ),
          ]),
          const SizedBox(height: 6),
          Row(children: [
            Flexible(
              child: Text(
                _miss ? d.notFound : d.relativeHint,
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
                style: TextStyle(
                    fontSize: 11, color: _miss ? Colors.red : Colors.grey[700]),
              ),
            ),
            const Spacer(),
            _action(d.previous, () => _run(-1)),
            const SizedBox(width: 4),
            _action(d.next, () => _run(1)),
            const SizedBox(width: 4),
            _action(d.title, () => _run(1, explicit: true), primary: true),
          ]),
        ]),
      ),
    );
  }

  Widget _tab(String label, {required bool enabled}) => Container(
        padding: const EdgeInsets.fromLTRB(8, 2, 8, 4),
        decoration: BoxDecoration(
          border: Border(
            bottom: BorderSide(
                color: enabled ? const Color(0xFF185ABD) : Colors.transparent,
                width: 2),
          ),
        ),
        child: Text(label,
            style: TextStyle(
                fontSize: 12,
                color: enabled ? const Color(0xFF232425) : Colors.grey)),
      );

  Widget _action(String label, VoidCallback onPressed,
          {bool primary = false}) =>
      SizedBox(
        height: 26,
        child: OutlinedButton(
          onPressed: onPressed,
          style: OutlinedButton.styleFrom(
            padding: const EdgeInsets.symmetric(horizontal: 9),
            backgroundColor: primary ? const Color(0xFFEAF1FC) : Colors.white,
            shape:
                RoundedRectangleBorder(borderRadius: BorderRadius.circular(4)),
          ),
          child: Text(label, style: const TextStyle(fontSize: 12)),
        ),
      );
}
