import 'dart:io';

import 'package:openmuse_host/src/host/plugin_cli.dart';

Future<void> main(List<String> args) async {
  exit(await runOpenMuseCli(args));
}
