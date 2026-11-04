import 'dart:io';
import 'dart:typed_data';

class HostFontFile {
  const HostFontFile(this.id, this.bytes);
  final String id;
  final Uint8List bytes;
}

/// Desktop Arial (or metric-compatible) candidates. Host-only — not used by word-core.
const kDesktopArialCandidates = <String>[
  '/System/Library/Fonts/Supplemental/Arial.ttf',
  '/Library/Fonts/Arial.ttf',
  r'C:\Windows\Fonts\arial.ttf',
  '/usr/share/fonts/truetype/msttcorefonts/Arial.ttf',
  '/usr/share/fonts/truetype/msttcorefonts/arial.ttf',
  '/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf',
];

/// CJK fallback glyphs. A document's 宋体 built-in metrics stay separate from
/// the substitute face loaded here when the platform has no 宋体 font file.
const kDesktopCjkCandidates = <String>[
  '/System/Library/Fonts/Supplemental/Arial Unicode.ttf',
  '/Library/Fonts/Arial Unicode.ttf',
  '/System/Library/Fonts/Hiragino Sans GB.ttc',
  '/System/Library/Fonts/Supplemental/Songti.ttc',
  '/System/Library/Fonts/STHeiti Light.ttc',
  r'C:\Windows\Fonts\simsun.ttc',
  r'C:\Windows\Fonts\msyh.ttc',
  '/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc',
  '/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc',
];

/// Files supplied by Android itself. Inject their bytes into the Rust
/// FontHost just like the desktop sources; do not ask Flutter to measure runs.
// Android Java's built-in serif header resolves through the device default
// typeface on PKM110. Read that installed face before the platform Roboto
// fallback so Rust layout and Flutter painting use the same glyph advances.
const kAndroidLatinCandidates = <String>[
  '/system/fonts/SysFont-Regular.ttf',
  '/system/fonts/Roboto-Regular.ttf',
];
// Java Paint follows the device's zh-Hans fallback in fonts.xml. On PKM110
// that face is SysSans-Hans; the vendor also exposes identical bytes under
// SysFont-Hans. Keep AOSP Noto as the fallback on other Android devices.
const kAndroidCjkCandidates = <String>[
  '/system/fonts/SysSans-Hans-Regular.ttf',
  '/system/fonts/SysFont-Hans-Regular.ttf',
  '/system/fonts/NotoSansCJK-Regular.ttc',
];

bool forceBundledFont() {
  final env = Platform.environment;
  if (env['WORD_CORE_FORCE_BUNDLED_FONT'] == '1') return true;
  if (env['WORD_CORE_ALLOW_HOST_FONTS_IN_TEST'] == '1') return false;
  if (env.containsKey('FLUTTER_TEST')) return true;
  return false;
}

Uint8List? _readIfExists(String path) {
  final f = File(path);
  if (!f.existsSync()) return null;
  return f.readAsBytesSync();
}

String _macProductFont(String name) =>
    '${File(Platform.resolvedExecutable).parent.parent.path}/Resources/WordFonts/$name';

/// Latin default (Arial / WORD_CORE_FONT).
Uint8List? tryLoadHostFontBytes() {
  final all = tryLoadHostFonts();
  if (all.isEmpty) return null;
  return all.first.bytes;
}

/// FontHost injection list: default latin, optional CJK glyph fallback.
/// CJK is not loaded on Hello boot — 22MB Arial Unicode via FFI + loadFontFromList
/// at startup is unnecessary; pass [includeCjk] when the doc has eastAsia text.
List<HostFontFile> tryLoadHostFonts({bool includeCjk = false}) {
  if (forceBundledFont()) return const [];

  final out = <HostFontFile>[];

  Uint8List? latin;
  final specified = Platform.environment['WORD_CORE_FONT'];
  if (specified != null && specified.isNotEmpty) {
    latin = _readIfExists(specified);
  }
  if (latin == null) {
    for (final p in Platform.isAndroid
        ? kAndroidLatinCandidates
        : Platform.isMacOS
            ? [_macProductFont('Roboto-Regular.ttf'), ...kDesktopArialCandidates]
            : kDesktopArialCandidates) {
      latin = _readIfExists(p);
      if (latin != null) break;
    }
  }
  if (latin != null) {
    out.add(HostFontFile('default', latin));
  }

  if (!includeCjk) return out;

  Uint8List? cjk;
  final cjkPath = Platform.environment['WORD_CORE_CJK_FONT'];
  if (cjkPath != null && cjkPath.isNotEmpty) {
    cjk = _readIfExists(cjkPath);
  }
  if (cjk == null) {
    for (final p in Platform.isAndroid
        ? kAndroidCjkCandidates
        : Platform.isMacOS
            ? [
                _macProductFont('NotoSansCJK-Regular.ttc'),
                ...kDesktopCjkCandidates
              ]
            : kDesktopCjkCandidates) {
      // Flutter loadFontFromList does not reliably accept TTC; skip for paint.
      if (!Platform.isAndroid &&
          p.toLowerCase().endsWith('.ttc') &&
          !p.contains('/WordFonts/')) continue;
      cjk = _readIfExists(p);
      if (cjk != null) break;
    }
  }
  if (cjk != null) {
    final sameAsLatin = out.isNotEmpty && out.first.bytes.length == cjk.length;
    if (!sameAsLatin) {
      out.add(HostFontFile('CJKFallback', cjk));
    }
  }

  return out;
}
