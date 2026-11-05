import { forwardRef, memo, useMemo } from 'react';

import { CalloutNode, EditorElementProps } from '@/components/editor/editor.type';
import { cn } from '@/lib/utils';
import { ColorEnum, renderColor, toBlockColor } from '@/utils/color';

export const Callout = memo(
  forwardRef<HTMLDivElement, EditorElementProps<CalloutNode>>(({ node, children, ...attributes }, ref) => {
    const { className: attrClassName = '', style: attrStyle, ...restAttributes } = attributes;

    const {
      bg: blockBgColor,
      border: blockBorderColor,
      text: blockTextColor,
    } = useMemo(() => {
      return toBlockColor((node?.data?.bgColor || '') as ColorEnum);
    }, [node?.data?.bgColor]);

    const effectiveBlockTextColor = useMemo(() => {
      const { text } = toBlockColor((node?.data?.textColor || '') as ColorEnum);

      return text || blockTextColor;
    }, [blockTextColor, node?.data?.textColor]);

    return (
      <div
        ref={ref}
        {...restAttributes}
        className={cn(
          attrClassName,
          'relative my-1 flex w-full flex-col overflow-hidden rounded-300 bg-fill-list-active px-3 py-2'
        )}
        style={{
          ...attrStyle,
          backgroundColor: renderColor(blockBgColor),
          color: renderColor(effectiveBlockTextColor),
        }}
      >
        <div
          className='absolute bottom-0 left-0 top-0 w-1 rounded-full'
          style={{
            backgroundColor: renderColor(blockBorderColor),
          }}
        />
        {children}
      </div>
    );
  })
);

export default Callout;
