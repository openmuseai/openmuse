import React, { useState } from 'react';
import { useTranslation } from 'react-i18next';

import { SelectOption, useReadOnly } from '@/application/database-yjs';
import { ReactComponent as CheckboxCheckSvg } from '@/assets/icons/check_filled.svg';
import { ReactComponent as DeleteIcon } from '@/assets/icons/delete.svg';
import { ReactComponent as CheckboxUncheckSvg } from '@/assets/icons/uncheck.svg';
import DragItem from '@/components/database/components/drag-and-drop/DragItem';
import { Button } from '@/components/ui/button';
import { dropdownMenuItemVariants } from '@/components/ui/dropdown-menu';
import { TextareaAutosize } from '@/components/ui/textarea-autosize';
import { cn } from '@/lib/utils';

function TaskItem({
  task,
  isSelected,
  onToggleChecked,
  onChange,
  onRemove,
}: {
  task: SelectOption;
  isSelected: boolean;
  onChange: (task: SelectOption) => void;
  onToggleChecked: (taskId: string) => void;
  onRemove: (taskId: string) => void;
}) {
  const [value, setValue] = useState<string>(task.name);
  const { t } = useTranslation();
  const handleSubmit = (value: string) => {
    if (value === task.name) {
      return;
    }

    const newTask = {
      ...task,
      name: value,
    };

    onChange(newTask);
  };

  const handleChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setValue(event.target.value);
    handleSubmit(event.target.value);
  };

  const [isHovered, setHovered] = useState(false);

  const readOnly = useReadOnly();

  return (
    <div
      onMouseEnter={() => {
        if (readOnly) return;
        setHovered(true);
      }}
      onMouseLeave={() => setHovered(false)}
      className={cn(dropdownMenuItemVariants({ variant: 'default' }), 'h-fit w-full gap-0')}
    >
      <DragItem id={task.id}>
        <span
          onClick={(e) => {
            if (readOnly) return;
            e.preventDefault();
            e.stopPropagation();

            onToggleChecked(task.id);
          }}
          className={'cursor-pointer'}
        >
          {isSelected ? <CheckboxCheckSvg /> : <CheckboxUncheckSvg className={'text-border-primary hover:text-border-primary-hover'} />}
        </span>
        <TextareaAutosize
          onKeyDown={(e) => {
            if (readOnly) return;
            if (e.key === 'Escape') {
              e.stopPropagation();
              (e.target as HTMLTextAreaElement).blur();
            }
          }}
          placeholder={t('grid.checklist.taskHint')}
          data-task-id={`${task.id}`}
          className={'w-full whitespace-pre-wrap break-words'}
          variant={'ghost'}
          value={value}
          onChange={handleChange}
          readOnly={readOnly}
        />
        <Button
          tabIndex={-1}
          onClick={(e) => {
            if (readOnly) return;
            e.preventDefault();
            e.stopPropagation();
            onRemove(task.id);
          }}
          variant={'ghost'}
          size={'icon-sm'}
          style={{
            visibility: isHovered ? 'visible' : 'hidden',
          }}
          danger
          className={'ml-auto'}
        >
          <DeleteIcon />
        </Button>
      </DragItem>
    </div>
  );
}

export default TaskItem;
