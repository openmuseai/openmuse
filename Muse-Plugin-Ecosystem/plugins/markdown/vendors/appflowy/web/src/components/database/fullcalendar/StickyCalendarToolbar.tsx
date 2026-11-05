import { CalendarApi } from '@fullcalendar/core';
import { useMemo } from 'react';

import { CalendarEvent, useDatabaseContext } from '@/application/database-yjs';

import { CustomToolbar } from './CustomToolbar';
import { CalendarViewType } from './types';

/**
 * Props for StickyCalendarToolbar component
 */
interface StickyCalendarToolbarProps {
  calendar?: CalendarApi | null;
  currentView?: CalendarViewType;
  onViewChange?: (view: CalendarViewType) => void;
  slideDirection?: 'up' | 'down' | null;
  emptyEvents?: CalendarEvent[];
  onDragStart?: (rowId: string) => void;
  draggingRowId?: string | null;
  onDragEnd?: () => void;
}

/**
 * Sticky calendar toolbar component that wraps CustomToolbar
 * Used for both normal and sticky positioning with proper spacing
 */
export function StickyCalendarToolbar(props: StickyCalendarToolbarProps) {
  const { paddingStart, paddingEnd } = useDatabaseContext();

  // Memoized style object matching calendar spacing
  const toolbarStyle = useMemo(
    () => ({
      marginLeft: paddingStart === undefined ? undefined : paddingStart,
      marginRight: paddingEnd === undefined ? undefined : paddingEnd,
    }),
    [paddingStart, paddingEnd]
  );

  // Memoized className matching calendar spacing
  const toolbarClassName = useMemo(
    () => 'mx-24 max-sm:!mx-6', // Same as calendar container
    []
  );

  return (
    <div style={toolbarStyle} className={toolbarClassName}>
      <CustomToolbar {...props} />
    </div>
  );
}