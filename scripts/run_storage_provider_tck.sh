#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

provider="${1:-}"
report_path="${2:-}"
if [[ ! "$provider" =~ ^(minio|rustfs|aws_s3)$ ]] || [[ -z "$report_path" ]]; then
  echo "usage: $0 <minio|rustfs|aws_s3> <report-path>" >&2
  exit 2
fi

if [[ "$report_path" != /* ]]; then
  report_path="$repo_root/$report_path"
fi

adapter_revision="$(git rev-parse HEAD)"
adapter_source_digest="$({
  printf '%s\n' Cargo.toml Cargo.lock
  find crates/openmuse-storage-contract crates/openmuse-storage-credentials crates/openmuse-storage-s3 crates/openmuse-storage-tck -type f -print
} | LC_ALL=C sort | while IFS= read -r file; do shasum -a 256 "$file"; done | shasum -a 256 | awk '{print $1}')"

run_tck() {
  OPENMUSE_TCK_ADAPTER_REVISION="$adapter_revision" \
  OPENMUSE_TCK_ADAPTER_SOURCE_DIGEST="$adapter_source_digest" \
  OPENMUSE_TCK_REPORT_PATH="$report_path" \
  cargo test -p openmuse-storage-s3 --test live_provider_tck -- --ignored --nocapture
}

if [[ "$provider" == "aws_s3" ]]; then
  required=(
    OPENMUSE_TCK_S3_ENDPOINT
    OPENMUSE_TCK_S3_REGION
    OPENMUSE_TCK_S3_BUCKET
    OPENMUSE_TCK_S3_PREFIX
    OPENMUSE_TCK_S3_ACCESS_KEY_ID
    OPENMUSE_TCK_S3_SECRET_ACCESS_KEY
    OPENMUSE_TCK_PROVIDER_VERSION
  )
  for name in "${required[@]}"; do
    if [[ -z "${!name:-}" ]]; then
      echo "$name is required for AWS S3 qualification" >&2
      exit 2
    fi
  done
  export OPENMUSE_TCK_S3_KIND=aws_s3
  run_tck
  exit 0
fi

for command in docker curl openssl shasum; do
  command -v "$command" >/dev/null 2>&1 || {
    echo "$command is required for local provider qualification" >&2
    exit 2
  }
done
docker info >/dev/null

case "$provider" in
  minio)
    image="${OPENMUSE_TCK_MINIO_IMAGE:-minio/minio@sha256:14cea493d9a34af32f524e538b8346cf79f3321eff8e708c1e2960462bd8936e}"
    health_path="/minio/health/ready"
    ;;
  rustfs)
    image="${OPENMUSE_TCK_RUSTFS_IMAGE:-ghcr.io/rustfs/rustfs@sha256:8cc9801755448b71a786705ce76692c77e14936cccd87cf2fc31842e58f4d1ff}"
    health_path="/health"
    ;;
esac

container_name="openmuse-st1-${provider}-$$"
temp_dir="$(mktemp -d "/tmp/openmuse-st1-${provider}.XXXXXX")"
cleanup() {
  docker rm -fv "$container_name" >/dev/null 2>&1 || true
  find "$temp_dir" -depth -delete
}
trap cleanup EXIT INT TERM

openssl req -x509 -newkey rsa:2048 -sha256 -days 2 -nodes \
  -subj '/CN=localhost' \
  -addext 'subjectAltName=DNS:localhost,IP:127.0.0.1' \
  -keyout "$temp_dir/private.key" \
  -out "$temp_dir/public.crt" >/dev/null 2>&1
chmod 755 "$temp_dir"
chmod 644 "$temp_dir/private.key" "$temp_dir/public.crt"

access_key="OPENMUSETCK"
secret_key="$(openssl rand -hex 24)"
if [[ "$provider" == "minio" ]]; then
  docker run -d --rm --name "$container_name" -p 127.0.0.1::9000 \
    -e MINIO_ROOT_USER="$access_key" \
    -e MINIO_ROOT_PASSWORD="$secret_key" \
    -v "$temp_dir:/certs:ro" \
    "$image" server --certs-dir /certs /data >/dev/null
else
  cp "$temp_dir/public.crt" "$temp_dir/rustfs_cert.pem"
  cp "$temp_dir/private.key" "$temp_dir/rustfs_key.pem"
  chmod 644 "$temp_dir/rustfs_cert.pem" "$temp_dir/rustfs_key.pem"
  docker run -d --rm --name "$container_name" -p 127.0.0.1::9000 \
    -e RUSTFS_ACCESS_KEY="$access_key" \
    -e RUSTFS_SECRET_KEY="$secret_key" \
    -e RUSTFS_TLS_PATH=/certs \
    -v "$temp_dir:/certs:ro" \
    "$image" >/dev/null
fi

port=""
for _ in {1..30}; do
  port="$(docker port "$container_name" 9000/tcp 2>/dev/null | sed -n 's/.*://p' | head -1)"
  if [[ -n "$port" ]] && curl -kfsS "https://127.0.0.1:$port$health_path" >/dev/null 2>&1; then
    break
  fi
  if ! docker ps --format '{{.Names}}' | rg -qx "$container_name"; then
    docker logs "$container_name" --tail 150 >&2
    exit 1
  fi
  sleep 2
done
if [[ -z "$port" ]] || ! curl -kfsS "https://127.0.0.1:$port$health_path" >/dev/null; then
  docker logs "$container_name" --tail 150 >&2
  exit 1
fi

image_id="$(docker inspect --format '{{.Image}}' "$container_name")"
if [[ "$provider" == "minio" ]]; then
  provider_version="$(docker exec "$container_name" minio --version | head -1) image $image_id"
else
  provider_version="RustFS 1.0.0 image $image_id"
fi

OPENMUSE_TCK_S3_KIND="$provider" \
OPENMUSE_TCK_S3_ENDPOINT="https://127.0.0.1:$port" \
OPENMUSE_TCK_S3_REGION=us-east-1 \
OPENMUSE_TCK_S3_BUCKET="openmuse-tck-${provider//_/-}-$$" \
OPENMUSE_TCK_S3_PREFIX="qualification-$provider-$(date +%s)-$$" \
OPENMUSE_TCK_S3_ACCESS_KEY_ID="$access_key" \
OPENMUSE_TCK_S3_SECRET_ACCESS_KEY="$secret_key" \
OPENMUSE_TCK_S3_CA_BUNDLE="$temp_dir/public.crt" \
OPENMUSE_TCK_S3_CREATE_BUCKET=1 \
OPENMUSE_TCK_PROVIDER_VERSION="$provider_version" \
run_tck
