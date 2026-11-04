import 'dart:convert';

class RenderPage {
  const RenderPage({
    required this.index,
    required this.width,
    required this.height,
    required this.commands,
  });

  final int index;
  final double width;
  final double height;
  final List<Map<String, dynamic>> commands;

  factory RenderPage.fromJson(Map<String, dynamic> value) => RenderPage(
        index: (value['index'] as num?)?.toInt() ?? 0,
        width: (value['width'] as num?)?.toDouble() ?? 960,
        height: (value['height'] as num?)?.toDouble() ?? 540,
        commands: _commands(value['commands']),
      );
}

List<Map<String, dynamic>> _commands(Object? value) {
  final result = <Map<String, dynamic>>[];
  if (value is List) {
    for (final item in value) {
      if (item is Map) result.add(Map<String, dynamic>.from(item));
    }
  }
  return result;
}

/// ppt-core document envelope. Legacy single-page JSON is promoted to one page.
class RenderEnvelope {
  RenderEnvelope({
    required this.ok,
    required this.pipeline,
    required this.width,
    required this.height,
    required this.commands,
    required this.pages,
    required this.pageCount,
    required this.selectedPage,
    this.error,
  });

  final bool ok;
  final String pipeline;
  final double width;
  final double height;
  final List<Map<String, dynamic>> commands;
  final List<RenderPage> pages;
  final int pageCount;
  final int selectedPage;
  final String? error;

  factory RenderEnvelope.fromJsonString(String raw) {
    final decoded = jsonDecode(raw);
    if (decoded is! Map) {
      throw FormatException('envelope is not a JSON object');
    }
    return RenderEnvelope.fromJson(Map<String, dynamic>.from(decoded));
  }

  factory RenderEnvelope.fromJson(Map<String, dynamic> m) {
    final width = (m['width'] as num?)?.toDouble() ?? 960;
    final height = (m['height'] as num?)?.toDouble() ?? 540;
    final cmds = _commands(m['commands']);
    final pages = <RenderPage>[];
    final rawPages = m['pages'];
    if (rawPages is List) {
      for (final item in rawPages) {
        if (item is Map) {
          pages.add(RenderPage.fromJson(Map<String, dynamic>.from(item)));
        }
      }
    }
    if (pages.isEmpty) {
      pages.add(
          RenderPage(index: 0, width: width, height: height, commands: cmds));
    }
    return RenderEnvelope(
      ok: m['ok'] != false,
      pipeline: m['pipeline']?.toString() ?? '',
      width: width,
      height: height,
      commands: cmds,
      pages: pages,
      pageCount: (m['page_count'] as num?)?.toInt() ?? pages.length,
      selectedPage: (m['selected_page'] as num?)?.toInt() ?? 0,
      error: m['error']?.toString(),
    );
  }

  RenderPage pageAt(int index) => pages[index.clamp(0, pages.length - 1)];
}
