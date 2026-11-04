import 'dart:convert';
import 'dart:io';

import 'package:openmuse_runtime_config/openmuse_runtime_config_core.dart';

/// Validate installed data and emit one --dart-define-from-file input.
Future<void> main(List<String> arguments) async {
  if (arguments.length != 4 ||
      arguments[0] != '--input' ||
      arguments[2] != '--output') {
    stderr.writeln(
      'Usage: compile_deployment --input bundle.json --output defines.json',
    );
    exitCode = 64;
    return;
  }
  try {
    final source = await File(arguments[1]).readAsString();
    DeploymentBundle.parse(source);
    final output = File(arguments[3]);
    if (output.absolute.path == File(arguments[1]).absolute.path) {
      throw const FormatException('Input and output must differ');
    }
    await output.parent.create(recursive: true);
    await output.writeAsString(
      '${jsonEncode({'OPENMUSE_DEPLOYMENT_JSON': jsonEncode(jsonDecode(source))})}\n',
      flush: true,
    );
    stdout.writeln(
      'Validated deployment; build defines written to ${output.path}',
    );
  } on Exception catch (error) {
    stderr.writeln('Deployment compilation failed: $error');
    exitCode = 1;
  }
}
