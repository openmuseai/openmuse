/**
 * TypeScript interface matching the Rust UserTimezone struct
 */
export interface UserTimezone {
  /** The default timezone (detected from browser) */
  default_timezone: string;
  
  /** User's explicitly set timezone (optional) */
  timezone?: string | null;
}

/**
 * Create a UserTimezone object for initial setup
 * @param detectedTimezone - The IANA timezone detected from the browser
 * @returns UserTimezone object with default_timezone set and timezone as null
 */
export function createInitialTimezone(detectedTimezone: string): UserTimezone {
  return {
    default_timezone: detectedTimezone,
    timezone: null,
  };
}

/**
 * Check if timezone needs to be set (when it's None/null)
 * @param currentTimezone - The current UserTimezone from the server
 * @returns true if timezone needs to be set
 */
export function needsTimezoneUpdate(currentTimezone?: UserTimezone | null): boolean {
  // If no timezone data exists, or if timezone field is null/undefined, it needs update
  return !currentTimezone || currentTimezone.timezone === null || currentTimezone.timezone === undefined;
}