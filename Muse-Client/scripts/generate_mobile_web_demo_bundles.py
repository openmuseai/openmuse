#!/usr/bin/env python3
"""Build reproducible bundled WebView/A2UI demonstration artifacts."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ASSETS = ROOT / 'packages/openmuse_mobile_plugin_runtime/assets'
CATALOG = 'https://a2ui.org/specification/v0_9/catalogs/basic/catalog.json'

STYLE = '''<style>
:root{color-scheme:light dark;--bg:#F5F4F1;--surface:#fff;--text:#191D25;--muted:#667080;--accent:#1765E9}
:root[data-openmuse-theme="dark"]{--bg:#171B22;--surface:#242A34;--text:#F1F3F6;--muted:#A9B3C1;--accent:#5B92FF}
*{box-sizing:border-box}body{margin:0;padding:24px;background:var(--bg);color:var(--text);font:16px -apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif}
h1{font-size:30px;letter-spacing:-.04em;margin:0 0 8px}p{color:var(--muted);line-height:1.5}section{background:var(--surface);border-radius:20px;padding:20px;margin:22px 0}
input{font:inherit;width:100%;padding:13px;border:1px solid #8886;border-radius:11px;color:var(--text);background:var(--surface)}
button{font:inherit;border:0;border-radius:11px;padding:12px 16px;background:var(--accent);color:white;min-height:44px;margin-top:12px}li{padding:10px 0;border-bottom:1px solid #8883}li:last-child{border:0}
small{color:var(--muted)}strong{font-size:25px}
</style>'''

DEMOS = {
    'task-board': {
        'title': '任务看板',
        'summary': '本地演示 · 打开后新增任务',
        'height': 'medium',
        'preview': ['本地任务清单', '点击进入并添加任务'],
        'html': '''<!doctype html><html lang="zh-CN"><head><meta name="viewport" content="width=device-width,initial-scale=1">''' + STYLE + '''</head><body>
<h1>任务看板</h1><p>本地交互演示。此数据仅在当前插件页面存在。</p>
<section><label for="task">新任务</label><input id="task" maxlength="80" placeholder="例如：整理本周计划"><button id="add">添加任务</button><ul id="tasks"></ul></section>
<script>
const tasks=['梳理 Mobile 插件协议','验证 Android 真机安装'];const list=document.getElementById('tasks');
function render(){list.replaceChildren();for(const task of tasks){const item=document.createElement('li');item.textContent=task;list.append(item)}}
document.getElementById('add').onclick=()=>{const input=document.getElementById('task');const value=input.value.trim().slice(0,80);if(value){tasks.push(value);input.value='';render()}};render();
</script></body></html>''',
    },
    'usage-budget': {
        'title': '用量与预算',
        'summary': '本地示例 · 非真实 DSH 用量',
        'height': 'small',
        'preview': ['示例预算 ¥100', '打开查看交互演示'],
        'html': '''<!doctype html><html lang="zh-CN"><head><meta name="viewport" content="width=device-width,initial-scale=1">''' + STYLE + '''</head><body>
<h1>用量与预算</h1><p>本地示例数据，不代表实际 DSH 账户用量。</p>
<section><small>示例预算</small><div><strong>¥100</strong></div><p id="spent"></p><p id="remaining"></p><button id="add">模拟增加 ¥5 用量</button></section>
<script>
let spent=36;function render(){document.getElementById('spent').textContent='模拟已用：¥'+spent;document.getElementById('remaining').textContent='模拟剩余：¥'+Math.max(0,100-spent)}
document.getElementById('add').onclick=()=>{spent+=5;render()};render();
</script></body></html>''',
    },
}


def preview(lines):
    return [
        {'version': 'v0.9', 'createSurface': {'surfaceId': 'main', 'catalogId': CATALOG}},
        {'version': 'v0.9', 'updateComponents': {'surfaceId': 'main', 'components': [
            {'id': 'root', 'component': 'Column', 'children': [f'line{i}' for i in range(len(lines))]},
            *[{'id': f'line{i}', 'component': 'Text', 'text': line} for i, line in enumerate(lines)],
        ]}},
    ]

market_path = ASSETS / 'market.json'
market = json.loads(market_path.read_text())
for key, demo in DEMOS.items():
    filename = f'{key}.json'
    old = json.loads((ASSETS / filename).read_text())
    manifest = old['manifest']
    manifest['version'] = '0.2.0'
    manifest['ui_runtime'] = {'kind': 'web-view', 'entrypoint': 'index.html'}
    manifest['execution_connector'] = {'kind': 'none'}
    manifest['requested_permissions'] = []
    digest = hashlib.sha256(demo['html'].encode()).hexdigest()
    for artifact in manifest['artifacts']:
        artifact['id'] = artifact['id'].replace('js-', 'web-')
        artifact['kind'] = 'web-bundle'
        artifact['digest']['value'] = digest
        artifact['abi'] = 'html5'
    bundle = {
        'format': 'openmuse-mobile-web/v1', 'uiProtocol': 'a2ui/v0.9',
        'id': old['id'], 'version': '0.2.0', 'html': demo['html'],
        'card': {'title': demo['title'], 'summary': demo['summary'], 'columns': 1, 'height': demo['height']},
        'preview': preview(demo['preview']), 'manifest': manifest,
    }
    data = json.dumps(bundle, ensure_ascii=False, separators=(',', ':')).encode()
    (ASSETS / filename).write_bytes(data)
    for entry in market:
        if entry['bundle'] == filename:
            entry['version'] = bundle['version']
            entry['runtime'] = 'webview'
            entry['permissions'] = []
            entry['sha256'] = hashlib.sha256(data).hexdigest()
            entry['description'] = demo['summary'] + '；详情采用平台 WebView，工作台卡片采用 A2UI。'
for entry in market:
    entry.setdefault('runtime', 'quickjs')
    entry.setdefault('permissions', [])
priority = {'task-board.json': 0, 'usage-budget.json': 1, 'pptx-mobile.json': 2}
market.sort(key=lambda entry: priority.get(entry['bundle'], 100))
market_path.write_text(json.dumps(market, ensure_ascii=False, indent=2) + '\n')
