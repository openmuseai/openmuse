import { useState } from 'react';

import { CustomIconPopover } from '@/components/_shared/cutsom-icon';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';

import { useEventIcon } from '../hooks/useEventIcon';

interface EventIconButtonProps {
  rowId: string;
  readOnly?: boolean;
  className?: string;
  iconSize?: number;
}

export function EventIconButton({ rowId, readOnly = false, iconSize, className }: EventIconButtonProps) {
  const { showIcon, isFlag, onSelectIcon, removeIcon, renderIcon } = useEventIcon(rowId);

  const [open, setOpen] = useState(false);
  const icon = renderIcon(iconSize);

  if (!showIcon || !icon) return null;

  return (
    <div className={cn('custom-icon relative h-4 w-4 flex-shrink-0 ', className)}>
      <Button
        onClick={(e) => {
          e.stopPropagation();
          setOpen(true);
        }}
        className='h-full w-full !rounded-100 p-0'
        variant={'ghost'}
        style={{
          color: 'unset',
        }}
        disabled={readOnly}
      >
        <div className={cn('flex h-full w-full items-center justify-center', isFlag && 'icon')}>{icon}</div>
      </Button>
      {open && (
        <CustomIconPopover
          open={open}
          onOpenChange={setOpen}
          defaultActiveTab={'emoji'}
          tabs={['emoji']}
          onSelectIcon={(icon) => {
            onSelectIcon(icon.value);
          }}
          removeIcon={removeIcon}
          enable={Boolean(!readOnly && showIcon)}
        >
          <div
            className='absolute left-0 top-0 h-full w-full'
            style={{
              zIndex: open ? 1 : -1,
              pointerEvents: open ? 'auto' : 'none',
            }}
          />
        </CustomIconPopover>
      )}
    </div>
  );
}
