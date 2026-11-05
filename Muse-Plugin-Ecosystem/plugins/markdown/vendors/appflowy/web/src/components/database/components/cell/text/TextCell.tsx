import { useCallback, useMemo, useRef } from 'react';

import { FieldType } from '@/application/database-yjs';
import { Cell, CellProps } from '@/application/database-yjs/cell.type';
import TextCellEditing from '@/components/database/components/cell/text/TextCellEditing';
import UrlActions from '@/components/database/components/cell/text/UrlActions';
import { cn } from '@/lib/utils';
import { openUrl, processUrl } from '@/utils/url';

export function TextCell({
  cell,
  style,
  placeholder,
  readOnly,
  fieldId,
  rowId,
  editing,
  setEditing,
  wrap,
  isHovering,
}: CellProps<Cell>) {
  const ref = useRef<HTMLDivElement>(null);
  const cellType = cell?.fieldType || FieldType.RichText;

  const middleware = useCallback((data: unknown) => {
    if (typeof data !== 'string' && typeof data !== 'number') {
      return '';
    }

    return (data as string) || '';
  }, []);

  const value = middleware(cell?.data);

  const isValidUrl = useCallback((url: string) => {
    return !!processUrl(url);
  }, []);

  const showUrlActions = useMemo(() => {
    return cellType === FieldType.URL && value && isValidUrl(value) && !editing && isHovering;
  }, [value, isValidUrl, editing, isHovering, cellType]);

  const focusToEnd = useCallback((el: HTMLTextAreaElement) => {
    if (el) {
      const length = el.value.length;

      el.setSelectionRange(length, length);
      el.focus();
    }
  }, []);

  return (
    <>
      <div
        ref={ref}
        style={style}
        onClick={(e) => {
          if (readOnly) {
            if (value && isValidUrl(value)) {
              e.stopPropagation();
              void openUrl(value, '_blank');
            }

            return;
          }
        }}
        className={cn(
          `text-cell w-full text-sm ${readOnly ? 'select-auto' : 'cursor-pointer'}`,
          !value && placeholder ? 'text-text-tertiary' : '',
          cellType === FieldType.URL ? '!text-text-action underline hover:text-text-action-hover' : '',
          wrap ? ' whitespace-pre-wrap break-words' : 'whitespace-nowrap'
        )}
      >
        {!editing ? (
          <>{value || placeholder || ''}</>
        ) : (
          <TextCellEditing
            ref={focusToEnd}
            defaultValue={value}
            placeholder={placeholder}
            fieldId={fieldId}
            rowId={rowId}
            onExit={() => {
              setEditing?.(false);
            }}
          />
        )}
        {showUrlActions && (
          <div className={'absolute right-1 top-1'}>
            <UrlActions url={value} />
          </div>
        )}
      </div>
    </>
  );
}
