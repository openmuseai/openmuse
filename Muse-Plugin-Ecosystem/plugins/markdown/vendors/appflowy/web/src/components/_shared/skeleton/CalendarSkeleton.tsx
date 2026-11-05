import TabbarSkeleton from '@/components/_shared/skeleton/TabbarSkeleton';
import TitleSkeleton from '@/components/_shared/skeleton/TitleSkeleton';

function CalendarSkeleton({
  includeTitle = true,
  includeTabs = true,
}: {
  includeTitle?: boolean;
  includeTabs?: boolean;
}) {
  const daysInWeek = 7;
  const weeksInMonth = 4;

  return (
    <div className={`w-full min-w-0  max-w-full overflow-x-auto px-24 max-sm:px-6 ${includeTitle ? 'py-2' : ''}`}>
      {includeTitle && (
        <>
          <div className='my-6 mb-2 flex h-20 w-full items-center'>
            <TitleSkeleton />
          </div>
        </>
      )}

      {includeTabs && (
        <div className='mb-2 flex h-10 w-full items-center'>
          <TabbarSkeleton />
        </div>
      )}
      {/* Calendar Header */}
      <div className='mb-2 flex items-center justify-between'>
        <div className='h-10 w-24 animate-pulse rounded bg-fill-content-hover'></div>
        <div className='flex gap-1'>
          <div className='h-10 w-10 animate-pulse rounded bg-fill-content-hover'></div>
          <div className='h-10 w-10 animate-pulse rounded bg-fill-content-hover'></div>
        </div>
      </div>

      {/* Weekday Names */}
      <div className='mb-1 grid grid-cols-7 gap-1'>
        {[...Array(daysInWeek)].map((_, index) => (
          <div key={index} className='h-8 animate-pulse rounded bg-fill-content-hover'></div>
        ))}
      </div>

      {/* Calendar Grid */}
      <div className='rounded border border-border-primary shadow'>
        <div className='grid grid-cols-7'>
          {[...Array(weeksInMonth * daysInWeek)].map((_, index) => (
            <div key={index} className='aspect-square border-b border-r border-border-primary p-1'>
              <div className='flex h-full flex-col'>
                <div className='h-5 w-1/3 animate-pulse self-end rounded bg-fill-content-hover'></div>
                <div className='mt-1 flex flex-1 flex-col justify-center'>
                  <div className='mb-0.5 h-4 w-4/5 animate-pulse rounded bg-fill-content-hover'></div>
                  <div className='h-4 w-3/5 animate-pulse rounded bg-fill-content-hover'></div>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

export default CalendarSkeleton;
