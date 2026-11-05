import React, { forwardRef, memo, useState } from 'react';

import { useUpdateCellDispatch } from '@/application/database-yjs/dispatch';
import { FieldId } from '@/application/types';
import { TextareaAutosize } from '@/components/ui/textarea-autosize';
import { createHotkey, HOT_KEY_NAME } from '@/utils/hotkeys';

function TextCellEditing(
  {
    defaultValue = '',
    placeholder,
    rowId,
    fieldId,
    onExit,
    onChange,
  }: {
    defaultValue?: string;
    rowId: string;
    fieldId: FieldId;
    placeholder?: string;
    onExit?: () => void;
    onChange?: (value: string) => void;
  },
  ref: React.Ref<HTMLTextAreaElement>
) {
  const onUpdateCell = useUpdateCellDispatch(rowId, fieldId);

  const [inputValue, setInputValue] = useState<string>(defaultValue);

  return (
    <TextareaAutosize
      ref={ref}
      onMouseDown={(e) => {
        e.stopPropagation();
      }}
      autoFocus
      value={inputValue}
      onChange={(e) => {
        setInputValue(e.target.value);
        onChange?.(e.target.value);
      }}
      onKeyDown={(e) => {
        e.stopPropagation();
        if (createHotkey(HOT_KEY_NAME.ENTER)(e.nativeEvent) || createHotkey(HOT_KEY_NAME.ESCAPE)(e.nativeEvent)) {
          if (inputValue !== defaultValue) {
            onUpdateCell(inputValue);
          }

          onExit?.();
        }
      }}
      onBlur={() => {
        if (inputValue !== defaultValue) {
          onUpdateCell(inputValue);
        }

        onExit?.();
      }}
      placeholder={placeholder}
      variant={'ghost'}
      size={'sm'}
      className={'w-full rounded-none  px-0 text-text-primary'}
    />
  );
}

export default memo(forwardRef(TextCellEditing));
