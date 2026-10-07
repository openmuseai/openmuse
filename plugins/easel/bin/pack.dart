import 'dart:io';

import '../tool/easel_package.dart';

void main(List<String> args) {
  String value(String name) {
    final at = args.indexOf(name);
    if (at < 0 || at + 1 >= args.length) {
      throw FormatException('Missing $name');
    }
    return args[at + 1];
  }

  final source = Directory(value('--easel'));
  final output = Directory(value('--output'));
  final plugin = Directory.current;
  final package = buildEaselPackage(source, plugin);
  writeEaselPackage(output, package);
  stdout.writeln('Packed ${output.path}/${EaselPluginPackage.fileName}');
}
