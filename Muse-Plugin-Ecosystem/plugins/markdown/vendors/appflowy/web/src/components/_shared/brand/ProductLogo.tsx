import { useTranslation } from 'react-i18next';

import { ReactComponent as Logo } from '@/assets/icons/logo.svg';
import { cn } from '@/lib/utils';

export function ProductLogo({
  showWordmark = true,
  className,
  iconClassName,
  wordmarkClassName,
}: {
  showWordmark?: boolean;
  className?: string;
  iconClassName?: string;
  wordmarkClassName?: string;
}) {
  const { t } = useTranslation();

  return (
    <span className={cn('inline-flex min-w-0 items-center gap-2 text-text-primary', className)}>
      <Logo className={cn('h-6 w-6 shrink-0', iconClassName)} />
      {showWordmark && (
        <span className={cn('truncate font-semibold', wordmarkClassName)}>{t('appName')}</span>
      )}
    </span>
  );
}

export default ProductLogo;
