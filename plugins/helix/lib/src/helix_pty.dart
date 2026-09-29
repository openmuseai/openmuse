import 'dart:async';
import 'dart:io';
import 'dart:isolate';
import 'dart:typed_data';

import 'package:flutter_pty/flutter_pty.dart';

/// Keeps Windows' synchronous ConPTY/CreateProcess call off the UI isolate.
/// The PTY and its native receive ports remain in the same worker isolate.
final class HelixPty {
  HelixPty._(
    this.pid,
    this.output,
    this.exitCode,
    this._direct,
    this._commands,
  );

  final int pid;
  final Stream<Uint8List> output;
  final Future<int> exitCode;
  final Pty? _direct;
  final SendPort? _commands;

  static Future<HelixPty> start(
    String executable, {
    required List<String> arguments,
    required String workingDirectory,
    required Map<String, String> environment,
    required int rows,
    required int columns,
  }) async {
    if (!Platform.isWindows) {
      final pty = Pty.start(
        executable,
        arguments: arguments,
        workingDirectory: workingDirectory,
        environment: environment,
        rows: rows,
        columns: columns,
      );
      return HelixPty._(pty.pid, pty.output, pty.exitCode, pty, null);
    }

    final receiver = ReceivePort();
    final output = StreamController<Uint8List>();
    final exit = Completer<int>();
    final ready = Completer<HelixPty>();
    var outputDone = false;
    var exited = false;
    receiver.listen((dynamic message) {
      final parts = message as List<dynamic>;
      switch (parts[0] as String) {
        case 'ready':
          ready.complete(
            HelixPty._(
              parts[1] as int,
              output.stream,
              exit.future,
              null,
              parts[2] as SendPort,
            ),
          );
        case 'output':
          output.add(parts[1] as Uint8List);
        case 'output_done':
          outputDone = true;
          unawaited(output.close());
        case 'exit':
          exited = true;
          exit.complete(parts[1] as int);
        case 'error':
          final error = StateError(parts[1] as String);
          if (!ready.isCompleted) {
            ready.completeError(error);
          } else {
            if (!exit.isCompleted) exit.completeError(error);
            if (!output.isClosed) output.addError(error);
          }
          outputDone = true;
          exited = true;
          unawaited(output.close());
      }
      if (outputDone && exited) receiver.close();
    });
    try {
      await Isolate.spawn(_helixPtyWorker, <Object>[
        receiver.sendPort,
        executable,
        arguments,
        workingDirectory,
        environment,
        rows,
        columns,
      ]);
      return await ready.future;
    } catch (_) {
      receiver.close();
      rethrow;
    }
  }

  void write(Uint8List bytes) {
    if (_direct case final pty?) {
      pty.write(bytes);
    } else {
      _commands!.send(<Object>['write', bytes]);
    }
  }

  void resize(int rows, int columns) {
    if (_direct case final pty?) {
      pty.resize(rows, columns);
    } else {
      _commands!.send(<Object>['resize', rows, columns]);
    }
  }

  void kill() {
    if (_direct case final pty?) {
      pty.kill();
    } else {
      _commands!.send(<Object>['kill']);
    }
  }
}

void _helixPtyWorker(List<Object> args) {
  final send = args[0] as SendPort;
  try {
    final pty = Pty.start(
      args[1] as String,
      arguments: args[2] as List<String>,
      workingDirectory: args[3] as String,
      environment: args[4] as Map<String, String>,
      rows: args[5] as int,
      columns: args[6] as int,
    );
    final commands = ReceivePort();
    commands.listen((dynamic message) {
      final parts = message as List<dynamic>;
      switch (parts[0] as String) {
        case 'write':
          pty.write(parts[1] as Uint8List);
        case 'resize':
          pty.resize(parts[1] as int, parts[2] as int);
        case 'kill':
          pty.kill();
      }
    });
    send.send(<Object>['ready', pty.pid, commands.sendPort]);
    pty.output.listen(
      (bytes) => send.send(<Object>['output', bytes]),
      onDone: () {
        send.send(<Object>['output_done']);
        commands.close();
      },
    );
    pty.exitCode.then((code) => send.send(<Object>['exit', code]));
  } catch (error) {
    send.send(<Object>['error', error.toString()]);
  }
}
