#!/usr/bin/env python3
"""Regression tests for the mobile exclusion boundary."""
import gzip
import hashlib
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


class Mobile103ClosureTests(unittest.TestCase):
    suffix = 'flutter_assets/assets/bundled/slides-0.9.7.mcpb'

    def bundle(self, content=b'GenOffice; Apache-2.0'):
        buffer = io.BytesIO()
        with zipfile.ZipFile(buffer, 'w', zipfile.ZIP_DEFLATED) as archive:
            archive.writestr('web/index.html', content)
            archive.writestr('licenses/genoffice.txt', b'GenOffice Apache-2.0')
        return buffer.getvalue()

    def profile(self, data):
        value = closure.load_profile('mobile-1.0.3')
        value['bundledArtifacts'][self.suffix] = hashlib.sha256(data).hexdigest()
        return value

    def test_only_exact_pinned_artifact_permits_genoffice(self):
        data = self.bundle()
        profile = self.profile(data)
        name = 'assets/' + self.suffix
        self.assertFalse(closure.inspect_payload(name, data, profile=profile))
        self.assertTrue(closure.inspect_payload(name, data))
        self.assertTrue(closure.inspect_payload('renamed.mcpb', data, profile=profile))

    def test_digest_mismatch_fails_closed(self):
        data = self.bundle()
        bad = closure.inspect_payload(self.suffix, data + b'x', profile=self.profile(data))
        self.assertTrue(any('SHA-256 mismatch' in item for item in bad))

    def test_other_engines_are_still_rejected_in_a_pinned_bundle(self):
        for marker in [b'SiYuan', b'word_core', b'pdfrx', b'gridcraft']:
            data = self.bundle(b'GenOffice ' + marker)
            self.assertTrue(closure.inspect_payload(self.suffix, data, profile=self.profile(data)))

    def test_nested_compressed_implementation_is_still_inspected(self):
        inner = io.BytesIO()
        with zipfile.ZipFile(inner, 'w') as archive:
            archive.writestr('NOTICES.Z', gzip.compress(b'SiYuan'))
        data = self.bundle(inner.getvalue())
        self.assertTrue(closure.inspect_payload(self.suffix, data, profile=self.profile(data)))

    def test_host_identifier_is_exact_not_a_global_engine_waiver(self):
        profile = closure.load_profile('mobile-1.0.3')
        self.assertFalse(closure.inspect_payload('libapp.so', b'io.openmuse/genoffice-pptx', profile=profile))
        self.assertTrue(closure.inspect_payload('libapp.so', b'io.openmuse/genoffice-pptx GenOffice', profile=profile))
        self.assertTrue(closure.inspect_payload('libapp.so', b'@genoffice/engine', profile=profile))

    def test_ui_bridge_key_is_allowed_only_inside_the_exact_reviewed_artifact(self):
        data = self.bundle(b'GenOffice aiOfficeTabs')
        self.assertFalse(closure.inspect_payload(self.suffix, data, profile=self.profile(data)))
        self.assertTrue(closure.inspect_payload('libapp.so', b'aiOfficeTabs', profile=self.profile(data)))
        data = self.bundle(b'GenOffice ioffice other_api')
        self.assertTrue(closure.inspect_payload(self.suffix, data, profile=self.profile(data)))

    def test_profile_requires_the_reviewed_asset(self):
        with tempfile.TemporaryDirectory() as directory:
            with self.assertRaises(SystemExit):
                closure.verify(Path(directory), closure.load_profile('mobile-1.0.3'))

    def test_exact_bundle_in_an_app_is_accepted_without_dropping_notices(self):
        data = self.bundle()
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            asset = root / 'Frameworks/App.framework' / self.suffix
            asset.parent.mkdir(parents=True)
            asset.write_bytes(data)
            closure.verify(root, self.profile(data))

    def test_utf16_host_identifiers_are_handled_without_allowing_others(self):
        profile = closure.load_profile('mobile-1.0.3')
        self.assertFalse(closure.inspect_payload('classes.dex', 'io.openmuse/genoffice-pptx'.encode('utf-16le'), profile=profile))
        self.assertTrue(closure.inspect_payload('classes.dex', 'SiYuan'.encode('utf-16le'), profile=profile))


if __name__ == '__main__':
    unittest.main()
