import 'dart:io';
import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:path/path.dart' as p;
import 'package:xterm/xterm.dart';

import 'helix_language_servers.dart';

enum HelixInputProfile {
  helixModal('helix-modal'),
  standardNonmodal('standard-nonmodal');

  const HelixInputProfile(this.id);
  final String id;

  static HelixInputProfile parse(Object? value) => values.firstWhere(
    (profile) => profile.id == value,
    orElse: () => helixModal,
  );
}

/// Plugin-owned, non-secret preferences. Host persists this JSON opaquely.
final class HelixPreferences {
  const HelixPreferences({
    this.theme = 'onelight',
    this.fontFamily = 'Menlo',
    this.fontSize = 14,
    this.inputProfile = HelixInputProfile.helixModal,
    this.vscodeKeymap = false,
    this.enableLsp = true,
    this.languageServerPaths = const {},
    this.languageServerConfigPaths = const {},
  });

  final String theme;
  final String fontFamily;
  final double fontSize;
  final HelixInputProfile inputProfile;

  /// Legacy shortcut preference; this does not enable nonmodal editing.
  final bool vscodeKeymap;
  final bool enableLsp;
  final Map<String, String> languageServerPaths;
  final Map<String, String> languageServerConfigPaths;

  bool get dark => theme == 'openmuse_dark';

  static const themes = ['onelight', 'openmuse_dark'];
  static const fonts = [
    'Menlo',
    'Monaco',
    'SF Mono',
    'JetBrains Mono',
    'monospace',
  ];

  factory HelixPreferences.fromJson(Map value) => HelixPreferences(
    theme: themes.contains(value['theme'])
        ? value['theme'] as String
        : 'onelight',
    fontFamily: fonts.contains(value['fontFamily'])
        ? value['fontFamily'] as String
        : 'Menlo',
    fontSize: value['fontSize'] is num
        ? (value['fontSize'] as num).toDouble().clamp(11, 20)
        : 14,
    inputProfile: HelixInputProfile.parse(value['inputProfile']),
    vscodeKeymap: value['vscodeKeymap'] == true,
    enableLsp: value['enableLsp'] != false,
    languageServerPaths: value['languageServerPaths'] is Map
        ? {
            for (final entry in (value['languageServerPaths'] as Map).entries)
              if (entry.key is String && entry.value is String)
                entry.key as String: entry.value as String,
          }
        : const {},
    languageServerConfigPaths: value['languageServerConfigPaths'] is Map
        ? {
            for (final entry
                in (value['languageServerConfigPaths'] as Map).entries)
              if (entry.key is String && entry.value is String)
                entry.key as String: entry.value as String,
          }
        : const {},
  );

  Map<String, Object?> toJson() => {
    'theme': theme,
    'fontFamily': fontFamily,
    'fontSize': fontSize,
    'inputProfile': inputProfile.id,
    'vscodeKeymap': vscodeKeymap,
    'enableLsp': enableLsp,
    'languageServerPaths': languageServerPaths,
    'languageServerConfigPaths': languageServerConfigPaths,
  };

  HelixPreferences copyWith({
    String? theme,
    String? fontFamily,
    double? fontSize,
    HelixInputProfile? inputProfile,
    bool? vscodeKeymap,
    bool? enableLsp,
    Map<String, String>? languageServerPaths,
    Map<String, String>? languageServerConfigPaths,
  }) => HelixPreferences(
    theme: theme ?? this.theme,
    fontFamily: fontFamily ?? this.fontFamily,
    fontSize: fontSize ?? this.fontSize,
    inputProfile: inputProfile ?? this.inputProfile,
    vscodeKeymap: vscodeKeymap ?? this.vscodeKeymap,
    enableLsp: enableLsp ?? this.enableLsp,
    languageServerPaths: languageServerPaths ?? this.languageServerPaths,
    languageServerConfigPaths:
        languageServerConfigPaths ?? this.languageServerConfigPaths,
  );

  String get languagesToml => languagesTomlFor();

  /// [rustLinkedProjects] lists Cargo manifests handed to rust-analyzer through
  /// `linkedProjects`, so navigation also works for a Rust buffer that lives
  /// outside every crate. An explicit configuration file for the server always
  /// wins over this generated block.
  String languagesTomlFor({List<String> rustLinkedProjects = const []}) {
    final allowed = {
      for (final spec in helixLanguageServers) spec.command: spec.arguments,
    };
    final blocks = <String>[];
    for (final spec in helixLanguageServers) {
      final executable = languageServerPaths[spec.command];
      if (executable != null &&
          p.isAbsolute(executable) &&
          File(executable).existsSync()) {
        blocks.add(
          '[language-server.${spec.command}]\n'
          'command = ${jsonEncode(executable)}\n'
          'args = ${jsonEncode(allowed[spec.command])}',
        );
      }
      var userConfigWritten = false;
      final configPath = languageServerConfigPaths[spec.command];
      if (configPath != null &&
          p.isAbsolute(configPath) &&
          File(configPath).existsSync()) {
        final raw = File(configPath).readAsStringSync().trim();
        if (raw.isEmpty) continue;
        if (p.extension(configPath).toLowerCase() == '.json') {
          final decoded = jsonDecode(raw);
          if (decoded is! Map) continue;
          final values = <String>[];
          void writeMap(Map map, String prefix) {
            for (final entry in map.entries) {
              if (entry.key is! String) continue;
              final key = prefix.isEmpty
                  ? entry.key as String
                  : '$prefix.${entry.key}';
              if (entry.value is Map) {
                writeMap(entry.value as Map, key);
              } else if (entry.value != null) {
                final tomlKey = key.split('.').map(jsonEncode).join('.');
                values.add('$tomlKey = ${jsonEncode(entry.value)}');
              }
            }
          }

          writeMap(decoded, '');
          blocks.add(
            '[language-server.${spec.command}.config]\n${values.join('\n')}',
          );
          userConfigWritten = true;
        } else {
          blocks.add('[language-server.${spec.command}.config]\n$raw');
          userConfigWritten = true;
        }
      }
      if (!userConfigWritten &&
          spec.command == 'rust-analyzer' &&
          rustLinkedProjects.isNotEmpty) {
        blocks.add(
          '[language-server.rust-analyzer.config]\n'
          'linkedProjects = ${jsonEncode(rustLinkedProjects)}',
        );
      }
    }
    return '${blocks.join('\n\n')}\n';
  }

  String get configToml =>
      '''
theme = "$theme"

[editor]
${inputProfile == HelixInputProfile.standardNonmodal ? 'input-profile = "standard-nonmodal"' : ''}
line-number = "absolute"
mouse = true
cursorline = true
true-color = true

[editor.lsp]
enable = $enableLsp
display-messages = true
display-inlay-hints = $enableLsp

[keys.insert]
${inputProfile == HelixInputProfile.standardNonmodal ? '' : 'C-s = ":write"'}
${inputProfile == HelixInputProfile.standardNonmodal
          ? 'C-z = "undo"\nC-y = "redo"\nC-f = "search"\nC-a = "select_all"\n"C-left" = "move_prev_word_start"\n"C-right" = "move_next_word_end"'
          : vscodeKeymap
          ? 'C-z = "undo"\nC-y = "redo"\n"C-left" = "move_prev_word_start"\n"C-right" = "move_next_word_end"'
          : ''}
$_hostNavigationKeys

[keys.normal]
C-s = ":write"
$_hostNavigationKeys

[keys.select]
$_hostNavigationKeys
''';

  TerminalStyle get terminalStyle =>
      TerminalStyle(fontSize: fontSize, height: 1.22, fontFamily: fontFamily);

  TerminalTheme get terminalTheme {
    final background = dark ? const Color(0xff20242c) : Colors.white;
    final foreground = dark ? const Color(0xffe5e7ee) : const Color(0xff24272d);
    return TerminalTheme(
      cursor: foreground,
      selection: dark ? const Color(0xff3e526d) : const Color(0xffdce7f8),
      foreground: foreground,
      background: background,
      black: const Color(0xff24272d),
      red: const Color(0xffbd4242),
      green: const Color(0xff399a58),
      yellow: const Color(0xffb58b35),
      blue: const Color(0xff4e87d0),
      magenta: const Color(0xff9c67ba),
      cyan: const Color(0xff278da3),
      white: const Color(0xffe5e7ee),
      brightBlack: const Color(0xff73777d),
      brightRed: const Color(0xffe06c75),
      brightGreen: const Color(0xff98c379),
      brightYellow: const Color(0xffe5c07b),
      brightBlue: const Color(0xff61afef),
      brightMagenta: const Color(0xffc678dd),
      brightCyan: const Color(0xff56b6c2),
      brightWhite: Colors.white,
      searchHitBackground: const Color(0xffffe6a1),
      searchHitBackgroundCurrent: const Color(0xffffc65e),
      searchHitForeground: const Color(0xff24272d),
    );
  }

  static int bundledGrammarCount(String executable) {
    final directory = Directory(
      '${File(executable).parent.path}/runtime/grammars',
    );
    if (!directory.existsSync()) return 0;
    return directory
        .listSync()
        .whereType<File>()
        .where(
          (file) =>
              file.path.endsWith('.dylib') ||
              file.path.endsWith('.so') ||
              file.path.endsWith('.dll'),
        )
        .length;
  }
}

// Host navigation shortcuts are delivered as CSI/control sequences by xterm.
const _hostNavigationKeys = '''
F12 = "goto_definition"
S-F12 = "goto_reference"
C-F12 = "goto_type_definition"
A-F12 = "goto_implementation"
F2 = "rename_symbol"
C-o = "jump_backward"
F6 = "jump_forward"
"A-left" = "jump_backward"
"A-right" = "jump_forward"
''';
