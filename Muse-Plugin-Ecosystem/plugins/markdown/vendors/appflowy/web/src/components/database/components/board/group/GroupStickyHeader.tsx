import React, { forwardRef } from 'react';

import { GroupColumn, Row, useDatabaseContext } from '@/application/database-yjs';
import GroupHeader from '@/components/database/components/board/group/GroupHeader';

const GroupStickyHeader = forwardRef<HTMLDivElement, {
  columns: GroupColumn[];
  fieldId: string;
  groupResult: Map<string, Row[]>
  onScrollLeft: (left: number) => void,
  addCardBefore: (id: string) => void;
  style?: React.CSSProperties;
  groupId: string;
}>(({
  columns,
  fieldId,
  groupResult,
  onScrollLeft,
  addCardBefore,
  style,
  groupId,
}, ref) => {
  const context = useDatabaseContext();
  const {
    paddingStart,
    paddingEnd,
  } = context;

  return (
    <div
      ref={ref}
      style={{
        paddingLeft: paddingStart,
        paddingRight: paddingEnd,
        scrollBehavior: 'auto',
        ...style,
      }}
      onScroll={e => {
        const scrollLeft = e.currentTarget.scrollLeft;

        onScrollLeft(scrollLeft);
      }}
      className={'max-sm:!px-6 h-fit absolute w-full left-0 right-0 top-0 flex flex-col border-t border-border-primary pb-1 px-24 appflowy-custom-scroller bg-background-primary overflow-x-auto'}
    >
      <GroupHeader
        addCardBefore={addCardBefore}
        columns={columns}
        fieldId={fieldId}
        groupResult={groupResult}
        groupId={groupId}
      />
    </div>

  );
});

export default GroupStickyHeader;