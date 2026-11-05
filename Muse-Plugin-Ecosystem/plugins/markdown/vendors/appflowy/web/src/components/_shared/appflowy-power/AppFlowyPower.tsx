import { Divider } from '@mui/material';

import ProductLogo from '@/components/_shared/brand/ProductLogo';
import { brand, isConfiguredUrl } from '@/config/brand';

function AppFlowyPower({ divider, width }: { divider?: boolean; width?: number }) {
  const siteUrl = isConfiguredUrl(brand.siteUrl) ? brand.siteUrl : undefined;

  return (
    <div
      style={{
        width,
      }}
      className={
        'sticky bottom-[-0.5px] flex w-full transform-gpu flex-col items-center justify-center rounded-[16px] bg-background-primary'
      }
    >
      {divider && <Divider className={'my-0 w-full'} />}

      <div
        onClick={() => {
          if (siteUrl) window.open(siteUrl, '_blank');
        }}
        style={{
          width,
        }}
        className={
          'flex w-full items-center justify-center gap-2 py-4 text-sm text-text-primary opacity-50' +
          (siteUrl ? ' cursor-pointer' : '')
        }
      >
        Powered by
        <ProductLogo wordmarkClassName='font-medium' iconClassName='h-5 w-5' />
      </div>
    </div>
  );
}

export default AppFlowyPower;
