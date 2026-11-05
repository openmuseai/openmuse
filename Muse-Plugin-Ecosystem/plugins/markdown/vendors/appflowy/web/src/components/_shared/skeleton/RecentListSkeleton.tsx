const RecentListSkeleton = ({ rows = 5 }) => {
  return (
    <div className='w-full max-w-[360px]'>
      {[...Array(rows)].map((_, index) => (
        <div key={index} className='mx-2 my-1 flex items-center gap-2'>
          <div className='h-5 w-5 animate-pulse rounded-full bg-fill-content-hover'></div>
          <div className='h-5 flex-1 animate-pulse rounded bg-fill-content-hover'></div>
        </div>
      ))}
    </div>
  );
};

export default RecentListSkeleton;
