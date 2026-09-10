import { createMuseBff } from "./muse-bff.js";

const env = (name: string, fallback = ""): string => process.env[name]?.trim() || fallback;

const port = Number(env("MUSE_BFF_PORT", "8010"));
const server = createMuseBff({
  gotrueUrl: env("MUSE_BFF_GOTRUE_URL", "http://127.0.0.1:9999"),
  tokenSecret: env("MUSE_DSH_TOKEN_SECRET", "muse-dsh-dev-secret-change-me"),
  tokenKid: env("MUSE_DSH_TOKEN_KID", "v1"),
  tokenTtlSecs: Number(env("MUSE_DSH_TOKEN_TTL_SECS", "900")),
  poolUrl: env("MUSE_DSH_POOL_URL"),
  onAccessLog: entry => process.stdout.write(`${JSON.stringify(entry)}\n`)
});
await new Promise<void>(resolve => server.listen(port, "127.0.0.1", resolve));
process.stdout.write(`muse-bff http://127.0.0.1:${port}\n`);
