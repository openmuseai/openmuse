import { lazy } from 'react';

export const Calendar = lazy(() => import('./FullCalendar'));
export * from './event';
