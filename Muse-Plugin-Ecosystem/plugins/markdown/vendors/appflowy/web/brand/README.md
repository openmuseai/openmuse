# Brand config

Edit [`config.yaml`](./config.yaml), then stamp TypeScript, `index.html`, and i18n:

```bash
python scripts/apply-brand.py
```

| Field | Meaning |
|---|---|
| `display.en` | English product name (OpenMuse) |
| `display.zh` | Chinese product name (思构) |
| `site_url` / `terms_url` / `privacy_url` / `download_url` | Marketing and legal links |
| `github_url` | Feedback / issues |
| `discord_url` / `docs_url` / `twitter_handle` / `whats_new_url` / `templates_url` / `forum_url` / `reddit_url` | Empty → hide the UI entry |
| `desktop_scheme` | Primary deep link (`openmuse://`) |
| `legacy_desktop_scheme` | Fallback (`appflowy-flutter://`) |
| `legacy_display` | Old names rewritten in translation **values** only (JSON keys stay) |

Runtime code reads `src/config/brand.ts`, which re-exports generated `brand.generated.ts`. Other locales keep the English name until they are translated.
