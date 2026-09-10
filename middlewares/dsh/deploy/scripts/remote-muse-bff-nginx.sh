#!/bin/bash
set -euo pipefail

# Insert `location /api/muse` on the apex site and point DSH auth_request at the BFF.

SITE="${SITE:-/etc/nginx/sites-enabled/openmuseai.com}"
BFF="${BFF:-http://127.0.0.1:8010}"
MARKER="muse-bff-api"

if [[ ! -f "${SITE}" ]]; then
  echo "ERROR: nginx site not found: ${SITE}" >&2
  exit 1
fi

python3 - "${SITE}" "${BFF}" "${MARKER}" <<'PY'
import sys
from pathlib import Path
site, bff, marker = sys.argv[1:]
text = Path(site).read_text()
block = f"""    # {marker}
    location /api/muse {{
        proxy_pass {bff};
        proxy_set_header Host $http_host;
        proxy_set_header Authorization $http_authorization;
        proxy_set_header Cookie $http_cookie;
        proxy_set_header X-Muse-Device-Token $http_x_muse_device_token;
        proxy_set_header X-Muse-Device-Id $http_x_muse_device_id;
        proxy_pass_request_headers on;
        proxy_read_timeout 200s;
        proxy_connect_timeout 5s;
    }}

"""
if marker not in text:
    needle = "    location /api {"
    if needle not in text:
        raise SystemExit("ERROR: could not find location /api")
    text = text.replace(needle, block + needle, 1)
# Switch existing DSH auth_request to the BFF ingress-auth.
text = text.replace(
    "proxy_pass http://127.0.0.1:9999/user;",
    f"proxy_pass {bff}/api/muse/dsh/ingress-auth;",
)
text = text.replace(
    "proxy_pass http://127.0.0.1:8000/api/muse/dsh/ingress-auth;",
    f"proxy_pass {bff}/api/muse/dsh/ingress-auth;",
)
Path(site).write_text(text)
PY

nginx -t
systemctl reload nginx
echo "    /api/muse -> ${BFF}; DSH auth_request -> BFF ingress-auth"
