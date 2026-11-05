import { forwardRef } from 'react';
import { createPortal } from 'react-dom';
import { useTranslation } from 'react-i18next';

import { ReactComponent as PlusIcon } from '@/assets/icons/plus.svg';
import { Button } from '@/components/ui/button';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';

interface AddButtonProps {
  visible: boolean;
  position: { top: number; left: number };
  date: Date | null;
  onClick: () => void;
  onMouseLeave: (e: React.MouseEvent) => void;
  container?: HTMLElement | null;
}

export const AddButton = forwardRef<HTMLButtonElement, AddButtonProps>(
  ({ visible, position, date, onClick, onMouseLeave, container }, ref) => {
    const { t } = useTranslation();

    if (!visible || !date) {
      return null;
    }

    return createPortal(
      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            ref={ref}
            data-add-button
            onClick={(e) => {
              e.stopPropagation();
              onClick();
            }}
            onMouseLeave={onMouseLeave}
            variant='outline'
            size='icon-sm'
            className='calendar-add-button absolute z-[50] !bg-surface-primary transition-opacity duration-150 hover:!bg-fill-content-hover'
            style={{
              top: position.top,
              left: position.left,
            }}
          >
            <PlusIcon className='h-4 w-4' />
          </Button>
        </TooltipTrigger>
        <TooltipContent>
          {t('calendar.addEventOn')} {date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })}
        </TooltipContent>
      </Tooltip>,
      container || document.body
    );
  }
);
