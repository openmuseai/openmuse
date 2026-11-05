import { cn } from '@/lib/utils';

interface DayCellContentArgs {
  date: Date;
  dayNumberText: string;
  isToday: boolean;
  isPopover?: boolean;
}

export const dayCellContent = (args: DayCellContentArgs) => {
  const isToday = args.isToday;
  const date = args.date;
  const dayNumber = date.getDate();
  const monthName = date.toLocaleDateString('en-US', { month: 'short' });
  const { isPopover = false } = args;

  if (!args.dayNumberText) return null;
  
  return (
    <div className='flex items-center gap-1'>
      {dayNumber === 1 || isPopover ? monthName : null}
      <span
        className={cn(
          isToday
            ? 'flex h-5 w-5 items-center justify-center rounded-200 bg-other-colors-filled-today text-text-inverse'
            : ''
        )}
      >
        {args.dayNumberText}
      </span>
    </div>
  );
};