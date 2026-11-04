# syntax=docker/dockerfile:1
FROM debian:12-slim
ARG FLUTTER_VERSION=3.44.2
ARG DEBIAN_MIRROR=http://mirrors.tuna.tsinghua.edu.cn/debian
ENV DEBIAN_FRONTEND=noninteractive
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt/lists,sharing=locked \
    sed -i "s#http://deb.debian.org/debian#${DEBIAN_MIRROR}#g" /etc/apt/sources.list.d/debian.sources \
    && rm -f /etc/apt/apt.conf.d/docker-clean \
    && apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates git curl python3 unzip xz-utils clang cmake ninja-build \
    pkg-config libgtk-3-dev liblzma-dev libwebkit2gtk-4.1-dev libjson-glib-dev libsecret-1-dev \
    libstdc++-12-dev libssl-dev rsync file dpkg-dev xvfb xauth dbus-x11
RUN git clone --depth 1 --branch ${FLUTTER_VERSION} https://github.com/flutter/flutter.git /opt/flutter
ENV PATH="/opt/flutter/bin:/root/.cargo/bin:${PATH}"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain 1.90.0 \
    && flutter config --enable-linux-desktop && flutter precache --linux
WORKDIR /workspace
CMD ["bash", "app/openmuse_linux/tool/build.sh"]
