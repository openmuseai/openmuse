import 'dart:io';
import 'package:analyzer/dart/analysis/utilities.dart';
import 'package:analyzer/dart/ast/ast.dart';
import 'package:analyzer/dart/ast/visitor.dart';

final class Capture extends RecursiveAstVisitor<void> {
  Capture(this.source);
  final String source;
  final List<({int offset, int length, String replacement})> edits = [];
  static final lookup = RegExp(r'stringsOf\s*\(\s*context\s*,?\s*\)');
  bool capture(FunctionBody body) {
    if (body is! BlockFunctionBody || !body.isAsynchronous) return false;
    final code = source.substring(body.block.leftBracket.end, body.block.rightBracket.offset);
    if (!lookup.hasMatch(code)) return false;
    var name = 'localeStrings';
    while (RegExp('\\b$name\\b').hasMatch(code)) { name += 'ForTask'; }
    edits.add((offset: body.block.leftBracket.end, length: code.length,
      replacement: '\n    final $name = stringsOf(context);\n${code.replaceAll(lookup, name)}'));
    return true;
  }
  @override void visitMethodDeclaration(MethodDeclaration node) { if (!capture(node.body)) super.visitMethodDeclaration(node); }
  @override void visitFunctionExpression(FunctionExpression node) { if (!capture(node.body)) super.visitFunctionExpression(node); }
}

void main(List<String> files) {
  for (final path in files) {
    final file = File(path);
    var source = file.readAsStringSync();
    final visitor = Capture(source);
    parseString(content: source, throwIfDiagnostics: false).unit.accept(visitor);
    for (final edit in visitor.edits.reversed) {
      source = source.replaceRange(edit.offset, edit.offset + edit.length, edit.replacement);
    }
    file.writeAsStringSync(source);
  }
}
