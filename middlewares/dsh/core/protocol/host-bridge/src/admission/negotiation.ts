import type { NegotiationResult, ProtocolOffer, VersionRange } from "../contract/types.js";
import { ProtocolViolation } from "../codec/errors.js";
import { assertWithinHardLimits, lowerLimits } from "../codec/limits.js";

const validRange = (range: VersionRange): boolean =>
  Number.isInteger(range.major) &&
  Number.isInteger(range.minMinor) &&
  Number.isInteger(range.maxMinor) &&
  range.major > 0 &&
  range.minMinor >= 0 &&
  range.maxMinor >= range.minMinor;

export const negotiateVersion = (client: ProtocolOffer, host: ProtocolOffer): NegotiationResult => {
  assertWithinHardLimits(client.limits);
  assertWithinHardLimits(host.limits);
  if (!client.versions.every(validRange) || !host.versions.every(validRange)) {
    throw new ProtocolViolation("INVALID_ENVELOPE", "version offer contains an invalid range");
  }

  let selectedMinor: number | undefined;
  for (const clientRange of client.versions) {
    if (clientRange.major !== 1) continue;
    for (const hostRange of host.versions) {
      if (hostRange.major !== 1) continue;
      const minimum = Math.max(clientRange.minMinor, hostRange.minMinor);
      const maximum = Math.min(clientRange.maxMinor, hostRange.maxMinor);
      if (minimum <= maximum && (selectedMinor === undefined || maximum > selectedMinor)) selectedMinor = maximum;
    }
  }
  if (selectedMinor === undefined) {
    throw new ProtocolViolation("UNSUPPORTED_PROTOCOL", "client and host have no common Muse Bridge v1 minor");
  }

  const hostFeatures = new Set(host.features);
  const features = [...new Set(client.features)].filter((feature) => hostFeatures.has(feature)).sort();
  return Object.freeze({ major: 1 as const, minor: selectedMinor, features, limits: lowerLimits(client.limits, host.limits) });
};
