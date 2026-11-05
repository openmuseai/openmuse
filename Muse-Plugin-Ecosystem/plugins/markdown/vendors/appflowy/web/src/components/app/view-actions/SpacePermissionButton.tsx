import { Button, Divider } from '@mui/material';
import React from 'react';
import { useTranslation } from 'react-i18next';

import { SpacePermission } from '@/application/types';
import { ReactComponent as ArrowDownIcon } from '@/assets/icons/alt_arrow_down.svg';
import { ReactComponent as LockIcon } from '@/assets/icons/lock.svg';
import { ReactComponent as PublicIcon } from '@/assets/icons/public.svg';
import { ReactComponent as TickIcon } from '@/assets/icons/tick.svg';
import { Popover } from '@/components/_shared/popover';

function SpacePermissionButton({
  onSelected,
  value,
}: {
  value: SpacePermission;
  onSelected?: (permission: SpacePermission) => void;
}) {
  const [anchorEl, setAnchorEl] = React.useState<null | HTMLElement>(null);
  const { t } = useTranslation();

  return (
    <>
      <Button
        size={'large'}
        className={'justify-start gap-4 py-3'}
        startIcon={SpacePermission.Private === value ? <LockIcon /> : <PublicIcon />}
        endIcon={<ArrowDownIcon />}
        color={'inherit'}
        variant={'outlined'}
        onClick={(e) => setAnchorEl(e.currentTarget)}
      >
        <div className={'flex w-full flex-col items-start'}>
          <div className={'font-normal text-text-primary'}>
            {SpacePermission.Private === value ? t('space.privatePermission') : t('space.publicPermission')}
          </div>
          <div className={'text-text-secondary'}>
            {SpacePermission.Private === value
              ? t('space.privatePermissionDescription')
              : t('space.publicPermissionDescription')}
          </div>
        </div>
      </Button>
      <Popover open={Boolean(anchorEl)} anchorEl={anchorEl} onClose={() => setAnchorEl(null)}>
        <div
          style={{
            width: anchorEl?.clientWidth,
          }}
          className={'flex flex-col gap-2 p-2'}
        >
          <Button
            className={'justify-start gap-2 px-4'}
            startIcon={<LockIcon />}
            color={'inherit'}
            onClick={() => {
              onSelected?.(SpacePermission.Private);
              setAnchorEl(null);
            }}
          >
            <div className={'flex w-full flex-col items-start'}>
              <div className={'text-base font-normal'}>{t('space.privatePermission')}</div>
              <div className={'text-text-secondary'}>{t('space.privatePermissionDescription')}</div>
            </div>
            {SpacePermission.Private === value && <TickIcon className={'h-6 w-6 text-function-success'} />}
          </Button>
          <Divider />
          <Button
            className={'justify-start gap-2 px-4'}
            startIcon={<PublicIcon />}
            color={'inherit'}
            onClick={() => {
              onSelected?.(SpacePermission.Public);
              setAnchorEl(null);
            }}
          >
            <div className={'flex w-full flex-col items-start'}>
              <div className={'text-base font-normal'}>{t('space.publicPermission')}</div>
              <div className={'text-text-secondary'}>{t('space.publicPermissionDescription')}</div>
            </div>
            {SpacePermission.Public === value && <TickIcon className={'h-6 w-6 text-function-success'} />}
          </Button>
        </div>
      </Popover>
    </>
  );
}

export default SpacePermissionButton;
