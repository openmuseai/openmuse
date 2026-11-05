import { useMemo } from 'react';

import { Row, useDatabaseContext, usePrimaryFieldId } from '@/application/database-yjs';
import { CardField } from '@/components/database/components/field/CardField';
import { dropdownMenuItemVariants } from '@/components/ui/dropdown-menu';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { cn } from '@/lib/utils';

export function HiddenItemMenu({ children, getRows }: { children: React.ReactNode; getRows: () => Row[] }) {
  const primaryFieldId = usePrimaryFieldId();
  const navigateToRow = useDatabaseContext().navigateToRow;
  const rows = useMemo(() => {
    if (!primaryFieldId) return null;
    return getRows().map((row) => {
      return (
        <div
          onClick={(e) => {
            e.stopPropagation();
            e.preventDefault();
            navigateToRow?.(row.id);
          }}
          key={row.id}
          className={cn(dropdownMenuItemVariants({ variant: 'default' }))}
        >
          <CardField rowId={row.id} fieldId={primaryFieldId} />
        </div>
      );
    });
  }, [getRows, navigateToRow, primaryFieldId]);

  return (
    <Popover>
      <PopoverTrigger asChild>{children}</PopoverTrigger>
      <PopoverContent>
        <div
          className={
            'appflowy-hidden-scroller flex max-h-[360px] max-w-[320px] flex-col overflow-y-auto overflow-x-hidden p-2'
          }
        >
          {rows}
        </div>
      </PopoverContent>
    </Popover>
  );
}
