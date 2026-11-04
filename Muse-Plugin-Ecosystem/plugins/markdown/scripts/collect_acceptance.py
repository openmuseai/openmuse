"""Collect native test results and fail unless all client checks passed."""
import hashlib
import json
from pathlib import Path
import shutil

ROOT = Path(__file__).resolve().parents[1]
REQUIRED = {
    'localKernel', 'unauthorizedLoopbackDenied', 'notebookAndMarkdownImport',
    'blocksAndSearchIndex', 'markdownExport', 'originalProtyleFrontend',
    'uiInputPersisted', 'nativeExportBridge', 'encryptedNotebookRoundTrip',
    'localOcrRuntime', 'deactivateReactivatePreservesData',
    'hostLifecycleProtected', 'kernelPlatform',
    'kernelLocalTimezone',
}


def main():
    destination = ROOT / 'docs/evidence'
    destination.mkdir(parents=True, exist_ok=True)
    for platform in ['macos', 'ios', 'android']:
        log = ROOT / f'.build/acceptance-{platform}.log'
        if platform == 'android':
            result = json.loads((ROOT / 'dist/acceptance/android.json').read_text())
        else:
            marker = 'SIYUAN_ACCEPTANCE_RESULT '
            lines = [line.split(marker, 1)[1] for line in log.read_text().splitlines() if marker in line]
            if not lines or 'All tests passed!' not in log.read_text():
                raise SystemExit(f'Missing successful native report for {platform}')
            result = json.loads(lines[-1])
        if result.get('platform') != platform or result.get('passed') is not True or any(
                result.get('checks', {}).get(check) is not True for check in REQUIRED):
            raise SystemExit(f'Incomplete or failed native acceptance: {platform}')
        result['logSha256'] = hashlib.sha256(log.read_bytes()).hexdigest()
        result['environment'] = {
            'kind': {'macos': 'Desktop', 'ios': 'Simulator', 'android': 'Physical device'}[platform],
            'architecture': 'arm64',
        }
        (destination / f'{platform}.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
        shutil.copy2(log, ROOT / f'dist/acceptance/{platform}.log')
        print(f'{platform}: {len(REQUIRED)} checks passed')


if __name__ == '__main__':
    main()
