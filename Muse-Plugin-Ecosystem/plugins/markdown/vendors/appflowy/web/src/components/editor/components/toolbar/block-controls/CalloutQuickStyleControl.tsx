import DOMPurify from 'dompurify';
import { useCallback, useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useSlateStatic } from 'slate-react';

import { YjsEditor } from '@/application/slate-yjs';
import { CustomEditor } from '@/application/slate-yjs/command';
import { ReactComponent as ChevronRightIcon } from '@/assets/icons/alt_arrow_right.svg';
import { ReactComponent as PaletteIcon } from '@/assets/icons/palette.svg';
import { Origins, Popover } from '@/components/_shared/popover';
import { CalloutNode } from '@/components/editor/editor.type';
import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { ColorEnum, renderColor, toBlockColor } from '@/utils/color';
import { getIcon } from '@/utils/emoji';

const popoverOrigins: Origins = {
  anchorOrigin: {
    vertical: 'top',
    horizontal: 'right',
  },
  transformOrigin: {
    vertical: 'top',
    horizontal: 'left',
  },
};

const nestedOrigins: Origins = {
  anchorOrigin: {
    vertical: 'center',
    horizontal: 'right',
  },
  transformOrigin: {
    vertical: 'center',
    horizontal: 'left',
  },
};

type QuickStyleOption = {
  key: string;
  labelKey: string;
  iconGroup: string;
  iconName: string;
  color: ColorEnum;
};

const QUICK_STYLE_OPTIONS = [
  {
    key: 'basic',
    labelKey: 'document.callout.basic',
    iconGroup: 'mail',
    iconName: 'chat-bubble-typing-oval',
    color: ColorEnum.Tint10,
  },
  {
    key: 'info',
    labelKey: 'document.callout.info',
    iconGroup: 'interface_essential',
    iconName: 'information-circle',
    color: ColorEnum.Tint9,
  },
  {
    key: 'tip',
    labelKey: 'document.callout.tip',
    iconGroup: 'interface_essential',
    iconName: 'help-question-1',
    color: ColorEnum.Tint8,
  },
  {
    key: 'warning',
    labelKey: 'document.callout.warning',
    iconGroup: 'interface_essential',
    iconName: 'warning-triangle',
    color: ColorEnum.Tint5,
  },
  {
    key: 'alert',
    labelKey: 'document.callout.alert',
    iconGroup: 'interface_essential',
    iconName: 'warning-octagon',
    color: ColorEnum.Tint13,
  },
] as const;

const MORE_STYLE_OPTIONS = [
  {
    key: 'insight',
    labelKey: 'document.callout.insight',
    iconGroup: 'artificial_intelligence',
    iconName: 'ai-upscale-spark',
    color: ColorEnum.Tint1,
  },
  {
    key: 'idea',
    labelKey: 'document.callout.idea',
    iconGroup: 'artificial_intelligence',
    iconName: 'ai-technology-spark',
    color: ColorEnum.Tint2,
  },
  {
    key: 'spark',
    labelKey: 'document.callout.spark',
    iconGroup: 'interface_essential',
    iconName: 'star-1',
    color: ColorEnum.Tint4,
  },
  {
    key: 'context',
    labelKey: 'document.callout.context',
    iconGroup: 'interface_essential',
    iconName: 'text-flow-rows',
    color: ColorEnum.Tint6,
  },
  {
    key: 'note',
    labelKey: 'document.callout.note',
    iconGroup: 'interface_essential',
    iconName: 'new-sticky-note',
    color: ColorEnum.Tint7,
  },
] as const;

function QuickStyleIcon({ option, colorVar }: { option: QuickStyleOption; colorVar: string }) {
  const [content, setContent] = useState<string>();

  useEffect(() => {
    let active = true;

    void getIcon(`${option.iconGroup}/${option.iconName}`).then((icon) => {
      if (!active) {
        return;
      }

      if (icon?.content) {
        const sanitized = DOMPurify.sanitize(
          icon.content.replaceAll('black', colorVar).replace('<svg', '<svg width="100%" height="100%"'),
          {
            USE_PROFILES: { svg: true, svgFilters: true },
          }
        );

        setContent(sanitized);
      } else {
        setContent(undefined);
      }
    });

    return () => {
      active = false;
    };
  }, [option.iconGroup, option.iconName, colorVar]);

  if (!content) {
    return <span className='h-3.5 w-3.5' />;
  }

  return <span className='h-3.5 w-3.5' dangerouslySetInnerHTML={{ __html: content }} />;
}

function CalloutQuickStyleControl({ node, onSelectStyle }: { node: CalloutNode; onSelectStyle: () => void }) {
  const { t } = useTranslation();
  const editor = useSlateStatic() as YjsEditor;
  const [open, setOpen] = useState(false);
  const [moreOpen, setMoreOpen] = useState(false);
  const anchorRef = useRef<HTMLButtonElement>(null);
  const moreAnchorRef = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    if (!open) {
      setMoreOpen(false);
    }
  }, [open]);

  const applyStyle = useCallback(
    (option: QuickStyleOption) => {
      const iconValue = JSON.stringify({
        color: '',
        groupName: option.iconGroup,
        iconName: option.iconName,
      });

      CustomEditor.setBlockData(editor, node.blockId, {
        bgColor: option.color,
        textColor: option.color,
        icon: iconValue,
        icon_type: 'icon',
      });

      setOpen(false);
      setMoreOpen(false);
      onSelectStyle();
    },
    [editor, node.blockId, onSelectStyle]
  );

  return (
    <>
      <Button
        ref={anchorRef}
        variant='ghost'
        size='sm'
        color='inherit'
        className='justify-start px-1'
        onClick={() => setOpen(true)}
      >
        <PaletteIcon className='h-5 w-5' />
        {t('document.callout.quickStyle')}
        <ChevronRightIcon className='ml-auto h-5 w-5 text-icon-tertiary' />
      </Button>
      <Popover open={open} anchorEl={anchorRef.current} onClose={() => setOpen(false)} {...popoverOrigins}>
        <div className='flex w-[240px] flex-col'>
          <div className='flex flex-col gap-1 p-2'>
            {QUICK_STYLE_OPTIONS.map((option) => {
              const { bg, border, text } = toBlockColor(option.color);

              return (
                <Button
                  key={option.key}
                  size='sm'
                  onClick={() => applyStyle(option)}
                  className='flex w-full items-center justify-start border border-transparent px-2 py-1.5 text-left hover:border-[var(--dynamic-border-color)]'
                  style={
                    {
                      background: renderColor(bg),
                      color: renderColor(text),
                      '--dynamic-border-color': renderColor(border),
                    } as React.CSSProperties
                  }
                >
                  <div className='flex h-5 w-5 items-center justify-center'>
                    <QuickStyleIcon option={option} colorVar={renderColor(text)} />
                  </div>
                  <span className='text-sm font-medium'>{t(option.labelKey)}</span>
                </Button>
              );
            })}
          </div>
          <Separator />
          <Button
            ref={moreAnchorRef}
            size='sm'
            variant='ghost'
            className='m-2 justify-start px-2 py-1.5'
            onClick={() => setMoreOpen((prev) => !prev)}
          >
            <PaletteIcon className='h-5 w-5' />
            {t('document.callout.moreStyles')}
            <ChevronRightIcon className='ml-auto h-5 w-5 text-icon-tertiary' />
          </Button>
        </div>
      </Popover>
      <Popover open={moreOpen} anchorEl={moreAnchorRef.current} onClose={() => setMoreOpen(false)} {...nestedOrigins}>
        <div className='flex w-[240px] flex-col gap-1 p-2'>
          {MORE_STYLE_OPTIONS.map((option) => {
            const { bg, border, text } = toBlockColor(option.color);

            return (
              <Button
                key={option.key}
                onClick={() => applyStyle(option)}
                size='sm'
                className='flex w-full items-center justify-start border border-transparent px-2 py-1.5 text-left hover:border-[var(--dynamic-border-color)]'
                style={
                  {
                    background: renderColor(bg),
                    color: renderColor(text),
                    '--dynamic-border-color': renderColor(border),
                  } as React.CSSProperties
                }
              >
                <div className='flex h-3.5 w-3.5 items-center justify-center'>
                  <QuickStyleIcon option={option} colorVar={renderColor(text)} />
                </div>
                <span className='text-sm font-medium'>{t(option.labelKey)}</span>
              </Button>
            );
          })}
        </div>
      </Popover>
    </>
  );
}

export default CalloutQuickStyleControl;
