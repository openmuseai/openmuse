#!/usr/bin/env python3
"""Reject incomplete or tampered generated runtime archives."""
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
import zipfile

from prepare_plugin_assets import collect_assets, verify_runtime


class GeneratedAssetTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.path = Path(self.temp.name) / 'runtime.zip'
        self.files = {name: b'fixture' for name in (
            'LICENSE', 'THIRD_PARTY_NOTICES.md', 'upstream.json',
            'app/appearance/theme.css', 'app/stage/resource.js', 'app/guide/start.md')}

    def archive(self, *, tamper=False, extra=False):
        catalog = {k: hashlib.sha256(v).hexdigest() for k, v in self.files.items()}
        with zipfile.ZipFile(self.path, 'w') as out:
            for name, data in self.files.items():
                out.writestr(name, data + b'changed' if tamper else data)
            if extra:
                out.writestr('uncataloged.js', b'injected')
            out.writestr('catalog.json', json.dumps(catalog))

    def test_complete_catalog_passes(self):
        self.archive()
        verify_runtime(self.path)

    def test_tampered_runtime_is_rejected(self):
        self.archive(tamper=True)
        with self.assertRaisesRegex(ValueError, 'digest mismatch'):
            verify_runtime(self.path)

    def test_uncataloged_code_is_rejected(self):
        self.archive(extra=True)
        with self.assertRaisesRegex(ValueError, 'cover the archive'):
            verify_runtime(self.path)

    def test_attribution_cannot_be_omitted(self):
        del self.files['LICENSE']
        self.archive()
        with self.assertRaisesRegex(ValueError, 'attribution missing'):
            verify_runtime(self.path)

    def test_empty_runtime_cannot_replace_real_assets(self):
        self.files = {k: v for k, v in self.files.items() if not k.startswith('app/')}
        self.archive()
        with self.assertRaisesRegex(ValueError, 'output missing'):
            verify_runtime(self.path)

    def test_traversal_is_rejected(self):
        self.files['../outside.js'] = b'bad'
        self.archive()
        with self.assertRaisesRegex(ValueError, 'Unsafe'):
            verify_runtime(self.path)

    def test_missing_wasm_fails_closed(self):
        root = Path(self.temp.name)
        with self.assertRaises(FileNotFoundError):
            collect_assets(root / 'Muse-Client', root / 'Muse-Plugin-Ecosystem')


if __name__ == '__main__':
    unittest.main()
