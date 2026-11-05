import { useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';

import { useDuplicateRowDispatch } from '@/application/database-yjs/dispatch';
import { ReactComponent as DeleteIcon } from '@/assets/icons/delete.svg';
import { ReactComponent as DuplicateIcon } from '@/assets/icons/duplicate.svg';
import { ReactComponent as EditIcon } from '@/assets/icons/edit.svg';
import { ReactComponent as MoreIcon } from '@/assets/icons/more.svg';
import { DeleteRowConfirm } from '@/components/database/components/database-row/DeleteRowConfirm';
import { Button } from '@/components/ui/button';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import { cn } from '@/lib/utils';

function CardToolbar({ onEdit, visible, rowId }: { rowId: string; visible: boolean; onEdit: () => void }) {
  const { t } = useTranslation();
  const onDuplicate = useDuplicateRowDispatch();
  const [deleteConfirm, setDeleteConfirm] = useState(false);

  const rowIds = useMemo(() => {
    return [rowId];
  }, [rowId]);

  return (
    <div
      style={{
        opacity: visible ? 1 : 0,
        pointerEvents: visible ? 'auto' : 'none',
      }}
      onClick={(e) => {
        e.stopPropagation();
      }}
      className={cn(
        'absolute right-1 top-1.5 flex items-center rounded-100 border border-border-primary bg-surface-primary shadow-toolbar'
      )}
    >
      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            variant={'ghost'}
            onClick={() => {
              onEdit();
            }}
            className={'h-6 w-6 rounded-none  border-r border-border-primary p-0 text-icon-secondary'}
          >
            <EditIcon className={'h-4 w-4'} />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{t('button.edit')}</TooltipContent>
      </Tooltip>

      <DropdownMenu>
        <Tooltip>
          <TooltipTrigger asChild>
            <DropdownMenuTrigger asChild>
              <Button variant={'ghost'} className={'h-6 w-6 rounded-none  p-0 text-icon-secondary'}>
                <MoreIcon className={'h-4 w-4'} />
              </Button>
            </DropdownMenuTrigger>
          </TooltipTrigger>
          <TooltipContent>{`${t('button.duplicate')}, ${t('button.delete')}`}</TooltipContent>
        </Tooltip>

        <DropdownMenuContent
          onCloseAutoFocus={(e) => {
            e.preventDefault();
          }}
          side={'right'}
        >
          <DropdownMenuGroup>
            <DropdownMenuItem
              onSelect={() => {
                void onDuplicate(rowId);
              }}
            >
              <DuplicateIcon />
              {t('button.duplicate')}
            </DropdownMenuItem>
          </DropdownMenuGroup>
          <DropdownMenuGroup>
            <DropdownMenuItem
              onSelect={() => {
                setDeleteConfirm(true);
              }}
              variant={'destructive'}
            >
              <DeleteIcon />
              {t('button.delete')}
            </DropdownMenuItem>
          </DropdownMenuGroup>
        </DropdownMenuContent>
      </DropdownMenu>
      {deleteConfirm && (
        <DeleteRowConfirm
          open={deleteConfirm}
          onClose={() => {
            setDeleteConfirm(false);
          }}
          rowIds={rowIds}
        />
      )}
    </div>
  );
}

export default CardToolbar;
