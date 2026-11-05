import { forwardRef, MouseEventHandler, useMemo } from 'react';

import { cn } from '@/lib/utils';

export const ColorTile = forwardRef<
  HTMLDivElement,
  {
    value: string;
    isText?: boolean;
    onClick: MouseEventHandler<HTMLDivElement> | undefined;
    active?: boolean;
  }
>(({ value, onClick, isText = false, active = false, ...props }, forwardedRef) => {
  const child = useMemo(() => {
    if (isText) {
      return (
        <span className='' style={{ color: value }}>
          A
        </span>
      );
    }

    return (
      <div
        className={cn(active ? 'h-5 w-5 rounded-[3px]' : 'min-h-6 min-w-6 rounded-[4px]')}
        style={{ background: value }}
      />
    );
  }, [active, isText, value]);

  return (
    <div
      ref={forwardedRef}
      {...props}
      onClick={onClick}
      className={cn(
        'flex h-7 w-7 cursor-pointer items-center justify-center rounded-[6px]',
        active
          ? 'border-[2px] border-fill-theme-thick'
          : 'border border-border-primary hover:border-border-primary-hover'
      )}
    >
      {child}
    </div>
  );
});

export function ColorTileIcon({ value }: { value: string }) {
  return (
    <div className='m-0.5 flex h-4 w-4 items-center justify-center rounded-[4px] border border-border-primary'>
      <div className='min-h-3 min-w-3 rounded-[2px]' style={{ background: value }} />
    </div>
  );
}
