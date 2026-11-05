import React from 'react';
import { useTranslation } from 'react-i18next';
import { toast } from 'sonner';

import { FieldType, useFieldType, useRowTimeString } from '@/application/database-yjs';
import { YjsDatabaseKey } from '@/application/types';
import { ReactComponent as CopyIcon } from '@/assets/icons/copy.svg';
import { Button } from '@/components/ui/button';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import { cn } from '@/lib/utils';
import { copyTextToClipboard } from '@/utils/copy';

export function RowCreateModifiedTime({
  rowId,
  fieldId,
  attrName,
  style,
  wrap,
  isHovering,
}: {
  rowId: string;
  fieldId: string;
  style?: React.CSSProperties;
  attrName: YjsDatabaseKey.last_modified | YjsDatabaseKey.created_at;
  wrap: boolean;
  isHovering?: boolean;
}) {
  const time = useRowTimeString(rowId, fieldId, attrName);
  const { t } = useTranslation();

  const fieldType = useFieldType(fieldId);

  const handleCopy = () => {
    if (!time) return;
    void copyTextToClipboard(time);
    toast.success(
      fieldType === FieldType.CreatedTime ? t('grid.field.copiedCreatedAt') : t('grid.field.copiedUpdatedAt')
    );
  };

  if (!time) return null;
  return (
    <div
      style={style}
      className={cn('flex w-full cursor-text select-text', wrap ? 'whitespace-pre-wrap break-all' : 'whitespace-nowrap')}
    >
      {time}
      {isHovering && (
        <div className={'absolute right-1 top-1'}>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                onClick={handleCopy}
                variant={'outline'}
                size={'icon'}
                className={'bg-surface-primary hover:bg-surface-primary-hover'}
              >
                <CopyIcon className={'h-5 w-5'} />
              </Button>
            </TooltipTrigger>
            <TooltipContent>{t('settings.menu.clickToCopy')}</TooltipContent>
          </Tooltip>
        </div>
      )}
    </div>
  );
}

export default RowCreateModifiedTime;
