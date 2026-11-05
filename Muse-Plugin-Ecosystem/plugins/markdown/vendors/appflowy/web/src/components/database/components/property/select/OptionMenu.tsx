import { useEffect, useMemo, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';

import { SelectOption, SelectOptionColor, useDatabaseContext } from '@/application/database-yjs';
import { useDeleteSelectOption, useUpdateSelectOption } from '@/application/database-yjs/dispatch';
import { ReactComponent as DeleteIcon } from '@/assets/icons/delete.svg';
import { ColorTile } from '@/components/_shared/color-picker';
import { useSubscriptionPlan } from '@/components/app/hooks/useSubscriptionPlan';
import { SelectOptionColorMap } from '@/components/database/components/cell/cell.const';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Input } from '@/components/ui/input';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';

function OptionMenu({
  open,
  onOpenChange,
  option,
  fieldId,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  option: SelectOption;
  fieldId: string;
}) {
  const { getSubscriptions } = useDatabaseContext();
  const { t } = useTranslation();

  const onDelete = useDeleteSelectOption(fieldId);
  const onUpdate = useUpdateSelectOption(fieldId);

  const { isPro } = useSubscriptionPlan(getSubscriptions);

  const colors = useMemo(() => {
    const baseColors = [
      {
        label: t('colors.mauve'),
        value: SelectOptionColor.OptionColor1,
        color: SelectOptionColorMap[SelectOptionColor.OptionColor1],
      },
      {
        label: t('colors.lilac'),
        value: SelectOptionColor.OptionColor2,
        color: SelectOptionColorMap[SelectOptionColor.OptionColor2],
      },
      {
        label: t('colors.camellia'),
        value: SelectOptionColor.OptionColor3,
        color: SelectOptionColorMap[SelectOptionColor.OptionColor3],
      },
      {
        label: t('colors.papaya'),
        value: SelectOptionColor.OptionColor4,
        color: SelectOptionColorMap[SelectOptionColor.OptionColor4],
      },
      {
        label: t('colors.mango'),
        value: SelectOptionColor.OptionColor5,
        color: SelectOptionColorMap[SelectOptionColor.OptionColor5],
      },
      {
        label: t('colors.olive'),
        value: SelectOptionColor.OptionColor6,
        color: SelectOptionColorMap[SelectOptionColor.OptionColor6],
      },
      {
        label: t('colors.grass'),
        value: SelectOptionColor.OptionColor7,
        color: SelectOptionColorMap[SelectOptionColor.OptionColor7],
      },
      {
        label: t('colors.jade'),
        value: SelectOptionColor.OptionColor8,
        color: SelectOptionColorMap[SelectOptionColor.OptionColor8],
      },
      {
        label: t('colors.azure'),
        value: SelectOptionColor.OptionColor9,
        color: SelectOptionColorMap[SelectOptionColor.OptionColor9],
      },
      {
        label: t('colors.iron'),
        value: SelectOptionColor.OptionColor10,
        color: SelectOptionColorMap[SelectOptionColor.OptionColor10],
      },
    ];

    if (isPro) {
      baseColors.push(
        {
          label: t('colors.mauveEmphasized'),
          value: SelectOptionColor.OptionColor11,
          color: SelectOptionColorMap[SelectOptionColor.OptionColor11],
        },
        {
          label: t('colors.lavenderEmphasized'),
          value: SelectOptionColor.OptionColor12,
          color: SelectOptionColorMap[SelectOptionColor.OptionColor12],
        },
        {
          label: t('colors.camelliaEmphasized'),
          value: SelectOptionColor.OptionColor13,
          color: SelectOptionColorMap[SelectOptionColor.OptionColor13],
        },
        {
          label: t('colors.papayaEmphasized'),
          value: SelectOptionColor.OptionColor14,
          color: SelectOptionColorMap[SelectOptionColor.OptionColor14],
        },
        {
          label: t('colors.mangoEmphasized'),
          value: SelectOptionColor.OptionColor15,
          color: SelectOptionColorMap[SelectOptionColor.OptionColor15],
        },
        {
          label: t('colors.oliveEmphasized'),
          value: SelectOptionColor.OptionColor16,
          color: SelectOptionColorMap[SelectOptionColor.OptionColor16],
        },
        {
          label: t('colors.grassEmphasized'),
          value: SelectOptionColor.OptionColor17,
          color: SelectOptionColorMap[SelectOptionColor.OptionColor17],
        },
        {
          label: t('colors.jadeEmphasized'),
          value: SelectOptionColor.OptionColor18,
          color: SelectOptionColorMap[SelectOptionColor.OptionColor18],
        },
        {
          label: t('colors.azureEmphasized'),
          value: SelectOptionColor.OptionColor19,
          color: SelectOptionColorMap[SelectOptionColor.OptionColor19],
        },
        {
          label: t('colors.ironEmphasized'),
          value: SelectOptionColor.OptionColor20,
          color: SelectOptionColorMap[SelectOptionColor.OptionColor20],
        }
      );
    }

    return baseColors;
  }, [isPro, t]);

  const [value, setValue] = useState<string>(option.name);
  const [editing, setEditing] = useState(false);

  const updateName = () => {
    if (value === option.name) {
      return;
    }

    onUpdate(option.id, {
      ...option,
      name: value,
    });
  };

  const inputRef = useRef<HTMLInputElement | null>(null);

  useEffect(() => {
    if (open) {
      setTimeout(() => {
        const el = inputRef.current;

        if (el) {
          el.focus();
          el.setSelectionRange(0, el.value.length);
        }
      }, 100);
    }
  }, [open]);

  return (
    <DropdownMenu open={open} onOpenChange={onOpenChange}>
      <DropdownMenuTrigger asChild>
        <div className={'absolute bottom-0 right-0 z-[-1] h-5 w-5'} />
      </DropdownMenuTrigger>
      <DropdownMenuContent
        onClick={(e) => {
          e.stopPropagation();
        }}
        side={'right'}
        align={'start'}
        className='!min-w-[200px] max-w-[200px] p-0'
      >
        <DropdownMenuGroup className='p-3'>
          <Input
            onMouseDown={(e) => {
              e.stopPropagation();
            }}
            className='w-full'
            ref={inputRef}
            value={value}
            onKeyDown={(e) => {
              e.stopPropagation();
              if (e.key === 'Enter') {
                updateName();
                setEditing(false);
                onOpenChange(false);
              }
            }}
            onBlur={() => {
              updateName();
              setEditing(false);
            }}
            onFocus={() => {
              setEditing(true);
            }}
            onChange={(e) => setValue(e.target.value)}
          />
        </DropdownMenuGroup>

        <DropdownMenuGroup>
          <DropdownMenuLabel className='mx-1.5'>{t('grid.selectOption.colorPanelTitle')}</DropdownMenuLabel>
          <div className='mx-3.5 grid grid-cols-5 gap-2 pb-1.5'>
            {colors.map((color) => (
              <Tooltip key={color.value}>
                <TooltipContent>{color.label}</TooltipContent>
                <TooltipTrigger asChild>
                  <ColorTile
                    value={`var(${SelectOptionColorMap[color.value]})`}
                    active={color.value === option.color}
                    onClick={(e) => {
                      if (color.value === option.color) {
                        e.preventDefault();
                        return;
                      }

                      onUpdate(option.id, {
                        ...option,
                        color: color.value,
                      });
                    }}
                  />
                </TooltipTrigger>
              </Tooltip>
            ))}
          </div>
        </DropdownMenuGroup>

        <DropdownMenuSeparator className='my-2' />

        <DropdownMenuItem
          variant={'destructive'}
          onSelect={() => {
            onDelete(option.id);
          }}
          className='mx-1.5 mb-1.5'
          {...(editing
            ? {
              onPointerMove: (e) => e.preventDefault(),
              onPointerEnter: (e) => e.preventDefault(),
              onPointerLeave: (e) => e.preventDefault(),
            }
            : undefined)}
        >
          <DeleteIcon />
          {t('grid.selectOption.deleteTag')}
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

export default OptionMenu;
