import { useCallback, useLayoutEffect, useMemo, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';

import { FieldType, parseSelectOptionTypeOptions, useFieldSelector } from '@/application/database-yjs';
import { useUpdateSelectOption } from '@/application/database-yjs/dispatch';
import { YjsDatabaseKey } from '@/application/types';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { cn } from '@/lib/utils';
import { createHotkey, HOT_KEY_NAME } from '@/utils/hotkeys';

function ColumnRename({
  id,
  fieldId,
  open,
  onOpenChange,
}: {
  id: string;
  fieldId: string;
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
}) {
  const { field, clock } = useFieldSelector(fieldId);
  const inputRef = useRef<HTMLInputElement | null>(null);
  const fieldType = Number(field?.get(YjsDatabaseKey.type)) as FieldType;
  const onUpdateOption = useUpdateSelectOption(fieldId);
  const { t } = useTranslation();

  const option = useMemo(() => {
    if (!field || ![FieldType.SingleSelect, FieldType.MultiSelect].includes(fieldType)) return;

    return parseSelectOptionTypeOptions(field)?.options.find((option) => option?.id === id);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [field, clock, id]);

  const [value, setValue] = useState(option?.name || '');

  const ref = useRef<HTMLButtonElement | null>(null);

  const [container, setContainer] = useState<HTMLDivElement | null>(null);
  const [focused, setFocused] = useState(false);

  useLayoutEffect(() => {
    if (!ref.current) return;

    const el = ref.current.parentElement as HTMLDivElement | null;

    if (!el) return;

    setContainer(el);
  }, []);

  const handleSubmit = useCallback(
    (inputValue: string) => {
      if (!option) {
        onOpenChange?.(false);
        return;
      }

      if (inputValue === option.name) {
        onOpenChange?.(false);
        return;
      }

      onUpdateOption(option.id, {
        ...option,
        name: inputValue,
      });
      onOpenChange?.(false);
    },
    [onOpenChange, onUpdateOption, option]
  );

  return (
    <Popover open={open} onOpenChange={onOpenChange}>
      <PopoverTrigger ref={ref} className={'absolute left-0 top-0'} />
      <PopoverContent
        container={container}
        align='start'
        side='bottom'
        alignOffset={0}
        sideOffset={-2}
        avoidCollisions={false}
        onCloseAutoFocus={(e) => e.preventDefault()}
        onKeyDown={(e) => {
          if (createHotkey(HOT_KEY_NAME.ENTER)(e.nativeEvent)) {
            handleSubmit(value);
          }
        }}
        className={'w-[190px] min-w-[190px]'}
      >
        <div
          className={cn(
            'flex w-full items-center gap-2 rounded-300 border border-border-primary px-2',
            focused && 'ring-1 ring-border-theme-thick'
          )}
        >
          <Input
            ref={(input: HTMLInputElement) => {
              if (!input) return;
              if (!inputRef.current) {
                setTimeout(() => {
                  input.setSelectionRange(0, input.value.length);
                }, 100);

                inputRef.current = input;
              }
            }}
            value={value}
            onChange={(e) => {
              setValue(e.target.value);
            }}
            className={'flex-1 px-0'}
            onFocus={() => {
              setFocused(true);
            }}
            variant='ghost'
            onBlur={() => {
              setFocused(false);
            }}
          />
          <Button
            onClick={() => {
              handleSubmit(value);
            }}
            size={'sm'}
            className={'h-[22px] rounded-200 px-2 text-xs'}
          >
            {t('button.done')}
          </Button>
        </div>
      </PopoverContent>
    </Popover>
  );
}

export default ColumnRename;
