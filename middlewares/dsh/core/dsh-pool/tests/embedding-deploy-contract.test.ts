import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const root = join(dirname(fileURLToPath(import.meta.url)), "../../..");

describe("embedding deploy contract (E3)", () => {
  it("E3-T5 nginx auth_request targets the Muse BFF, not official Cloud :8000/api/muse", () => {
    const conf = readFileSync(join(root, "deploy/nginx/cloud-same-origin-dsh.conf"), "utf8");
    expect(conf).not.toMatch(/127\.0\.0\.1:8000\/api\/muse/);
    expect(conf).toMatch(/127\.0\.0\.1:8010\/api\/muse\/dsh\/ingress-auth/);
    expect(conf).toMatch(/location \/u\/[\s\S]*proxy_set_header Host \$http_host;/);
    expect(conf).not.toMatch(/location \/u\/[\s\S]*proxy_set_header Host 127\.0\.0\.1:13080;/);
    expect(conf).toMatch(/location \/dsh\/[\s\S]*proxy_set_header Host \$http_host;/);
    expect(conf).not.toMatch(/location \/dsh\/[\s\S]*proxy_set_header Host 127\.0\.0\.1:3080;/);
  });

  it("E3-T7 instance.env.example does not pin PORT or DSH_HOME", () => {
    const env = readFileSync(join(root, "deploy/instance.env.example"), "utf8");
    expect(env).not.toMatch(/^PORT=/m);
    expect(env).not.toMatch(/^DSH_HOME=/m);
    expect(env).toMatch(/^MUSE_DOCUMENT_CLOUD_URL=/m);
    expect(env).toMatch(/^MUSE_REQUIRE_HOST_AUTH=1/m);
    expect(env).toMatch(/^MUSE_MOBILE_BRIDGE=1/m);
    expect(env).toMatch(/^DSH_TRUSTED_HOST=openmuseai\.com,/m);
    expect(env).not.toMatch(/^DSH_TRUSTED_HOST=dsh\.openmuseai\.com$/m);
  });

  it("seeds gateway defaults and mounts dsh-model-capabilities", () => {
    const defaults = readFileSync(join(root, "deploy/defaults/settings.yaml"), "utf8");
    expect(defaults).toMatch(/opencode-custom:/);
    expect(defaults).toMatch(/x-opencode-session: "\{\{session\}\}"/);
    const patch = readFileSync(join(root, "plugins/dsh-appflowy/cordis.patch.yml"), "utf8");
    expect(patch).toContain("name: dsh-model-capabilities");
    const start = readFileSync(join(root, "deploy/scripts/start-instance.sh"), "utf8");
    expect(start).toContain("dsh-model-capabilities");
    expect(start).toContain("seed-instance-settings.sh");
  });
});
