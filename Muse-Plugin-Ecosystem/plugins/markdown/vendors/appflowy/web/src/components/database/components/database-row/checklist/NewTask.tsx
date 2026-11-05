import { useRef } from 'react';
import { useTranslation } from 'react-i18next';

import { ReactComponent as AddIcon } from '@/assets/icons/add_new_page.svg';
import { Separator } from '@/components/ui/separator';
import { TextareaAutosize } from '@/components/ui/textarea-autosize';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import { cn } from '@/lib/utils';

function NewTask({
  hovering,
  value,
  onChange,
  showCreateInput,
  setShowCreateInput,
  onCreateTask,
}: {
  hovering: boolean;
  value: string;
  onChange: (value: string) => void;
  showCreateInput: boolean;
  setShowCreateInput: (show: boolean) => void;
  onCreateTask: (value: string) => void;
}) {
  const { t } = useTranslation();
  const ref = useRef<HTMLTextAreaElement>(null);

  return (
    <div className={'flex w-full flex-col'}>
      {showCreateInput && (
        <div className={'mt-2 flex w-full items-center rounded-200 bg-fill-content-hover p-2'}>
          <TextareaAutosize
            data-task-id='create'
            placeholder={t('grid.checklist.taskHint')}
            ref={ref}
            value={value}
            onChange={(e) => {
              onChange(e.target.value);
            }}
            variant={'ghost'}
            autoFocus
            onBlur={() => {
              setShowCreateInput(false);
            }}
            className={'w-full rounded-none p-0 text-text-primary'}
          />
        </div>
      )}

      <Tooltip>
        <TooltipTrigger
          tabIndex={-1}
          onMouseDown={(e) => {
            e.stopPropagation();
            e.preventDefault();
            if (ref.current?.value) {
              onCreateTask(ref.current.value);
            }

            setShowCreateInput(true);
          }}
          className={cn(
            'flex w-full transform items-center gap-2 rounded-200 p-1 transition-all',
            hovering ? 'opacity-100' : 'opacity-0'
          )}
        >
          <Separator className={'flex-1'} />
          <AddIcon className={'h-5 w-5'} />
          <Separator className={'flex-1'} />
        </TooltipTrigger>
        <TooltipContent side={'bottom'}>{t('grid.checklist.addNew')}</TooltipContent>
      </Tooltip>
    </div>
  );
}

export default NewTask;
