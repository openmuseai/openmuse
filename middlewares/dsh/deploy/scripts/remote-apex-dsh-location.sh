#!/bin/bash
set -euo pipefail

# Insert same-origin /dsh/ and /u/ in front of apex `location /`.
# AUTH_BACKEND=gotrue  → loopback GoTrue /user (until Muse Cloud is deployed)
# AUTH_BACKEND=cloud   → /api/muse/dsh/ingress-auth
# Does not change dsh.<domain>.

SITE="${SITE:-/etc/nginx/sites-enabled/openmuseai.com}"
CLOUD_UPSTREAM="${CLOUD_UPSTREAM:-http://127.0.0.1:8000}"
GOTRUE_UPSTREAM="${GOTRUE_UPSTREAM:-http://127.0.0.1:9999}"
DSH_UPSTREAM="${DSH_UPSTREAM:-http://127.0.0.1:3080}"
POOL_UPSTREAM="${POOL_UPSTREAM:-http://127.0.0.1:13080}"
AUTH_BACKEND="${AUTH_BACKEND:-gotrue}"
MARKER="muse-dsh-same-origin"

if [[ "${AUTH_BACKEND}" == "cloud" ]]; then
  AUTH_PASS="${CLOUD_UPSTREAM}/api/muse/dsh/ingress-auth"
else
  AUTH_PASS="${GOTRUE_UPSTREAM}/user"
fi

if [[ ! -f "${SITE}" ]]; then
  echo "ERROR: nginx site not found: ${SITE}" >&2
  exit 1
fi

backup="${SITE}.bak.$(date +%Y%m%d%H%M%S)"
cp -a "${SITE}" "${backup}"
echo "    backup ${backup}"

python3 - "${SITE}" "${AUTH_PASS}" "${DSH_UPSTREAM}" "${POOL_UPSTREAM}" "${MARKER}" <<'PY'
import sys
from pathlib import Path

site, auth_pass, dsh, pool, marker = sys.argv[1:]
text = Path(site).read_text()

auth_map = """map $http_authorization $muse_ingress_auth {
    default $http_authorization;
    ""      "Bearer $cookie_access_token";
}

"""
if "map $http_authorization $muse_ingress_auth" not in text:
    needle_map = "map $http_upgrade $connection_upgrade {\n    default upgrade;\n    '' close;\n}\n"
    if needle_map not in text:
        raise SystemExit("ERROR: unexpected nginx map block; aborting")
    text = text.replace(needle_map, needle_map + "\n" + auth_map, 1)

block = f"""    # {marker}
    location = /internal/muse-dsh-auth {{
        internal;
        proxy_pass {auth_pass};
        proxy_pass_request_body off;
        proxy_set_header Content-Length "";
        proxy_set_header Authorization $muse_ingress_auth;
        proxy_set_header Cookie $http_cookie;
        proxy_set_header X-Muse-Device-Token $http_x_muse_device_token;
    }}

    location /dsh/ {{
        auth_request /internal/muse-dsh-auth;
        proxy_pass {dsh}/;
        proxy_http_version 1.1;
        proxy_set_header Host 127.0.0.1:3080;
        proxy_set_header Cookie "";
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection $connection_upgrade;
        proxy_read_timeout 86400s;
        proxy_buffering off;
        proxy_hide_header X-Frame-Options;
        proxy_hide_header Content-Security-Policy;
    }}

    location /u/ {{
        auth_request /internal/muse-dsh-auth;
        proxy_pass {pool}/u/;
        proxy_http_version 1.1;
        proxy_set_header Host $http_host;
        proxy_set_header Cookie "";
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection $connection_upgrade;
        proxy_read_timeout 86400s;
        proxy_buffering off;
        proxy_hide_header X-Frame-Options;
        proxy_hide_header Content-Security-Policy;
    }}

"""
if marker in text:
    print("    locations already present; map updated if needed")
else:
    needle = "    location / {"
    if needle not in text:
        raise SystemExit("ERROR: could not find apex location / to insert before")
    text = text.replace(needle, block + needle, 1)

Path(site).write_text(text)
PY

nginx -t
systemctl reload nginx
echo "    apex /dsh/ and /u/ enabled (auth_request -> ${AUTH_PASS})"
