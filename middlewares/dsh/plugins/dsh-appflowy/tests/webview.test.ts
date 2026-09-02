import { describe, expect, it } from "vitest";
import {
  encodeScopedPluginUrls,
  SCOPED_PLUGIN_URL_PATCH_SCRIPT,
} from "../src/webview.js";

const bootScript = (url: string): string =>
  `<script>window.__DSH_BOOT__ = {"entries":[{"id":"@deepseek-ai/dsh-api-gateway","url":"${url}"}]}</script>`;

describe("WKWebView plugin URL encoding", () => {
  it("encodes scoped /plugins/@ URLs already present in the html", () => {
    const html = [
      "<head>",
      bootScript("/plugins/@deepseek-ai/dsh-api-gateway/client.js?rev=abc"),
      '<script src="/assets/shell.js"></script>',
      "</head>",
    ].join("");
    const encoded = encodeScopedPluginUrls(html);
    expect(encoded).toContain(
      "/plugins/%40deepseek-ai/dsh-api-gateway/client.js?rev=abc",
    );
    expect(encoded).not.toContain("/plugins/@deepseek-ai/");
    expect(encoded).toContain("/assets/shell.js");
  });

  it("injects a runtime src patch even when the boot graph is not yet in the html", () => {
    const html = "<head><script src=/assets/shell.js></script></head>";
    const out = encodeScopedPluginUrls(html);
    expect(out.startsWith("<head>" + SCOPED_PLUGIN_URL_PATCH_SCRIPT)).toBe(true);
    expect(out).toContain("HTMLScriptElement.prototype");
    expect(out).toContain("/plugins/%40");
  });

  it("still rewrites boot URLs after a later inject-at-head tap (client-modules order)", () => {
    const beforeBoot = encodeScopedPluginUrls(
      "<head><script src=/assets/shell.js></script></head>",
    );
    const afterBoot = beforeBoot.replace(
      "<head>",
      `<head>${bootScript("/plugins/@deepseek-ai/dsh-api-gateway/client.js?rev=1")}`,
    );
    const bootIdx = afterBoot.indexOf("__DSH_BOOT__");
    const patchIdx = afterBoot.indexOf("HTMLScriptElement.prototype");
    expect(bootIdx).toBeGreaterThan(-1);
    expect(patchIdx).toBeGreaterThan(bootIdx);
  });

  it("does not put a raw < in the patch script body (HTML parser)", () => {
    const body = SCOPED_PLUGIN_URL_PATCH_SCRIPT.slice(
      "<script>".length,
      -"</script>".length,
    );
    expect(body).not.toContain("<");
    expect(body.toLowerCase()).not.toContain("</script");
  });
});
