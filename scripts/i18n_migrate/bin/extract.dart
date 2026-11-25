import 'dart:convert';
import 'dart:io';

import 'package:analyzer/dart/analysis/utilities.dart';
import 'package:analyzer/dart/ast/ast.dart';
import 'package:analyzer/dart/ast/visitor.dart';
import 'package:crypto/crypto.dart';

String literal(StringLiteral node, Map<String, String> parameters) {
  String escape(String text) => text.replaceAll("'", "''").replaceAll('{', "'{'").replaceAll('}', "'}'");
  if (node is SimpleStringLiteral) return escape(node.value);
  if (node is AdjacentStrings) return node.strings.map((s) => literal(s, parameters)).join();
  if (node is StringInterpolation) {
    return node.elements.map((element) {
      if (element is InterpolationString) return escape(element.value);
      final expression = (element as InterpolationExpression).expression.toSource();
      final name = parameters.putIfAbsent(expression, () => 'value${parameters.length + 1}');
      return '{$name}';
    }).join();
  }
  throw StateError('Unsupported literal');
}

final class Extractor extends RecursiveAstVisitor<void> {
  Extractor(this.file, this.source, this.output);
  final String file;
  final String source;
  final List<Map<String, Object?>> output;

  @override
  void visitConditionalExpression(ConditionalExpression node) {
    final flag = node.condition.toSource();
    if (!{'mobileEnglishBuild', 'officeEnglishBuild', 'zh'}.contains(flag)) {
      super.visitConditionalExpression(node);
      return;
    }
    if (node.thenExpression is! StringLiteral || node.elseExpression is! StringLiteral) {
      output.add({'file': file, 'offset': node.offset, 'unsupported': node.toSource()});
      return;
    }
    final parameters = <String, String>{};
    final en = literal((flag == 'zh' ? node.elseExpression : node.thenExpression) as StringLiteral, parameters);
    final zh = literal((flag == 'zh' ? node.thenExpression : node.elseExpression) as StringLiteral, parameters);
    final digest = sha256.convert(utf8.encode('$en\n$zh')).toString().substring(0, 8);
    var slug = en.toLowerCase().replaceAll(RegExp(r'\{[^}]+\}'), '').replaceAll(RegExp('[^a-z0-9]+'), '_').replaceAll(RegExp(r'^_|_$'), '');
    if (slug.length > 42) slug = slug.substring(0, 42).replaceAll(RegExp(r'_$'), '');
    if (slug.isEmpty) slug = 'message';
    final namespace = flag == 'officeEnglishBuild' ? 'office' : flag == 'zh' ? 'updates' : 'mobile';
    final constEdits = <String, Map<String, Object?>>{};
    var hasContext = false;
    String? declaringClass;
    String? member;
    for (AstNode? ancestor = node.parent; ancestor != null; ancestor = ancestor.parent) {
      if (ancestor is MethodDeclaration) {
        member ??= ancestor.name.lexeme;
        if (ancestor.parameters?.toSource().contains(RegExp(r'BuildContext\s+context\b')) ?? false) hasContext = true;
      }
      if (ancestor is FunctionDeclaration && (ancestor.functionExpression.parameters?.toSource().contains(RegExp(r'BuildContext\s+context\b')) ?? false)) hasContext = true;
      if (ancestor is FunctionExpression && (ancestor.parameters?.toSource().contains(RegExp(r'BuildContext\s+context\b')) ?? false)) hasContext = true;
      if (ancestor is ClassDeclaration) {
        declaringClass = ancestor.name.lexeme;
        if (member != null && (ancestor.extendsClause?.toSource().contains(RegExp(r'\bState\s*<')) ?? false)) hasContext = true;
      }
      void add(int offset, int length, String replacement) => constEdits['$offset'] = {'offset': offset, 'length': length, 'replacement': replacement};
      if (ancestor is InstanceCreationExpression && ancestor.keyword?.lexeme == 'const') add(ancestor.keyword!.offset, ancestor.keyword!.length, '');
      if (ancestor is ListLiteral && ancestor.constKeyword != null) add(ancestor.constKeyword!.offset, ancestor.constKeyword!.length, '');
      if (ancestor is SetOrMapLiteral && ancestor.constKeyword != null) add(ancestor.constKeyword!.offset, ancestor.constKeyword!.length, '');
      if (ancestor is VariableDeclarationList && ancestor.keyword?.lexeme == 'const') add(ancestor.keyword!.offset, ancestor.keyword!.length, 'final');
    }
    output.add({
      'file': file, 'offset': node.offset, 'length': node.length,
      'id': '$namespace.$slug.$digest', 'namespace': namespace,
      'en': en, 'zh-Hans': zh,
      'parameters': parameters.map((expression, name) => MapEntry(name, expression)),
      'context': hasContext, 'class': declaringClass, 'member': member,
      'constEdits': constEdits.values.toList(),
    });
  }
}

void main(List<String> arguments) {
  final report = <Map<String, Object?>>[];
  for (final file in arguments) {
    final source = File(file).readAsStringSync();
    parseString(content: source, path: file, throwIfDiagnostics: false).unit.accept(Extractor(file, source, report));
  }
  stdout.writeln(const JsonEncoder.withIndent('  ').convert(report));
}
