#!/bin/bash
set -euo pipefail

# Remote infra: app dir, env skeleton, nginx vhost for DSH_HOST.
# Requires AppFlowy-Cloud deploy-infra (TLS certs) first.
DSH_HOST="${DSH_HOST:?DSH_HOST is required}"
BASE_DOMAIN="${BASE_DOMAIN:?BASE_DOMAIN is required}"
WEB_ORIGIN="${WEB_ORIGIN:?WEB_ORIGIN is required}"
APP_DIR="${APP_DIR:-/opt/muse-dsh}"
DSH_PORT="${DSH_PORT:-3080}"
SSL_DIR="${SSL_DIR:-/etc/nginx/ssl/${BASE_DOMAIN}}"

echo "==> Creating Muse DSH directory ${APP_DIR}"
mkdir -p "${APP_DIR}/runtime" "${APP_DIR}/images"

DSH_TRUSTED_HOST="${DSH_TRUSTED_HOST:-${DSH_HOST}}"
MUSE_DOCUMENT_CLOUD_URL="${MUSE_DOCUMENT_CLOUD_URL:-}"

if [[ ! -f "${APP_DIR}/.env" ]]; then
  cat > "${APP_DIR}/.env" << ENV_EOF
# Fill DEEPSEEK_API_KEY on the server. Do not commit this file.
DEEPSEEK_API_KEY=
DSH_HOME=/var/lib/muse-dsh
HOST=0.0.0.0
PORT=3080
DSH_TRUSTED_HOST=${DSH_TRUSTED_HOST}
MUSE_DOCUMENT_CLOUD_URL=${MUSE_DOCUMENT_CLOUD_URL}
ENV_EOF
  chmod 600 "${APP_DIR}/.env"
  echo "    wrote ${APP_DIR}/.env (empty DEEPSEEK_API_KEY — fill before runtime)"
else
  echo "    keeping existing ${APP_DIR}/.env"
fi

LE_PEM="/etc/letsencrypt/live/${DSH_HOST}/fullchain.pem"
LE_KEY="/etc/letsencrypt/live/${DSH_HOST}/privkey.pem"
APEX_PEM="${SSL_DIR}/${BASE_DOMAIN}.pem"
APEX_KEY="${SSL_DIR}/${BASE_DOMAIN}.key"
if [[ -f "${LE_PEM}" && -f "${LE_KEY}" ]]; then
  CERT_PEM="${LE_PEM}"
  CERT_KEY="${LE_KEY}"
  echo "    using Let's Encrypt cert for ${DSH_HOST}"
elif [[ -f "${APEX_PEM}" && -f "${APEX_KEY}" ]]; then
  CERT_PEM="${APEX_PEM}"
  CERT_KEY="${APEX_KEY}"
  echo "WARNING: no Let's Encrypt cert for ${DSH_HOST}; using apex cert (browser name mismatch until issued)."
else
  echo "ERROR: TLS material not found (need ${LE_PEM} or ${APEX_PEM})" >&2
  echo "Run AppFlowy-Cloud deploy-infra.sh first, or issue a cert for ${DSH_HOST}." >&2
  exit 1
fi
mkdir -p /var/www/letsencrypt

# Parent Web is served on both apex and www; Chrome maps CSP frame-ancestors
# mismatches to “refused to connect” inside an iframe (top-level tabs still work).
_web="${WEB_ORIGIN%/}"
_scheme="${_web%%://*}"
_host="${_web#*://}"
_host="${_host%%/*}"
FRAME_ANCESTORS="${FRAME_ANCESTORS:-${_web}}"
if [[ "${_host}" == www.* ]]; then
  FRAME_ANCESTORS="${FRAME_ANCESTORS} ${_scheme}://${_host#www.}"
else
  FRAME_ANCESTORS="${FRAME_ANCESTORS} ${_scheme}://www.${_host}"
fi
if [[ -n "${APP_DOMAIN:-}" ]]; then
  FRAME_ANCESTORS="${FRAME_ANCESTORS} https://${APP_DOMAIN}"
fi

SITE="/etc/nginx/sites-available/${DSH_HOST}"
echo "==> Writing nginx vhost ${SITE}"
cat > "${SITE}" << NGINX_EOF
map \$http_upgrade \$connection_upgrade {
    default upgrade;
    '' close;
}

server {
    listen 80;
    listen [::]:80;
    server_name ${DSH_HOST};

    location ^~ /.well-known/acme-challenge/ {
        root /var/www/letsencrypt;
        default_type text/plain;
    }

    location / {
        return 301 https://\$host\$request_uri;
    }
}

server {
    listen 443 ssl;
    listen [::]:443 ssl;
    server_name ${DSH_HOST};

    ssl_certificate ${CERT_PEM};
    ssl_certificate_key ${CERT_KEY};
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;

    client_max_body_size 32m;
    add_header Content-Security-Policy "frame-ancestors ${FRAME_ANCESTORS}" always;
    add_header X-Content-Type-Options nosniff always;

    location = /healthz {
        add_header Access-Control-Allow-Origin "${WEB_ORIGIN}" always;
        add_header Access-Control-Allow-Methods "GET, OPTIONS" always;
        add_header Content-Type text/plain;
        return 200 "ok\n";
    }

    location / {
        proxy_pass http://127.0.0.1:${DSH_PORT};
        proxy_http_version 1.1;
        proxy_set_header Host \$host;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection \$connection_upgrade;
        proxy_read_timeout 86400s;
        proxy_buffering off;
    }
}
NGINX_EOF

ln -sf "${SITE}" "/etc/nginx/sites-enabled/${DSH_HOST}"
nginx -t
systemctl enable nginx >/dev/null 2>&1 || true
systemctl reload nginx

touch "${APP_DIR}/.infra-ready"
echo "==> DSH infra ready"
echo "    vhost: https://${DSH_HOST} -> 127.0.0.1:${DSH_PORT}"
echo "    frame-ancestors: ${FRAME_ANCESTORS}"
echo "    Next: fill ${APP_DIR}/.env then deploy-runtime.sh"
