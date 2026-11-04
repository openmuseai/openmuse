#!/usr/bin/env python3
"""Build script-free English legal/help pages for the overseas HTTPS origin."""
import argparse
import html
import json
import re
import shutil
import subprocess
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
p = argparse.ArgumentParser()
p.add_argument('--output', required=True, type=Path)
a = p.parse_args()
legal = json.loads((ROOT.parent / 'Muse-WebSite/config/legal-documents.json').read_text())['en']
if re.search('[\u4e00-\u9fff]', json.dumps(legal, ensure_ascii=False)):
    raise SystemExit('English legal copy contains Chinese text')
def write(slug, title, body):
    target = a.output / 'en' / slug / 'index.html'
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text('<!doctype html><html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>'+html.escape(title)+' — OpenMuse</title><style>body{font:18px/1.7 system-ui,sans-serif;color:#182230;max-width:820px;margin:48px auto;padding:0 24px}a{color:#365ec9}h1,h2{line-height:1.3}h2{margin-top:48px;scroll-margin-top:24px}section{margin:36px 0}img{display:block;max-width:100%;width:360px;height:auto;margin:24px 0;border:1px solid #e2e6ee;border-radius:12px}li{margin:10px 0}footer{margin-top:64px;border-top:1px solid #ddd;padding-top:20px}</style><header><a href="/">OpenMuse</a></header><main><h1>'+html.escape(title)+'</h1>'+body+'</main><footer>Copyright © OpenMuse 2026 · <a href="mailto:developer@openmuseai.com">Contact support</a></footer></html>')
for kind in ['privacy','terms']:
    doc=legal[kind]; body='<p>'+html.escape(doc['updated'])+'</p>'
    for section in doc['sections']:
        body+='<section><h2>'+html.escape(section['heading'])+'</h2>'
        for block in section['blocks']:
            body+=('<ul>'+''.join('<li>'+html.escape(t)+'</li>' for t in block['items'])+'</ul>') if block['kind']=='list' else '<p>'+html.escape(block['text'])+'</p>'
        body+='</section>'
    write(kind,doc['title'],body)
write('account-deletion','Delete your OpenMuse account','<p>In OpenMuse, open your account settings and choose Delete account. Read the permanent deletion confirmation before continuing.</p><p>You can also request deletion by emailing <a href="mailto:developer@openmuseai.com?subject=OpenMuse%20account%20deletion">developer@openmuseai.com</a> from the registered email address. Specify the United States region. We verify ownership before processing the request. Do not send your password or session tokens.</p><p>The request covers your OpenMuse account and associated cloud data, including device registrations, owned cloud workspaces and submitted AI safety reports. Files you separately saved on your devices or exported to another service must be managed on those devices or services.</p><h2 id="reports-only">Delete safety reports while keeping your account</h2><p>You may ask us to delete your submitted AI safety reports without deleting your account. Email <a href="mailto:developer@openmuseai.com?subject=OpenMuse%20report%20data%20deletion">developer@openmuseai.com</a> from your registered email address and request report deletion for the United States region. We verify account ownership before removing the reports. Other account and workspace data remain. Reports otherwise expire after 30 days.</p>')
write('support','OpenMuse Support','<p>For help with OpenMuse, contact <a href="mailto:developer@openmuseai.com">developer@openmuseai.com</a>. Include your app version, device platform and selected service region. Do not include passwords, session tokens or private documents.</p><p><a href="/en/guide/">Read the user guide</a> for installation, document editing and connecting a Desktop.</p><p><a href="/en/account-deletion/">Account deletion</a> · <a href="/en/privacy/">Privacy Policy</a> · <a href="/en/terms/">Terms of Service</a></p>')
guide=(ROOT/'packages/openmuse_resource_center_host/assets/welcome/OpenMuse-User-Guide.md').read_text()
body = subprocess.check_output(['node', str(ROOT/'scripts/release/render-guide.mjs')], input=guide, text=True)
body = '<p><a href="OpenMuse-User-Guide.sydoc" download>Download the illustrated Notes document (.sydoc)</a> to open in a supported OpenMuse native app.</p>' + body
write('guide','OpenMuse User Guide',body)
shutil.copytree(ROOT/'packages/openmuse_resource_center_host/assets/welcome/guide-images', a.output/'en/guide/guide-images', dirs_exist_ok=True)
shutil.copy2(ROOT/'packages/openmuse_resource_center_host/assets/welcome/OpenMuse-User-Guide.sydoc', a.output/'en/guide/OpenMuse-User-Guide.sydoc')
print('Built English privacy, terms, account deletion, support and user guide pages.')
