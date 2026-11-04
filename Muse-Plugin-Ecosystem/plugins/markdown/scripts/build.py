"""Build plugin-owned artifacts from a fixed, read-only SiYuan checkout."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import zipfile

ROOT = Path(__file__).resolve().parents[1]
VENDOR = ROOT / 'vendors/siyuan'
BUILD = ROOT / '.build'
STAGED = BUILD / 'siyuan'
RUNTIME = ROOT / 'flutter_runtime'
PIN = json.loads((ROOT / 'upstream.json').read_text())


def run(*args, cwd=ROOT, env=None):
    subprocess.run([str(a) for a in args], cwd=cwd, env=env, check=True)


def patch(text, before, after):
    if text.count(before) != 1:
        raise ValueError(f'Upstream embedding boundary changed: {before!r}')
    return text.replace(before, after)


def write_changed(path, text):
    if not path.exists() or path.read_text() != text:
        temp = path.with_suffix(path.suffix + '.tmp')
        temp.write_text(text)
        temp.replace(path)


def prepare():
    commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=VENDOR, text=True).strip()
    if commit != PIN['commit']:
        raise ValueError(f'Expected upstream {PIN["commit"]}, found {commit}')
    if subprocess.check_output(['git', 'status', '--porcelain'], cwd=VENDOR, text=True).strip():
        raise ValueError('Upstream has local changes; refusing to build an unrecorded source tree')
    if not (STAGED / 'kernel/go.mod').exists():
        for name in ['kernel', 'app']:
            shutil.copytree(VENDOR / name, STAGED / name,
                            ignore=shutil.ignore_patterns('node_modules', '.git', 'build', 'kernel.aar'))
    scripts = STAGED / 'scripts'
    scripts.mkdir(exist_ok=True)
    for name in ['prepare-ocr.py', 'ocr-assets.json']:
        write_changed(scripts / name, (VENDOR / 'scripts' / name).read_text())
    target = STAGED / 'kernel/museplugin'
    target.mkdir(parents=True, exist_ok=True)
    write_changed(target / 'main.go', (ROOT / 'native/main.go').read_text())
    policy = STAGED / 'kernel/musepluginpolicy'
    policy.mkdir(exist_ok=True)
    for path in (ROOT / 'native/policy').glob('*.go'):
        write_changed(policy / path.name, path.read_text())
    # Pin listening to loopback even if a notebook's persisted system settings
    # enable networkServe. This is an embedded client, never a LAN server.
    source = (VENDOR / 'kernel/server/serve.go').read_text()
    source = patch(source, 'if confSystemSnapshot.NetworkServe || util.ContainerDocker == util.Container {', 'if false { // OpenMuse plugin: loopback only')
    source = patch(source, '"github.com/gin-gonic/gin"',
                   '"github.com/gin-gonic/gin"\n "github.com/siyuan-note/siyuan/kernel/musepluginpolicy"')
    source = patch(source, 'ginServer := gin.New()', 'ginServer := gin.New()\n ginServer.Use(musepluginpolicy.Session)')
    write_changed(STAGED / 'kernel/server/serve.go', source)
    # OpenMuse owns synchronization and provider consent. The embedded editor
    # must not start SiYuan's separate cloud/update/account polling services.
    # Keep upstream untouched and retain local persistence, indexing and history.
    source = (VENDOR / 'kernel/job/cron.go').read_text()
    for subscription in [
        '\tgo every(5*time.Second, model.SyncDataJob)\n',
        '\tgo every(6*time.Hour, util.RefreshRhyResultJob, "RefreshRhyResultJob")\n',
        '\tgo every(2*time.Hour, model.RefreshCheckJob2H)\n',
        '\tgo every(6*time.Hour, model.RefreshCheckJob6H)\n',
        '\tif util.IsMobileContainer() {\n\t\tgo every(3*time.Second, model.AutoConsumeShorthandsJob)\n\t}\n',
    ]:
        source = patch(source, subscription, '')
    write_changed(STAGED / 'kernel/job/cron.go', source)
    # WKWebView exposes messageHandlers in browser mode too. Recognize the
    # original native SiYuan bridge only when its actual handler is installed.
    relative = 'app/src/protyle/util/compatibility.ts'
    source = (VENDOR / relative).read_text()
    source = patch(source, 'window.siyuan.config.system.container === "ios" && window.webkit?.messageHandlers;',
                   'window.siyuan.config.system.container === "ios" && window.webkit?.messageHandlers?.startKernelFast;')
    source = 'import {saveClientExport} from "../../util/openMuseClientExport";\n' + source
    source = patch(source, '            const openUrl = new URL(uri, `${location.origin}/`);',
                   '            const nativeResult = await saveClientExport(uri);\n            if (nativeResult) {\n                result = nativeResult;\n                hasCompletionResult = true;\n            } else {\n            const openUrl = new URL(uri, `${location.origin}/`);')
    source = patch(source, '            result = {status: "success"};\n        }\n        if (msgId)',
                   '            result = {status: "success"};\n            }\n        }\n        if (msgId)')
    write_changed(STAGED / relative, source)
    relative = 'app/src/editor/util.ts'
    source = (VENDOR / relative).read_text()
    source = patch(source, '    const response = await fetchSyncPost("/api/block/getBlockInfo", {id: options.id, notebook: options.notebookId});',
                   '    if ((window as any).openMuseResource) { await (window as any).openMuseResource.openBlock(options.id); return; }\n    const response = await fetchSyncPost("/api/block/getBlockInfo", {id: options.id, notebook: options.notebookId});')
    source = patch(source, 'export const openFile = async (options: IOpenFileOptions) => {',
                   'export const openFile = async (options: IOpenFileOptions) => {\n    if ((window as any).openMuseResource && (options.id || options.rootID)) { await (window as any).openMuseResource.openBlock(options.id || options.rootID); return; }')
    write_changed(STAGED / relative, source)
    relative = 'app/src/layout/getAll.ts'
    source = (VENDOR / relative).read_text()
    source = patch(source, '    const editors: Protyle[] = [];',
                   '    const editors: Protyle[] = [];\n    /// #if !MOBILE\n    if ((window as any).openMuseResourceEditor) editors.push((window as any).openMuseResourceEditor);\n    /// #endif')
    write_changed(STAGED / relative, source)
    relative = 'app/src/layout/status.ts'
    source = (VENDOR / relative).read_text()
    # Protyle selection/keyboard handlers also serve standalone editors. Word
    # counting must not require the original application status bar to exist.
    for signature in [
        'export const countSelectWord = (range: Range, context?: string | IProtyle) => {',
        'export const countBlockWord = (ids: string[], context?: string | IProtyle, clearCache = false) => {',
        'export const clearCounter = () => {',
        'export const renderStatusbarCounter = (stat: IBlockStat, statWithEmbed?: IBlockStat, embedStat?: IEmbedStat) => {',
    ]:
        source = patch(source, signature, signature + '\n    if (!document.getElementById("status")) return;')
    write_changed(STAGED / relative, source)
    relative = 'kernel/model/import.go'
    source = (VENDOR / relative).read_text()
    before = '''			assetPathMap[assetPath] = assetPath
			return nil
		}); err != nil {
			return nil, err
		}
		if err = filelock.Copy(assetsDir, dataAssets); err != nil {
			logging.LogErrorf("copy assets from [%s] to [%s] failed: %s", assetsDir, dataAssets, err)
			return nil, err
		}'''
    after = '''			data, readErr := filelock.ReadFile(currentPath)
			if readErr != nil { return readErr }
			// 每个导入会话分配附件身份，正文和数据库引用由原生重写器一起更新。
			name := util.AssetName(util.RemoveID(filepath.Base(currentPath)), ast.NewNodeID())
			diskName, storeErr := storeAssetForBox(boxID, dataAssets, name, data)
			if storeErr != nil { return storeErr }
			assetPathMap[assetPath] = path.Join("assets", diskName)
			annotation := currentPath + ".sya"
			if filelock.IsExist(annotation) {
				if copyErr := filelock.Copy(annotation, filepath.Join(dataAssets, diskName+".sya")); copyErr != nil { return copyErr }
			}
			return nil
		}); err != nil {
			return nil, err
		}'''
    source = patch(source, before, after)
    source = patch(source, '\tdataAssets := filepath.Join(util.DataDir, "assets")',
                   '\tdataAssets := filepath.Join(util.DataDir, "assets")\n\tif err = os.MkdirAll(dataAssets, 0755); err != nil { return nil, err }')
    write_changed(STAGED / relative, source)
    relative = 'kernel/model/conf.go'
    source = (VENDOR / relative).read_text()
    source = patch(source, 'if util.IsMobileContainer() {\n\t\tlogging.LogInfof("exited kernel, waiting for mobile host to quit")',
                   'if true { // Embedded library: never terminate the Flutter host\n\t\tlogging.LogInfof("exited kernel, waiting for embedding host to quit")')
    write_changed(STAGED / relative, source)
    relative = 'kernel/util/working_mobile.go'
    source = (VENDOR / relative).read_text()
    source = patch(source, '\tos.Setenv("TMPDIR", osTmpDir)\n\tos.Setenv("TEMP", osTmpDir)\n\tos.Setenv("TMP", osTmpDir)',
                   '\t// Embedded plugin: preserve the Flutter process environment.\n\t// Persistent notebook work files already use util.TempDir explicitly.')
    write_changed(STAGED / relative, source)
    write_changed(STAGED / 'app/src/util/openMuseClientExport.ts', (ROOT / 'web/native_export.ts').read_text())
    write_changed(STAGED / 'app/src/openMuseResource.ts', (ROOT / 'web/resource_entry.ts').read_text())
    write_changed(STAGED / 'app/src/assets/template/desktop/openMuseResource.tpl', (ROOT / 'web/resource.tpl').read_text())
    write_changed(STAGED / 'app/src/assets/template/mobile/openMuseResource.tpl', (ROOT / 'web/resource_mobile.tpl').read_text())
    write_changed(STAGED / 'app/webpack.resource.cjs', (ROOT / 'web/webpack.resource.cjs').read_text())
    relative = 'app/src/util/functions.ts'
    source = (VENDOR / relative).read_text()
    source = patch(source, 'return document.getElementById("sidebar") ? true : false;',
                   'return document.documentElement.dataset.openmuseResourcePlatform === "mobile" || !!document.getElementById("sidebar");')
    write_changed(STAGED / relative, source)
    relative = 'app/src/mobile/util/initFramework.ts'
    source = (VENDOR / relative).read_text()
    source = patch(source, 'const initEditorName = () => {', 'export const initEditorName = () => {')
    # Reuse the complete upstream mobile title validation, IME and Enter logic;
    # expose its existing pending save to the enclosing resource commit boundary.
    assert source.rstrip().endswith('};')
    source = source.rstrip()[:-2] + '    let savedValue: string; let savedRoot: string;\n    return async () => { const root = window.siyuan.mobile.editor?.protyle.block.rootID; if (root === savedRoot && inputElement.value === savedValue) return titleSavePromise; await saveTitle(); savedRoot = root; savedValue = inputElement.value; };\n};\n'
    write_changed(STAGED / relative, source)
    relative = 'app/src/mobile/editor.ts'
    source = (VENDOR / relative).read_text()
    source = patch(source, '    clearMobileSearchHighlight();\n    const closing = closeMobileEditorSheets();',
                   '    if ((window as any).openMuseResource) return (window as any).openMuseResource.openBlock(id).then(() => true);\n    clearMobileSearchHighlight();\n    const closing = closeMobileEditorSheets();')
    write_changed(STAGED / relative, source)
    relative = 'app/src/protyle/header/Title.ts'
    source = (VENDOR / relative).read_text()
    source = patch(source, '    }\n\n    public render(protyle: IProtyle, response: IWebSocketData) {',
                   '        (window as any).openMuseResource?.refreshTitle();\n    }\n\n    public render(protyle: IProtyle, response: IWebSocketData) {')
    write_changed(STAGED / relative, source)
    relative = 'app/src/editor/openLink.ts'
    source = (VENDOR / relative).read_text()
    source = patch(source, '    if (processSiYuanUri(app, linkAddress)) {',
                   '    if ((window as any).openMuseResource?.openLink(linkAddress, isPreviewableAsset(linkAddress))) return;\n    if (processSiYuanUri(app, linkAddress)) {')
    source = patch(source, '    if (processSiYuanUri(window.siyuan.ws.app, uri)) {',
                   '    if ((window as any).openMuseResource?.openLink(uri)) return;\n    if (processSiYuanUri(window.siyuan.ws.app, uri)) {')
    write_changed(STAGED / relative, source)
    relative = 'kernel/model/ocr.go'
    source = (VENDOR / relative).read_text()
    source = source.replace('"github.com/siyuan-note/siyuan/kernel/util"',
                            '"github.com/siyuan-note/siyuan/kernel/util"\n "github.com/siyuan-note/siyuan/kernel/musepluginpolicy"')
    source = patch(source, '\t\tresult.Library = "libonnxruntime.so"\n\t}\n\treturn result',
                   '\t\tresult.Library = "libonnxruntime.so"\n\t}\n\tif musepluginpolicy.OCRLibrary != "" { result.Library = musepluginpolicy.OCRLibrary }\n\treturn result')
    write_changed(STAGED / relative, source)
    BUILD.mkdir(exist_ok=True)
    (BUILD / 'source.json').write_text(json.dumps(PIN, indent=2) + '\n')


def web():
    prepare()
    app = STAGED / 'app'
    run(sys.executable, STAGED / 'scripts/prepare-ocr.py', '--runtime', 'none', cwd=STAGED)
    run('pnpm', 'install', '--ignore-scripts', cwd=app)
    # Build only the isolated copy. Never clean or overwrite vendor stage/build.
    for name in ['desktop', 'mobile', 'export']:
        run('pnpm', 'run', f'build:{name}', cwd=app)
    run('pnpm', 'exec', 'webpack', '--mode', 'production', '--config', 'webpack.resource.cjs', cwd=app)
    run('pnpm', 'exec', 'webpack', '--mode', 'production', '--config', 'webpack.resource.cjs', '--env', 'mobile=true', cwd=app)
    pack_web()


def resource_web():
    prepare()
    run('pnpm', 'exec', 'webpack', '--mode', 'production', '--config', 'webpack.resource.cjs', cwd=STAGED / 'app')
    run('pnpm', 'exec', 'webpack', '--mode', 'production', '--config', 'webpack.resource.cjs', '--env', 'mobile=true', cwd=STAGED / 'app')
    pack_web()


def pack_web():
    app = STAGED / 'app'
    files = []
    for folder in ['appearance', 'stage', 'guide']:
        files += [p for p in (app / folder).rglob('*') if p.is_file() and '/stage/ocr/runtime/' not in str(p)]
    files += [VENDOR / 'LICENSE', VENDOR / 'THIRD_PARTY_NOTICES.md', ROOT / 'upstream.json']
    assets = RUNTIME / 'assets'
    assets.mkdir(parents=True, exist_ok=True)
    catalog = {}
    with zipfile.ZipFile(assets / 'runtime.zip', 'w', zipfile.ZIP_DEFLATED) as archive:
        for path in sorted(files):
            name = 'app/' + path.relative_to(app).as_posix() if path.is_relative_to(app) else path.name
            data = path.read_bytes()
            catalog[name] = hashlib.sha256(data).hexdigest()
            info = zipfile.ZipInfo(name, (2020, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            archive.writestr(info, data)
        archive.writestr('catalog.json', json.dumps(catalog, sort_keys=True))
    print(assets / 'runtime.zip', flush=True)


def native(target):
    prepare()
    env = os.environ.copy()
    env['CGO_ENABLED'] = '1'
    env.setdefault('GOCACHE', str(BUILD / 'go-cache'))
    go = env.get('SIYUAN_GO', 'go')
    run(go, 'version')
    run(str(Path(go).with_name('gofmt')) if '/' in go else 'gofmt', '-w', STAGED / 'kernel/museplugin/main.go', STAGED / 'kernel/musepluginpolicy/policy.go')
    if target == 'macos':
        output = RUNTIME / 'macos/Libraries/libmuse_siyuan.dylib'
        env.update(GOOS='darwin', GOARCH='arm64', MACOSX_DEPLOYMENT_TARGET='12.0')
        env['CGO_CFLAGS'] = env.get('CGO_CFLAGS', '') + ' -mmacosx-version-min=12.0'
        env['CGO_LDFLAGS'] = env.get('CGO_LDFLAGS', '') + ' -mmacosx-version-min=12.0'
        mode = 'c-shared'
        run(sys.executable, STAGED / 'scripts/prepare-ocr.py', '--runtime', 'darwin-arm64', cwd=STAGED)
        output.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(STAGED / 'app/stage/ocr/runtime/darwin-arm64/libonnxruntime.dylib', output.parent / 'libonnxruntime.dylib')
    elif target == 'android':
        run(sys.executable, ROOT / 'scripts/android_onnx.py')
        sdk = Path(env.get('ANDROID_HOME', str(Path.home() / 'Library/Android/sdk')))
        ndk = Path(env.get('ANDROID_NDK_HOME', str(sdk / 'ndk/28.2.13676358')))
        env.update(GOOS='android', GOARCH='arm64',
                   CC=str(ndk / 'toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android24-clang'))
        output = RUNTIME / 'android/src/main/jniLibs/arm64-v8a/libmuse_siyuan.so'
        mode = 'c-shared'
    else:
        simulator = target == 'ios-simulator'
        sdk = 'iphonesimulator' if simulator else 'iphoneos'
        sdk_path = subprocess.check_output(['xcrun', '--sdk', sdk, '--show-sdk-path'], text=True).strip()
        compiler = BUILD / f'clang-{target}'
        triple = 'arm64-apple-ios15.0-simulator' if simulator else 'arm64-apple-ios15.0'
        compiler.write_text(f'#!/bin/sh\nexec xcrun --sdk {sdk} clang -target {triple} -isysroot "{sdk_path}" "$@"\n')
        compiler.chmod(0o755)
        env.update(GOOS='ios', GOARCH='arm64', CC=str(compiler))
        output = RUNTIME / f'ios/Libraries/{target}/libmuse_siyuan.a'
        mode = 'c-archive'
    output.parent.mkdir(parents=True, exist_ok=True)
    run(go, 'build', '-tags', 'fts5 sqlcipher', '-buildmode=' + mode,
        '-ldflags=-s -w', '-o', output, './museplugin', cwd=STAGED / 'kernel', env=env)
    if target == 'macos':
        run('install_name_tool', '-id', '@rpath/libmuse_siyuan.dylib', output)
    elif target in ['ios', 'ios-simulator']:
        framework = RUNTIME / 'ios/Libraries/MuseSiYuan.xcframework'
        if framework.exists(): shutil.rmtree(framework)
        command = ['xcodebuild', '-create-xcframework']
        for name in ['ios', 'ios-simulator']:
            library = RUNTIME / f'ios/Libraries/{name}/libmuse_siyuan.a'
            if library.exists(): command += ['-library', library]
        run(*command, '-output', framework)
    provenance = dict(PIN)
    provenance['embeddingSources'] = {
        path.relative_to(ROOT).as_posix(): hashlib.sha256(path.read_bytes()).hexdigest()
        for path in sorted([Path(__file__), *(ROOT / 'native').rglob('*.go')])
    }
    provenance['artifactSha256'] = hashlib.sha256(output.read_bytes()).hexdigest()
    (output.parent / 'source.json').write_text(json.dumps(provenance, indent=2) + '\n')
    print(output, flush=True)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('target', choices=['prepare', 'web', 'resource-web', 'macos', 'android', 'ios', 'ios-simulator', 'test'])
    args = parser.parse_args()
    if args.target == 'test':
        prepare()
        run(os.environ.get('SIYUAN_GO', 'go'), 'test', './musepluginpolicy', cwd=STAGED / 'kernel')
    elif args.target == 'prepare': prepare()
    elif args.target == 'web': web()
    elif args.target == 'resource-web': resource_web()
    else: native(args.target)
