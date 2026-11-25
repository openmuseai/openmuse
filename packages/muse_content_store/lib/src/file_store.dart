import 'dart:convert';
import 'dart:io';
import '../snapshot_store.dart';

/// Same-filesystem staging + file fsync + atomic rename. Concurrent writers use
/// distinct staging files, so replacement cannot interleave or steal a temp file.
/// The owning session still serializes semantic revisions/CAS; last rename wins.
final class FileSnapshotStore implements SnapshotStore {
  FileSnapshotStore(this.file);
  final File file;
  @override
  Future<Map<String, dynamic>?> read() async {
    if (!await file.exists()) return null;
    return (jsonDecode(await file.readAsString()) as Map)
        .cast<String, dynamic>();
  }

  @override
  Future<void> write(Map<String, dynamic> snapshot) async {
    await file.parent.create(recursive: true);
    final staging = await file.parent.createTemp('.muse-snapshot-');
    try {
      final temporary = File('${staging.path}/snapshot.json');
      final out = await temporary.open(mode: FileMode.write);
      try {
        await out.writeString(jsonEncode(snapshot));
        await out.flush();
      } finally {
        await out.close();
      }
      await temporary.rename(file.path);
    } finally {
      // This directory was created by this call and contains no user files.
      // Cleanup failure must not obscure the write/rename outcome.
      try {
        await staging.delete(recursive: true);
      } on FileSystemException {
        /* orphan cleanup is separate */
      }
    }
  }
}
