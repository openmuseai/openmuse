#!/usr/bin/env bash
# Native Linux build. Run from any directory; packages stay under this app.
set -euo pipefail
APP="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ROOT="$(cd "$APP/../.." && pwd)"
PROFILE=release
TESTS=1
GRAMMARS=1
REGION=cn
while [[ $# -gt 0 ]]; do
  case "$1" in
    --debug) PROFILE=debug; shift ;;
    --skip-tests) TESTS=0; shift ;;
    --without-grammars) GRAMMARS=0; shift ;;
    --region) REGION="${2:?--region requires cn or us}"; shift 2 ;;
    -h|--help) echo 'build.sh [--debug] [--skip-tests] [--without-grammars] [--region cn|us]'; exit 0 ;;
    *) echo "Unknown option: $1" >&2; exit 2 ;;
  esac
done
case "$REGION" in cn|us) ;; *) echo 'Region must be cn or us' >&2; exit 2 ;; esac
[[ $(uname -s) == Linux ]] || { echo 'A Linux host is required; use tool/Dockerfile on macOS.' >&2; exit 1; }
case $(uname -m) in
  x86_64) ARCH=x64; DEB_ARCH=amd64 ;;
  aarch64|arm64) ARCH=arm64; DEB_ARCH=arm64 ;;
  *) echo 'Supported architectures: x86_64, aarch64' >&2; exit 1 ;;
esac
for command in flutter cargo rustup python3 curl tar sha256sum rsync dpkg-deb dpkg-shlibdeps file; do
  command -v "$command" >/dev/null || { echo "Missing $command; read README.md" >&2; exit 1; }
done
pkg-config --atleast-version=2.40 webkit2gtk-4.1
STAGE="$ROOT/target/linux-$ARCH"
DIST="$APP/build/dist/$DEB_ARCH/$REGION"
mkdir -p "$STAGE" "$DIST"
NODE_VERSION=22.19.0
NODE_ARCHIVE="node-v$NODE_VERSION-linux-$ARCH.tar.gz"
NODE_ROOT="$STAGE/node-v$NODE_VERSION-linux-$ARCH"
if [[ ! -f "$STAGE/$NODE_ARCHIVE" ]]; then
  curl --fail --location --retry 3 --retry-all-errors "https://nodejs.org/dist/v$NODE_VERSION/$NODE_ARCHIVE" -o "$STAGE/$NODE_ARCHIVE.part"
  mv "$STAGE/$NODE_ARCHIVE.part" "$STAGE/$NODE_ARCHIVE"
fi
expected=$(awk -v name="$NODE_ARCHIVE" '$2 == name {print $1}' "$ROOT/third_party/node/v$NODE_VERSION/SHASUMS256.txt")
[[ -n $expected ]] || { echo 'Missing pinned Node checksum' >&2; exit 1; }
(cd "$STAGE" && echo "$expected  $NODE_ARCHIVE" | sha256sum --check)
tar -xzf "$STAGE/$NODE_ARCHIVE" -C "$STAGE"
export PATH="$NODE_ROOT/bin:$PATH"
CLOSURE="$STAGE/dsh-closure"
if [[ -f "$CLOSURE/package-lock.json" ]] &&
   cmp -s "$CLOSURE/package-lock.json" "$ROOT/third_party/dsh/package-lock.json" &&
   cmp -s "$CLOSURE/package.json" "$ROOT/third_party/dsh/package.json"; then
  python3 "$ROOT/scripts/build_dsh_closure.py" --out "$CLOSURE" --validate-only
else
  # Rebuild only generated output belonging to this architecture.
  rm -rf "$CLOSURE"
  python3 "$ROOT/scripts/build_dsh_closure.py" --out "$CLOSURE"
fi
python3 "$ROOT/scripts/test_dsh_runtime.py" --node "$NODE_ROOT/bin/node" --closure "$CLOSURE"

echo 'Building the repository Helix fork for Linux'
export HELIX_DISABLE_AUTO_GRAMMAR_BUILD=1
export CARGO_TARGET_DIR="$STAGE/helix-target"
(cd "$ROOT/third_party/helix" && cargo +1.90.0 build -p helix-term --release --locked)
HX="$CARGO_TARGET_DIR/release/hx"
"$HX" --version

cd "$APP"
flutter pub get --enforce-lockfile
flutter analyze
if (( TESTS )); then flutter test --reporter expanded; fi
deployment_profile=china
if [[ "$REGION" == us ]]; then deployment_profile=global-us; fi
runtime_defines="$(mktemp)"
trap 'rm -f "$runtime_defines"' EXIT
bash "$ROOT/scripts/release/prepare-runtime-config.sh" "$deployment_profile" "$runtime_defines"
flutter_args=("--$PROFILE" "--dart-define-from-file=$runtime_defines")
flutter build linux "${flutter_args[@]}"
BUNDLE="$APP/build/linux/$ARCH/$PROFILE/bundle"
test -x "$BUNDLE/openmuse_linux"
dart build cli -t bin/openmuse.dart -o build/openmuse-cli
mkdir -p "$BUNDLE/openmuse/cli"
rsync -a --delete "$APP/build/openmuse-cli/bundle/" "$BUNDLE/openmuse/cli/"
test -x "$BUNDLE/openmuse/cli/bin/openmuse"
ENGINE="$BUNDLE/data/flutter_assets/packages/openmuse_helix_plugin/assets/engines/helix"
mkdir -p "$ENGINE/runtime"
install -m755 "$HX" "$ENGINE/hx"
rsync -a --exclude=grammars --exclude=sources \
  "$ROOT/plugins/helix/assets/engines/helix/runtime/" "$ENGINE/runtime/"
if (( GRAMMARS )); then
  GRAMMAR_CONFIG="$STAGE/grammar-config"
  mkdir -p "$GRAMMAR_CONFIG/helix"
  cp "$ROOT/scripts/helix-grammars.toml" "$GRAMMAR_CONFIG/helix/languages.toml"
  XDG_CONFIG_HOME="$GRAMMAR_CONFIG" HELIX_RUNTIME="$ENGINE/runtime" "$HX" --grammar fetch --strict
  XDG_CONFIG_HOME="$GRAMMAR_CONFIG" HELIX_RUNTIME="$ENGINE/runtime" "$HX" --grammar build --strict
  mkdir -p "$ENGINE/runtime/grammars"
  while IFS= read -r grammar; do
    install -m644 "$GRAMMAR_CONFIG/helix/runtime/grammars/$grammar.so" "$ENGINE/runtime/grammars/$grammar.so"
  done < <(python3 -c 'import tomllib,sys; print("\n".join(tomllib.load(open(sys.argv[1],"rb"))["use-grammars"]["only"]))' "$ROOT/scripts/helix-grammars.toml")
fi
DSH="$BUNDLE/openmuse/dsh"
mkdir -p "$DSH/node/bin" "$BUNDLE/licenses"
rsync -a --delete "$CLOSURE/node_modules/" "$DSH/node_modules/"
# Koffi ships both libc variants in one architecture package. Debian uses glibc.
rm -rf "$DSH/node_modules/@koromix/koffi-linux-$ARCH/musl_$ARCH"
rm -rf "$DSH/node_modules/@deepseek-ai/node-addon-system-linux-$ARCH/bin/musl"
for platform in linux-arm64 linux-x64 darwin-arm64 darwin-x64 win32-arm64 win32-x64; do
  if [[ "$platform" != "linux-$ARCH" ]]; then
    rm -rf "$DSH/node_modules/node-pty/prebuilds/$platform"
  fi
done
install -m755 "$NODE_ROOT/bin/node" "$DSH/node/bin/node"
cp "$CLOSURE/package.json" "$CLOSURE/package-lock.json" "$DSH/"
cp "$ROOT/LICENSE" "$BUNDLE/licenses/OpenMuse-AGPL-3.0.txt"
cp "$ROOT/third_party/helix/LICENSE" "$BUNDLE/licenses/Helix-MPL-2.0.txt"
cp "$NODE_ROOT/LICENSE" "$BUNDLE/licenses/Node-LICENSE.txt"
cp "$ROOT/third_party/dsh/LICENSE" "$BUNDLE/licenses/DSH-MIT.txt"
cp "$APP/docs/THIRD-PARTY.md" "$BUNDLE/licenses/THIRD-PARTY.md"
python3 "$APP/tool/package.py" --bundle "$BUNDLE" --dist "$DIST" --arch "$DEB_ARCH" \
  --profile "$PROFILE" --grammars "$GRAMMARS" --region "$REGION"
echo "Linux packages ready: $DIST"
