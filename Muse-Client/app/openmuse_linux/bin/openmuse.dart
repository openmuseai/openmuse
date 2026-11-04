import 'dart:io';

import 'package:openmuse_linux/src/host/plugin_cli.dart';

Future<void> main(List<String> args) async {
  exit(await runOpenMuseCli(args));
}
