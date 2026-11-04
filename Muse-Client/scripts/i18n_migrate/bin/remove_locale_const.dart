import 'dart:io';
import 'package:analyzer/dart/analysis/utilities.dart';
import 'package:analyzer/dart/ast/ast.dart';
import 'package:analyzer/dart/ast/visitor.dart';
class Fix extends RecursiveAstVisitor<void> {
  final offsets = <int, int>{};
  @override void visitMethodInvocation(MethodInvocation node) {
    if (node.methodName.name == 'stringsOf' || node.methodName.name.startsWith('dsh')) {
      for (AstNode? parent = node.parent; parent != null; parent = parent.parent) {
        if (parent is InstanceCreationExpression && parent.keyword?.lexeme == 'const') offsets[parent.keyword!.offset] = parent.keyword!.length;
        if (parent is ListLiteral && parent.constKeyword != null) offsets[parent.constKeyword!.offset] = parent.constKeyword!.length;
      }
    }
    super.visitMethodInvocation(node);
  }
}
void main(List<String> paths) {
  for (final path in paths) {
    final file = File(path); var source = file.readAsStringSync();
    final fix = Fix(); parseString(content: source, throwIfDiagnostics: false).unit.accept(fix);
    for (final offset in fix.offsets.keys.toList()..sort((a,b)=>b.compareTo(a))) source=source.replaceRange(offset,offset+fix.offsets[offset]!, '');
    file.writeAsStringSync(source);
  }
}
