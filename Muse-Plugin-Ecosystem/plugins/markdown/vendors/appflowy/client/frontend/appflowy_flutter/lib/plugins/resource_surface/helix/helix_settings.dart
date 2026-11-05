import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:appflowy/plugins/resource_surface/helix/helix_commands.dart';
import 'package:appflowy/plugins/resource_surface/helix/helix_install.dart';
import 'package:appflowy/plugins/resource_surface/helix/helix_language_servers.dart';
import 'package:appflowy/core/config/kv.dart';
import 'package:appflowy/core/config/kv_keys.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:flutter/material.dart';
import 'package:path/path.dart' as p;
import 'package:xterm/xterm.dart';

enum HelixKeymapPreset { helix, vscode }

final class HelixSettings {
  const HelixSettings({
    required this.theme,
    required this.keymap,
    required this.fontFamily,
    required this.fontSize,
    required this.backgroundColor,
    required this.enableLsp,
  });

  static const defaults = HelixSettings(
    theme: 'onedark',
    keymap: HelixKeymapPreset.vscode,
    fontFamily: 'Menlo',
    fontSize: 13,
    backgroundColor: Color(0xFF282C34),
    enableLsp: true,
  );

  static const themes = <String>[
    'onedark',
    'onelight',
    'catppuccin_mocha',
    'github_dark',
    'github_light',
    'tokyonight',
    'monokai',
    'jetbrains_dark',
    'gruvbox',
    'dracula',
    'solarized_dark',
    'solarized_light',
  ];

  static const fonts = <String>[
    'Menlo',
    'Monaco',
    'JetBrains Mono',
    'Fira Code',
    'SF Mono',
    'Courier New',
    'monospace',
  ];

  static const themeBackgrounds = <String, Color>{
    'onedark': Color(0xFF282C34),
    'onelight': Color(0xFFFAFAFA),
    'catppuccin_mocha': Color(0xFF1E1E2E),
    'github_dark': Color(0xFF0D1117),
    'github_light': Color(0xFFFFFFFF),
    'tokyonight': Color(0xFF1A1B26),
    'monokai': Color(0xFF272822),
    'jetbrains_dark': Color(0xFF2B2B2B),
    'gruvbox': Color(0xFF282828),
    'dracula': Color(0xFF282A36),
    'solarized_dark': Color(0xFF002B36),
    'solarized_light': Color(0xFFFDF6E3),
  };

  final String theme;
  final HelixKeymapPreset keymap;
  final String fontFamily;
  final double fontSize;
  final Color backgroundColor;
  final bool enableLsp;

  bool get isLight => switch (theme) {
        'onelight' || 'github_light' || 'solarized_light' => true,
        _ => false,
      };

  HelixSettings copyWith({
    String? theme,
    HelixKeymapPreset? keymap,
    String? fontFamily,
    double? fontSize,
    Color? backgroundColor,
    bool? enableLsp,
  }) =>
      HelixSettings(
        theme: theme ?? this.theme,
        keymap: keymap ?? this.keymap,
        fontFamily: fontFamily ?? this.fontFamily,
        fontSize: fontSize ?? this.fontSize,
        backgroundColor: backgroundColor ?? this.backgroundColor,
        enableLsp: enableLsp ?? this.enableLsp,
      );

  Map<String, Object> toJson() => {
        'theme': theme,
        'keymap': keymap.name,
        'fontFamily': fontFamily,
        'fontSize': fontSize,
        'background': _hexArgb(backgroundColor),
        'enableLsp': enableLsp,
      };

  static HelixSettings fromJson(Map<String, dynamic> json) {
    final theme = json['theme'] as String? ?? defaults.theme;
    final storedBg = json['background'] as String?;
    return HelixSettings(
      theme: theme,
      keymap: HelixKeymapPreset.values.firstWhere(
        (value) => value.name == json['keymap'],
        orElse: () => defaults.keymap,
      ),
      fontFamily: json['fontFamily'] as String? ?? defaults.fontFamily,
      fontSize: (json['fontSize'] as num?)?.toDouble() ?? defaults.fontSize,
      backgroundColor: storedBg == null
          ? (themeBackgrounds[theme] ?? defaults.backgroundColor)
          : Color(int.parse(storedBg.replaceFirst('#', ''), radix: 16)),
      enableLsp: json['enableLsp'] as bool? ?? defaults.enableLsp,
    );
  }

  String get configToml {
    final keymapBlock = switch (keymap) {
      HelixKeymapPreset.vscode => _kVscodeKeymap,
      HelixKeymapPreset.helix => _kHelixInsertKeymap,
    };
    return '''
theme = "openmuse_host"

[editor]
mouse = true
cursorline = true
bufferline = "never"
color-modes = true
idle-timeout = 5
true-color = true
line-number = "absolute"

[editor.cursor-shape]
insert = "bar"
normal = "block"
select = "underline"

[editor.lsp]
enable = $enableLsp
display-messages = true
display-inlay-hints = $enableLsp

$keymapBlock
''';
  }

  String get overlayThemeToml {
    final bg = _hexRgb(backgroundColor);
    final fg = isLight ? '#1F2328' : '#ABB2BF';
    return '''
inherits = "$theme"
"ui.background" = { bg = "$bg" }
"ui.text" = { fg = "$fg" }
''';
  }

  TerminalTheme get terminalTheme {
    final bg = backgroundColor;
    final fg = isLight ? const Color(0xFF1F2328) : const Color(0xFFABB2BF);
    return TerminalTheme(
      cursor: isLight ? const Color(0xFF1F2328) : const Color(0xFFAEAFAD),
      selection: const Color(0x66888888),
      foreground: fg,
      background: bg,
      black: const Color(0xFF1E1E1E),
      red: const Color(0xFFE06C75),
      green: const Color(0xFF98C379),
      yellow: const Color(0xFFE5C07B),
      blue: const Color(0xFF61AFEF),
      magenta: const Color(0xFFC678DD),
      cyan: const Color(0xFF56B6C2),
      white: const Color(0xFFABB2BF),
      brightBlack: const Color(0xFF5C6370),
      brightRed: const Color(0xFFE06C75),
      brightGreen: const Color(0xFF98C379),
      brightYellow: const Color(0xFFE5C07B),
      brightBlue: const Color(0xFF61AFEF),
      brightMagenta: const Color(0xFFC678DD),
      brightCyan: const Color(0xFF56B6C2),
      brightWhite: const Color(0xFFFFFFFF),
      searchHitBackground: const Color(0xFFFFFF2B),
      searchHitBackgroundCurrent: const Color(0xFF31FF26),
      searchHitForeground: const Color(0xFF000000),
    );
  }

  TerminalStyle get terminalStyle => TerminalStyle(
        fontSize: fontSize,
        fontFamily: fontFamily,
      );
}

const _kHelixInsertKeymap = '''
[keys.insert]
C-s = ":write"
C-z = "undo"
C-y = "redo"
$helixHostNavigationKeymap

[keys.normal]
$helixHostNavigationKeymap

[keys.select]
$helixHostNavigationKeymap
''';

const _kVscodeKeymap = '''
[keys.insert]
C-s = ":write"
C-z = "undo"
C-y = "redo"
"C-left" = "move_prev_word_start"
"C-right" = "move_next_word_end"
$helixHostNavigationKeymap

[keys.normal]
$helixHostNavigationKeymap

[keys.select]
$helixHostNavigationKeymap
''';

final class HelixSettingsController extends ChangeNotifier {
  HelixSettingsController(this.installer) {
    installer.addListener(notifyListeners);
  }

  final HelixLanguageServerInstaller installer;
  HelixSettings settings = HelixSettings.defaults;
  bool loaded = false;
  String? grammarStatus;

  @override
  void dispose() {
    installer.removeListener(notifyListeners);
    super.dispose();
  }

  void setGrammarStatus(String status) {
    grammarStatus = status;
    notifyListeners();
  }

  Future<void> ensureLoaded() async {
    if (loaded) return;
    final raw = await getIt<KeyValueStorage>().get(KVKeys.helixSettings);
    if (raw != null && raw.isNotEmpty) {
      try {
        settings = HelixSettings.fromJson(
          jsonDecode(raw) as Map<String, dynamic>,
        );
      } on Object {
        settings = HelixSettings.defaults;
      }
    }
    loaded = true;
    notifyListeners();
    unawaited(installer.refresh());
  }

  Future<void> update(HelixSettings next) async {
    settings = next;
    notifyListeners();
    await getIt<KeyValueStorage>().set(
      KVKeys.helixSettings,
      jsonEncode(next.toJson()),
    );
  }

  Future<void> reset() => update(HelixSettings.defaults);

  Future<void> installHighlightGrammars() async {
    final install = await HelixInstall.resolve();
    grammarStatus = '正在编译语法高亮（首次需要 git 与 C 编译器）…';
    notifyListeners();
    try {
      await installer.buildGrammars(
        hxBinary: install.binary,
        helixRuntime: install.runtime,
      );
      final count = helixGrammarLibraryCount(
        installer.grammarRuntimes(install.runtime),
      );
      grammarStatus = count > 0
          ? '语法高亮已就绪（$count 个 tree-sitter grammars）'
          : '语法高亮未生成，请查看安装日志';
    } on Object catch (error) {
      grammarStatus = '$error';
      rethrow;
    } finally {
      notifyListeners();
    }
  }
}

String _hexRgb(Color color) {
  final r = (color.r * 255).round().clamp(0, 255).toRadixString(16).padLeft(2, '0');
  final g = (color.g * 255).round().clamp(0, 255).toRadixString(16).padLeft(2, '0');
  final b = (color.b * 255).round().clamp(0, 255).toRadixString(16).padLeft(2, '0');
  return '#$r$g$b';
}

String _hexArgb(Color color) {
  final a = (color.a * 255).round().clamp(0, 255).toRadixString(16).padLeft(2, '0');
  return '$a${_hexRgb(color).substring(1)}';
}

Directory helixProjectRoot(File file) {
  var dir = file.parent;
  const markers = [
    '.git',
    'pubspec.yaml',
    'Cargo.toml',
    'package.json',
    'go.mod',
    'pyproject.toml',
    '.helix',
  ];
  for (var i = 0; i < 16; i++) {
    for (final marker in markers) {
      final candidate = p.join(dir.path, marker);
      if (File(candidate).existsSync() || Directory(candidate).existsSync()) {
        return dir;
      }
    }
    if (dir.parent.path == dir.path) break;
    dir = dir.parent;
  }
  return file.parent;
}

int helixGrammarCount(String runtime) => helixGrammarLibraryCount([runtime]);

int helixGrammarLibraryCount(Iterable<String> runtimes) {
  final names = <String>{};
  for (final runtime in runtimes) {
    final dir = Directory(p.join(runtime, 'grammars'));
    if (!dir.existsSync()) continue;
    for (final entity in dir.listSync()) {
      if (entity is! File) continue;
      final ext = p.extension(entity.path);
      if (ext != '.so' && ext != '.dylib' && ext != '.dll') continue;
      names.add(p.basenameWithoutExtension(entity.path));
    }
  }
  return names.length;
}

bool helixHasGrammar(String name, Iterable<String> runtimes) {
  for (final runtime in runtimes) {
    for (final ext in const ['.dylib', '.so', '.dll']) {
      if (File(p.join(runtime, 'grammars', '$name$ext')).existsSync()) {
        return true;
      }
    }
  }
  return false;
}

String? helixGrammarNameForFile(String path) {
  return switch (p.extension(path).toLowerCase()) {
    '.dart' => 'dart',
    '.rs' => 'rust',
    '.py' => 'python',
    '.js' || '.mjs' || '.cjs' => 'javascript',
    '.ts' => 'typescript',
    '.tsx' => 'tsx',
    '.json' => 'json',
    '.toml' => 'toml',
    '.yaml' || '.yml' => 'yaml',
    '.md' => 'markdown',
    '.html' || '.htm' => 'html',
    '.css' => 'css',
    '.sh' || '.bash' => 'bash',
    '.go' => 'go',
    '.c' || '.h' => 'c',
    '.cc' || '.cpp' || '.hpp' || '.cxx' => 'cpp',
    _ => null,
  };
}

Future<int> copyHelixGrammars({
  required String destRuntime,
  List<String> extraSearchRuntimes = const [],
}) async {
  final existing = helixGrammarLibraryCount([destRuntime, ...extraSearchRuntimes]);
  if (existing > 0) return existing;
  final dest = Directory(p.join(destRuntime, 'grammars'));
  for (final source in [
    ...extraSearchRuntimes.map((path) => Directory(p.join(path, 'grammars'))),
    ..._grammarSearchRoots(),
  ]) {
    if (!source.existsSync()) continue;
    await dest.create(recursive: true);
    var copied = 0;
    for (final entity in source.listSync()) {
      if (entity is! File) continue;
      final ext = p.extension(entity.path);
      if (ext != '.so' && ext != '.dylib' && ext != '.dll') continue;
      await entity.copy(p.join(dest.path, p.basename(entity.path)));
      copied += 1;
    }
    if (copied > 0) return copied;
  }
  return 0;
}

List<Directory> _grammarSearchRoots() {
  final home = Platform.environment['HOME'] ?? '';
  return [
    Directory(p.join(home, '.config', 'helix', 'runtime', 'grammars')),
    Directory(p.join(home, '.local', 'share', 'helix', 'runtime', 'grammars')),
    Directory('/opt/homebrew/opt/helix/libexec/runtime/grammars'),
    Directory('/usr/local/opt/helix/libexec/runtime/grammars'),
  ];
}
