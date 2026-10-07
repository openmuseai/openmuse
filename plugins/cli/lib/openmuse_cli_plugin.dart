import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_pty/flutter_pty.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:xterm/xterm.dart';

final class OpenMuseCliPlugin implements OpenMusePlugin {
  String _workingDirectory = Directory.current.path;

  @override
  OpenMusePluginDescriptor get descriptor => const OpenMusePluginDescriptor(
    id: 'com.openmuse.cli',
    name: 'OpenMuse CLI',
    version: '0.1.0',
    runtime: OpenMusePluginRuntime.builtIn,
    activationEvents: ['onStartup'],
    permissions: {'workspace.context.read'},
    panels: [
      OpenMusePanelContribution(
        id: 'cli.console',
        region: OpenMuseSurfaceRegion.bottomPanel,
      ),
    ],
  );

  @override
  Future<void> activate(OpenMusePluginContext context) async {
    try {
      final snapshot = await context.executeHostCommand(
        'workspace.snapshot',
        null,
      );
      if (snapshot is Map && snapshot['activeMountPath'] is String) {
        final candidate = snapshot['activeMountPath'] as String;
        if (Directory(candidate).existsSync()) _workingDirectory = candidate;
      }
    } on Object {
      // The console remains usable when the workspace service is unavailable.
    }
  }

  @override
  Future<void> deactivate() async {}

  @override
  Widget buildEditor(BuildContext context, OpenMuseResource resource) =>
      const Center(child: Text('CLI 插件不编辑文件'));

  @override
  Widget? buildPanel(BuildContext context, String panelId) =>
      panelId == 'cli.console'
      ? OpenMuseCliConsole(workingDirectory: _workingDirectory)
      : null;
}

@immutable
final class OpenMuseShellProfile {
  const OpenMuseShellProfile(this.name, this.executable, this.arguments);

  final String name;
  final String executable;
  final List<String> arguments;
}

List<OpenMuseShellProfile> availableOpenMuseShells() {
  if (Platform.isWindows) {
    var hasPwsh = false;
    try {
      hasPwsh = Process.runSync('where.exe', ['pwsh.exe']).exitCode == 0;
    } on ProcessException {
      // Windows PowerShell remains available as the system fallback.
    }
    return [
      if (hasPwsh) const OpenMuseShellProfile('PowerShell 7', 'pwsh.exe', []),
      OpenMuseShellProfile('Windows PowerShell', 'powershell.exe', []),
    ];
  }
  if (Platform.isMacOS) {
    return const [
      OpenMuseShellProfile('zsh', '/bin/zsh', ['-l']),
      OpenMuseShellProfile('bash', '/bin/bash', ['-l']),
    ];
  }
  return const [
    OpenMuseShellProfile('bash', '/bin/bash', ['-l']),
  ];
}

final class OpenMuseCliConsole extends StatefulWidget {
  const OpenMuseCliConsole({super.key, required this.workingDirectory});

  final String workingDirectory;

  @override
  State<OpenMuseCliConsole> createState() => _OpenMuseCliConsoleState();
}

final class _ConsoleSession {
  _ConsoleSession(this.profile, this.pty) : terminal = Terminal(maxLines: 5000);

  final OpenMuseShellProfile profile;
  final Pty pty;
  final Terminal terminal;
  StreamSubscription<String>? output;
  int? exitCode;

  void close() {
    output?.cancel();
    pty.kill();
  }
}

final class _OpenMuseCliConsoleState extends State<OpenMuseCliConsole> {
  final List<_ConsoleSession> _sessions = [];
  late final List<OpenMuseShellProfile> _profiles = availableOpenMuseShells();
  late OpenMuseShellProfile _selectedProfile = _profiles.first;
  int _selected = 0;
  String? _error;

  @override
  void dispose() {
    for (final session in _sessions) {
      session.close();
    }
    super.dispose();
  }

  void _start() {
    try {
      final environment = Map<String, String>.of(Platform.environment);
      final cliBin = environment['OPENMUSE_CLI_BIN'] ?? _bundledCliBin();
      if (cliBin != null && File(cliBin).existsSync()) {
        environment['PATH'] =
            '${File(cliBin).parent.path}${Platform.isWindows ? ';' : ':'}${environment['PATH'] ?? ''}';
      }
      final pty = Pty.start(
        _selectedProfile.executable,
        arguments: _selectedProfile.arguments,
        workingDirectory: widget.workingDirectory,
        environment: environment,
        rows: 24,
        columns: 100,
      );
      final session = _ConsoleSession(_selectedProfile, pty);
      session.terminal.onOutput = (data) => pty.write(utf8.encode(data));
      session.terminal.onResize = (columns, rows, _, _) =>
          pty.resize(rows, columns);
      session.output = utf8.decoder
          .bind(pty.output)
          .listen(session.terminal.write);
      unawaited(
        pty.exitCode.then((code) {
          session.exitCode = code;
          if (mounted) setState(() {});
        }),
      );
      setState(() {
        _sessions.add(session);
        _selected = _sessions.length - 1;
        _error = null;
      });
    } on Object catch (error) {
      setState(() => _error = error.toString());
    }
  }

  void _close(int index) {
    final session = _sessions.removeAt(index);
    session.close();
    setState(
      () => _selected = _sessions.isEmpty
          ? 0
          : _selected.clamp(0, _sessions.length - 1),
    );
  }

  @override
  Widget build(BuildContext context) => ColoredBox(
    color: const Color(0xff17191d),
    child: Column(
      children: [
        SizedBox(
          height: 35,
          child: Row(
            children: [
              const SizedBox(width: 10),
              const Icon(Icons.terminal, size: 16, color: Colors.white70),
              const SizedBox(width: 7),
              const Text('控制台', style: TextStyle(color: Colors.white70)),
              const SizedBox(width: 16),
              for (var index = 0; index < _sessions.length; index++)
                TextButton(
                  key: ValueKey('cli-tab-$index'),
                  onPressed: () => setState(() => _selected = index),
                  child: Text(
                    '${_sessions[index].profile.name}${_sessions[index].exitCode == null ? '' : ' (${_sessions[index].exitCode})'}',
                  ),
                ),
              if (_sessions.isNotEmpty)
                IconButton(
                  tooltip: '关闭终端',
                  icon: const Icon(Icons.close, size: 16),
                  onPressed: () => _close(_selected),
                ),
              const Spacer(),
              DropdownButton<OpenMuseShellProfile>(
                value: _selectedProfile,
                dropdownColor: const Color(0xff242831),
                style: const TextStyle(color: Colors.white, fontSize: 12),
                items: [
                  for (final profile in _profiles)
                    DropdownMenuItem(value: profile, child: Text(profile.name)),
                ],
                onChanged: (value) {
                  if (value != null) setState(() => _selectedProfile = value);
                },
              ),
              IconButton(
                key: const ValueKey('cli-new-terminal'),
                tooltip: '新建终端',
                icon: const Icon(Icons.add, size: 18),
                onPressed: _start,
              ),
            ],
          ),
        ),
        Expanded(
          child: _error != null
              ? Center(
                  child: Text(
                    _error!,
                    style: const TextStyle(color: Colors.redAccent),
                  ),
                )
              : _sessions.isEmpty
              ? Center(
                  child: TextButton.icon(
                    onPressed: _start,
                    icon: const Icon(Icons.add),
                    label: Text('启动 ${_selectedProfile.name}'),
                  ),
                )
              : IndexedStack(
                  index: _selected,
                  children: [
                    for (final session in _sessions)
                      TerminalView(
                        session.terminal,
                        keyboardType: TextInputType.text,
                      ),
                  ],
                ),
        ),
      ],
    ),
  );
}

String? _bundledCliBin() {
  if (!Platform.isMacOS) return null;
  final contents = File(Platform.resolvedExecutable).parent.parent;
  final candidate = File(
    '${contents.path}/Resources/openmuse/cli/bin/openmuse',
  );
  return candidate.existsSync() ? candidate.path : null;
}
