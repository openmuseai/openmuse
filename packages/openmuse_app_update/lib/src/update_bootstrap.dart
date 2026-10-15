import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:pub_semver/pub_semver.dart';

import 'update_feed.dart';

/// Reads the version embedded in the installed binary, then checks the public
/// release feed before the application presents its main workflow.
Future<OpenMuseUpdateController?> bootOpenMuseUpdates() async {
  final String? target = Platform.isMacOS
      ? 'macos-universal'
      : Platform.isWindows
      ? 'windows-x64'
      : Platform.isAndroid
      ? 'android-arm64'
      : Platform.isIOS
      ? 'ios-arm64'
      : null;
  if (target == null) return null;
  const origin = String.fromEnvironment(
    'OPENMUSE_UPDATE_ORIGIN',
    defaultValue: 'https://openmuseai.com',
  );
  const channel = String.fromEnvironment(
    'OPENMUSE_UPDATE_CHANNEL',
    defaultValue: 'stable',
  );
  final info = await PackageInfo.fromPlatform();
  final controller = OpenMuseUpdateController(
    feed: OpenMuseUpdateFeed(
      origin: Uri.parse(origin),
      channel: channel,
      target: target,
      allowInsecureLoopback: !kReleaseMode,
    ),
    installedVersion: Version.parse(info.version),
    installedBuild: int.tryParse(info.buildNumber) ?? 0,
  );
  await controller.check();
  return controller;
}
