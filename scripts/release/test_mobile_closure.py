#!/usr/bin/env python3
"""Regression tests for the mobile exclusion boundary."""
import gzip
import importlib.util
import io
from pathlib import Path
import tempfile
import unittest
import zipfile

spec = importlib.util.spec_from_file_location('closure', Path(__file__).with_name('verify-mobile-closure.py'))
closure = importlib.util.module_from_spec(spec)
spec.loader.exec_module(closure)


class MobileClosureTests(unittest.TestCase):
    def test_neutral_file_name_does_not_hide_implementation(self):
        self.assertTrue(closure.inspect_payload('libapp.so', b'package:openmuse_siyuan_client/view.dart'))
        self.assertTrue(closure.inspect_payload('classes.dex', 'GenOffice'.encode('utf-16le')))

    def test_flutter_notices_are_inspected_after_decompression(self):
        self.assertTrue(closure.inspect_payload('NOTICES.Z', gzip.compress(b'SiYuan upstream')))

    def test_nested_offline_plugin_is_inspected(self):
        buffer = io.BytesIO()
        with zipfile.ZipFile(buffer, 'w', zipfile.ZIP_DEFLATED) as archive:
            archive.writestr('editor/index.js', b'GenOffice')
        self.assertTrue(closure.inspect_payload('neutral.mcpb', buffer.getvalue()))

    def test_smart_document_and_required_other_notices_are_allowed(self):
        self.assertFalse(closure.inspect_payload('NOTICES.Z', gzip.compress(b'AppFlowy MPL-2.0; SmartDocument')))

    def test_resolved_dependency_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            config = Path(directory) / 'package_config.json'
            config.write_text('{"packages":[{"name":"openmuse_siyuan_client"}]}')
            with self.assertRaises(SystemExit):
                closure.verify(config)

    def test_expansion_budget_fails_closed(self):
        with self.assertRaises(ValueError):
            closure.inspect_payload('NOTICES.Z', gzip.compress(b'a' * 10000), budget=[1000])


if __name__ == '__main__':
    unittest.main()
