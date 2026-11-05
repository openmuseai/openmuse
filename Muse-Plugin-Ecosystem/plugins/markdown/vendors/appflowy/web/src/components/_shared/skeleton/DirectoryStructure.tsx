export const DirectoryStructure = () => {
  return (
    <div className='w-full'>
      <DirectoryItem />
      <div className='pl-6'>
        <DirectoryItem />
        <div className='pl-6'>
          <DirectoryItem />
          <DirectoryItem />
        </div>
        <DirectoryItem />
      </div>
      <DirectoryItem />
      <div className='pl-6'>
        <DirectoryItem />
        <DirectoryItem />
      </div>
    </div>
  );
};

const DirectoryItem = () => (
  <div className='mb-2 flex items-center space-x-2'>
    <div className='h-5 w-5 animate-pulse rounded-full bg-fill-content-hover'></div>
    <div className='h-5 flex-1 animate-pulse rounded bg-fill-content-hover'></div>
  </div>
);

export default DirectoryStructure;
