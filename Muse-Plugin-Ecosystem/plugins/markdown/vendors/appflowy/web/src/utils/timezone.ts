import { getTimezoneOffset, toZonedTime } from 'date-fns-tz';

export interface TimezoneInfo {
  timezone: string;
  offset: number;
  offsetString: string;
  locale: string;
}

/**
 * Validates if a timezone string is a valid IANA timezone identifier
 * that can be parsed by both date-fns-tz and chrono-tz in Rust
 */
export function isValidIANATimezone(timezone: string): boolean {
  try {
    // date-fns-tz uses the Intl API under the hood for timezone data
    // If this succeeds, the timezone is valid IANA format
    const testDate = new Date();

    toZonedTime(testDate, timezone);

    return true;
  } catch {
    return false;
  }
}

/**
 * Get the user's timezone information using date-fns-tz
 * Returns IANA timezone identifiers compatible with chrono-tz
 */
export function getUserTimezoneInfo(): TimezoneInfo {
  // Get the IANA timezone identifier from the browser
  // This returns strings like "America/New_York", "Europe/London", etc.
  // which are exactly what chrono-tz expects
  const timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

  // Validate the timezone using date-fns-tz
  if (!isValidIANATimezone(timezone)) {
    console.warn(`Invalid IANA timezone detected: ${timezone}, falling back to UTC`);
    return {
      timezone: 'UTC',
      offset: 0,
      offsetString: 'UTC+00:00',
      locale: navigator.language || 'en-US',
    };
  }

  // Use date-fns-tz to get the offset
  const now = new Date();
  const offsetInMs = getTimezoneOffset(timezone, now);
  const offsetInMinutes = offsetInMs / (1000 * 60);

  // Format the offset string
  const hours = Math.floor(Math.abs(offsetInMinutes) / 60);
  const minutes = Math.abs(offsetInMinutes) % 60;
  const sign = offsetInMinutes >= 0 ? '+' : '-';
  const offsetString = `UTC${sign}${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}`;

  const locale = navigator.language || 'en-US';

  return {
    timezone,
    offset: offsetInMinutes,
    offsetString,
    locale,
  };
}
