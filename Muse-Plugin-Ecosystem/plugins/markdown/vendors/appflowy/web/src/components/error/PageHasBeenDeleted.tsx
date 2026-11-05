import { ReactComponent as TrashIcon } from '@/assets/icons/delete.svg';
import emptyImageSrc from '@/assets/images/empty.png';

const DeletedPageComponent = () => {
  return (
    <div className={'flex h-full w-full flex-col items-center justify-center'}>
      <div className={'flex items-center gap-4  text-2xl font-bold text-function-error'}>
        <TrashIcon className={'h-12 w-12 opacity-70'} />
        Page Deleted
      </div>
      <div className={'mt-4 whitespace-pre text-center text-base text-text-primary opacity-80'}>
        You can restore it from the trash or create a new page
      </div>
      <div className={'mt-6 flex gap-4'}>
        {/*<button className={'px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition'}>*/}
        {/*  Restore Page*/}
        {/*</button>*/}
        {/*<button className={'px-4 py-2 bg-gray-200 text-gray-700 rounded hover:bg-gray-300 transition'}>*/}
        {/*  Create New Page*/}
        {/*</button>*/}
      </div>
      <img src={emptyImageSrc} alt={'Empty state'} className={'mt-8'} />
    </div>
  );
};

export default DeletedPageComponent;
