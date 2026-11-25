import 'dart:io';

import 'package:openmuse_workbench_layout/openmuse_workbench_layout.dart';

export 'package:openmuse_workbench_layout/openmuse_workbench_layout.dart';

final class LayoutStore implements LayoutSnapshotWriter {
  const LayoutStore(this.file, {this.fallback});

  final File file;
  final WorkbenchLayoutSnapshot? fallback;

  Future<WorkbenchLayoutSnapshot> load({
    WorkbenchLayoutSnapshot? fallback,
  }) async {
    final safeFallback =
        fallback ?? this.fallback ?? createDefaultWorkbenchLayout();
    try {
      if (!await file.exists()) return safeFallback;
      final parsed = WorkbenchLayoutSnapshot.tryDecode(
        await file.readAsString(),
      );
      return parsed ?? safeFallback;
    } on FileSystemException {
      return safeFallback;
    }
  }

  @override
  Future<void> save(WorkbenchLayoutSnapshot snapshot) async {
    await file.parent.create(recursive: true);
    final temporary = File('${file.path}.tmp');
    await temporary.writeAsString(snapshot.encode(), flush: true);
    try {
      await temporary.rename(file.path);
    } on FileSystemException {
      // Some platforms cannot replace an existing file with rename. Preserve
      // the old file until the complete temporary file has reached disk.
      final backup = File('${file.path}.bak');
      if (await backup.exists()) await backup.delete();
      if (await file.exists()) await file.rename(backup.path);
      try {
        await temporary.rename(file.path);
        if (await backup.exists()) await backup.delete();
      } on Object {
        if (await backup.exists() && !await file.exists()) {
          await backup.rename(file.path);
        }
        rethrow;
      }
    }
  }
}
