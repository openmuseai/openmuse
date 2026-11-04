FROM rust:1.86-bookworm

RUN apt-get update \
    && apt-get install -y --no-install-recommends ripgrep \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --gid 10001 openmuse \
    && useradd --uid 10001 --gid 10001 --home-dir /home/agent --no-create-home openmuse \
    && mkdir -p /workspace /runtime /home/agent /run/openmuse

ENV HOME=/home/agent
ENV PATH=/usr/local/cargo/bin:/usr/local/rustup/toolchains/1.86.0-x86_64-unknown-linux-gnu/bin:/usr/local/bin:/usr/bin:/bin
WORKDIR /workspace
USER 10001:10001
CMD ["bash"]
