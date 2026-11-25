import 'dart:convert';
import 'dart:io';

final class WorkspaceMountStore {
  const WorkspaceMountStore(this.file);

  final File file;

  Future<List<String>> load() async {
    if (!await file.exists()) return const [];
    final decoded = jsonDecode(await file.readAsString());
    if (decoded is! List) return const [];
    return decoded.whereType<String>().toList(growable: false);
  }

  Future<void> save(Iterable<String> paths) async {
    await file.parent.create(recursive: true);
    final temporary = File('${file.path}.tmp');
    await temporary.writeAsString(jsonEncode(paths.toList()), flush: true);
    if (await file.exists()) await file.delete();
    await temporary.rename(file.path);
  }
}
