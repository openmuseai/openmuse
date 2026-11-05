import dayjs from 'dayjs';
import { useMemo } from 'react';
import { useTranslation } from 'react-i18next';

import { FieldType, useCalendarLayoutSetting, usePropertiesSelector } from '@/application/database-yjs';
import { useUpdateCalendarSetting } from '@/application/database-yjs/dispatch';
import { ReactComponent as CalendarIcon } from '@/assets/icons/calendar.svg';
import { FieldDisplay } from '@/components/database/components/field';
import {
  DropdownMenuItem,
  DropdownMenuItemTick,
  DropdownMenuLabel,
  DropdownMenuPortal,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
} from '@/components/ui/dropdown-menu';

function CalendarLayoutSettings() {
  const { t } = useTranslation();
  const calendarSetting = useCalendarLayoutSetting();
  const updateCalendarSetting = useUpdateCalendarSetting();

  const { properties: allProperties } = usePropertiesSelector(false);
  const dateTimeProperties = useMemo(() => {
    return allProperties.filter((property) =>
      [FieldType.DateTime, FieldType.LastEditedTime, FieldType.CreatedTime].includes(property.type)
    );
  }, [allProperties]);

  const weekDays = useMemo(() => {
    return Array.from({ length: 2 }, (_, i) => {
      const day = dayjs().day(i);

      return {
        value: i,
        name: day.format('ddd'),
      };
    });
  }, []);

  const handleFieldChange = (fieldId: string) => {
    updateCalendarSetting({ fieldId });
  };

  const handleFirstDayOfWeekChange = (firstDayOfWeek: number) => {
    updateCalendarSetting({ firstDayOfWeek });
  };

  return (
    <DropdownMenuSub>
      <DropdownMenuSubTrigger>
        <CalendarIcon />
        {t('grid.settings.calendarSettings')}
      </DropdownMenuSubTrigger>
      <DropdownMenuPortal>
        <DropdownMenuSubContent className={'appflowy-scroller max-w-[240px] overflow-y-auto'}>
          <DropdownMenuLabel>{t('calendar.settings.layoutDateField')}</DropdownMenuLabel>
          {dateTimeProperties.map((property) => (
            <DropdownMenuItem
              key={property.id}
              className={'w-full'}
              onSelect={(e) => {
                e.preventDefault();
                handleFieldChange(property.id);
              }}
            >
              <FieldDisplay fieldId={property.id} />
              {calendarSetting?.fieldId === property.id && <DropdownMenuItemTick />}
            </DropdownMenuItem>
          ))}

          {dateTimeProperties.length > 0 && <DropdownMenuSeparator />}

          <DropdownMenuLabel>{t('calendar.settings.firstDayOfWeek')}</DropdownMenuLabel>
          {weekDays.map((day) => (
            <DropdownMenuItem
              key={day.value}
              className={'w-full'}
              onSelect={(e) => {
                e.preventDefault();
                handleFirstDayOfWeekChange(day.value);
              }}
            >
              {day.name}
              {calendarSetting?.firstDayOfWeek === day.value && <DropdownMenuItemTick />}
            </DropdownMenuItem>
          ))}
        </DropdownMenuSubContent>
      </DropdownMenuPortal>
    </DropdownMenuSub>
  );
}

export default CalendarLayoutSettings;
