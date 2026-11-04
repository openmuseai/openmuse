#!/usr/bin/env bash
set -euo pipefail
[[ $(uname -s) == Linux ]] || { echo 'Run inside Linux (see tool/Dockerfile).' >&2; exit 1; }
if [[ $EUID == 0 ]]; then elevate=(); else elevate=(sudo); fi
"${elevate[@]}" apt-get update
"${elevate[@]}" apt-get install -y clang cmake ninja-build pkg-config libgtk-3-dev \
  liblzma-dev libwebkit2gtk-4.1-dev libjson-glib-dev libsecret-1-dev libstdc++-12-dev libssl-dev \
  git curl python3 unzip xz-utils rsync file dpkg-dev xvfb xauth dbus-x11 fonts-noto-cjk
flutter config --enable-linux-desktop
flutter precache --linux
pkg-config --atleast-version=2.40 webkit2gtk-4.1
