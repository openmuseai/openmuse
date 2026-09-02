import { describe, expect, it } from "vitest";
import { LAYOUT_ADAPTER_SCRIPT, MOBILE_SURFACE_CSS, MOBILE_SURFACE_SNIPPET } from "../src/inject.js";
import { injectAtHead } from "../src/html.js";

describe("mobile surface inject", () => {
  it("scopes CSS to Muse markers and frozen data-* only", () => {
    expect(MOBILE_SURFACE_CSS).toContain("[data-muse-surface=mobile]");
    expect(MOBILE_SURFACE_CSS).toContain("[data-muse-mobile-frame]");
    expect(MOBILE_SURFACE_CSS).toContain("[data-sidebar-collapsed]");
    expect(MOBILE_SURFACE_CSS).toContain("[data-muse-mobile-header]");
    expect(MOBILE_SURFACE_CSS).toContain("[data-composer-card]");
    expect(MOBILE_SURFACE_CSS).toContain("[data-time-hover-root]");
    expect(MOBILE_SURFACE_CSS).toContain("[data-muse-token-chrome]");
    expect(MOBILE_SURFACE_CSS).not.toContain("[class*=module_]");
    expect(MOBILE_SURFACE_CSS).not.toContain(":nth-child");
  });

  it("does not put a raw < in the adapter script body", () => {
    const body = LAYOUT_ADAPTER_SCRIPT.slice("<script>".length, -"</script>".length);
    expect(body).not.toContain("<");
    expect(body.toLowerCase()).not.toContain("</script");
    expect(body).toContain("data-shell-overlay");
    expect(body).toContain("data-muse-mobile-frame");
    expect(body).toContain("打开侧边栏");
    expect(body).toContain("新建会话");
    expect(body).toContain("data-muse-token-chrome");
  });

  it("stays inert unless the Flutter JS channel is present", () => {
    const body = LAYOUT_ADAPTER_SCRIPT;
    expect(body).toContain("MuseNativeCapability");
    expect(body).toContain("postMessage");
    expect(body).toContain("setAttribute(\"data-muse-surface\",\"mobile\")");
    expect(body).not.toContain("pointer:coarse");
    expect(body).not.toContain("max-width:767px");
  });

  it("injects at head", () => {
    const html = injectAtHead("<head></head>", MOBILE_SURFACE_SNIPPET);
    expect(html.startsWith("<head>" + MOBILE_SURFACE_SNIPPET)).toBe(true);
  });
});
