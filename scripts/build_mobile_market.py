#!/usr/bin/env python3
"""Build checked-in, application-signed Mobile sample artifacts."""
import hashlib
import json
import runpy
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] / 'packages/openmuse_mobile_plugin_runtime'
ASSETS = ROOT / 'assets'
ASSETS.mkdir(exist_ok=True)

CATALOG = [
    dict(id='com.openmuse.pptx', name='PPTX 阅读器',
         version='0.1.0', description='通过 OpenMuse Host 的受限 PPTX 引擎，在 Android Cloud Workspace 查看幻灯片文字。',
         author='OpenMuse · GenOffice Core', platforms=['android'], source='https://github.com/genspark-ai/genoffice',
         categories=['办公'], media=[], license='Apache-2.0',
         file='pptx.js', bundle='pptx-mobile.json'),
    dict(id='com.openmuse.mobile.task-board', name='任务看板 Mobile 适配演示',
         version='0.1.0', description='可填写、添加任务的 A2UI + QuickJS 看板。功能参考 DSH task-board 类型，非原包移植。',
         author='OpenMuse', platforms=['android', 'ios'], source='https://www.npmjs.com/package/@linxin666/dsh-client-ui-task-board',
         categories=['办公', '编程'],
         media=[dict(type='image', uri='asset:packages/openmuse_market_ui/assets/task-board-editorial.png')],
         file='task_board.js', bundle='task-board.json'),
    dict(id='com.openmuse.mobile.usage-budget', name='用量与预算 Mobile 适配演示',
         version='0.1.0', description='本地示例预算交互；真实 DSH 用量需要后续 Host 授权接入。',
         author='OpenMuse', platforms=['android', 'ios'], source='https://dsh.directory/plugins/laoyuehanni/dsh-token-usage',
         categories=['办公'], media=[],
         file='usage_budget.js', bundle='usage-budget.json'),
]
entries = []
for entry in CATALOG:
    source = (ROOT / 'market_sources' / entry['file']).read_text()
    source_digest = hashlib.sha256(source.encode()).hexdigest()
    targets = [
        dict(os='android', arch='aarch64', libc='bionic') if platform == 'android'
        else dict(os='ios', arch='aarch64', libc='darwin')
        for platform in entry['platforms']
    ]
    manifest = dict(
        manifest_version=2, id=entry['id'], name=entry['name'], version=entry['version'],
        protocol=dict(major=1, minor=0),
        ui_runtime=dict(kind='a2ui', entrypoint='main'),
        execution_connector=dict(kind='quickjs', protocol='openmuse.quickjs.bridge/v1'),
        compatibility=dict(targets=[dict(target=target, status='supported') for target in targets]),
        artifacts=[dict(id=f"js-{target['os']}", kind='js-bundle', target=target,
                        digest=dict(algorithm='sha256', value=source_digest),
                        license=entry.get('license', 'AGPL-3.0'), abi='quickjs-es2020') for target in targets],
        activation_events=['onPanel:mobile.plugin'], requested_permissions=[],
        presentation=dict(surfaces=['editor'], remote_capable=False),
        contributes=dict(panels=[dict(id=f"mobile.plugin.{entry['id']}", region='editor')]),
    )
    bundle = dict(format='openmuse-mobile-js/v1', uiProtocol='a2ui/v0.9', engine='quickjs',
                  id=entry['id'], version=entry['version'], js=source, manifest=manifest)
    data = (json.dumps(bundle, ensure_ascii=False, separators=(',', ':')) + '\n').encode()
    (ASSETS / entry['bundle']).write_bytes(data)
    entries.append({k: v for k, v in entry.items() if k not in ('file', 'license')} |
                   {'sha256': hashlib.sha256(data).hexdigest()})
(ASSETS / 'market.json').write_text(json.dumps(entries, ensure_ascii=False, indent=2) + '\n')

# Keep the canonical build command in sync with the WebView/A2UI catalog.
# It generates the legacy PPTX package first, then upgrades the two demos.
runpy.run_path(str(Path(__file__).with_name('generate_mobile_web_demo_bundles.py')))
