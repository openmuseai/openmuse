import { describe, expect, it } from "vitest";
import { MOBILE_INPUT_SCRIPT } from "../src/inject.js";

describe("mobile input inject", () => {
  const body = MOBILE_INPUT_SCRIPT.slice("<script>".length, -"</script>".length);

  it("does not put a raw < in the script body", () => {
    expect(body).not.toContain("<");
    expect(body.toLowerCase()).not.toContain("</script");
  });

  it("uses the frozen capability protocol and never auto-submits", () => {
    expect(body).toContain("muse.native-capability/v1");
    expect(body).toContain("speech.start");
    expect(body).toContain("MuseNativeCapability");
    expect(body).toContain("data-composer-seat");
    expect(body).toContain("data-composer-card");
    expect(body).toContain("data-muse-attach");
    expect(body).toContain("data-muse-mic");
    expect(body).not.toContain("submit(");
    expect(body).not.toContain("inputActions.submit");
  });

  it("keeps file bytes on the hidden file input, not the JS channel", () => {
    expect(body).toContain("input");
    expect(body).toContain("image/png");
    expect(body).not.toContain("base64");
    expect(body).not.toContain("content://");
  });
});
