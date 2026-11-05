import TitleSkeleton from '@/components/_shared/skeleton/TitleSkeleton';

function DocumentSkeleton() {
  return (
    <div className='mx-auto flex w-full max-w-full flex-col items-center'>
      <div className='h-[40vh] max-h-[288px] min-h-[130px] w-full animate-pulse bg-fill-content-hover max-sm:h-[180px]'></div>

      <div className='h-[60px]'></div>

      <div className='mb-2 flex h-20 w-full max-w-[952px] items-center px-24 max-sm:px-6'>
        <TitleSkeleton />
      </div>

      <div className='mt-2 w-full max-w-[952px] px-24 max-sm:px-6'>
        <div className='mb-4 h-10 w-full animate-pulse bg-fill-content-hover'></div>
        <div className='mb-4 h-6 w-1/2 animate-pulse bg-fill-content-hover'></div>
        <div className='mb-4 h-8 w-3/5 animate-pulse bg-fill-content-hover'></div>
        <div className='mb-4 h-7 w-4/5 animate-pulse bg-fill-content-hover'></div>
        <div className='mb-2 h-5 w-full animate-pulse bg-fill-content-hover'></div>
        <div className='h-5 w-full animate-pulse bg-fill-content-hover'></div>
      </div>
    </div>
  );
}

export default DocumentSkeleton;
