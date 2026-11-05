import { useTranslation } from 'react-i18next';

import { ReactComponent as CloseIcon } from '@/assets/icons/close.svg';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog';
import { Progress } from '@/components/ui/progress';

interface RemoveAccessConfirmDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onConfirm: () => void;
  loading?: boolean;
  title?: string;
  description?: string;
}

export function RemoveAccessConfirmDialog({
  open,
  onOpenChange,
  onConfirm,
  loading = false,
  title,
  description,
}: RemoveAccessConfirmDialogProps) {
  const { t } = useTranslation();

  return (
    <AlertDialog open={open} onOpenChange={onOpenChange}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <div className='flex items-center justify-between'>
            <AlertDialogTitle className='text-sm font-normal text-text-primary'>
              {title || t('shareAction.removeYourAccess')}
            </AlertDialogTitle>
            <button
              type='button'
              className='flex h-6 w-6 items-center justify-center rounded-300 text-text-secondary hover:bg-fill-content-hover hover:text-text-primary'
              onClick={() => onOpenChange(false)}
            >
              <CloseIcon className='h-5 w-5' />
            </button>
          </div>
        </AlertDialogHeader>
        {description && (
          <AlertDialogDescription>
            {description}
          </AlertDialogDescription>
        )}
        <AlertDialogFooter>
          <AlertDialogCancel>{t('button.cancel')}</AlertDialogCancel>
          <AlertDialogAction
            className={'bg-fill-error-thick text-text-on-fill hover:bg-fill-error-thick-hover'}
            onClick={onConfirm}
            disabled={loading}
          >
            {loading && <Progress variant='primary' />}
            {t('button.remove')}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}