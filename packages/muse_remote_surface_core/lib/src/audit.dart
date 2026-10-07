// The directory parameter stays public while the stored field is private.
// ignore_for_file: prefer_initializing_formals

import 'dart:convert';
import 'dart:io';

import 'package:muse_remote_surface_contract/muse_remote_surface_contract.dart';

final class RemoteAuditEntry {
  const RemoteAuditEntry({
    required this.atMs,
    required this.actorRef,
    required this.mobileDeviceRef,
    required this.desktopDeviceRef,
    required this.workspaceRef,
    required this.actionId,
    required this.status,
    required this.idempotencyKey,
    this.errorCode,
    this.decisionRef,
  });

  final int atMs;
  final String actorRef;
  final String mobileDeviceRef;
  final String desktopDeviceRef;
  final String workspaceRef;
  final String actionId;
  final String status;
  final String idempotencyKey;
  final String? errorCode;
  final String? decisionRef;

  Map<String, Object?> toJson() => {
    'atMs': atMs,
    'actorRef': actorRef,
    'mobileDeviceRef': mobileDeviceRef,
    'desktopDeviceRef': desktopDeviceRef,
    'workspaceRef': workspaceRef,
    'actionId': actionId,
    'status': status,
    'idempotencyKey': idempotencyKey,
    if (errorCode != null) 'errorCode': errorCode,
    if (decisionRef != null) 'decisionRef': decisionRef,
  };

  factory RemoteAuditEntry.fromJson(Map<String, Object?> json) =>
      RemoteAuditEntry(
        atMs: json['atMs']! as int,
        actorRef: json['actorRef']! as String,
        mobileDeviceRef: json['mobileDeviceRef']! as String,
        desktopDeviceRef: json['desktopDeviceRef']! as String,
        workspaceRef: json['workspaceRef']! as String,
        actionId: json['actionId']! as String,
        status: json['status']! as String,
        idempotencyKey: json['idempotencyKey']! as String,
        errorCode: json['errorCode'] as String?,
        decisionRef: json['decisionRef'] as String?,
      );
}

final class RemoteAuditLog {
  RemoteAuditLog({Directory? directory}) : _directory = directory {
    _load();
  }

  final Directory? _directory;
  final entries = <RemoteAuditEntry>[];

  void record(RemoteAuditEntry entry) {
    entries.add(entry);
    final file = _file;
    if (file == null) return;
    file.writeAsStringSync(
      '${jsonEncode(entry.toJson())}\n',
      mode: FileMode.append,
    );
  }

  void _load() {
    final file = _file;
    if (file == null || !file.existsSync()) return;
    for (final line in file.readAsLinesSync()) {
      if (line.isEmpty) continue;
      entries.add(
        RemoteAuditEntry.fromJson(
          (jsonDecode(line) as Map).cast<String, Object?>(),
        ),
      );
    }
  }

  File? get _file =>
      _directory == null ? null : File('${_directory.path}/audit.jsonl');
}

final class RemoteStoredJob {
  const RemoteStoredJob({
    required this.actionId,
    required this.canonical,
    required this.receipt,
  });

  final String actionId;
  final String canonical;
  final RemoteControlReceipt receipt;
}

final class RemoteJobLedger {
  RemoteJobLedger({Directory? directory}) : _directory = directory {
    _load();
  }

  final Directory? _directory;
  final _jobs = <String, RemoteStoredJob>{};

  RemoteStoredJob? find(String idempotencyKey) => _jobs[idempotencyKey];

  void put({
    required String idempotencyKey,
    required String actionId,
    required String canonical,
    required RemoteControlReceipt receipt,
  }) {
    if (_jobs.containsKey(idempotencyKey)) return;
    final stored = RemoteStoredJob(
      actionId: actionId,
      canonical: canonical,
      receipt: receipt,
    );
    _jobs[idempotencyKey] = stored;
    final file = _file;
    if (file == null) return;
    file.writeAsStringSync(
      '${jsonEncode({'idempotencyKey': idempotencyKey, 'actionId': actionId, 'canonical': canonical, 'receipt': receipt.toJson()})}\n',
      mode: FileMode.append,
    );
  }

  void _load() {
    final file = _file;
    if (file == null || !file.existsSync()) return;
    for (final line in file.readAsLinesSync()) {
      if (line.isEmpty) continue;
      final json = (jsonDecode(line) as Map).cast<String, Object?>();
      final key = json['idempotencyKey']! as String;
      _jobs.putIfAbsent(
        key,
        () => RemoteStoredJob(
          actionId: json['actionId']! as String,
          canonical: json['canonical']! as String,
          receipt: RemoteControlReceipt.fromJson(
            (json['receipt']! as Map).cast<String, Object?>(),
          ),
        ),
      );
    }
  }

  File? get _file =>
      _directory == null ? null : File('${_directory.path}/jobs.jsonl');
}
