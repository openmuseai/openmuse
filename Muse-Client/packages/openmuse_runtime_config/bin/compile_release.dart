import 'dart:convert';
import 'dart:io';
import 'package:openmuse_runtime_config/openmuse_runtime_config_core.dart';

Future<void> main(List<String> arguments) async {
  if (arguments.length != 4 ||
      arguments[0] != '--input' ||
      arguments[2] != '--output') {
    stderr.writeln(
      'Usage: compile_release --input installed-release.json --output defines.json',
    );
    exitCode = 64;
    return;
  }
  try {
    final input = File(arguments[1]), output = File(arguments[3]);
    if (input.absolute.path == output.absolute.path) {
      throw const FormatException('Input and output must differ');
    }
    final source = await input.readAsString();
    InstalledRelease.parse(source);
    await output.parent.create(recursive: true);
    await output.writeAsString(
      '${jsonEncode({'OPENMUSE_RELEASE_JSON': jsonEncode(jsonDecode(source))})}\n',
      flush: true,
    );
    stdout.writeln('Validated installation release written to ${output.path}');
  } catch (error) {
    stderr.writeln('Release compilation failed: $error');
    exitCode = 1;
  }
}
