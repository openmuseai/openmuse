import DOMPurify from 'dompurify';
import React, { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { Element } from 'slate';
import { useReadOnly, useSlateStatic } from 'slate-react';

import { YjsEditor } from '@/application/slate-yjs';
import { CustomEditor } from '@/application/slate-yjs/command';
import { ViewIconType } from '@/application/types';
import { CustomIconPopover } from '@/components/_shared/cutsom-icon';
import { CalloutNode } from '@/components/editor/editor.type';
import { cn } from '@/lib/utils';
import { ColorEnum, renderColor, toBlockColor } from '@/utils/color';
import { getIcon, isFlagEmoji } from '@/utils/emoji';

function CalloutIcon({ block: node }: { block: CalloutNode; className: string }) {
  const ref = useRef<HTMLButtonElement>(null);
  const editor = useSlateStatic();
  const readOnly = useReadOnly() || editor.isElementReadOnly(node as unknown as Element);
  const blockId = node.blockId;
  const [iconContent, setIconContent] = useState<string | undefined>(undefined);
  const [open, setOpen] = useState(false);

  const { icon: blockIconColor, bgHover: blockBgHoverColor } = useMemo(() => {
    return toBlockColor((node?.data?.bgColor || '') as ColorEnum);
  }, [node?.data?.bgColor]);

  const handleChangeIcon = useCallback(
    (icon: { ty: ViewIconType; value: string; color?: string; content?: string }) => {
      setOpen(false);
      const iconType = icon.ty === ViewIconType.Icon ? 'icon' : 'emoji';
      let value;

      if (icon.ty === ViewIconType.Icon) {
        value = JSON.stringify({
          color: icon.color,
          groupName: icon.value.split('/')[0],
          iconName: icon.value.split('/')[1],
        });
      } else {
        value = icon.value;
      }

      CustomEditor.setBlockData(editor as YjsEditor, blockId, { icon: value, icon_type: iconType });
    },
    [editor, blockId]
  );

  const handleRemoveIcon = useCallback(() => {
    setOpen(false);
    CustomEditor.setBlockData(editor as YjsEditor, blockId, { icon: null });
  }, [blockId, editor]);

  const data = node.data;

  const emoji = useMemo(() => {
    if (!data.icon || data.icon_type !== 'emoji') return;

    return data.icon;
  }, [data]);

  const isFlag = useMemo(() => {
    return emoji ? isFlagEmoji(emoji) : false;
  }, [emoji]);

  useEffect(() => {
    try {
      let id = '';
      let iconColor = '';

      if (data.icon && data.icon_type === 'icon') {
        const json = JSON.parse(data.icon);

        id = `${json.groupName}/${json.iconName}`;
        iconColor = json.color || blockIconColor;
      } else {
        id = 'mail/chat-bubble-typing-oval';
        iconColor = blockIconColor;
      }

      void getIcon(id).then((item) => {
        setIconContent(
          item?.content.replaceAll('black', renderColor(iconColor)).replace('<svg', '<svg width="100%" height="100%"')
        );
      });
    } catch (e) {
      console.error(e, data.icon);
    }
  }, [blockIconColor, data.icon, data.icon_type]);

  const icon = useMemo(() => {
    if (iconContent) {
      const cleanSvg = DOMPurify.sanitize(iconContent, {
        USE_PROFILES: { svg: true, svgFilters: true },
      });

      return (
        <span
          style={{
            width: 14,
            height: 14,
          }}
          dangerouslySetInnerHTML={{
            __html: cleanSvg,
          }}
        />
      );
    }

    return null;
  }, [iconContent]);

  return (
    <CustomIconPopover
      open={open}
      onOpenChange={setOpen}
      onSelectIcon={handleChangeIcon}
      removeIcon={handleRemoveIcon}
      defaultActiveTab={'emoji'}
      tabs={['emoji', 'icon']}
      enable={!readOnly}
    >
      <span
        data-testid='callout-icon-button'
        contentEditable={false}
        ref={ref}
        className={cn(
          'relative mr-2 flex h-7 w-7 flex-shrink-0 items-start justify-center',
          !readOnly && 'cursor-pointer'
        )}
      >
        <span
          className={cn(
            'absolute flex h-7 w-7 items-center justify-center text-[18px]',
            isFlag && 'icon',
            !readOnly && 'rounded-[6px] hover:bg-[var(--dynamic-background-color)]'
          )}
          style={{ '--dynamic-background-color': renderColor(blockBgHoverColor) } as React.CSSProperties}
        >
          {emoji || icon}
        </span>
      </span>
    </CustomIconPopover>
  );
}

export default React.memo(CalloutIcon);
