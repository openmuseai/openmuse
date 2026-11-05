import { RequestAccessError } from '@/components/app/hooks/useWorkspaceData';
import { RequestAccessContent } from '@/components/app/share/RequestAccessContent';
import ProductLogo from '@/components/_shared/brand/ProductLogo';

interface RequestAccessProps {
  error?: RequestAccessError;
}

function RequestAccess({ error }: RequestAccessProps) {
  return (
    <div className='flex h-screen w-screen flex-col bg-background-primary'>
      <div className='absolute left-0 top-0 flex h-[60px] w-full items-center justify-between gap-[10px] p-4'>
        <span
          onClick={() => {
            window.open('/app', '_self');
          }}
          className='flex h-full cursor-pointer items-center'
        >
          <ProductLogo iconClassName='h-8 w-8' />
        </span>
      </div>
      <div className='flex w-full flex-1 items-center justify-center'>
        <RequestAccessContent error={error} />
      </div>
    </div>
  );
}

export default RequestAccess;
