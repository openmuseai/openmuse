#!/usr/bin/env python3
"""Verify the shipped illustrated guide is a portable rich native document."""
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import zipfile

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


def verify():
    markdown = (BASE / 'OpenMuse-User-Guide.md').read_text()
    links = re.findall(r'(?<!!)\[[^\]]*\]\(([^)]+)\)', markdown)
    images = re.findall(r'!\[[^\]]*\]\(([^)]+)\)', markdown)
    assert len(images) >= 3 and len(links) >= 15
    assert len(re.findall(r'^## ', markdown, re.M)) >= 15
    assert not re.search('[\u4e00-\u9fff]', markdown), 'Overseas guide must be English'
    document = BASE / 'OpenMuse-User-Guide.sydoc'
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
    print(f'Welcome guide PASS: {len(images)} embedded screenshots, {len(links)} real hyperlinks, 15 sections.')


if __name__ == '__main__':
    verify()
