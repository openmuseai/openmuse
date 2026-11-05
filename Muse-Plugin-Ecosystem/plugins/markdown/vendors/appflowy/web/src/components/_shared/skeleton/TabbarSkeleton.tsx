function TabbarSkeleton() {
  const tabCount = 4;

  return (
    <div className='mx-auto w-full'>
      <div className='border-b border-border-primary'>
        <nav className='-mb-px flex'>
          {[...Array(tabCount)].map((_, index) => (
            <div key={index} className='mr-2'>
              <div className='min-w-[100px] border-b-2 border-transparent px-4 py-2'>
                <div className='h-5 w-20 animate-pulse rounded bg-fill-content-hover'></div>
              </div>
            </div>
          ))}
        </nav>
      </div>
    </div>
  );
}

export default TabbarSkeleton;
