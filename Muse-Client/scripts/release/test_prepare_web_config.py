import copy
import importlib.util
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('web_config', ROOT / 'scripts/release/prepare-web-config.py')
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class WebInstallationBoundary(unittest.TestCase):
    def test_installed_region_does_not_accept_another_authentication_origin(self):
        profiles = json.loads((ROOT / 'config/web-deployments.json').read_text())['regions']
        for region, profile in profiles.items():
            with self.subTest(region=region):
                source = json.loads((ROOT / profile['releaseFile']).read_text())
                installed = MODULE.fixed_release(region, source, profile)
                self.assertEqual(installed['fixedRegion'], region)
                self.assertEqual(len(installed['regions']), 1)
                crossed = copy.deepcopy(installed)
                crossed['regions'][0]['bundle']['snapshot']['deployment']['services']['auth']['baseUrl'] = 'https://other-region.example.test/gotrue'
                with self.assertRaises(ValueError):
                    MODULE.fixed_release(region, crossed, profile)
                wrong_market = copy.deepcopy(installed)
                wrong_market['market'] = 'overseas' if profile['market'] == 'china' else 'china'
                with self.assertRaises(ValueError):
                    MODULE.fixed_release(region, wrong_market, profile)


if __name__ == '__main__':
    unittest.main()
