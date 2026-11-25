#!/usr/bin/env python3
"""Verify the shipped illustrated guide is a portable rich native document."""
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import zipfile
import argparse
import base64
import subprocess
import uuid

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / 'packages/openmuse_resource_center_host/assets/welcome'


def nodes(value):
    if isinstance(value, dict):
        yield value
        for child in value.values():
            yield from nodes(child)
    elif isinstance(value, list):
        for child in value:
            yield from nodes(child)


def verify(china=False):
    stem = 'OpenMuse-User-Guide.zh-CN' if china else 'OpenMuse-User-Guide'
    markdown = (BASE / (stem + '.md')).read_text()
    links = re.findall(r'(?<!!)\[[^\]]*\]\(([^)]+)\)', markdown)
    images = re.findall(r'!\[[^\]]*\]\(([^)]+)\)', markdown)
    assert len(images) >= 3 and len(links) >= 15
    assert len(re.findall(r'^## ', markdown, re.M)) >= 15
    if china:
        assert re.search('[\u4e00-\u9fff]', markdown), 'China guide must include Chinese'
        assert all('us.openmuseai.com' not in link for link in links), 'China guide must not send users to US services'
    else:
        assert not re.search('[\u4e00-\u9fff]', markdown), 'Overseas guide must be English'
    document = BASE / (stem + '.sydoc')
    provenance = json.loads(document.with_suffix('.provenance.json').read_text())
    assert hashlib.sha256(document.read_bytes()).hexdigest() == provenance['sha256']
    with zipfile.ZipFile(document) as archive:
        names = archive.namelist()
        assert len(names) == len(set(names)) and sum(i.file_size for i in archive.infolist()) < 16 * 1024 * 1024
        assert all(not PurePosixPath(n).is_absolute() and '..' not in PurePosixPath(n).parts and '\\' not in n for n in names)
        documents = [json.loads(archive.read(n)) for n in names if n.endswith('.sy')]
        assert len(documents) == 1
        all_nodes = list(nodes(documents))
        destinations = [n['TextMarkAHref'] for n in all_nodes if n.get('Type') == 'NodeTextMark' and n.get('TextMarkType') == 'a']
        assert sorted(destinations) == sorted(links), 'Native archive lost or changed hyperlinks'
        assert all(l.startswith(('https://', 'mailto:')) for l in destinations)
        image_nodes = [n for n in all_nodes if n.get('Type') == 'NodeImage']
        assert len(image_nodes) == len(images)
        for reference in images:
            source = (BASE / reference).resolve()
            source.relative_to(BASE.resolve())
            blob = source.read_bytes()
            digest = hashlib.sha256(blob).hexdigest()
            embedded = [n for n in names if '/assets/' in n and hashlib.sha256(archive.read(n)).hexdigest() == digest]
            assert len(embedded) == 1, 'Screenshot missing from native document'
        assert sum(n.get('Type') == 'NodeHeading' for n in all_nodes) >= 16
    print(f'{stem} PASS: {len(images)} embedded screenshots, {len(links)} real hyperlinks, 15 sections.')


def verify_smart_document():
    plugin = ROOT.parent / 'Muse-Plugin-Ecosystem/plugins/markdown'
    generator = plugin / 'smart_document/rust/examples/portable_document.rs'
    host = subprocess.check_output(['rustc', '-vV'], text=True)
    host = re.search(r'^host: ([a-zA-Z0-9_-]+)$', host, re.M).group(1)
    for locale in ['en', 'zh-CN']:
        stem = 'OpenMuse-User-Guide.v1.0.3.' + locale
        source = BASE / (stem + '.md')
        document = BASE / (stem + '.musedoc')
        provenance = json.loads((BASE / (stem + '.provenance.json')).read_text())
        markdown = source.read_text()
        assert hashlib.sha256(source.read_bytes()).hexdigest() == provenance['sourceSha256']
        assert hashlib.sha256(document.read_bytes()).hexdigest() == provenance['sha256']
        assert hashlib.sha256(generator.read_bytes()).hexdigest() == provenance['generatorSha256']
        value = json.loads(document.read_bytes())
        assert set(value) == {'profile', 'objectId', 'docState'}
        assert value['profile'] == 'muse.smart-document/phase1'
        uuid.UUID(value['objectId'])
        assert 100 < len(base64.b64decode(value['docState'], validate=True)) < 8 * 1024 * 1024
        assert len(re.findall(r'^## ', markdown, re.M)) >= 15
        assert markdown.count('```text') >= 3
        links = re.findall(r'(?<!!)\[[^\]]*\]\(([^)]+)\)', markdown)
        assert len(links) >= 15 and all(link.startswith('https://') for link in links)
        assert not re.findall(r'!\[[^\]]*\]\(([^)]+)\)', markdown), 'Guide must not depend on remote illustrations'
        if locale == 'zh-CN':
            assert re.search('[\u4e00-\u9fff]', markdown)
            assert all('us.openmuseai.com' not in link for link in links)
        else:
            assert not re.search('[\u4e00-\u9fff]', markdown)
        result = subprocess.run([
            'cargo', 'run', '--quiet', '--locked', '--release', '--target', host,
            '--manifest-path', str(plugin / 'smart_document/rust/Cargo.toml'),
            '--example', 'portable_document', '--', 'verify', str(document), str(source),
        ], check=True, text=True, capture_output=True, timeout=300)
        native = json.loads(result.stdout.strip().splitlines()[-1])
        assert native['originalCoreReopen'] and native['sourceLinks'] == len(links)
        assert native['blockKinds']['heading'] >= 15 and native['blockKinds']['code'] >= 3
        print(stem + ' PASS: real Smart Document core reopen, source links, sections and text diagrams')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--profile', choices=['legacy', 'smart-document'], default='legacy')
    args = parser.parse_args()
    if args.profile == 'smart-document':
        verify_smart_document()
    else:
        # Keep historical native-archive verification available for old releases.
        verify()
        verify(china=True)
