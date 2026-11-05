import { useCallback } from 'react';

import {
  FileMediaCell as CellType,
} from '@/application/database-yjs/cell.type';
import AddFileOrMedia from '@/components/database/components/cell/file-media/AddFileOrMedia';
import FileMediaList from '@/components/database/components/cell/file-media/FileMediaList';
import FileMediaUpload from '@/components/database/components/cell/file-media/FileMediaUpload';
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover';
import { Separator } from '@/components/ui/separator';

function FileMediaCellMenu ({ showUpload, open, onOpenChange, cell, fieldId, rowId, onPreview }: {
  open?: boolean;
  onOpenChange?: (open: boolean) => void;
  cell?: CellType;
  fieldId: string;
  rowId: string;
  onPreview: (index: number) => void;
  showUpload: boolean;
}) {

  const handleClose = useCallback(() => {
    if (onOpenChange) {
      onOpenChange(false);
    }
  }, [onOpenChange]);

  return (
    <Popover
      open={open}
      onOpenChange={onOpenChange}
    >
      <PopoverTrigger
        style={{
          zIndex: open ? 1 : -1,
        }}
        onPointerDown={() => {
          onOpenChange?.(false);
        }}
        className={'absolute left-0 top-0 w-full h-full z-[-1]'}
      />
      <PopoverContent
        side={'bottom'}
        align={'start'}
        className={'overflow-hidden appflowy-scroller overflow-y-auto max-w-[360px] max-h-[376px]'}
      >
        {showUpload ? <FileMediaUpload
          rowId={rowId}
          fieldId={fieldId}
          cell={cell}
          onClose={handleClose}
        /> : <><FileMediaList
          rowId={rowId}
          fieldId={fieldId}
          cell={cell}
          onPreview={onPreview}
        />
          <div className={'sticky bg-surface-primary z-[1] bottom-0 left-0 w-full'}>
            <Separator />
            <div className={'p-2 px-1'}>
              <AddFileOrMedia
                fieldId={fieldId}
                rowId={rowId}
                cell={cell}
              />
            </div>

          </div>

        </>}

      </PopoverContent>
    </Popover>
  );
}

export default FileMediaCellMenu;