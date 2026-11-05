import * as timezoneUtils from '@/utils/timezone';
import { expect } from '@jest/globals';
import { act, renderHook, waitFor } from '@testing-library/react';
import { useUserTimezone } from '../useUserTimezone';

// Mock the timezone utilities
jest.mock('@/utils/timezone', () => ({
  getUserTimezoneInfo: jest.fn(() => ({
    timezone: 'America/New_York',
    offset: -300,
    offsetString: 'UTC-05:00',
    locale: 'en-US',
  })),
}));

describe('useUserTimezone hook', () => {
  let mockGetUserTimezoneInfo: jest.SpyInstance;

  beforeEach(() => {
    // Clear all mocks
    jest.clearAllMocks();

    // Clear sessionStorage
    sessionStorage.clear();

    // Mock console methods
    jest.spyOn(console, 'log').mockImplementation();
    jest.spyOn(console, 'error').mockImplementation();

    // Setup timezone mock with default return value
    mockGetUserTimezoneInfo = jest.spyOn(timezoneUtils, 'getUserTimezoneInfo').mockReturnValue({
      timezone: 'America/New_York',
      offset: -300,
      offsetString: 'UTC-05:00',
      locale: 'en-US',
    });
  });

  afterEach(() => {
    jest.restoreAllMocks();
    jest.clearAllTimers();
  });

  describe('initial detection', () => {
    it('should detect timezone on mount', async () => {
      const onTimezoneChange = jest.fn();

      const { result } = renderHook(() =>
        useUserTimezone({
          onTimezoneChange,
          updateInterval: 0,
        })
      );

      // Wait for the effect to run
      await waitFor(() => {
        expect(mockGetUserTimezoneInfo).toHaveBeenCalled();
      });

      expect(result.current).toEqual({
        timezone: 'America/New_York',
        offset: -300,
        offsetString: 'UTC-05:00',
        locale: 'en-US',
      });

      expect(onTimezoneChange).toHaveBeenCalledWith('America/New_York');
    });

    it('should store timezone in sessionStorage', async () => {
      const { result } = renderHook(() =>
        useUserTimezone({
          updateInterval: 0,
        })
      );

      await waitFor(() => {
        expect(result.current).not.toBeNull();
      });

      expect(sessionStorage.getItem('userTimezone')).toBe('America/New_York');
    });

    it('should only detect once when updateInterval is 0', async () => {
      const onTimezoneChange = jest.fn();

      renderHook(() =>
        useUserTimezone({
          onTimezoneChange,
          updateInterval: 0,
        })
      );

      await waitFor(() => {
        expect(onTimezoneChange).toHaveBeenCalledTimes(1);
      });

      // Wait a bit more to ensure it's not called again
      await new Promise(resolve => setTimeout(resolve, 100));

      expect(onTimezoneChange).toHaveBeenCalledTimes(1);
      expect(mockGetUserTimezoneInfo).toHaveBeenCalledTimes(1);
    });
  });

  describe('periodic monitoring', () => {
    beforeEach(() => {
      jest.useFakeTimers();
    });

    afterEach(() => {
      jest.useRealTimers();
    });

    it('should check periodically when updateInterval > 0', async () => {
      const onTimezoneChange = jest.fn();

      renderHook(() =>
        useUserTimezone({
          onTimezoneChange,
          updateInterval: 1000, // Check every second
        })
      );

      // Initial call
      await waitFor(() => {
        expect(mockGetUserTimezoneInfo).toHaveBeenCalledTimes(1);
      });

      // Fast-forward time
      act(() => {
        jest.advanceTimersByTime(1000);
      });

      expect(mockGetUserTimezoneInfo).toHaveBeenCalledTimes(2);

      act(() => {
        jest.advanceTimersByTime(1000);
      });

      expect(mockGetUserTimezoneInfo).toHaveBeenCalledTimes(3);
    });

    it('should detect timezone changes during monitoring', async () => {
      const onTimezoneChange = jest.fn();

      // Start with New York timezone
      mockGetUserTimezoneInfo.mockReturnValue({
        timezone: 'America/New_York',
        offset: -300,
        offsetString: 'UTC-05:00',
        locale: 'en-US',
      });

      renderHook(() =>
        useUserTimezone({
          onTimezoneChange,
          updateInterval: 1000,
        })
      );

      await waitFor(() => {
        expect(onTimezoneChange).toHaveBeenCalledWith('America/New_York');
      });

      // Change to London timezone
      mockGetUserTimezoneInfo.mockReturnValue({
        timezone: 'Europe/London',
        offset: 0,
        offsetString: 'UTC+00:00',
        locale: 'en-GB',
      });

      act(() => {
        jest.advanceTimersByTime(1000);
      });

      await waitFor(() => {
        expect(onTimezoneChange).toHaveBeenCalledWith('Europe/London');
      });

      expect(sessionStorage.getItem('userTimezone')).toBe('Europe/London');
    });

    it('should not call onChange if timezone hasn\'t changed', async () => {
      const onTimezoneChange = jest.fn();

      renderHook(() =>
        useUserTimezone({
          onTimezoneChange,
          updateInterval: 1000,
        })
      );

      await waitFor(() => {
        expect(onTimezoneChange).toHaveBeenCalledTimes(1);
      });

      // Advance time multiple times without changing timezone
      act(() => {
        jest.advanceTimersByTime(1000);
      });

      act(() => {
        jest.advanceTimersByTime(1000);
      });

      // Should still only be called once (initial detection)
      expect(onTimezoneChange).toHaveBeenCalledTimes(1);
    });
  });

  describe('visibility change handling', () => {
    it('should check timezone when tab becomes visible', async () => {
      const onTimezoneChange = jest.fn();

      renderHook(() =>
        useUserTimezone({
          onTimezoneChange,
          updateInterval: 60000, // Enable monitoring
        })
      );

      await waitFor(() => {
        expect(mockGetUserTimezoneInfo).toHaveBeenCalledTimes(1);
      });

      // Simulate tab becoming hidden
      Object.defineProperty(document, 'hidden', {
        value: true,
        writable: true,
      });

      // Simulate tab becoming visible again
      Object.defineProperty(document, 'hidden', {
        value: false,
        writable: true,
      });

      act(() => {
        document.dispatchEvent(new Event('visibilitychange'));
      });

      expect(mockGetUserTimezoneInfo).toHaveBeenCalledTimes(2);
    });

    it('should not check when tab is hidden', async () => {
      const onTimezoneChange = jest.fn();

      renderHook(() =>
        useUserTimezone({
          onTimezoneChange,
          updateInterval: 60000,
        })
      );

      await waitFor(() => {
        expect(mockGetUserTimezoneInfo).toHaveBeenCalledTimes(1);
      });

      // Simulate tab remaining hidden
      Object.defineProperty(document, 'hidden', {
        value: true,
        writable: true,
      });

      act(() => {
        document.dispatchEvent(new Event('visibilitychange'));
      });

      // Should not check again since tab is hidden
      expect(mockGetUserTimezoneInfo).toHaveBeenCalledTimes(1);
    });
  });

  describe('cleanup', () => {
    beforeEach(() => {
      jest.useFakeTimers();
    });

    afterEach(() => {
      jest.useRealTimers();
    });

    it('should cleanup interval on unmount', () => {
      const clearIntervalSpy = jest.spyOn(global, 'clearInterval');

      const { unmount } = renderHook(() =>
        useUserTimezone({
          updateInterval: 1000,
        })
      );

      unmount();

      expect(clearIntervalSpy).toHaveBeenCalled();
    });

    it('should remove visibility change listener on unmount', () => {
      const removeEventListenerSpy = jest.spyOn(document, 'removeEventListener');

      const { unmount } = renderHook(() =>
        useUserTimezone({
          updateInterval: 1000,
        })
      );

      unmount();

      expect(removeEventListenerSpy).toHaveBeenCalledWith(
        'visibilitychange',
        expect.any(Function)
      );
    });
  });

  describe('edge cases', () => {
    it('should handle missing onTimezoneChange callback', async () => {
      const { result } = renderHook(() =>
        useUserTimezone({
          updateInterval: 0,
        })
      );

      await waitFor(() => {
        expect(result.current).not.toBeNull();
      });

      // Should not throw error
      expect(result.current?.timezone).toBe('America/New_York');
    });

    it('should work with default options', async () => {
      const { result } = renderHook(() => useUserTimezone());

      await waitFor(() => {
        expect(result.current).not.toBeNull();
      });

      expect(result.current?.timezone).toBe('America/New_York');
    });
  });
});