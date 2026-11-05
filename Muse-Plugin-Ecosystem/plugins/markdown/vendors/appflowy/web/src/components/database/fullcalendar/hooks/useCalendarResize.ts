import { CalendarApi } from '@fullcalendar/core';
import { debounce } from 'lodash-es';
import { useEffect, useRef } from 'react';

/**
 * Custom hook to handle calendar resize events
 * Manages both window resize and element resize observers
 */
export function useCalendarResize(
  onRendered?: () => void,
  expanded?: boolean,
  isDocumentBlock?: boolean,
  calendar?: CalendarApi
) {
  const containerRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    const el = containerRef.current;

    if (!el) return;

    // Debounced resize handler to prevent excessive re-renders
    const onResize = debounce(() => {
      onRendered?.();
      calendar?.updateSize();
      // FullCalendar React component handles resize automatically
    }, 200);

    onResize();

    // Add window resize listener for general resize events
    window.addEventListener('resize', onResize);

    // Add element resize listener if available (requires ResizeObserver)
    let resizeObserver: ResizeObserver | null = null;

    if (window.ResizeObserver) {
      resizeObserver = new ResizeObserver(onResize);
      resizeObserver.observe(el);
    }

    return () => {
      window.removeEventListener('resize', onResize);
      if (resizeObserver) {
        resizeObserver.disconnect();
      }
    };
  }, [calendar, onRendered, expanded, isDocumentBlock]);

  return containerRef;
}
