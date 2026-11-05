import { useCallback, useMemo } from 'react';

import { FieldType, parseSelectOptionTypeOptions, useFieldSelector } from '@/application/database-yjs';
import { CellProps, SelectOptionCell as SelectOptionCellType } from '@/application/database-yjs/cell.type';
import { Tag } from '@/components/_shared/tag';
import { SelectOptionColorMap, SelectOptionFgColorMap } from '@/components/database/components/cell/cell.const';
import SelectOptionCellMenu from '@/components/database/components/cell/select-option/SelectOptionCellMenu';
import { cn } from '@/lib/utils';

export function SelectOptionCell({
  editing,
  setEditing,
  cell,
  fieldId,
  style,
  placeholder,
  rowId,
  wrap,
}: CellProps<SelectOptionCellType>) {
  const selectOptionIds = useMemo(() => {
    if (!cell || !cell.data || typeof cell.data !== 'string') return [];
    const options = cell.data.split(',');

    if (cell.fieldType === FieldType.MultiSelect) {
      return options;
    }

    return options.slice(0, 1);
  }, [cell]);
  const { field, clock } = useFieldSelector(fieldId);
  const typeOption = useMemo(() => {
    if (!field) return null;
    return parseSelectOptionTypeOptions(field);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [field, clock]);

  const renderSelectedOptions = useCallback(
    (selected: string[]) =>
      selected.map((id) => {
        const option = typeOption?.options?.find((option) => option?.id === id);

        if (!option) return null;
        return (
          <div key={option.id} className={'min-w-fit max-w-[120px]'}>
            <Tag
              bgColor={SelectOptionColorMap[option.color]}
              textColor={SelectOptionFgColorMap[option.color]}
              label={option.name}
            />
          </div>
        );
      }),
    [typeOption]
  );

  const isEmpty = !typeOption || !selectOptionIds?.length;

  const handleOpenChange = useCallback(
    (status: boolean) => {
      setEditing?.(status);
    },
    [setEditing]
  );

  return (
    <div
      style={style}
      data-testid={`select-option-cell-${rowId}-${fieldId}`}
      className={cn(
        'select-option-cell flex w-full items-center gap-1',
        isEmpty && placeholder ? 'text-text-tertiary' : '',
        wrap
          ? 'flex-wrap overflow-x-hidden'
          : 'appflowy-hidden-scroller h-full w-full flex-nowrap overflow-x-auto overflow-y-hidden'
      )}
    >
      {isEmpty ? placeholder || null : renderSelectedOptions(selectOptionIds)}
      {editing ? (
        <SelectOptionCellMenu
          cell={cell}
          fieldId={fieldId}
          rowId={rowId}
          open={editing}
          onOpenChange={handleOpenChange}
          selectOptionIds={selectOptionIds}
        />
      ) : null}
    </div>
  );
}
