"""Build an ordinary OpenMuse client with the compiled SiYuan plugin for review."""
import argparse
from datetime import datetime
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
CLIENT = ROOT.parents[2] / 'Muse-Client'
PACKAGE = 'com.openmuseai.mobile.siyuanreview'
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('platform', choices=['macos', 'android', 'ios-simulator', 'ios'])
parser.add_argument('--simulator', help='Booted ARM64 simulator UDID, required for ios-simulator')
parser.add_argument('--output', type=Path, default=CLIENT / 'target/manual-acceptance/siyuan' / datetime.now().strftime('%Y-%m-%d'))
args = parser.parse_args()
if args.platform == 'ios-simulator' and not args.simulator:
    parser.error('--simulator is required for ios-simulator')
args.output.mkdir(parents=True, exist_ok=True)
app = CLIENT / ('app/openmuse_host' if args.platform == 'macos' else 'app/openmuse_mobile')
env = os.environ.copy()
env['GRADLE_USER_HOME'] = os.path.expanduser(env.get('GRADLE_USER_HOME', '~/.gradle'))
env['GRADLE_OPTS'] = env.get('GRADLE_OPTS', '') + ' -Dorg.gradle.daemon=false'
log = args.output / f'{args.platform}-build.log'
commands = []
with log.open('w') as output:
    def run(command, cwd=app):
        commands.append(command)
        subprocess.run(command, cwd=cwd, env=env, stdout=output, stderr=subprocess.STDOUT, check=True)
    if args.platform == 'macos':
        run(['flutter', 'build', 'macos', '--debug', '--no-pub', '--target', 'lib/main.dart'])
        source = app / 'build/macos/Build/Products/Debug/OpenMuse.app'
        artifact = args.output / 'OpenMuse-macos.app'
    elif args.platform == 'android':
        env['OPENMUSE_ANDROID_DEBUG_APPLICATION_ID_SUFFIX'] = '.siyuanreview'
        run(['./gradlew', '-I', str(ROOT / 'scripts/android_arm64.init.gradle'), '--no-daemon', '-Ptarget-platform=android-arm64', '-Ptarget=' + str(app / 'lib/main.dart'), '-Pbase-application-name=android.app.Application', '-Pdart-obfuscation=false', '-Ptrack-widget-creation=true', '-Ptree-shake-icons=false', 'assembleDebug'], cwd=app / 'android')
        source = app / 'build/app/outputs/flutter-apk/app-debug.apk'
        artifact = args.output / 'OpenMuse-android-arm64-debug.apk'
    elif args.platform == 'ios-simulator':
        run(['flutter', 'build', 'ios', '--simulator', '--debug', '--no-pub', '--config-only', '--target', 'lib/main.dart'])
        run(['xcodebuild', '-workspace', 'Runner.xcworkspace', '-scheme', 'Runner', '-configuration', 'Debug', '-sdk', 'iphonesimulator',
             '-destination', 'id=' + args.simulator, 'ARCHS=arm64', 'ONLY_ACTIVE_ARCH=YES', 'EXCLUDED_ARCHS=x86_64',
             'BUILD_DIR=' + str(app / 'build/ios'), 'CODE_SIGNING_ALLOWED=YES', 'CODE_SIGN_IDENTITY=-',
             'OPENMUSE_DEBUG_BUNDLE_IDENTIFIER=' + PACKAGE, '-quiet'], cwd=app / 'ios')
        source = app / 'build/ios/Debug-iphonesimulator/Runner.app'
        artifact = args.output / 'OpenMuse-iOS-simulator.app'
    else:
        run(['flutter', 'build', 'ios', '--release', '--no-codesign', '--no-pub', '--target', 'lib/main.dart'])
        source = app / 'build/ios/iphoneos/Runner.app'
        artifact = args.output / 'OpenMuse-iOS-device-unsigned.app'

# Replace generated review outputs only; application sandbox data lives elsewhere.
staging = artifact.with_name(artifact.name + '.staging')
if staging.is_dir(): shutil.rmtree(staging)
elif staging.exists(): staging.unlink()
subprocess.run(['/bin/cp', '-cR' if source.is_dir() else '-c', str(source), str(staging)], check=True)
if artifact.is_dir(): shutil.rmtree(artifact)
elif artifact.exists(): artifact.unlink()
staging.rename(artifact)
# Flutter incremental native-asset hooks may update already-signed nested
# frameworks. Seal the review bundle after all build and clone steps complete.
signature_resealed = False
if args.platform == 'macos':
    verify = ['codesign', '--verify', '--deep', '--strict', str(artifact)]
    checked = subprocess.run(verify, capture_output=True, text=True)
    with log.open('a') as output:
        if checked.returncode:
            output.write(checked.stderr)
            command = ['codesign', '--force', '--sign', '-', '--preserve-metadata=identifier,entitlements,flags,requirements,runtime', str(artifact)]
            commands.append(command)
            subprocess.run(command, stdout=output, stderr=subprocess.STDOUT, check=True)
            signature_resealed = True
        commands.append(verify)
        subprocess.run(verify, stdout=output, stderr=subprocess.STDOUT, check=True)

record = {'platform': args.platform, 'artifact': str(artifact.resolve()), 'passed': True, 'commands': commands,
          'runtimeSha256': hashlib.sha256((ROOT / 'flutter_runtime/assets/runtime.zip').read_bytes()).hexdigest(),
          'logSha256': hashlib.sha256(log.read_bytes()).hexdigest(), 'upstream': json.loads((ROOT / 'upstream.json').read_text())}
native = {
    'macos': ['macos/Libraries/libmuse_siyuan.dylib', 'macos/Libraries/libonnxruntime.dylib'],
    'android': ['android/src/main/jniLibs/arm64-v8a/libmuse_siyuan.so', 'android/src/main/jniLibs/arm64-v8a/libsiyuan_ort.so'],
    'ios-simulator': ['ios/Libraries/ios-simulator/libmuse_siyuan.a'],
    'ios': ['ios/Libraries/ios/libmuse_siyuan.a'],
}[args.platform]
record['nativeSourceArtifacts'] = {name: hashlib.sha256((ROOT / 'flutter_runtime' / name).read_bytes()).hexdigest() for name in native}
record['adapterSourceSha256'] = hashlib.sha256((CLIENT / 'plugins/siyuan-client/lib/openmuse_siyuan_plugin.dart').read_bytes()).hexdigest()
if args.platform == 'macos': record.update(signatureVerified=True, signatureResealed=signature_resealed)
if args.platform in ['android', 'ios-simulator']: record['package'] = PACKAGE
if artifact.is_file(): record['sha256'] = hashlib.sha256(artifact.read_bytes()).hexdigest()
(args.output / f'{args.platform}.json').write_text(json.dumps(record, ensure_ascii=False, indent=2) + '\n')
print(json.dumps(record, ensure_ascii=False, indent=2), flush=True)
