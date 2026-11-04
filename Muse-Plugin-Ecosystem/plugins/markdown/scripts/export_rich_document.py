#!/usr/bin/env python3
"""Author a portable .sydoc using the shipped, isolated native Notes engine.

Input is trusted release Markdown. Local images are embedded; no account,
workspace, cloud settings, or provider credentials are read from the host.
"""
import argparse
import ctypes
import hashlib
import json
import re
import secrets
import tempfile
import time
import urllib.request
import zipfile
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--markdown', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--title', default='OpenMuse User Guide')
    args = parser.parse_args()
    markdown = args.markdown.resolve().read_text()
    with tempfile.TemporaryDirectory(prefix='openmuse-document-authoring-') as temp:
        work = Path(temp)
        resources = work / 'resources'
        resources.mkdir()
        with zipfile.ZipFile(ROOT / 'flutter_runtime/assets/runtime.zip') as runtime:
            catalog = json.loads(runtime.read('catalog.json'))
            seen, budget = set(), 0
            for entry in runtime.infolist():
                name = entry.filename
                path = PurePosixPath(name)
                if path.is_absolute() or '\\' in name or '..' in path.parts or name in seen or (entry.external_attr >> 16) & 0o170000 == 0o120000:
                    raise ValueError('Unsafe runtime entry')
                seen.add(name)
                if entry.is_dir() or name == 'catalog.json':
                    continue
                budget += entry.file_size
                if budget > 512 * 1024 * 1024 or len(seen) > 20000:
                    raise ValueError('Runtime budget exceeded')
                data = runtime.read(entry)
                if catalog.pop(name, None) != hashlib.sha256(data).hexdigest():
                    raise ValueError('Runtime integrity mismatch')
                destination = resources / name
                destination.parent.mkdir(parents=True, exist_ok=True)
                destination.write_bytes(data)
            if catalog:
                raise ValueError('Runtime incomplete')
        data_dir = work / 'userdata'
        assets = data_dir / 'siyuan/data/assets'
        assets.mkdir(parents=True)
        embedded = {}

        def image(match):
            alt, reference = match.groups()
            if '://' in reference or reference.startswith('/'):
                raise ValueError('Guide images must be local')
            source = (args.markdown.resolve().parent / reference).resolve()
            source.relative_to(args.markdown.resolve().parent)
            blob = source.read_bytes()
            digest = hashlib.sha256(blob).hexdigest()
            name = f'openmuse-guide-{digest[:16]}{source.suffix.lower()}'
            (assets / name).write_bytes(blob)
            embedded[name] = digest
            return f'![{alt}](assets/{name})'

        markdown = re.sub(r'!\[([^\]]*)\]\(([^\s)]+)\)', image, markdown)
        library = ctypes.CDLL(str(ROOT / 'flutter_runtime/macos/Libraries/libmuse_siyuan.dylib'))
        library.muse_siyuan_free.argtypes = [ctypes.c_void_p]
        for name in ['start', 'status', 'flush']:
            getattr(library, 'muse_siyuan_' + name).restype = ctypes.c_void_p
        library.muse_siyuan_start.argtypes = [ctypes.c_char_p]

        def invoke(name, value=None):
            function = getattr(library, 'muse_siyuan_' + name)
            pointer = function(json.dumps(value).encode()) if value is not None else function()
            try:
                result = json.loads(ctypes.string_at(pointer))
            finally:
                library.muse_siyuan_free(pointer)
            if result.get('error'):
                raise RuntimeError(result['error'])
            return result

        session = secrets.token_hex(32)
        state = invoke('start', {'resources': str(resources), 'data': str(data_dir), 'platform': 'macos', 'language': 'en', 'session': session, 'timezone': 'UTC', 'ocrLibrary': str(ROOT / 'flutter_runtime/macos/Libraries/libonnxruntime.dylib')})
        for _ in range(900):
            if state['state'] == 'ready':
                break
            time.sleep(.1)
            state = invoke('status')
        else:
            raise TimeoutError('Local authoring engine startup')
        origin = state['origin']
        if not re.fullmatch(r'http://127\.0\.0\.1:[0-9]+', origin):
            raise ValueError('Unexpected kernel origin')
        opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))

        def request(path, payload=None):
            req = urllib.request.Request(origin + path, data=json.dumps(payload).encode() if payload is not None else None, headers={'Content-Type': 'application/json', 'X-OpenMuse-Local-Session': session})
            with opener.open(req, timeout=90) as response:
                return response.read()

        def api(path, payload):
            result = json.loads(request(path, payload))
            if result['code'] != 0:
                raise RuntimeError(result.get('msg', 'Native authoring error'))
            return result['data']

        box = api('/api/notebook/createNotebook', {'name': 'OpenMuse guide authoring'})['notebook']['id']
        for _ in range(300):
            if api('/api/notebook/getNotebookConf', {'notebook': box}).get('documentCapabilities', {}).get('indexState') == 'ready':
                break
            time.sleep(.1)
        doc = api('/api/filetree/createDocWithMd', {'notebook': box, 'path': '/' + args.title, 'markdown': markdown})
        api('/api/attr/setBlockAttrs', {'id': doc, 'attrs': {'custom-openmuse-resource-entry': 'true'}})
        invoke('flush')
        exported = api('/api/export/exportSYs', {'ids': [doc]})['zip']
        if not exported.startswith('/export/') or '..' in PurePosixPath(exported).parts:
            raise ValueError('Invalid export path')
        blob = request(exported)
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_bytes(blob)
        with zipfile.ZipFile(args.output) as archive:
            trees = [json.loads(archive.read(n)) for n in archive.namelist() if n.endswith('.sy')]
            def nodes(value):
                if isinstance(value, dict):
                    yield value
                    for child in value.values():
                        yield from nodes(child)
                elif isinstance(value, list):
                    for child in value:
                        yield from nodes(child)
            all_nodes = list(nodes(trees))
            image_nodes = sum(n.get('Type') == 'NodeImage' for n in all_nodes)
            link_nodes = sum(n.get('Type') == 'NodeTextMark' and n.get('TextMarkType') == 'a' and n.get('TextMarkAHref', '').startswith(('https://', 'mailto:')) for n in all_nodes)
            for name, digest in embedded.items():
                found = [n for n in archive.namelist() if n.endswith('/assets/' + name)]
                if len(found) != 1 or hashlib.sha256(archive.read(found[0])).hexdigest() != digest:
                    raise ValueError('Missing or changed embedded image')
            if image_nodes < len(embedded) or link_nodes < 5:
                raise ValueError('Rich document lost image or hyperlink nodes')
        evidence = {'source': args.markdown.name, 'sha256': hashlib.sha256(blob).hexdigest(), 'embeddedImages': embedded, 'imageNodes': image_nodes, 'linkNodes': link_nodes, 'engine': 'shipped native Notes Markdown importer and SY exporter'}
        args.output.with_suffix('.provenance.json').write_text(json.dumps(evidence, indent=2) + '\n')
        print(json.dumps({'document': str(args.output), 'images': image_nodes, 'links': link_nodes, 'bytes': len(blob)}))
        invoke('flush')


if __name__ == '__main__':
    main()
