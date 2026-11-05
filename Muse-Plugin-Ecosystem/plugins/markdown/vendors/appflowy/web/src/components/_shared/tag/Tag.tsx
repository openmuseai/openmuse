import { FC, useMemo } from 'react';

import { ReactComponent as CircleIcon } from '@/assets/icons/circle.svg';
import { cn } from '@/lib/utils';

export interface TagProps {
  bgColor?: string;
  textColor?: string;
  label?: string;
  badge?: string;
}

export const Tag: FC<TagProps> = ({ bgColor, textColor, label, badge }) => {
  const className = useMemo(() => {
    return cn(
      'text-[0.75rem] leading-[1.5] truncate',
      'min-w-[22px] w-fit flex items-center gap-0.5 py-[1px] px-2 max-w-full justify-center, rounded-[6px]',
      badge && 'px-2 gap-1'
    );
  }, [badge]);

  return (
    <div
      style={{
        backgroundColor: bgColor ? `var(${bgColor})` : undefined,
        color: `var(${textColor || '--text-primary'})`,
      }}
      className={className}
    >
      {badge && (
        <CircleIcon
          style={{
            color: `var(${badge})`,
          }}
          className={`!h-1.5 !w-1.5 min-w-1.5`}
        />
      )}
      <div className={'truncate'}>{label}</div>
    </div>
  );
};
