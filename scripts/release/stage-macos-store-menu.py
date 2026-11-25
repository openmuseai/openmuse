#!/usr/bin/env python3
"""Compile the Store's fixed native menu without optional workbench actions."""
import argparse
from pathlib import Path
import plistlib
import re
import subprocess
import tempfile
import xml.etree.ElementTree as ET


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--client', type=Path, required=True)
    parser.add_argument('--app', type=Path, required=True)
    args = parser.parse_args()
    source = args.client / 'app/openmuse_host/macos/Runner/Base.lproj/MainMenu.xib'
    tree = ET.parse(source)
    removed = set()
    selectors = {'showAgentView:', 'showIdeView:', 'toggleTerminal:'}
    for parent in tree.getroot().iter():
        for child in list(parent):
            actions = {node.get('selector') for node in child.findall('./connections/action')}
            if child.tag == 'menuItem' and (
                actions & selectors or child.get('title') in {'终端', 'Terminal'}
            ):
                removed.update(n.get('id') for n in child.iter() if n.get('id'))
                parent.remove(child)
    for parent in tree.getroot().iter():
        for child in list(parent):
            if child.get('destination') in removed:
                parent.remove(child)
    output = args.app / 'Contents/Resources/Base.lproj/MainMenu.nib'
    output.parent.mkdir(parents=True, exist_ok=True)
    info = plistlib.loads((args.app / 'Contents/Info.plist').read_bytes())
    module = re.sub(r'[^A-Za-z0-9_]', '_', info['CFBundleExecutable'])
    with tempfile.TemporaryDirectory(prefix='openmuse-store-menu-') as directory:
        xib = Path(directory) / 'MainMenu.xib'
        tree.write(xib, encoding='UTF-8', xml_declaration=True)
        subprocess.run(['xcrun', 'ibtool', '--compile', str(output), str(xib),
                        '--module', module, '--minimum-deployment-target', '12.0'], check=True)
    print(f'Store native menu compiled; removed {len(removed)} menu/action object IDs.')


if __name__ == '__main__':
    main()
