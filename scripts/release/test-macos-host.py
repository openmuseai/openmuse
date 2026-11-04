#!/usr/bin/env python3
"""Run Host checks with the bundled signed products and local native engines."""
import argparse
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    env = os.environ.copy()
    assets = ROOT / 'app/openmuse_mobile/assets'
    versions = {}
    for product, prefix in [('word', 'WORD_STACK'), ('pdf', 'PDF_PRODUCT_E2E'),
                            ('spreadsheet', 'SPREADSHEET_PRODUCT_E2E')]:
        directory = assets / f'{product}_product'
        identity = json.loads((directory / 'identity.json').read_text())
        versions[product] = identity['version']
        env.update({f'{prefix}_BUNDLE': str(directory / f'{product}.mcpb'),
                    f'{prefix}_PUBLIC_KEY': str(directory / 'public.pem'),
                    f'{prefix}_KEY_ID': identity['keyId'],
                    f'{prefix}_VERSION': identity['version']})
    office = ROOT.parent / 'ioffice'
    flutter = Path(subprocess.check_output(['which', 'flutter'], text=True).strip()).resolve().parents[1]
    env.update(
        WORD_STACK_PLUGIN_VERSION=versions['word'],
        WORD_CORE_ALLOW_HOST_FONTS_IN_TEST='1',
        WORD_CORE_FONT='/System/Library/Fonts/Supplemental/Arial.ttf',
        WORD_CORE_CJK_FONT='/System/Library/Fonts/Supplemental/Arial Unicode.ttf',
        WORD_CORE_FFI=str(office / 'samples/docx-mvp-render/native/target/aarch64-apple-darwin/release/libword_core_ffi.dylib'),
        OPENMUSE_GRIDCRAFT_DYLIB=str(office / 'samples/libs/spreadsheet/gridcraft_bridge/target/release/libopenmuse_gridcraft_bridge.dylib'),
        KSP_MAC_EDIT_DYLIB=str(office / 'koffice/pdf/rebuild/pdf-flutter-app/assets/native/libkspfreetext.dylib'),
        OPENMUSE_SAMPLE_DOCX=str(office / 'samples/docx-mvp-render/flutter/assets/hello.docx'),
        OPENMUSE_SAMPLE_PDF=str(office / '.agenticOffice/test-cases/pdf/samples/sample.pdf'),
        OPENMUSE_SAMPLE_DIR=str(ROOT / 'app/openmuse_host/test/fixtures'),
        OPENMUSE_SAMPLE_XLSX_FILES='spreadsheet-shell.xlsx;spreadsheet-geometry.xlsx',
        SPREADSHEET_PRODUCT_E2E_SCREENSHOT_DIR=str(output / 'spreadsheet'),
        SPREADSHEET_PRODUCT_E2E_UI_FONT='/System/Library/Fonts/Supplemental/Arial Unicode.ttf',
        SPREADSHEET_PRODUCT_E2E_ICON_FONT=str(flutter / 'bin/cache/artifacts/material_fonts/MaterialIcons-Regular.otf'),
    )
    results = {'kind': 'host-automated-tests', 'signedReleaseAcceptance': False,
               'bundledProductVersions': versions, 'checks': {}}
    for name, command in [('analyze', ['flutter', 'analyze', '--no-pub']),
                          ('tests', ['flutter', 'test', '--no-pub', '--reporter', 'expanded'])]:
        with (output / f'{name}.txt').open('w') as log:
            result = subprocess.run(command, cwd=ROOT / 'app/openmuse_host', env=env,
                                    stdout=log, stderr=subprocess.STDOUT)
        results['checks'][name] = {'exitCode': result.returncode, 'log': f'{name}.txt'}
        print(f'{name}: exit {result.returncode}', flush=True)
    (output / 'result.json').write_text(json.dumps(results, indent=2) + '\n')
    raise SystemExit(0 if all(c['exitCode'] == 0 for c in results['checks'].values()) else 1)


if __name__ == '__main__':
    main()
