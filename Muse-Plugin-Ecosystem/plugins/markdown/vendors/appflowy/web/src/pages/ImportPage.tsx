
import Typography from '@mui/material/Typography';
import React from 'react';
import { useSearchParams } from 'react-router-dom';

import ProductLogo from '@/components/_shared/brand/ProductLogo';
import Import from '@/components/_shared/more-actions/importer/Import';

function ImportPage() {
  const [search] = useSearchParams();
  const redirectTo = search.get('redirectToImport');
  const onSuccess = React.useCallback(() => {
    if (redirectTo) {
      window.location.href = redirectTo;
    }
  }, [redirectTo]);

  return (
    <div className={'flex h-screen w-screen flex-col bg-[#EEEEFD]'}>
      <div className={'h-[64px] w-full px-6 py-4'}>
        <Typography variant='h3' className={'mb-[27px] flex items-center gap-4 text-text-primary'} gutterBottom>
          <>
            <ProductLogo iconClassName={'h-8 w-8'} />
          </>
        </Typography>
      </div>
      <Import onSuccessfulImport={onSuccess} disableClose={true} />
    </div>
  );
}

export default ImportPage;
