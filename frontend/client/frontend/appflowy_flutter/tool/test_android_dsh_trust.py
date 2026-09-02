"""Offline guards for debug-only, exact-domain Android DSH CA trust.

Run: python3 client/frontend/appflowy_flutter/tool/test_android_dsh_trust.py
Requires OpenSSL; never downloads or changes device/system trust stores.
"""

import base64
import hashlib
from pathlib import Path
import subprocess
import unittest
import xml.etree.ElementTree as ET


FLUTTER = Path(__file__).resolve().parents[1]
SOURCE = FLUTTER / "android/app/src"
DEBUG = SOURCE / "debug"
CERTIFICATE = DEBUG / "res/raw/dsh_trusted_ca.pem"
CONFIG = DEBUG / "res/xml/dsh_network_security_config.xml"
ANDROID = "{http://schemas.android.com/apk/res/android}"
FINGERPRINT = "e14ffcad5b0025731006caa43a121a22d8e9700f4fb9cf852f02a708aa5d5666"


class DshDebugTrustTest(unittest.TestCase):
    def test_exact_domain_only_without_global_overrides(self):
        config = ET.parse(CONFIG).getroot()
        self.assertEqual(config.tag, "network-security-config")
        self.assertEqual([child.tag for child in config], ["domain-config"])
        domain_config = config[0]
        self.assertEqual(domain_config.attrib, {"cleartextTrafficPermitted": "false"})
        self.assertEqual([child.tag for child in domain_config], ["domain", "trust-anchors"])
        domain = domain_config.find("domain")
        self.assertEqual(domain.text, "dsh.openmuseai.com")
        self.assertEqual(domain.attrib, {"includeSubdomains": "false"})
        anchors = domain_config.find("trust-anchors")
        self.assertEqual([child.tag for child in anchors], ["certificates", "certificates"])
        self.assertEqual(
            [child.attrib for child in anchors],
            [{"src": "system"}, {"src": "@raw/dsh_trusted_ca"}],
        )

    def test_manifest_opt_in_is_debug_only(self):
        debug_app = ET.parse(DEBUG / "AndroidManifest.xml").find("application")
        self.assertEqual(
            debug_app.attrib,
            {ANDROID + "networkSecurityConfig": "@xml/dsh_network_security_config"},
        )
        for variant in ("main", "profile", "release"):
            manifest = SOURCE / variant / "AndroidManifest.xml"
            if manifest.exists():
                app = ET.parse(manifest).find("application")
                self.assertTrue(app is None or ANDROID + "networkSecurityConfig" not in app.attrib)
            self.assertFalse((SOURCE / variant / "res/raw/dsh_trusted_ca.pem").exists())
            self.assertFalse((SOURCE / variant / "res/xml/dsh_network_security_config.xml").exists())

    def test_only_the_reviewed_public_root_is_bundled(self):
        pem = CERTIFICATE.read_text(encoding="ascii")
        self.assertEqual(pem.count("-----BEGIN CERTIFICATE-----"), 1)
        self.assertNotIn("PRIVATE KEY", pem)
        lines = pem.strip().splitlines()
        self.assertEqual(lines[0], "-----BEGIN CERTIFICATE-----")
        self.assertEqual(lines[-1], "-----END CERTIFICATE-----")
        der = base64.b64decode("".join(lines[1:-1]), validate=True)
        self.assertEqual(hashlib.sha256(der).hexdigest(), FINGERPRINT)

    def test_ca_constraints_and_self_signature(self):
        details = subprocess.check_output(
            ["openssl", "x509", "-in", str(CERTIFICATE), "-noout", "-text"], text=True,
        )
        self.assertIn("CA:TRUE", details)
        self.assertIn("Certificate Sign", details)
        self.assertIn("Root YE", details)
        subprocess.run(
            ["openssl", "verify", "-check_ss_sig", "-CAfile", str(CERTIFICATE), str(CERTIFICATE)],
            check=True, capture_output=True, text=True,
        )

    def test_webview_keeps_rejecting_invalid_tls(self):
        page = (FLUTTER / "lib/plugins/dsh_agent/dsh_mobile_agent_page.dart").read_text()
        self.assertIn("onSslAuthError:", page)
        self.assertIn("error.cancel()", page)
        self.assertNotIn(".proceed()", page)


if __name__ == "__main__":
    unittest.main(verbosity=2)
