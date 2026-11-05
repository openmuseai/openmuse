import 'dart:convert';
import 'dart:math' as math;

import 'package:appflowy/plugins/version_diff/domain/diff_workbench_contract.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_engine.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_models.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_runtime.dart';
import 'package:crypto/crypto.dart';
import 'package:path/path.dart' as p;

final class MuseTextDiffProvider
    implements MuseDiffProvider<MuseTextDiffPayload>, MuseSemanticDiffProvider {
  MuseTextDiffProvider({
    this.contextLines = 3,
    MuseDiffTextRuntime? runtime,
  }) : runtime = runtime ?? MuseDiffTextRuntime.instance;

  static const supportedExtensions = {
    'txt',
    'md',
    'markdown',
    'dart',
    'rs',
    'ts',
    'tsx',
    'js',
    'jsx',
    'java',
    'c',
    'h',
    'cc',
    'cpp',
    'cs',
    'go',
    'py',
    'rb',
    'php',
    'swift',
    'kt',
    'kts',
    'sh',
    'bash',
    'zsh',
    'fish',
    'sql',
    'html',
    'htm',
    'css',
    'scss',
    'sass',
    'less',
    'xml',
    'json',
    'jsonc',
    'yaml',
    'yml',
    'toml',
    'ini',
    'conf',
    'gradle',
    'properties',
    'vue',
    'svelte',
  };

  final int contextLines;
  final MuseDiffTextRuntime runtime;

  @override
  String get id => 'muse.diff.text.v1';

  @override
  String get semanticProviderId => id;

  @override
  String get semanticProviderVersion => '1.0.0';

  @override
  String get rendererType => 'muse.diff-viewer.text.v1';

  @override
  bool supports(MuseResourceRef resource) {
    final extension =
        p.extension(resource.locator).replaceFirst('.', '').toLowerCase();
    return resource.mediaType.startsWith('text/') ||
        supportedExtensions.contains(extension);
  }

  @override
  bool supportsSemanticDiff(MuseResourceRef resource) => supports(resource);

  @override
  Future<MuseSemanticChangeSet> compareSemantic({
    required MuseComparison comparison,
    required MuseVersion base,
    required MuseVersion target,
    required MuseContentResolver contentResolver,
  }) async {
    final result = await compare(
      comparison: comparison,
      base: base,
      target: target,
      contentResolver: contentResolver,
    );
    return toSemanticChangeSet(
      comparison: comparison,
      payload: result.payload,
    );
  }

  @override
  Future<MuseResourceDiff<MuseTextDiffPayload>> compare({
    required MuseComparison comparison,
    required MuseVersion base,
    required MuseVersion target,
    required MuseContentResolver contentResolver,
  }) async {
    final baseBytes = await contentResolver.resolve(base);
    final targetBytes = await contentResolver.resolve(target);
    final baseText = utf8.decode(baseBytes, allowMalformed: false);
    final targetText = utf8.decode(targetBytes, allowMalformed: false);
    final payload = await compareTextAsync(
      resource: comparison.resource,
      baseText: baseText,
      targetText: targetText,
    );
    return MuseResourceDiff(
      comparison: comparison,
      payload: payload,
      changeCount: payload.changeCount,
      summary: '${payload.changeCount} changes, '
          '+${payload.additions} -${payload.deletions}',
    );
  }

  /// Host path: background isolate + native change-block FFI when available.
  Future<MuseTextDiffPayload> compareTextAsync({
    required MuseResourceRef resource,
    required String baseText,
    required String targetText,
  }) async {
    final result = await runtime.compare(before: baseText, after: targetText);
    return assemble(
        resource: resource,
        baseText: baseText,
        targetText: targetText,
        result: result);
  }

  /// Synchronous Dart assembler used by widget tests and presentation fixtures.
  MuseTextDiffPayload compareText({
    required MuseResourceRef resource,
    required String baseText,
    required String targetText,
  }) {
    final result = const MuseDartTextDiffEngine().compare(baseText, targetText);
    return assemble(
      resource: resource,
      baseText: baseText,
      targetText: targetText,
      result: result,
    );
  }

  MuseTextDiffPayload assemble({
    required MuseResourceRef resource,
    required String baseText,
    required String targetText,
    required MuseTextDiffEngineResult result,
  }) {
    final oldLines = MuseDartTextDiffEngine.splitLines(baseText);
    final newLines = MuseDartTextDiffEngine.splitLines(targetText);
    final hunks = <MuseTextDiffHunk>[];
    if (result.blocks.isNotEmpty) {
      for (final range in _hunkRanges(result.blocks)) {
        hunks.add(
          _buildHunk(
            resource,
            oldLines,
            newLines,
            result.blocks.sublist(range.$1, range.$2 + 1),
          ),
        );
      }
    }
    return MuseTextDiffPayload(
      language: _languageFor(resource.locator),
      hunks: hunks,
      additions: result.additions,
      deletions: result.deletions,
      baseText: baseText,
      targetText: targetText,
      engine: result.engine,
      quality: result.quality,
      similarBoundaries: result.similarBoundaries,
    );
  }

  MuseSemanticChangeSet toSemanticChangeSet({
    required MuseComparison comparison,
    required MuseTextDiffPayload payload,
  }) {
    final changes = <MuseSemanticChange>[];
    for (final hunk in payload.hunks) {
      for (final change in hunk.changes) {
        changes.add(
          MuseSemanticChange(
            id: change.id,
            kind: switch (change.kind) {
              MuseTextChangeKind.insert => MuseUniversalChangeKind.insert,
              MuseTextChangeKind.delete => MuseUniversalChangeKind.delete,
              MuseTextChangeKind.replace => MuseUniversalChangeKind.modify,
            },
            semanticPath: change.semanticLabel.isEmpty
                ? 'text'
                : 'text/${change.semanticLabel}',
            label: change.semanticLabel.isEmpty
                ? 'Text change at line ${change.newStart}'
                : change.semanticLabel,
            before: [
              MuseTextRangeAnchor(
                startLine: change.oldStart,
                endLine: change.oldStart + change.oldCount,
              ),
            ],
            after: [
              MuseTextRangeAnchor(
                startLine: change.newStart,
                endLine: change.newStart + change.newCount,
              ),
            ],
            attribution: MuseChangeAttribution(actor: comparison.actor),
            properties: {
              'oldLineCount': change.oldCount,
              'newLineCount': change.newCount,
            },
          ),
        );
      }
    }
    return MuseSemanticChangeSet(
      providerId: id,
      providerVersion: semanticProviderVersion,
      comparisonId: comparison.id,
      resource: comparison.resource,
      changes: changes,
      quality: MuseDiffQuality(kind: payload.quality),
      summary: {
        'changeCount': payload.changeCount,
        'addedLines': payload.additions,
        'deletedLines': payload.deletions,
        'engine': payload.engine,
      },
      domainIndex: {'language': payload.language},
    );
  }

  List<(int, int)> _hunkRanges(List<MuseChangeBlock> blocks) {
    final result = <(int, int)>[];
    var first = 0;
    var last = 0;
    for (var index = 1; index < blocks.length; index++) {
      final previous = blocks[last];
      final current = blocks[index];
      final oldGap = current.oldStart - (previous.oldStart + previous.oldCount);
      final newGap = current.newStart - (previous.newStart + previous.newCount);
      final equalGap = oldGap < newGap ? oldGap : newGap;
      if (equalGap > contextLines * 2) {
        result.add((first, last));
        first = index;
      }
      last = index;
    }
    result.add((first, last));
    return result;
  }

  MuseTextDiffHunk _buildHunk(
    MuseResourceRef resource,
    List<String> oldLines,
    List<String> newLines,
    List<MuseChangeBlock> blocks,
  ) {
    final rows = <MuseTextDiffRow>[];
    final changes = <MuseTextChange>[];
    final first = blocks.first;
    final last = blocks.last;
    var oldCursor = _extendStart(first.oldStart);
    var newCursor = _extendStart(first.newStart);
    final oldEnd = _extendEnd(last.oldStart + last.oldCount, oldLines.length);
    final newEnd = _extendEnd(last.newStart + last.newCount, newLines.length);

    void emitEquals(int oldStop, int newStop) {
      final count = math.min(oldStop - oldCursor, newStop - newCursor);
      for (var offset = 0; offset < count; offset++) {
        final oldIndex = oldCursor + offset;
        final newIndex = newCursor + offset;
        if (oldIndex < 0 ||
            newIndex < 0 ||
            oldIndex >= oldLines.length ||
            newIndex >= newLines.length) {
          break;
        }
        rows.add(
          MuseTextDiffRow(
            kind: MuseTextRowKind.context,
            text: oldLines[oldIndex],
            oldLine: oldIndex + 1,
            newLine: newIndex + 1,
          ),
        );
      }
      oldCursor = oldStop;
      newCursor = newStop;
    }

    for (final block in blocks) {
      emitEquals(block.oldStart, block.newStart);
      final deleted = [
        for (var offset = 0; offset < block.oldCount; offset++)
          if (block.oldStart + offset < oldLines.length)
            oldLines[block.oldStart + offset],
      ];
      final inserted = [
        for (var offset = 0; offset < block.newCount; offset++)
          if (block.newStart + offset < newLines.length)
            newLines[block.newStart + offset],
      ];
      final oldStart =
          deleted.isEmpty ? block.oldStart + 1 : block.oldStart + 1;
      final newStart =
          inserted.isEmpty ? block.newStart + 1 : block.newStart + 1;
      final fingerprint = [
        resource.id,
        oldStart,
        newStart,
        ...deleted,
        '=>',
        ...inserted,
      ].join('\u001f');
      final changeId = 'change:${sha256.convert(utf8.encode(fingerprint))}';
      final semanticLabel = _semanticLabel(
        oldLines,
        newLines,
        block,
        resource.locator,
      );
      final inline = deleted.length == 1 && inserted.length == 1
          ? _inlineDiff(deleted.single, inserted.single)
          : null;
      for (var offset = 0; offset < deleted.length; offset++) {
        rows.add(
          MuseTextDiffRow(
            kind: MuseTextRowKind.deletion,
            text: deleted[offset],
            oldLine: block.oldStart + offset + 1,
            changeId: changeId,
            inlineSpans: inline?.$1 ?? const [],
          ),
        );
      }
      for (var offset = 0; offset < inserted.length; offset++) {
        rows.add(
          MuseTextDiffRow(
            kind: MuseTextRowKind.insertion,
            text: inserted[offset],
            newLine: block.newStart + offset + 1,
            changeId: changeId,
            inlineSpans: inline?.$2 ?? const [],
          ),
        );
      }
      changes.add(
        MuseTextChange(
          id: changeId,
          kind: block.kind,
          oldStart: oldStart,
          oldCount: deleted.length,
          newStart: newStart,
          newCount: inserted.length,
          semanticLabel: semanticLabel,
        ),
      );
      oldCursor = block.oldStart + block.oldCount;
      newCursor = block.newStart + block.newCount;
    }
    emitEquals(oldEnd, newEnd);

    final oldRows = rows.where((row) => row.oldLine != null).toList();
    final newRows = rows.where((row) => row.newLine != null).toList();
    final hunkOldStart = oldRows.isEmpty ? 0 : oldRows.first.oldLine!;
    final hunkNewStart = newRows.isEmpty ? 0 : newRows.first.newLine!;
    final label = changes.map((change) => change.semanticLabel).firstWhere(
          (label) => label.isNotEmpty,
          orElse: () => '',
        );
    final hunkFingerprint = '${resource.id}:$hunkOldStart:$hunkNewStart:'
        '${changes.map((change) => change.id).join(',')}';
    return MuseTextDiffHunk(
      id: 'hunk:${sha256.convert(utf8.encode(hunkFingerprint))}',
      oldStart: hunkOldStart,
      oldCount: oldRows.length,
      newStart: hunkNewStart,
      newCount: newRows.length,
      semanticLabel: label,
      rows: rows,
      changes: changes,
    );
  }

  int _extendStart(int index) =>
      index - contextLines < 0 ? 0 : index - contextLines;

  int _extendEnd(int index, int length) {
    final extended = index + contextLines;
    return extended > length ? length : extended;
  }

  String _semanticLabel(
    List<String> oldLines,
    List<String> newLines,
    MuseChangeBlock block,
    String locator,
  ) {
    final extension = p.extension(locator).toLowerCase();
    final start = block.oldCount > 0
        ? math.min(block.oldStart, oldLines.length - 1)
        : math.min(block.newStart, newLines.length - 1);
    final source = block.oldCount > 0 ? oldLines : newLines;
    if (source.isEmpty || start < 0) return '';
    for (var index = start; index >= 0; index--) {
      final text = source[index].trim();
      if ({'.md', '.markdown'}.contains(extension) &&
          RegExp(r'^#{1,6}\s+').hasMatch(text)) {
        return text.replaceFirst(RegExp(r'^#{1,6}\s+'), '');
      }
      final codeSymbol = RegExp(
        r'^(?:class|interface|enum|struct|trait|fn|func|function|def|void|public|private|protected|export)\s+([A-Za-z_$][\w$]*)',
      ).firstMatch(text);
      if (codeSymbol != null) return codeSymbol.group(1) ?? text;
      if (index == 0) break;
    }
    return '';
  }

  (List<MuseTextInlineSpan>, List<MuseTextInlineSpan>) _inlineDiff(
    String oldValue,
    String newValue,
  ) {
    var prefix = 0;
    final limit =
        oldValue.length < newValue.length ? oldValue.length : newValue.length;
    while (prefix < limit && oldValue[prefix] == newValue[prefix]) {
      prefix++;
    }
    var suffix = 0;
    while (suffix < limit - prefix &&
        oldValue[oldValue.length - 1 - suffix] ==
            newValue[newValue.length - 1 - suffix]) {
      suffix++;
    }

    List<MuseTextInlineSpan> spans(String value) {
      final result = <MuseTextInlineSpan>[];
      if (prefix > 0) {
        result.add(
          MuseTextInlineSpan(
            text: value.substring(0, prefix),
            changed: false,
          ),
        );
      }
      final changeEnd = value.length - suffix;
      if (changeEnd > prefix) {
        result.add(
          MuseTextInlineSpan(
            text: value.substring(prefix, changeEnd),
            changed: true,
          ),
        );
      }
      if (suffix > 0) {
        result.add(
          MuseTextInlineSpan(
            text: value.substring(value.length - suffix),
            changed: false,
          ),
        );
      }
      return result;
    }

    return (spans(oldValue), spans(newValue));
  }

  String _languageFor(String locator) {
    final extension = p.extension(locator).replaceFirst('.', '').toLowerCase();
    return switch (extension) {
      'md' || 'markdown' => 'markdown',
      'rs' => 'rust',
      'ts' || 'tsx' => 'typescript',
      'js' || 'jsx' => 'javascript',
      'py' => 'python',
      'kt' || 'kts' => 'kotlin',
      'yml' || 'yaml' => 'yaml',
      _ => extension.isEmpty ? 'text' : extension,
    };
  }
}
