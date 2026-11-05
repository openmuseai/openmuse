import { useTranslation } from 'react-i18next';

import { useUpdateDateTimeFieldFormat } from '@/application/database-yjs/dispatch';
import { YjsDatabaseKey } from '@/application/types';
import { useFieldTypeOption } from '@/components/database/components/cell/Cell.hooks';
import DateTimeFormatGroup from '@/components/database/components/property/date/DateTimeFormatGroup';
import {
  DropdownMenuGroup,
  DropdownMenuSeparator,
  DropdownMenuItem,
  DropdownMenuShortcut,
} from '@/components/ui/dropdown-menu';
import { Switch } from '@/components/ui/switch';

function DataTimePropertyMenuContent({
  fieldId,
  enableInclusivitiesTime,
}: {
  fieldId: string;
  enableInclusivitiesTime?: boolean;
}) {
  const { t } = useTranslation();

  const typeOption = useFieldTypeOption(fieldId);
  const includeTimeRaw = typeOption?.get(YjsDatabaseKey.include_time);
  const includeTime = typeof includeTimeRaw === 'boolean' ? includeTimeRaw : Boolean(includeTimeRaw);

  const updateFormat = useUpdateDateTimeFieldFormat(fieldId);

  return (
    <>
      <DropdownMenuSeparator />
      <DateTimeFormatGroup fieldId={fieldId} />
      <DropdownMenuGroup className={'max-w-[240px] overflow-hidden'}>
        {enableInclusivitiesTime && (
          <DropdownMenuItem
            onSelect={(e) => {
              e.preventDefault();
              updateFormat({
                includeTime: !includeTime,
              });
            }}
          >
            {t('grid.field.includeTime')}
            <DropdownMenuShortcut className={'flex items-center'}>
              <Switch checked={includeTime} />
            </DropdownMenuShortcut>
          </DropdownMenuItem>
        )}
      </DropdownMenuGroup>
    </>
  );
}

export default DataTimePropertyMenuContent;
