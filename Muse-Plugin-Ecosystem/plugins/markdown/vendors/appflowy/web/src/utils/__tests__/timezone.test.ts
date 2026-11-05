/// <reference types="jest" />
import { getUserTimezoneInfo, isValidIANATimezone } from '../timezone';

// Mock date-fns-tz
jest.mock('date-fns-tz', () => ({
  getTimezoneOffset: jest.fn(() => -300 * 60 * 1000), // -5 hours in milliseconds
  toZonedTime: jest.fn((date) => date),
}));

describe('timezone utilities', () => {
  // Save original values
  const originalTimeZone = Intl.DateTimeFormat().resolvedOptions().timeZone;
  const originalLanguage = navigator.language;

  beforeEach(() => {
    // Reset mocks
    jest.clearAllMocks();
    
    // Mock console methods
    jest.spyOn(console, 'warn').mockImplementation();
  });

  afterEach(() => {
    jest.restoreAllMocks();
  });

  describe('isValidIANATimezone', () => {
    it('should validate correct IANA timezone identifiers', () => {
      const validTimezones = [
        'America/New_York',
        'Europe/London',
        'Asia/Tokyo',
        'UTC',
        'GMT',
        'Australia/Sydney',
        'America/Argentina/Buenos_Aires',
      ];

      validTimezones.forEach(tz => {
        expect(isValidIANATimezone(tz)).toBe(true);
      });
    });

    it('should reject invalid timezone identifiers', () => {
      // Mock toZonedTime to throw for invalid timezones
      const { toZonedTime } = require('date-fns-tz');
      
      const invalidTimezones = [
        'Invalid/Timezone',
        'Not_A_Timezone',
        '',
      ];

      invalidTimezones.forEach(tz => {
        toZonedTime.mockImplementationOnce(() => {
          throw new Error('Invalid timezone');
        });
        expect(isValidIANATimezone(tz)).toBe(false);
      });
    });

    it('should handle timezone validation errors gracefully', () => {
      // Mock toZonedTime to throw error
      const { toZonedTime } = require('date-fns-tz');
      toZonedTime.mockImplementationOnce(() => {
        throw new Error('Invalid timezone');
      });

      expect(isValidIANATimezone('Bad/Timezone')).toBe(false);
    });
  });

  describe('getUserTimezoneInfo', () => {
    it('should return correct timezone information', () => {
      // Mock Intl.DateTimeFormat
      const mockResolvedOptions = jest.fn(() => ({
        timeZone: 'America/New_York',
      }));
      
      jest.spyOn(Intl, 'DateTimeFormat').mockImplementation(() => ({
        resolvedOptions: mockResolvedOptions,
        format: jest.fn(),
        formatToParts: jest.fn(),
        formatRange: jest.fn(),
        formatRangeToParts: jest.fn(),
      } as any));

      // Mock navigator.language
      Object.defineProperty(navigator, 'language', {
        value: 'en-US',
        configurable: true,
      });

      const result = getUserTimezoneInfo();

      expect(result).toEqual({
        timezone: 'America/New_York',
        offset: -300, // -5 hours in minutes
        offsetString: 'UTC-05:00',
        locale: 'en-US',
      });
    });

    it('should fallback to UTC for invalid timezone', () => {
      // Mock Intl to return invalid timezone
      jest.spyOn(Intl, 'DateTimeFormat').mockImplementation(() => ({
        resolvedOptions: () => ({ timeZone: 'Invalid/Zone' }),
        format: jest.fn(),
        formatToParts: jest.fn(),
        formatRange: jest.fn(),
        formatRangeToParts: jest.fn(),
      } as any));

      // Mock toZonedTime to throw for invalid timezone
      const { toZonedTime } = require('date-fns-tz');
      toZonedTime.mockImplementationOnce(() => {
        throw new Error('Invalid timezone');
      });

      const result = getUserTimezoneInfo();

      expect(result).toEqual({
        timezone: 'UTC',
        offset: 0,
        offsetString: 'UTC+00:00',
        locale: 'en-US',
      });
      
      expect(console.warn).toHaveBeenCalledWith(
        'Invalid IANA timezone detected: Invalid/Zone, falling back to UTC'
      );
    });

    it('should handle positive timezone offsets correctly', () => {
      // Mock for Tokyo (UTC+9)
      jest.spyOn(Intl, 'DateTimeFormat').mockImplementation(() => ({
        resolvedOptions: () => ({ timeZone: 'Asia/Tokyo' }),
        format: jest.fn(),
        formatToParts: jest.fn(),
        formatRange: jest.fn(),
        formatRangeToParts: jest.fn(),
      } as any));

      // Mock positive offset (9 hours = 540 minutes)
      const { getTimezoneOffset } = require('date-fns-tz');
      getTimezoneOffset.mockReturnValue(540 * 60 * 1000);

      const result = getUserTimezoneInfo();

      expect(result.offset).toBe(540);
      expect(result.offsetString).toBe('UTC+09:00');
    });

    it('should handle timezone with minutes offset', () => {
      // Mock for India (UTC+5:30)
      jest.spyOn(Intl, 'DateTimeFormat').mockImplementation(() => ({
        resolvedOptions: () => ({ timeZone: 'Asia/Kolkata' }),
        format: jest.fn(),
        formatToParts: jest.fn(),
        formatRange: jest.fn(),
        formatRangeToParts: jest.fn(),
      } as any));

      // Mock offset with minutes (5.5 hours = 330 minutes)
      const { getTimezoneOffset } = require('date-fns-tz');
      getTimezoneOffset.mockReturnValue(330 * 60 * 1000);

      const result = getUserTimezoneInfo();

      expect(result.offset).toBe(330);
      expect(result.offsetString).toBe('UTC+05:30');
    });

    it('should use fallback locale when navigator.language is undefined', () => {
      jest.spyOn(Intl, 'DateTimeFormat').mockImplementation(() => ({
        resolvedOptions: () => ({ timeZone: 'UTC' }),
        format: jest.fn(),
        formatToParts: jest.fn(),
        formatRange: jest.fn(),
        formatRangeToParts: jest.fn(),
      } as any));

      // Mock navigator.language as undefined
      Object.defineProperty(navigator, 'language', {
        value: undefined,
        configurable: true,
      });

      const result = getUserTimezoneInfo();

      expect(result.locale).toBe('en-US');
    });
  });

});