import { useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';

import { formats, NumberFormat, parseNumberTypeOptions, useFieldSelector } from '@/application/database-yjs';
import { useUpdateNumberTypeOption } from '@/application/database-yjs/dispatch';
import {
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuPortal,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuItemTick,
} from '@/components/ui/dropdown-menu';
import { SearchInput } from '@/components/ui/search-input';

function NumberPropertyMenuContent({ fieldId }: { fieldId: string }) {
  const { field } = useFieldSelector(fieldId);
  const { t } = useTranslation();
  const onUpdateFormat = useUpdateNumberTypeOption();

  const format = useMemo(() => (field ? parseNumberTypeOptions(field).format : NumberFormat.Num), [field]);

  const selectFormatValue = useMemo(() => {
    return formats.find((item) => item.value === format);
  }, [format]);

  const [searchValue, setSearchValue] = useState('');

  const filteredOptions = useMemo(() => {
    return formats.filter((item) => {
      return item.label.toLowerCase().includes(searchValue.toLowerCase());
    });
  }, [searchValue]);

  const [open, setOpen] = useState(false);

  return (
    <>
      <DropdownMenuSeparator />
      <DropdownMenuGroup>
        <DropdownMenuLabel>{t('grid.field.numberFormat')}</DropdownMenuLabel>
        <DropdownMenuSub open={open} onOpenChange={setOpen}>
          <DropdownMenuSubTrigger
            {...(open && {
              onPointerMove: (e) => e.preventDefault(),
              onPointerLeave: (e) => e.preventDefault(),
            })}
          >
            {selectFormatValue?.label}
          </DropdownMenuSubTrigger>
          <DropdownMenuPortal>
            <DropdownMenuSubContent className={'appflowy-scroller max-h-[450px] max-w-[240px] overflow-y-auto pt-0'}>
              <div className={'sticky top-0 z-[1] flex flex-col bg-surface-primary pt-2'}>
                <SearchInput
                  inputRef={(el) => {
                    if (el) {
                      setTimeout(() => {
                        el.focus();
                      }, 100);
                    }
                  }}
                  placeholder={t('searchLabel')}
                  value={searchValue}
                  onChange={(e) => {
                    setSearchValue(e.target.value);
                  }}
                  onKeyDown={(e) => {
                    e.stopPropagation();
                  }}
                />
                <DropdownMenuSeparator />
              </div>

              {filteredOptions.map((item) => {
                return (
                  <DropdownMenuItem
                    key={item.value}
                    onPointerMove={(e) => {
                      e.preventDefault();
                    }}
                    onPointerEnter={(e) => {
                      e.preventDefault();
                    }}
                    onPointerLeave={(e) => {
                      e.preventDefault();
                    }}
                    onSelect={() => {
                      onUpdateFormat(fieldId, item.value);
                    }}
                  >
                    {item.label}
                    {item.value === format ? <DropdownMenuItemTick /> : ''}
                  </DropdownMenuItem>
                );
              })}
            </DropdownMenuSubContent>
          </DropdownMenuPortal>
        </DropdownMenuSub>
      </DropdownMenuGroup>
    </>
  );
}

export default NumberPropertyMenuContent;
