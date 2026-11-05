#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
APP_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
WORKSPACE_ROOT=$(CDPATH= cd -- "$APP_DIR/../../../../.." && pwd)
VIEWER_VENDOR="$WORKSPACE_ROOT/vendors/open-file-viewer"
HELIX_VENDOR="$WORKSPACE_ROOT/vendors/helix"
VIEWER_OUT="$APP_DIR/assets/engines/open-file-viewer"
HELIX_OUT="$APP_DIR/assets/engines/helix"

mkdir -p "$VIEWER_OUT" "$HELIX_OUT"
pnpm --dir "$VIEWER_VENDOR" install --frozen-lockfile
# WKWebView loads Flutter assets through file:// on macOS. Safari/WebKit does
# not execute module entry points from that opaque local origin, so the macOS
# viewer must remain a classic self-contained script. Windows has a virtual
# HTTPS host and keeps its separately built ESM/code-split bundle.
find "$VIEWER_OUT" -maxdepth 1 -type f -name 'lazy-*.js' -delete
pnpm --dir "$VIEWER_VENDOR" exec esbuild \
  "$APP_DIR/tool/open_file_viewer/muse_viewer.ts" \
  --bundle --format=iife --platform=browser --target=safari16 \
  --outfile="$VIEWER_OUT/viewer.js"
cp "$APP_DIR/tool/open_file_viewer/index.html" "$VIEWER_OUT/index.html"
cp "$VIEWER_VENDOR/packages/core/src/style.css" "$VIEWER_OUT/viewer.css"
cp "$APP_DIR/tool/open_file_viewer/shell.css" "$VIEWER_OUT/shell.css"
PDFJS="$VIEWER_VENDOR/packages/core/node_modules/pdfjs-dist"
cp "$PDFJS/build/pdf.worker.mjs" "$VIEWER_OUT/pdf.worker.mjs"
rm -rf "$VIEWER_OUT/cmaps" "$VIEWER_OUT/standard_fonts"
cp -R "$PDFJS/cmaps" "$VIEWER_OUT/cmaps"
cp -R "$PDFJS/standard_fonts" "$VIEWER_OUT/standard_fonts"

# Build the Helix binary without compiling the full 300+ grammar catalog.
# A small Host language set is fetched/built into runtime/grammars below.
HELIX_DISABLE_AUTO_GRAMMAR_BUILD=1 cargo +1.90.0 build \
  --manifest-path "$HELIX_VENDOR/Cargo.toml" \
  --release \
  -p helix-term \
  --bin hx
cp "$HELIX_VENDOR/target/release/hx" "$HELIX_OUT/hx"
chmod 0755 "$HELIX_OUT/hx"
rm -rf "$HELIX_OUT/runtime"
mkdir -p "$HELIX_OUT/runtime"
cp "$HELIX_VENDOR/languages.toml" "$HELIX_OUT/runtime/languages.toml"
cp -R "$HELIX_VENDOR/runtime/themes" "$HELIX_OUT/runtime/themes"
cp -R "$HELIX_VENDOR/runtime/queries" "$HELIX_OUT/runtime/queries"
mkdir -p "$HELIX_OUT/runtime/grammars"

# Highlighting needs compiled tree-sitter libraries, not Language Servers.
# Fetch/build only the Host language set; skip the full 300+ grammar catalog.
GRAMMAR_XDG=$(mktemp -d "${TMPDIR:-/tmp}/helix-grammars.XXXXXX")
mkdir -p "$GRAMMAR_XDG/helix"
cat > "$GRAMMAR_XDG/helix/languages.toml" <<'EOF'
use-grammars = { only = ["dart", "rust", "json", "toml", "markdown", "markdown_inline", "comment"] }
EOF
unset CARGO_MANIFEST_DIR || true
if HELIX_RUNTIME="$HELIX_VENDOR/runtime" XDG_CONFIG_HOME="$GRAMMAR_XDG" \
  "$HELIX_OUT/hx" --grammar fetch &&
   HELIX_RUNTIME="$HELIX_VENDOR/runtime" XDG_CONFIG_HOME="$GRAMMAR_XDG" \
  "$HELIX_OUT/hx" --grammar build; then
  find "$GRAMMAR_XDG/helix/runtime/grammars" -maxdepth 1 \( -name '*.dylib' -o -name '*.so' -o -name '*.dll' \) \
    -exec cp {} "$HELIX_OUT/runtime/grammars/" \;
else
  echo "warning: Helix grammar fetch/build failed; syntax highlighting will require in-app install" >&2
fi
rm -rf "$GRAMMAR_XDG"

# Flutter asset directories are intentionally shallow. Keep the complete
# language query tree in one direct asset and unpack it into a private runtime
# directory on first open.
tar -cf "$HELIX_OUT/runtime.tar" -C "$HELIX_OUT" runtime

echo "Engine assets ready under $APP_DIR/assets/engines"
