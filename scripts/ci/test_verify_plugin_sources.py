#!/usr/bin/env python3
"""Exercise source-pin checks against actual temporary Git repositories."""
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

from verify_plugin_sources import verify


def git(root, *args):
    return subprocess.check_output(['git', '-C', str(root), *args], text=True, stderr=subprocess.PIPE).strip()


class SourceCompositionTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.plugin = self.root / 'plugin'
        self.plugin.mkdir()
        git(self.plugin, 'init', '-b', 'qa/source-pin')
        git(self.plugin, 'config', 'user.name', 'Source pin test')
        git(self.plugin, 'config', 'user.email', 'source-pin@example.invalid')
        (self.plugin / 'source.txt').write_text('fixed source\n')
        git(self.plugin, 'add', '.')
        git(self.plugin, 'commit', '-m', 'fixture')
        commit = git(self.plugin, 'rev-parse', 'HEAD')
        self.eco = self.root / 'ecosystem'
        self.eco.mkdir()
        git(self.eco, 'init', '-b', 'qa/composition')
        git(self.eco, 'config', 'user.name', 'Source pin test')
        git(self.eco, 'config', 'user.email', 'source-pin@example.invalid')
        plugins = []
        for i in range(15):
            name = 'plugins/plugin-' + str(i)
            git(self.eco, 'clone', '--shared', str(self.plugin), name)
            git(self.eco, 'update-index', '--add', '--cacheinfo', '160000,' + commit + ',' + name)
            plugins.append({'path': name, 'commit': commit})
        self.lock = self.eco / 'lock.json'
        self.lock.write_text(json.dumps({'plugins': plugins}))
        git(self.eco, 'add', 'lock.json')
        git(self.eco, 'commit', '-m', 'composition')
        self.composition = self.root / 'composition.json'
        self.composition.write_text(json.dumps({'ecosystem': {
            'commit': git(self.eco, 'rev-parse', 'HEAD'), 'lock': 'lock.json',
            'lockSha256': hashlib.sha256(self.lock.read_bytes()).hexdigest(),
        }}))

    def test_exact_git_composition_passes(self):
        self.assertEqual(verify(self.composition, self.eco)['verifiedPlugins'], 15)

    def test_checkout_can_move_with_absorbed_submodule_gitdirs(self):
        for i in range(15):
            git(self.eco, 'config', '-f', '.gitmodules',
                'submodule.plugin-' + str(i) + '.path', 'plugins/plugin-' + str(i))
            git(self.eco, 'config', '-f', '.gitmodules',
                'submodule.plugin-' + str(i) + '.url', str(self.plugin))
        git(self.eco, 'add', '.gitmodules')
        git(self.eco, 'commit', '-m', 'submodule metadata')
        pin = json.loads(self.composition.read_text())
        pin['ecosystem']['commit'] = git(self.eco, 'rev-parse', 'HEAD')
        self.composition.write_text(json.dumps(pin))
        git(self.eco, 'submodule', 'absorbgitdirs')
        destination = self.root / 'Muse-Plugin-Ecosystem'
        self.eco.rename(destination)
        self.assertEqual(verify(self.composition, destination)['verifiedPlugins'], 15)

    def test_lock_tampering_is_rejected(self):
        self.lock.write_text(self.lock.read_text() + ' ')
        with self.assertRaisesRegex(ValueError, 'lock digest mismatch'):
            verify(self.composition, self.eco)

    def test_changed_plugin_head_is_rejected(self):
        plugin = self.eco / 'plugins/plugin-0'
        git(plugin, '-c', 'user.name=Source pin test', '-c', 'user.email=source-pin@example.invalid',
            'commit', '--allow-empty', '-m', 'unexpected revision')
        with self.assertRaisesRegex(ValueError, 'checkout commit mismatch'):
            verify(self.composition, self.eco)

    def test_modified_tracked_plugin_source_is_rejected(self):
        (self.eco / 'plugins/plugin-0/source.txt').write_text('uncommitted change\n')
        with self.assertRaisesRegex(ValueError, 'tracked source is dirty'):
            verify(self.composition, self.eco)

    def test_missing_plugin_cannot_fall_back_to_parent_repo(self):
        (self.eco / 'plugins/plugin-0').rename(self.eco / 'missing-plugin')
        with self.assertRaisesRegex(ValueError, 'Missing or symlinked'):
            verify(self.composition, self.eco)


if __name__ == '__main__':
    unittest.main()
