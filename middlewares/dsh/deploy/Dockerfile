ARG BASE_IMAGE=node:22.19.0-bookworm
FROM ${BASE_IMAGE}

RUN corepack enable && corepack prepare pnpm@11.7.0 --activate \
    && apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates python3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /muse/dsh

COPY dsh/ /muse/dsh/
COPY packages/ /muse/packages/
COPY patch.yml /muse/patch.yml
COPY entrypoint.sh /muse/entrypoint.sh
COPY loopback-proxy.mjs /muse/loopback-proxy.mjs
COPY wire-harness-aliases.py /muse/wire-harness-aliases.py
RUN chmod +x /muse/entrypoint.sh

ARG NPM_REGISTRY=
RUN { \
      echo "fetch-timeout=600000"; \
      echo "fetch-retries=8"; \
      if [ -n "${NPM_REGISTRY}" ]; then echo "registry=${NPM_REGISTRY}"; fi; \
    } >> .npmrc \
    && pnpm install --frozen-lockfile \
    && pnpm add -w canonicalize@4.0.0 ajv@8.20.0 ajv-formats@3.0.1 @noble/hashes@2.3.0 \
    && mkdir -p /muse/dsh/node_modules/@muse \
    && cp -a /muse/packages/. /muse/dsh/node_modules/@muse/ \
    && python3 /muse/wire-harness-aliases.py /muse/dsh

# Optional community market (pure JS). build-dsh-image.sh fills dshmarket-optional/.
COPY dshmarket-optional/ /tmp/dshmarket-optional/
RUN if [ -f /tmp/dshmarket-optional/package.json ]; then \
      rm -rf /muse/dsh/node_modules/dshmarket && \
      mkdir -p /muse/dsh/node_modules/dshmarket && \
      cp -a /tmp/dshmarket-optional/. /muse/dsh/node_modules/dshmarket/; \
    fi

ENV DSH_HOME=/var/lib/muse-dsh
ENV PORT=3080
ENV DSH_LOOPBACK_PORT=13080
EXPOSE 3080

VOLUME ["/var/lib/muse-dsh"]

ENTRYPOINT ["/muse/entrypoint.sh"]
