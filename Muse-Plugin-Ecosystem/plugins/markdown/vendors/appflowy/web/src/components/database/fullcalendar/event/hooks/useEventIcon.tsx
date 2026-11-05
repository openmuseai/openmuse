import { useMemo } from 'react';

import { RowMetaKey, useRowMetaSelector } from '@/application/database-yjs';
import { useUpdateRowMetaDispatch } from '@/application/database-yjs/dispatch';
import { ReactComponent as DocumentSvg } from '@/assets/icons/page.svg';
import { isFlagEmoji } from '@/utils/emoji';


export function useEventIcon(rowId: string) {
  const meta = useRowMetaSelector(rowId);
  const onUpdateMeta = useUpdateRowMetaDispatch(rowId);

  const hasDocument = meta?.isEmptyDocument === false;
  const icon = meta?.icon;
  const showIcon = icon || hasDocument;

  const isFlag = useMemo(() => {
    if (!icon) return false;
    return isFlagEmoji(icon);
  }, [icon]);

  const onSelectIcon = (iconValue: string) => {
    onUpdateMeta(RowMetaKey.IconId, iconValue);
  };

  const removeIcon = () => {
    onUpdateMeta(RowMetaKey.IconId, undefined);
  };

  const renderIcon = (iconSize?: number) => {
    if (icon) {
      return icon;
    }

    if (hasDocument) {
      return (
        <DocumentSvg
          style={{
            height: iconSize,
            width: iconSize,
          }}
          className='h-full event-icon w-full'
        />
      );
    }

    return null;
  };

  return {
    icon,
    showIcon,
    isFlag,
    onSelectIcon,
    removeIcon,
    renderIcon,
    hasDocument,
  };
}