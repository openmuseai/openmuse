#!/usr/bin/env python3
"""Exercise release gates and archive recovery/corruption boundaries."""
import importlib.util
import json
import os
from pathlib import Path
import tempfile
import unittest
import zipfile

SPEC = importlib.util.spec_from_file_location('release_pipeline', Path(__file__).with_name('pipeline.py'))
pipeline = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(pipeline)


class PipelineTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.previous_root = pipeline.ROOT
        self.previous_definition = pipeline.DEFINITION
        self.root = Path(self.tmp.name).resolve()
        pipeline.ROOT = self.root
        pipeline.DEFINITION = self.root / 'definition.json'
        stages = []
        for number in range(1, 10):
            sid = f'{number:02}-step'
            (self.root / f'{sid}.md').write_text(sid)
            stages.append({'id': sid, 'title': sid, 'document': f'{sid}.md',
                           'needs': [] if number == 1 else [f'{number-1:02}-step']})
        pipeline.write_json(pipeline.DEFINITION, {'stages': stages})
        self.run = self.root / 'run'
        pipeline.initialize(self.run, 'test-release', '1.0.1', ['play'])
        self.payload = self.root / 'notes.txt'
        self.payload.write_text('Public release material')
        self.set_inputs([self.item()])

    def tearDown(self):
        pipeline.ROOT = self.previous_root
        pipeline.DEFINITION = self.previous_definition
        self.tmp.cleanup()

    def item(self):
        return {'source': 'notes.txt', 'target': 'materials/notes.txt',
                'category': 'notes', 'sha256': pipeline.sha256(self.payload),
                'reviewedForSecrets': True}

    def set_inputs(self, files):
        pipeline.write_json(self.run / 'archive-inputs.json', {'files': files})

    def pass_stages(self):
        for p in (self.run / 'stages').glob('*.json'):
            d = pipeline.read_json(p)
            d.update(status='passed', owner='release reviewer', evidence=['notes.txt'])
            pipeline.write_json(p, d)
        p = self.run / 'release.json'
        d = pipeline.read_json(p)
        d.update(sourceFrozen=True, sourceRevisions={'client': 'a' * 40})
        pipeline.write_json(p, d)

    def test_new_refuses_overwrite(self):
        with self.assertRaisesRegex(ValueError, 'overwrite'):
            pipeline.initialize(self.run, 'test-release', '1.0.1', ['play'])

    def test_macos_lanes_need_channel_specific_binary_provenance(self):
        for lane in ['macos-store-us', 'macos-direct-us']:
            with self.subTest(lane=lane):
                mac_run = self.root / lane
                pipeline.initialize(mac_run, lane, '1.0.0', [lane])
                pipeline.load_run(mac_run)
                self.pass_stages()
                release = pipeline.read_json(self.run / 'release.json')
                release['lanes'] = [lane]
                release['artifacts'] = [{'lane': lane, 'path': 'notes.txt',
                    'sha256': pipeline.sha256(self.payload), 'buildRecord': 'build.json'}]
                pipeline.write_json(self.run / 'release.json', release)
                record = {'status': 'store-candidate', 'dirtySource': False,
                    'testsPassed': True, 'sha256': pipeline.sha256(self.payload)}
                pipeline.write_json(self.root / 'build.json', record)
                with self.assertRaisesRegex(ValueError, 'macOS channel/signing'):
                    pipeline.check_run(self.run, '09-step')
                record.update(distributionChannel=lane, teamId='EXAMPLE')
                pipeline.write_json(self.root / 'build.json', record)
                with self.assertRaisesRegex(ValueError, 'sandbox|notarization'):
                    pipeline.check_run(self.run, '09-step')
                record.update(sandboxEnabled=True, signingIdentityType='Developer ID Application',
                    hardenedRuntime=True, notarization={'status': 'Accepted',
                        'submissionId': 'example-submission', 'stapled': True})
                pipeline.write_json(self.root / 'build.json', record)
                pipeline.check_run(self.run, '09-step')

    def test_pending_dependency_blocks_gate(self):
        with self.assertRaisesRegex(ValueError, 'Gate blocked'):
            pipeline.check_run(self.run, '09-step')

    def test_historical_submission_does_not_authorize_future_gate(self):
        self.pass_stages()
        p = self.run / 'release.json'
        d = pipeline.read_json(p)
        d['mode'] = 'historical'
        pipeline.write_json(p, d)
        with self.assertRaisesRegex(ValueError, 'historical'):
            pipeline.check_run(self.run, '07-step')

    def test_metadata_only_cannot_pass_gate(self):
        self.pass_stages()
        with self.assertRaisesRegex(ValueError, 'metadata-only'):
            pipeline.check_run(self.run, '07-step', metadata_only=True)

    def test_gate_checks_prerequisites_without_preapproving_its_own_result(self):
        self.pass_stages()
        p = self.run / 'stages/07-step.json'
        d = pipeline.read_json(p)
        d.update(status='pending', evidence=[], owner='')
        pipeline.write_json(p, d)
        pipeline.check_run(self.run, '07-step')
        self.assertEqual(pipeline.read_json(p)['status'], 'pending')

    def test_unreviewed_or_changed_material_is_rejected(self):
        item = self.item()
        item['reviewedForSecrets'] = False
        self.set_inputs([item])
        with self.assertRaisesRegex(ValueError, 'unreviewed'):
            pipeline.archive_run(self.run, self.root / 'archive.zip')
        self.set_inputs([self.item()])
        self.payload.write_text('Changed since reviewed')
        with self.assertRaisesRegex(ValueError, 'changed after review'):
            pipeline.archive_run(self.run, self.root / 'archive.zip')

    def test_duplicate_paths_are_rejected(self):
        self.set_inputs([self.item(), self.item()])
        with self.assertRaisesRegex(ValueError, 'Duplicate'):
            pipeline.archive_run(self.run, self.root / 'archive.zip')

    def test_traversal_symlink_and_credentials_are_rejected(self):
        with self.assertRaises(ValueError):
            pipeline.local_path('../outside')
        (self.root / 'link.txt').symlink_to(self.payload)
        with self.assertRaises(ValueError):
            pipeline.local_path('link.txt')
        with self.assertRaises(ValueError):
            pipeline.archive_name('keys/private.p12')
        with self.assertRaises(ValueError):
            pipeline.archive_name('../outside')
        self.payload.write_text('-----BEGIN ' + 'PRIVATE KEY-----\n' + 'A' * 48 + '\n')
        self.set_inputs([self.item()])
        with self.assertRaisesRegex(ValueError, 'Private key'):
            pipeline.archive_run(self.run, self.root / 'archive.zip')

    def test_archive_roundtrip_preserves_unfinished_history_and_detects_tampering(self):
        output = self.root / 'archive.zip'
        pipeline.archive_run(self.run, output)
        self.assertEqual(output.stat().st_mode & 0o777, 0o600)
        with zipfile.ZipFile(output) as z:
            self.assertEqual(json.loads(z.read('run/stages/01-step.json'))['status'], 'pending')
            entries = {n: z.read(n) for n in z.namelist()}
        entries['materials/notes.txt'] = b'tampered'
        corrupted = self.root / 'corrupted.zip'
        with zipfile.ZipFile(corrupted, 'w') as z:
            for name, data in entries.items():
                z.writestr(name, data)
        with self.assertRaisesRegex(ValueError, 'hash mismatch'):
            pipeline.verify_archive(corrupted)
        with self.assertRaisesRegex(ValueError, 'already exists'):
            pipeline.archive_run(self.run, output)

    def test_archive_supports_documented_relative_run_path(self):
        previous = Path.cwd()
        try:
            os.chdir(self.root)
            pipeline.archive_run(Path('run'), self.root / 'relative.zip')
        finally:
            os.chdir(previous)
        pipeline.verify_archive(self.root / 'relative.zip')

    def test_internally_signed_or_dirty_build_cannot_pass_submit_gate(self):
        self.pass_stages()
        p = self.run / 'release.json'
        d = pipeline.read_json(p)
        d['artifacts'] = [{'lane': 'play', 'path': 'notes.txt',
                           'sha256': pipeline.sha256(self.payload), 'buildRecord': 'build.json'}]
        pipeline.write_json(self.root / 'build.json', {'status': 'internal-test-only',
            'dirtySource': True, 'testsPassed': True, 'sha256': pipeline.sha256(self.payload)})
        pipeline.write_json(p, d)
        with self.assertRaisesRegex(ValueError, 'store-candidate provenance'):
            pipeline.check_run(self.run, '09-step')


if __name__ == '__main__':
    unittest.main()
