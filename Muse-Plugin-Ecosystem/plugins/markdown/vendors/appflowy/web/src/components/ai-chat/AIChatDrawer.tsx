import React from 'react';

import { useAIChatContext } from '@/components/ai-chat/AIChatProvider';
import DrawerContent from '@/components/ai-chat/DrawerContent';
import DrawerHeader from '@/components/ai-chat/DrawerHeader';
import Pinned from '@/components/ai-chat/Pinned';
import { useDshAgentContextOptional } from '@/components/dsh-agent';

import Resizer from './Resizer';

function AIChatDrawer() {
  const { drawerOpen, openViewId, drawerWidth, onSetDrawerWidth } = useAIChatContext();
  const dshAgent = useDshAgentContextOptional();
  const rightOffset = dshAgent?.open ? dshAgent.width : 0;

  return (
    <div
      className={'fixed top-0 h-screen transform bg-background-primary transition-transform'}
      style={{ right: rightOffset }}
    >
      <div
        style={{
          width: drawerOpen ? drawerWidth : 0,
        }}
        className={'h-full overflow-hidden border-l border-line-border'}
      >
        {openViewId && (
          <div className={'appflowy-scroller flex h-full flex-col overflow-auto overflow-x-hidden'}>
            <DrawerHeader />
            <DrawerContent openViewId={openViewId} />
          </div>
        )}

        <Resizer drawerWidth={drawerWidth} onResize={onSetDrawerWidth} />
      </div>

      {!drawerOpen && openViewId && <Pinned />}
    </div>
  );
}

export default React.memo(AIChatDrawer);
