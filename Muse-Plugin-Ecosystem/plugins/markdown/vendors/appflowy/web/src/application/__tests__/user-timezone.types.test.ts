import { expect } from '@jest/globals';
import { createInitialTimezone, needsTimezoneUpdate, UserTimezone } from '../user-timezone.types';

describe('UserTimezone types and utilities', () => {
  describe('createInitialTimezone', () => {
    it('should create UserTimezone with default_timezone set and timezone as null', () => {
      const detectedTimezone = 'America/New_York';
      const result = createInitialTimezone(detectedTimezone);

      expect(result).toEqual({
        default_timezone: 'America/New_York',
        timezone: null,
      });
    });

    it('should handle different timezone strings', () => {
      const timezones = [
        'Europe/London',
        'Asia/Tokyo',
        'UTC',
        'America/Los_Angeles',
      ];

      timezones.forEach(tz => {
        const result = createInitialTimezone(tz);
        expect(result.default_timezone).toBe(tz);
        expect(result.timezone).toBeNull();
      });
    });
  });

  describe('needsTimezoneUpdate', () => {
    it('should return true when no timezone data exists', () => {
      expect(needsTimezoneUpdate(undefined)).toBe(true);
      expect(needsTimezoneUpdate(null)).toBe(true);
    });

    it('should return true when timezone field is null', () => {
      const timezoneData: UserTimezone = {
        default_timezone: 'America/New_York',
        timezone: null,
      };

      expect(needsTimezoneUpdate(timezoneData)).toBe(true);
    });

    it('should return true when timezone field is undefined', () => {
      const timezoneData: UserTimezone = {
        default_timezone: 'America/New_York',
        timezone: undefined,
      };

      expect(needsTimezoneUpdate(timezoneData)).toBe(true);
    });

    it('should return false when timezone is set', () => {
      const timezoneData: UserTimezone = {
        default_timezone: 'America/New_York',
        timezone: 'Europe/London', // User has set a custom timezone
      };

      expect(needsTimezoneUpdate(timezoneData)).toBe(false);
    });

    it('should return false when timezone is empty string', () => {
      // Empty string is considered "set" (different from null/undefined)
      const timezoneData: UserTimezone = {
        default_timezone: 'America/New_York',
        timezone: '',
      };

      expect(needsTimezoneUpdate(timezoneData)).toBe(false);
    });
  });

  describe('UserTimezone interface', () => {
    it('should match expected structure', () => {
      const validTimezone: UserTimezone = {
        default_timezone: 'America/New_York',
        timezone: 'Europe/London',
      };

      expect(validTimezone).toHaveProperty('default_timezone');
      expect(validTimezone).toHaveProperty('timezone');
    });

    it('should allow optional timezone field', () => {
      const timezoneWithNull: UserTimezone = {
        default_timezone: 'UTC',
        timezone: null,
      };

      const timezoneWithValue: UserTimezone = {
        default_timezone: 'UTC',
        timezone: 'Asia/Tokyo',
      };

      expect(timezoneWithNull.timezone).toBeNull();
      expect(timezoneWithValue.timezone).toBe('Asia/Tokyo');
    });
  });
});