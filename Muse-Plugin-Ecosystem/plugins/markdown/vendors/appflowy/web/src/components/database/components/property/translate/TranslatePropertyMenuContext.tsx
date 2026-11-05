import { useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';

import { useFieldSelector } from '@/application/database-yjs';
import { useUpdateTranslateLanguage } from '@/application/database-yjs/dispatch';
import { languageTexts, parseAITranslateTypeOption } from '@/application/database-yjs/fields/ai-translate';
import {
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuItemTick,
  DropdownMenuLabel,
  DropdownMenuPortal,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
} from '@/components/ui/dropdown-menu';

function TranslatePropertyMenuContext({ fieldId }: { fieldId: string }) {
  const { t } = useTranslation();
  const [open, setOpen] = useState(false);
  const { field, clock } = useFieldSelector(fieldId);

  const updateLanguage = useUpdateTranslateLanguage(fieldId);
  const language = useMemo(() => {
    return parseAITranslateTypeOption(field).language;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [field, clock]);

  const selectItem = useMemo(() => {
    return languageTexts.find((option) => option.value === language) || languageTexts[0];
  }, [language]);

  return (
    <>
      <DropdownMenuSeparator />
      <DropdownMenuGroup>
        <DropdownMenuLabel>{t('grid.field.translateTo')}</DropdownMenuLabel>
        <DropdownMenuSub open={open} onOpenChange={setOpen}>
          <DropdownMenuSubTrigger
            {...(open && {
              onPointerMove: (e) => e.preventDefault(),
              onPointerLeave: (e) => e.preventDefault(),
            })}
          >
            {selectItem?.label}
          </DropdownMenuSubTrigger>
          <DropdownMenuPortal>
            <DropdownMenuSubContent className={'appflowy-scroller w-[240px] overflow-y-auto'}>
              {languageTexts.map((item) => {
                return (
                  <DropdownMenuItem
                    key={item.value}
                    onSelect={() => {
                      updateLanguage(item.value);
                    }}
                  >
                    {item.label}
                    {item.value === language ? <DropdownMenuItemTick /> : ''}
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

export default TranslatePropertyMenuContext;
