import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:pub_semver/pub_semver.dart';

import 'update_feed.dart';

/// Reads the version embedded in the installed binary, then checks the public
/// release feed before the application presents its main workflow.
Future<OpenMuseUpdateController?> bootOpenMuseUpdates({
  required RuntimeConfig config,
}) async {
  // Store binaries use the App Store for updates. Do not even request the
  // external release feed; a remotely configurable UI flag is insufficient.
  if (Platform.isMacOS &&
      const bool.fromEnvironment('OPENMUSE_MAC_APP_STORE')) return null;
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
  const originOverride = String.fromEnvironment('OPENMUSE_UPDATE_ORIGIN');
  final origin = !kDebugMode || originOverride.isEmpty
      ? config.serviceUri(ServiceId.updates)
      : Uri.parse(originOverride);
  const channelOverride = String.fromEnvironment('OPENMUSE_UPDATE_CHANNEL');
  final channel = !kDebugMode || channelOverride.isEmpty
      ? config.updateChannel
      : channelOverride;
  final info = await PackageInfo.fromPlatform();
  final controller = OpenMuseUpdateController(
    feed: OpenMuseUpdateFeed(
      origin: origin,
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
