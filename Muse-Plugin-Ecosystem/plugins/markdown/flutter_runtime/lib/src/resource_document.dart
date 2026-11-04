import 'dart:convert';
import 'dart:io';
import 'dart:math';
import 'dart:typed_data';

import 'package:archive/archive.dart';
import 'kernel.dart';

const siYuanDocumentMediaType = 'application/vnd.openmuse.siyuan.document+zip';
const _entryAttribute = 'custom-openmuse-resource-entry';
final _nodeId = RegExp(r'^\d{14}-[a-z0-9]{7}$');

/// Plugin-owned native document archive. No Host paths or kernel identifiers are
/// used as canonical resource identity. Import remaps IDs using SiYuan itself.
final class SiYuanDocumentArchive {
  SiYuanDocumentArchive._(this.bytes, this.documents);
  final List<int> bytes;
  final Map<String, Map<String, dynamic>> documents;
  static SiYuanDocumentArchive decode(List<int> bytes, {bool raw = false}) {
    if (bytes.length > 64 * 1024 * 1024)
      throw const FormatException('Archive too large');
    if (raw) {
      final tree = _tree(bytes);
      final name = 'OpenMuse/${tree['ID']}.sy';
      final archive = Archive()
        ..addFile(ArchiveFile(name, bytes.length, bytes));
      return SiYuanDocumentArchive._(ZipEncoder().encode(archive), {
        name: tree,
      });
    }
    final decoder = ZipDecoder();
    final archive = decoder.decodeBytes(bytes);
    final rawNames = <String>{};
    var rawSize = 0;
    for (final header in decoder.directory.fileHeaders) {
      final name = header.filename;
      rawSize += header.uncompressedSize;
      if (name.startsWith('/') ||
          name.split('/').first.contains(':') ||
          name.contains('\\') ||
          name.split('/').any((p) => p == '..' || p == '.') ||
          !rawNames.add(name) ||
          rawNames.length > 10000 ||
          rawSize > 128 * 1024 * 1024) {
        throw const FormatException('Unsafe archive directory');
      }
    }
    var size = 0;
    final names = <String>{};
    final docs = <String, Map<String, dynamic>>{};
    for (final file in archive) {
      if (file.isSymbolicLink ||
          file.name.startsWith('/') ||
          file.name.contains('\\') ||
          file.name.split('/').any((p) => p == '..' || p == '.') ||
          !names.add(file.name)) {
        throw const FormatException('Unsafe archive');
      }
      size += file.size;
      if (size > 128 * 1024 * 1024 || names.length > 10000)
        throw const FormatException('Archive budget exceeded');
      if (file.isFile && file.name.endsWith('.sy'))
        docs[file.name] = _tree(file.content);
    }
    if (docs.isEmpty) throw const FormatException('No native documents');
    if (docs.keys.any((name) => !name.contains('/'))) {
      // Older OpenMuse starter resources and raw .sy wrappers put documents
      // at ZIP root. SiYuan's native importer requires one containing folder.
      // Normalize only the transient import copy, preserving every document,
      // asset and relative path. Never re-seed or overwrite an edited resource.
      final wrapped = Archive();
      for (final file in archive) {
        file.name = 'OpenMuse/${file.name}';
        wrapped.addFile(file);
      }
      return SiYuanDocumentArchive._(ZipEncoder().encode(wrapped), {
        for (final entry in docs.entries) 'OpenMuse/${entry.key}': entry.value,
      });
    }
    return SiYuanDocumentArchive._(List<int>.of(bytes), docs);
  }

  static Map<String, dynamic> _tree(List<int> bytes) {
    if (bytes.length > 16 * 1024 * 1024)
      throw const FormatException('Document too large');
    final value = jsonDecode(utf8.decode(bytes));
    if (value is! Map<String, dynamic> ||
        value['Type'] != 'NodeDocument' ||
        value['ID'] is! String ||
        !_nodeId.hasMatch(value['ID'] as String)) {
      throw const FormatException('Invalid native document');
    }
    return value;
  }

  String conversationText() {
    final result = StringBuffer();
    void walk(Object? node, int depth) {
      if (depth > 100) throw const FormatException('Document depth exceeded');
      if (node is Map) {
        final text = node['Data'];
        if (text is String) result.writeln(text);
        final children = node['Children'];
        if (children is List)
          for (final c in children) {
            walk(c, depth + 1);
          }
      }
    }

    for (final tree in documents.values) {
      walk(tree, 0);
    }
    return result.toString();
  }
}

final class SiYuanResourceApi {
  SiYuanResourceApi(this.kernel);
  final SiYuanKernel kernel;
  Future<dynamic> call(
    String path, [
    Map<String, Object?> data = const {},
  ]) async {
    final origin = await kernel.start();
    final client = HttpClient()
      ..connectionTimeout = const Duration(seconds: 15);
    try {
      final request = await client.postUrl(origin.resolve(path));
      kernel.localSessionHeaders.forEach(request.headers.set);
      request.headers.contentType = ContentType.json;
      request.write(jsonEncode(data));
      final response = await request.close().timeout(
        const Duration(seconds: 60),
      );
      if (response.statusCode != 200)
        throw HttpException('Local kernel ${response.statusCode}');
      final result =
          jsonDecode(await utf8.decoder.bind(response).join()) as Map;
      if (result['code'] != 0) throw StateError('${result['msg']}');
      return result['data'];
    } finally {
      client.close(force: true);
    }
  }

  Future<String> createNotebook() async {
    final value = await call('/api/notebook/createNotebook', {
      'name': 'OpenMuse resource ${DateTime.now().microsecondsSinceEpoch}',
    });
    final id = value['notebook']['id'] as String;
    for (var i = 0; i < 100; i++) {
      final state = await call('/api/notebook/getNotebookConf', {
        'notebook': id,
      });
      if (state['documentCapabilities']?['indexState'] == 'ready') return id;
      await Future<void>.delayed(const Duration(milliseconds: 100));
    }
    throw StateError('Notebook indexing timed out');
  }

  Future<List<int>> seed(String title) async {
    final box = await createNotebook();
    try {
      final id = await call('/api/filetree/createDocWithMd', {
        'notebook': box,
        'path': '/${title.replaceAll('/', ' ').replaceAll('\\', ' ')}',
        'markdown': '\n',
      });
      await call('/api/attr/setBlockAttrs', {
        'id': id,
        'attrs': {_entryAttribute: 'true'},
      });
      await kernel.flush();
      return await export(box, id as String);
    } finally {
      await call('/api/notebook/removeNotebook', {'notebook': box});
    }
  }

  Future<List<Map<String, dynamic>>> documents(String box) async {
    if (!_nodeId.hasMatch(box)) throw const FormatException('Invalid notebook');
    // IDs are generated by the local kernel, never interpolated from a resource.
    final value = await call('/api/query/sql', {
      'stmt':
          "SELECT id, content, path FROM blocks WHERE box='$box' AND type='d' ORDER BY path LIMIT 10000",
    });
    return (value as List)
        .map((v) => Map<String, dynamic>.from(v as Map))
        .toList();
  }

  Future<SiYuanDocumentLease> open(List<int> bytes, {bool raw = false}) async {
    final archive = SiYuanDocumentArchive.decode(bytes, raw: raw);
    final box = await createNotebook();
    try {
      await _import(box, archive.bytes);
      await kernel.flush();
      List<Map<String, dynamic>> docs = [];
      for (var i = 0; i < 100; i++) {
        docs = await documents(box);
        if (docs.isNotEmpty) break;
        await Future<void>.delayed(const Duration(milliseconds: 100));
      }
      if (docs.isEmpty) throw StateError('Imported document missing');
      String root = docs.first['id'] as String;
      for (final doc in docs) {
        final attrs = await call('/api/attr/getBlockAttrs', {'id': doc['id']});
        if (attrs[_entryAttribute] == 'true') {
          root = doc['id'] as String;
          break;
        }
      }
      await call('/api/attr/setBlockAttrs', {
        'id': root,
        'attrs': {_entryAttribute: 'true'},
      });
      return SiYuanDocumentLease(this, box, root);
    } catch (_) {
      await call('/api/notebook/removeNotebook', {'notebook': box});
      rethrow;
    }
  }

  Future<void> _import(String box, List<int> bytes) async {
    final origin = await kernel.start();
    final boundary = 'openmuse${Random.secure().nextInt(1 << 32)}';
    final client = HttpClient();
    try {
      final request = await client.postUrl(
        origin.resolve('/api/import/importSY'),
      );
      kernel.localSessionHeaders.forEach(request.headers.set);
      request.headers.set(
        HttpHeaders.contentTypeHeader,
        'multipart/form-data; boundary=$boundary',
      );
      request.add(
        utf8.encode(
          '--$boundary\r\nContent-Disposition: form-data; name="notebook"\r\n\r\n$box\r\n--$boundary\r\nContent-Disposition: form-data; name="file"; filename="document.sy.zip"\r\nContent-Type: application/zip\r\n\r\n',
        ),
      );
      request.add(bytes);
      request.add(utf8.encode('\r\n--$boundary--\r\n'));
      final response = await request.close();
      final result =
          jsonDecode(await utf8.decoder.bind(response).join()) as Map;
      if (response.statusCode != 200 || result['code'] != 0)
        throw StateError('${result['msg']}');
    } finally {
      client.close(force: true);
    }
  }

  Future<Uint8List> export(String box, String root) async {
    await kernel.flush();
    final docs = await documents(box);
    final ids = [
      root,
      for (final d in docs)
        if (d['id'] != root) d['id'],
    ];
    final result = await call('/api/export/exportSYs', {'ids': ids});
    final path = result['zip'] as String;
    final uri = Uri.parse(path);
    if (uri.hasScheme || uri.hasAuthority || !uri.path.startsWith('/export/'))
      throw const FormatException('Invalid export URI');
    final client = HttpClient();
    try {
      final request = await client.getUrl(
        (await kernel.start()).resolveUri(uri),
      );
      kernel.localSessionHeaders.forEach(request.headers.set);
      final response = await request.close();
      if (response.statusCode != 200)
        throw HttpException('Export ${response.statusCode}');
      final builder = BytesBuilder(copy: false);
      await for (final chunk in response) {
        builder.add(chunk);
        if (builder.length > 64 * 1024 * 1024)
          throw const FormatException('Export too large');
      }
      final bytes = builder.takeBytes();
      SiYuanDocumentArchive.decode(bytes);
      return bytes;
    } finally {
      client.close(force: true);
    }
  }
}

final class SiYuanDocumentLease {
  SiYuanDocumentLease(this.api, this.notebook, this.root);
  final SiYuanResourceApi api;
  final String notebook, root;
  bool _closed = false;
  Future<List<int>> snapshot() => api.export(notebook, root);
  Future<void> close() async {
    if (_closed) return;
    _closed = true;
    await api.call('/api/notebook/removeNotebook', {'notebook': notebook});
  }
}
