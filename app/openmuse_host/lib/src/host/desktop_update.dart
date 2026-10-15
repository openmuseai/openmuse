import 'dart:async';
import 'dart:io';

import 'package:crypto/crypto.dart';
import 'package:flutter/foundation.dart';
import 'package:openmuse_app_update/openmuse_app_update.dart';
import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

enum OpenMuseDesktopUpdateStatus { idle, downloading, ready, failed }

/// Downloads a release into the user's application support directory. A
/// completed file is reused only after its size and digest are checked again.
final class OpenMuseDesktopUpdate {
  OpenMuseDesktopUpdate({this.downloadClientFactory, this.stagingDirectory});

  final HttpClient Function()? downloadClientFactory;
  final Directory? stagingDirectory;
  final progress = ValueNotifier<double>(0);
  final status = ValueNotifier<OpenMuseDesktopUpdateStatus>(
    OpenMuseDesktopUpdateStatus.idle,
  );
  String? releaseSha256;
  HttpClient? _activeClient;
  bool _disposed = false;

  void _setProgress(double value) {
    if (_disposed) return;
    // Updating the entire workbench for every network chunk would make a
    // background download compete with editor input. Report whole percents.
    if (value > 0 &&
        value < 1 &&
        (value * 100).floor() == (progress.value * 100).floor()) {
      return;
    }
    progress.value = value;
  }

  Future<File> stage(OpenMuseRelease release) async {
    if (!Platform.isMacOS || release.installKind != 'macos-zip') {
      throw UnsupportedError('此平台尚未提供应用内安装器');
    }
    final support = stagingDirectory ?? await getApplicationSupportDirectory();
    final directory = Directory(p.join(support.path, 'updates'));
    await directory.create(recursive: true);
    final archive = File(
      p.join(
        directory.path,
        'OpenMuse-${release.version}-${release.build}-${release.sha256}.zip',
      ),
    );
    if (await _valid(archive, release)) {
      _setProgress(1);
      return archive;
    }
    if (await archive.exists()) await archive.delete();
    final partial = File('${archive.path}.partial');
    if (await partial.exists() && await partial.length() > release.sizeBytes) {
      await partial.delete();
    }
    _setProgress(
      (await partial.exists() ? await partial.length() : 0) / release.sizeBytes,
    );
    Object? lastError;
    for (var attempt = 0; attempt < 8; attempt++) {
      if (_disposed) throw StateError('更新下载已取消');
      try {
        final received = await _downloadRemaining(partial, release);
        if (received != release.sizeBytes) {
          throw HttpException('下载中断：已接收 $received / ${release.sizeBytes} 字节');
        }
        final digest = await sha256.bind(partial.openRead()).first;
        if (digest.toString() != release.sha256) {
          await partial.delete();
          throw const FormatException('更新文件 SHA-256 校验失败');
        }
        await partial.rename(archive.path);
        _setProgress(1);
        return archive;
      } on FormatException {
        rethrow;
      } catch (error) {
        lastError = error;
        if (attempt == 7) break;
        await Future<void>.delayed(
          Duration(milliseconds: 500 * (1 << (attempt > 4 ? 4 : attempt))),
        );
      }
    }
    throw StateError('更新下载未完成，可点击重试继续：$lastError');
  }

  Future<int> _downloadRemaining(File partial, OpenMuseRelease release) async {
    var offset = await partial.exists() ? await partial.length() : 0;
    if (offset == release.sizeBytes) return offset;
    final client = downloadClientFactory?.call() ?? HttpClient();
    _activeClient = client;
    client.connectionTimeout = const Duration(seconds: 15);
    try {
      final request = await client.getUrl(release.downloadUri);
      request.followRedirects = false;
      request.headers.set(HttpHeaders.acceptEncodingHeader, 'identity');
      if (offset > 0) {
        request.headers.set(HttpHeaders.rangeHeader, 'bytes=$offset-');
      }
      final response = await request.close();
      if (offset > 0 && response.statusCode == HttpStatus.ok) {
        // The origin does not support Range. Start this response from byte 0.
        await partial.delete();
        offset = 0;
      } else if (offset > 0 &&
          response.statusCode == HttpStatus.partialContent) {
        final expected =
            'bytes $offset-${release.sizeBytes - 1}/${release.sizeBytes}';
        if (response.headers.value(HttpHeaders.contentRangeHeader) !=
            expected) {
          await response.drain<void>();
          throw const FormatException('断点续传范围与更新清单不一致');
        }
      } else if (response.statusCode != HttpStatus.ok) {
        await response.drain<void>();
        throw HttpException('下载失败：HTTP ${response.statusCode}');
      }
      final expectedLength = release.sizeBytes - offset;
      if (response.contentLength >= 0 &&
          response.contentLength != expectedLength) {
        await response.drain<void>();
        throw const FormatException('下载大小与更新清单不一致');
      }
      final output = partial.openWrite(
        mode: offset == 0 ? FileMode.write : FileMode.append,
      );
      var received = offset;
      try {
        await for (final chunk in response.timeout(
          const Duration(seconds: 30),
        )) {
          received += chunk.length;
          if (received > release.sizeBytes) {
            throw const FormatException('更新文件超过清单声明的大小');
          }
          output.add(chunk);
          _setProgress(received / release.sizeBytes);
        }
        await output.flush();
      } finally {
        await output.close();
      }
      return received;
    } finally {
      client.close(force: true);
      if (identical(_activeClient, client)) _activeClient = null;
    }
  }

  Future<bool> _valid(File file, OpenMuseRelease release) async {
    if (!await file.exists() || await file.length() != release.sizeBytes) {
      return false;
    }
    final stream = file.openRead();
    final digest = await sha256.bind(stream).first;
    return digest.toString() == release.sha256;
  }

  /// The helper lives outside the bundle while it replaces the old app.
  /// A helper copy is launched only after the user chooses restart now.
  Future<void> installAndRestart(File archive, OpenMuseRelease release) async {
    final executable = Platform.resolvedExecutable;
    final marker = '/Contents/MacOS/';
    final markerAt = executable.indexOf(marker);
    if (markerAt < 0) throw StateError('无法定位当前 OpenMuse.app');
    final appPath = executable.substring(0, markerAt);
    if (p.basename(appPath) != 'OpenMuse.app') {
      throw StateError('应用包名称不正确');
    }
    final helper = File(
      p.join(appPath, 'Contents/Resources/openmuse/update/install_macos.sh'),
    );
    if (!await helper.exists()) throw StateError('更新安装器未包含在应用中');
    final parent = Directory(p.dirname(appPath));
    // Moving app bundles requires write access to their containing folder.
    final probe = File(p.join(parent.path, '.openmuse-update-probe-$pid'));
    try {
      await probe.writeAsString('');
    } on FileSystemException {
      throw StateError('应用目录不可写，请将 OpenMuse.app 移到用户可写目录后更新');
    } finally {
      if (await probe.exists()) await probe.delete();
    }
    final staging = await getApplicationSupportDirectory();
    final copy = File(p.join(staging.path, 'updates', 'install_macos.sh'));
    await helper.copy(copy.path);
    final result = await Process.run('/bin/chmod', ['700', copy.path]);
    if (result.exitCode != 0) throw StateError('无法准备更新安装器');
    await Process.start('/bin/bash', [
      copy.path,
      archive.path,
      appPath,
      '${release.version}',
      '${release.build}',
      '$pid',
    ], mode: ProcessStartMode.detached);
    // The helper waits for this process to exit before replacing the bundle.
    exit(0);
  }

  void dispose() {
    _disposed = true;
    _activeClient?.close(force: true);
    progress.dispose();
    status.dispose();
  }
}
