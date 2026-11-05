# Brand config

Edit [`config.yaml`](./config.yaml), then stamp native / Dart / i18n files:

```bash
python frontend/client/scripts/apply-brand.py
```

| Field | Meaning |
|---|---|
| `display.en` | English product name (OpenMuse) |
| `display.zh` | Chinese product name (思构) |
| `binary.*` | `OpenMuse.exe` / `openmuse` / `OpenMuse.app` |
| `data_dir` | User data folder name |
| `artifact_prefix` | Zip / apk / ipa prefix |
| `legacy_display` | Old names rewritten in translations and kept as data-dir fallbacks |

Pack and install scripts read `config.yaml` directly (no stamp needed). Xcode, CMake, Android, and `lib/brand/brand_values.g.dart` need `apply-brand.py` before the next build.

Other locales keep the English name until they are translated.
