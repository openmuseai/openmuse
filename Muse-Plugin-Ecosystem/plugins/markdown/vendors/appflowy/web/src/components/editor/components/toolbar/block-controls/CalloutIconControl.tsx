import { useCallback, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useSlateStatic } from 'slate-react';

import { YjsEditor } from '@/application/slate-yjs';
import { CustomEditor } from '@/application/slate-yjs/command';
import { ViewIconType } from '@/application/types';
import { ReactComponent as AddEmojiIcon } from '@/assets/icons/add_emoji.svg';
import { ReactComponent as ChevronRightIcon } from '@/assets/icons/alt_arrow_right.svg';
import { CustomIconPopover } from '@/components/_shared/cutsom-icon';
import { CalloutNode } from '@/components/editor/editor.type';
import { Button } from '@/components/ui/button';

function CalloutIconControl({ node, onSelectIcon }: { node: CalloutNode; onSelectIcon: () => void }) {
  const { t } = useTranslation();
  const editor = useSlateStatic() as YjsEditor;
  const [open, setOpen] = useState(false);

  const handleChangeIcon = useCallback(
    (icon: { ty: ViewIconType; value: string; color?: string; content?: string }) => {
      const iconType = icon.ty === ViewIconType.Icon ? 'icon' : 'emoji';
      let value: string | null = null;

      if (icon.ty === ViewIconType.Icon) {
        value = JSON.stringify({
          color: icon.color,
          groupName: icon.value.split('/')[0],
          iconName: icon.value.split('/')[1],
        });
      } else {
        value = icon.value;
      }

      CustomEditor.setBlockData(editor, node.blockId, { icon: value, icon_type: iconType });
      setOpen(false);
      onSelectIcon();
    },
    [editor, node.blockId, onSelectIcon]
  );

  const handleRemoveIcon = useCallback(() => {
    CustomEditor.setBlockData(editor, node.blockId, { icon: null });
    setOpen(false);
    onSelectIcon();
  }, [editor, node.blockId, onSelectIcon]);

  return (
    <CustomIconPopover
      open={open}
      onOpenChange={setOpen}
      onSelectIcon={handleChangeIcon}
      removeIcon={handleRemoveIcon}
      defaultActiveTab='emoji'
      tabs={['emoji', 'icon']}
      hideRemove={false}
      popoverContentProps={{ side: 'right', align: 'start' }}
    >
      <Button
        size='sm'
        variant='ghost'
        className='justify-start px-1'
        onClick={(event) => {
          event.preventDefault();
          event.stopPropagation();
          setOpen(true);
        }}
      >
        <AddEmojiIcon className='h-5 w-5' />
        {t('document.callout.changeIcon')}
        <ChevronRightIcon className='ml-auto h-5 w-5 text-icon-tertiary' />
      </Button>
    </CustomIconPopover>
  );
}

export default CalloutIconControl;
