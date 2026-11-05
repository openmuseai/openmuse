import { useTranslation } from 'react-i18next';

import { ReactComponent as DownIcon } from '@/assets/icons/alt_arrow_down.svg';
import { cn } from '@/lib/utils';

function RowSwitchFieldsHidden({
  onToggleSwitchFieldsHidden,
  isFilterHidden,
  hideCount,
}: {
  hideCount: number;
  isFilterHidden: boolean;
  onToggleSwitchFieldsHidden: () => void;
}) {
  const { t } = useTranslation();
  const handleClick = () => {
    onToggleSwitchFieldsHidden();
  };

  if (hideCount === 0) return null;
  return (
    <div
      onClick={handleClick}
      className={
        'mx-6 flex h-[36px] cursor-pointer items-center gap-1.5 rounded-300 bg-fill-content px-1 py-2 text-sm font-medium text-text-secondary hover:bg-fill-content-hover'
      }
    >
      <DownIcon className={cn('h-5 w-5 text-text-secondary', 'transform', !isFilterHidden && 'rotate-180')} />
      {isFilterHidden
        ? t('grid.rowPage.showHiddenFields', {
            count: hideCount,
          })
        : t('grid.rowPage.hideHiddenFields', {
            count: hideCount,
          })}
    </div>
  );
}

export default RowSwitchFieldsHidden;
