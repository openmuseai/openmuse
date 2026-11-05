import { CalendarApi } from '@fullcalendar/core';
import { useCallback, useEffect } from 'react';

import { createHotkey, HOT_KEY_NAME, isInputElement } from '@/utils/hotkeys';

import { CalendarViewType } from '../types';

interface UseCalendarKeyboardShortcutsProps {
  calendar?: CalendarApi | null;
  currentView: CalendarViewType;
  onViewChange?: (view: CalendarViewType) => void;
  onPrev?: () => void;
  onNext?: () => void;
  onToday?: () => void;
}

export const useCalendarKeyboardShortcuts = ({
  calendar,
  onViewChange,
  onPrev,
  onNext,
  onToday,
}: UseCalendarKeyboardShortcutsProps) => {
  const isMonthViewHotkey = createHotkey(HOT_KEY_NAME.CALENDAR_MONTH_VIEW);
  const isWeekViewHotkey = createHotkey(HOT_KEY_NAME.CALENDAR_WEEK_VIEW);
  const isPrevHotkey = createHotkey(HOT_KEY_NAME.CALENDAR_PREV);
  const isNextHotkey = createHotkey(HOT_KEY_NAME.CALENDAR_NEXT);
  const isTodayHotkey = createHotkey(HOT_KEY_NAME.CALENDAR_TODAY);

  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      if (isInputElement()) {
        return;
      }

      if (isMonthViewHotkey(event)) {
        event.preventDefault();
        onViewChange?.(CalendarViewType.DAY_GRID_MONTH);
        return;
      }

      if (isWeekViewHotkey(event)) {
        event.preventDefault();
        onViewChange?.(CalendarViewType.TIME_GRID_WEEK);
        return;
      }

      if (isPrevHotkey(event)) {
        event.preventDefault();
        
        onPrev?.();
        return;
      }

      if (isNextHotkey(event)) {
        event.preventDefault();
        
        onNext?.();
        return;
      }

      if (isTodayHotkey(event)) {
        event.preventDefault();
        onToday?.();
        return;
      }
    },
    [onViewChange, onPrev, onNext, onToday, isMonthViewHotkey, isWeekViewHotkey, isPrevHotkey, isNextHotkey, isTodayHotkey]
  );

  useEffect(() => {
    if (!calendar) return;

    document.addEventListener('keydown', handleKeyDown);

    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [calendar, handleKeyDown]);
};