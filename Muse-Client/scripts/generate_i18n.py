#!/usr/bin/env python3
"""Generate the typed Dart facade and offline baseline from the reviewed registry."""
import json
from pathlib import Path
import re
import argparse
import subprocess
import shutil
import os

ROOT = Path(__file__).resolve().parents[1]
PACKAGE = ROOT / 'packages/openmuse_i18n'


def dart_string(value):
    return json.dumps(value, ensure_ascii=False).replace('$', r'\$')


def method_name(key):
    parts = re.split('[^A-Za-z0-9]+', key)
    return parts[0] + ''.join(p[:1].upper()+p[1:] for p in parts[1:] if p)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    outputs = {}
    data = json.loads((PACKAGE / 'assets/catalog.json').read_text())
    payload = json.dumps(data, ensure_ascii=False, separators=(',', ':'))
    outputs[PACKAGE / 'lib/src/generated_catalog.dart'] = (
        '// GENERATED from assets/catalog.json; edit the registry.\n'
        'const builtInCatalogJson = ' + dart_string(payload) + ';\n')
    lines = ["// GENERATED from assets/catalog.json; edit the registry.",
             "import 'catalog.dart';", '', 'final class OpenMuseStrings {',
             '  OpenMuseStrings(this._catalog, this._locale);',
             '  factory OpenMuseStrings.forLocale(String tag, {OpenMuseCatalog? catalog}) { final value = catalog ?? OpenMuseCatalog.builtIn; return OpenMuseStrings(() => value, () => value.resolve([tag])); }',
             '  static final fallback = OpenMuseStrings.forLocale(OpenMuseCatalog.builtIn.fallbackLocale);',
             '  final OpenMuseCatalog Function() _catalog;', '  final String Function() _locale;',
             '  String get locale => _locale();',
             '  Map<String, Object?> namespaceData(String namespace) => _catalog().namespaceData(locale, namespace);',
             '  String format(String id, [Map<String, Object?> arguments = const {}]) => _catalog().format(locale, id, arguments);']
    names = set()
    for key, definition in sorted(data['entries'].items()):
        name = method_name(key)
        assert name not in names, name
        names.add(name)
        parameters = definition['parameters']
        types = definition.get('parameterTypes', {})
        assert all(re.fullmatch('[A-Za-z][A-Za-z0-9_]*', p) for p in parameters)
        assert all(t in {'String', 'int', 'double', 'num', 'Object?'} for t in types.values())
        signature = ('{' + ', '.join('required ' + types.get(p, 'Object?') + ' ' + p for p in parameters) + '}') if parameters else ''
        arguments = ', {' + ', '.join(dart_string(p) + ': ' + p for p in parameters) + '}' if parameters else ''
        lines.append(f'  String {name}({signature}) => format({dart_string(key)}{arguments});')
    lines.append('}')
    outputs[PACKAGE / 'lib/src/generated_strings.dart'] = '\n'.join(lines) + '\n'
    # Preserve the auth plugin's public abstract API while resolving its text
    # through the same live message provider as every other surface.
    arb = json.loads((ROOT / 'plugins/auth-gotrue/lib/l10n/app_en.arb').read_text())
    adapter = ["// GENERATED adapter from the auth ARB message contract.",
               "import 'package:openmuse_i18n/openmuse_i18n.dart';",
               "import '../l10n/openmuse_auth_localizations.dart';", '',
               'final class RuntimeAuthLocalizations extends OpenMuseAuthLocalizations {',
               '  RuntimeAuthLocalizations(this.strings) : super(localeFromTag(strings.locale).languageCode);',
               '  final OpenMuseStrings strings;']
    for key in arb:
        if key.startswith('@'):
            continue
        placeholders = arb.get('@' + key, {}).get('placeholders', {})
        params = ', '.join(v.get('type', 'Object') + ' ' + p for p, v in placeholders.items())
        signature = key + '(' + params + ')' if placeholders else 'get ' + key
        values = ', '.join(p + ': ' + p for p in placeholders)
        adapter.append(f'  @override\n  String {signature} => strings.{method_name("auth." + key)}({values});')
    adapter.append('}')
    outputs[ROOT / 'plugins/auth-gotrue/lib/src/runtime_auth_localizations.dart'] = '\n'.join(adapter) + '\n'
    # Deployable message data is separate from developer-only context metadata.
    for locale in [v['tag'] for v in data['metadata']['locales']]:
        namespaces = sorted(set(v['namespace'] for v in data['entries'].values()))
        for namespace in namespaces:
            messages = {k: v['messages'][locale] for k, v in data['entries'].items()
                        if v['namespace'] == namespace and locale in v['messages']}
            path = PACKAGE / 'assets/messages' / locale / f'{namespace}.json'
            outputs[path] = json.dumps({'schemaVersion': 1, 'locale': locale, 'namespace': namespace,
                                       'sourceRevision': data['metadata']['sourceRevision'], 'messages': messages}, ensure_ascii=False, indent=2) + '\n'
    dart = os.environ.get('DART_BIN') or shutil.which('dart')
    if not dart:
        raise SystemExit('Set DART_BIN or add the Flutter Dart SDK to PATH.')
    changed = []
    for path, source in outputs.items():
        if path.suffix == '.dart':
            source = subprocess.run([dart, 'format', '--summary=none'], input=source,
                                    text=True, capture_output=True, check=True).stdout
        if not path.exists() or path.read_text() != source:
            changed.append(str(path.relative_to(ROOT)))
            if not args.check:
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(source)
    if args.check and changed:
        raise SystemExit('Generated messages are stale:\n' + '\n'.join(changed))
    print(f'Generated {len(names)} typed messages and baseline catalog.')


if __name__ == '__main__':
    main()
