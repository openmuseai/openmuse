import { ViewItem } from '@/components/chat/components/view/view-item';
import { View } from '@/components/chat/types';
import { CheckStatus } from '@/components/chat/types/checkbox';

export function ViewChildren({
  item,
  getCheckStatus,
  onToggle,
  getInitialExpand,
}: {
  item: View;
  getCheckStatus: (view: View) => CheckStatus;
  onToggle: (view: View) => void;
  getInitialExpand: (viewId: string) => boolean;
}) {
  if(!item.children || item.children.length === 0) {
    return null;
  }

  return (
    <div className={'flex pl-4 flex-col gap-1'}>
      {item.children.map((view: View) => {
        return (
          <ViewItem
            key={view.view_id}
            view={view}
            getCheckStatus={getCheckStatus}
            onToggle={onToggle}
            getInitialExpand={getInitialExpand}
          >
            <ViewChildren
              item={view}
              getCheckStatus={getCheckStatus}
              onToggle={onToggle}
              getInitialExpand={getInitialExpand}
            />
          </ViewItem>
        );
      })}
    </div>
  );
}

export default ViewChildren;