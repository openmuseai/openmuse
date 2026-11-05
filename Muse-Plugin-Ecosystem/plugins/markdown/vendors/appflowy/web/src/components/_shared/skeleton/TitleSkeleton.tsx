function TitleSkeleton() {
  return (
    <div className='mb-2 flex h-20 w-full items-center gap-2'>
      <div className='h-16 w-16 flex-shrink-0 animate-pulse rounded-full bg-fill-content-hover'></div>
      <div className='ml-4 flex-grow'>
        <div className='h-10 animate-pulse rounded bg-fill-content-hover'></div>
      </div>
    </div>
  );
}

export default TitleSkeleton;
