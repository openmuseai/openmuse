import subprocess
import os

files = [
    'src/cn/wps/io/dom/ElementPath.rs',
    'src/cn/wps/io/dom/dtd/AttributeDecl.rs',
    'src/cn/wps/io/dom/dtd/ElementDecl.rs',
    'src/cn/wps/io/dom/dtd/ExternalEntityDecl.rs',
    'src/cn/wps/io/dom/dtd/InternalEntityDecl.rs',
    'src/cn/wps/io/xwpf/ChartSrc.rs',
    'src/cn/wps/io/xwpf/DiagramColors.rs',
    'src/cn/wps/io/xwpf/DiagramData.rs',
    'src/cn/wps/io/xwpf/DiagramDrawing.rs',
    'src/cn/wps/io/xwpf/DiagramLayout.rs',
    'src/cn/wps/io/xwpf/DiagramStyle.rs',
    'src/cn/wps/io/xwpf/RecipientData.rs',
    'src/cn/wps/io/xwpf/element_handler/IRunTrackChangeChildHandler.rs',
    'src/cn/wps/io/xwpf/element_handler/IRunTrackChangeHandler.rs',
    'src/cn/wps/io/xwpf/element_handler/prop/IPropBaseHandlerHelper.rs',
]

passed = 0
failed = 0
for f in files:
    r = subprocess.run(f'java2rust run check --file {f}', shell=True, capture_output=True, text=True, encoding='utf-8', errors='replace')
    out = r.stdout + r.stderr
    if 'PASSED' in out:
        passed += 1
    else:
        failed += 1
        fname = os.path.basename(f)
        for line in out.split('\n'):
            if 'Missing' in line or 'FAILED' in line:
                print(f'FAIL {fname}: {line.strip()[:120]}')
                break

print(f'\n=== {passed} passed, {failed} failed ===')
