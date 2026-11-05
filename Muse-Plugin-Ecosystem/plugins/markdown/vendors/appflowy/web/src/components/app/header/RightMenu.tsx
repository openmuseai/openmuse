import { Divider, IconButton, Tooltip } from '@mui/material';
import { useMemo } from 'react';
import { useTranslation } from 'react-i18next';

import { isDatabaseContainer } from '@/application/view-utils';
import { ReactComponent as AiChat } from '@/assets/icons/ai_chat.svg';
import { ReactComponent as Logo } from '@/assets/icons/logo.svg';
import { findView } from '@/components/_shared/outline/utils';
import { useAppOutline, useAppView, useAppViewId } from '@/components/app/app.hooks';
import { useDshAgentContextOptional } from '@/components/dsh-agent';
import { openOrDownload } from '@/utils/open_schema';

import ShareButton from 'src/components/app/share/ShareButton';

import FavoriteButton from './FavoriteButton';
import MoreActions from './MoreActions';
import { Users } from './Users';

function RightMenu() {
  const { t } = useTranslation();
  const routeViewId = useAppViewId();
  const outline = useAppOutline();
  const routeView = useAppView(routeViewId);
  const dshAgent = useDshAgentContextOptional();
  const actionViewId = useMemo(() => {
    if (!routeViewId || !routeView?.parent_view_id) {
      return routeViewId;
    }

    const parentView = findView(outline || [], routeView.parent_view_id);

    return parentView && isDatabaseContainer(parentView) ? parentView.view_id : routeViewId;
  }, [outline, routeView?.parent_view_id, routeViewId]);

  return (
    <div className={'flex items-center gap-2'}>
      <Users viewId={routeViewId} />
      {actionViewId && <ShareButton viewId={actionViewId} />}
      {actionViewId && <FavoriteButton viewId={actionViewId} />}
      {actionViewId && <MoreActions viewId={actionViewId} />}

      {dshAgent && (
        <Tooltip title={dshAgent.open ? 'Hide DeepSeek Agent' : 'Show DeepSeek Agent'}>
          <IconButton
            size='small'
            data-testid='dsh-agent-toggle'
            onClick={dshAgent.toggle}
            aria-pressed={dshAgent.open}
            className={dshAgent.open ? 'bg-fill-secondary' : undefined}
          >
            <AiChat className={'h-5 w-5 text-text-secondary'} />
          </IconButton>
        </Tooltip>
      )}

      <Divider orientation={'vertical'} className={'mx-2'} flexItem />
      <Tooltip title={t('publish.downloadApp')}>
        <button onClick={() => openOrDownload()}>
          <Logo className={'h-6 w-6'} />
        </button>
      </Tooltip>
    </div>
  );
}

export default RightMenu;
