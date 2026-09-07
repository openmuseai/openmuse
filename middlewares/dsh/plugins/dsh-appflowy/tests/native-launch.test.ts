import { describe, expect, it } from "vitest";
import { assertNativeEndpoint, isPrivateLaunchFile } from "../src/native-launch.js";

describe("native launch descriptor", () => {
  it("accepts a Windows named-pipe endpoint without treating it as a UDS path", () => {
    expect(() => assertNativeEndpoint("\\\\.\\pipe\\appflowy-muse-host-1", "win32")).not.toThrow();
    expect(() => assertNativeEndpoint("/tmp/appflowy-muse-host-0.sock", "win32")).toThrow();
  });

  it("keeps POSIX UDS endpoints absolute paths on darwin and linux", () => {
    expect(() => assertNativeEndpoint("/tmp/appflowy-muse-host-501.sock", "darwin")).not.toThrow();
    expect(() => assertNativeEndpoint("\\\\.\\pipe\\muse", "linux")).toThrow();
  });

  it("does not use POSIX mode bits as a privacy check on Windows", () => {
    const worldWritable = {
      isFile: () => true,
      uid: 0,
      mode: 0o666
    };
    expect(isPrivateLaunchFile(worldWritable, "win32", undefined)).toBe(true);
    expect(isPrivateLaunchFile(worldWritable, "darwin", () => 501)).toBe(false);
    expect(isPrivateLaunchFile({ isFile: () => true, uid: 501, mode: 0o600 }, "darwin", () => 501)).toBe(true);
    expect(isPrivateLaunchFile({ isFile: () => true, uid: 502, mode: 0o600 }, "darwin", () => 501)).toBe(false);
  });
});
