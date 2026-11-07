import 'dart:io';
import 'package:code_assets/code_assets.dart';
import 'package:hooks/hooks.dart';

Future<void> main(List<String> args) async {
  await build(args, (input, output) async {
    if (!input.config.buildCodeAssets) return;
    final os = input.config.code.targetOS;
    final arch = input.config.code.targetArchitecture;
    // Select the actual target ABI, including simulator vs physical iOS.
    final target = switch ((os, arch)) {
      (OS.macOS, Architecture.arm64) => 'aarch64-apple-darwin',
      (OS.macOS, Architecture.x64) => 'x86_64-apple-darwin',
      (OS.linux, Architecture.x64) => 'x86_64-unknown-linux-gnu',
      (OS.linux, Architecture.arm64) => 'aarch64-unknown-linux-gnu',
      (OS.iOS, Architecture.arm64) =>
        input.config.code.iOS.targetSdk == IOSSdk.iPhoneSimulator
            ? 'aarch64-apple-ios-sim'
            : 'aarch64-apple-ios',
      (OS.iOS, Architecture.x64) => 'x86_64-apple-ios',
      (OS.android, Architecture.arm64) => 'aarch64-linux-android',
      (OS.android, Architecture.x64) => 'x86_64-linux-android',
      _ => throw UnsupportedError(
        'Realtime codec native artifact is not available for $os/$arch',
      ),
    };
    final rust = Directory.fromUri(
      input.packageRoot.resolve(
        '../../../Muse-Plugin-Ecosystem/plugins/markdown/smart_document/runtime/appflowy_realtime/',
      ),
    );
    final environment = <String, String>{
      'CARGO_TARGET_DIR': '${rust.path}/target',
    };
    if (os == OS.iOS) {
      final sdk = input.config.code.iOS.targetSdk == IOSSdk.iPhoneSimulator
          ? 'iphonesimulator'
          : 'iphoneos';
      final sdkRoot = await Process.run('xcrun', [
        '--sdk',
        sdk,
        '--show-sdk-path',
      ]);
      if (sdkRoot.exitCode != 0) throw StateError('iOS SDK not available');
      environment['SDKROOT'] = '${sdkRoot.stdout}'.trim();
      environment['IPHONEOS_DEPLOYMENT_TARGET'] =
          '${input.config.code.iOS.targetVersion}.0';
    }
    if (os == OS.android) {
      final compiler = input.config.code.cCompiler;
      if (compiler == null)
        throw StateError('Android NDK compiler is required');
      final key = target.replaceAll('-', '_').toUpperCase();
      final abi = target == 'aarch64-linux-android'
          ? 'aarch64-linux-android'
          : 'x86_64-linux-android';
      environment['CARGO_TARGET_${key}_LINKER'] = compiler.compiler
          .toFilePath();
      environment['CARGO_TARGET_${key}_RUSTFLAGS'] =
          '-Clink-arg=--target=$abi${input.config.code.android.targetNdkApi}';
    }
    final result = await Process.run('cargo', [
      'build',
      '--locked',
      '--release',
      '--manifest-path',
      '${rust.path}/Cargo.toml',
      '--target',
      target,
    ], environment: environment);
    if (result.exitCode != 0)
      throw StateError('Rust build failed: ${result.stderr}');
    final name = os == OS.macOS || os == OS.iOS
        ? 'libmuse_appflowy_realtime.dylib'
        : 'libmuse_appflowy_realtime.so';
    final source = File('${rust.path}/target/$target/release/$name');
    final artifact = input.outputDirectory.resolve(name);
    await source.copy(artifact.toFilePath());
    output.dependencies.addAll([
      rust.uri.resolve('Cargo.toml'),
      rust.uri.resolve('Cargo.lock'),
    ]);
    output.dependencies.addAll(
      Directory.fromUri(
        rust.uri.resolve('src/'),
      ).listSync(recursive: true).whereType<File>().map((file) => file.uri),
    );
    for (final relative in [
      '../../vendor/appflowy_collab/',
      '../../vendor/appflowy_proto/',
    ]) {
      output.dependencies.addAll(
        Directory.fromUri(
          rust.uri.resolve(relative),
        ).listSync(recursive: true).whereType<File>().map((file) => file.uri),
      );
    }
    output.assets.code.add(
      CodeAsset(
        package: input.packageName,
        name: 'src/native_core_io.dart',
        file: artifact,
        linkMode: DynamicLoadingBundled(),
      ),
    );
  });
}
