import { useState } from 'react';

import { SelectOption, useReadOnly } from '@/application/database-yjs';
import TaskItem from '@/components/database/components/cell/checklist/TaskItem';
import { DragContext, useDragContextValue } from '@/components/database/components/drag-and-drop/useDragContext';
import { cn } from '@/lib/utils';

function TaskList({
  tasks,
  selectedTasks,
  onChangeTask,
  onToggleChecked,
  onRemoveTask,
  onReorderTasks,
  scrollable,
}: {
  tasks: SelectOption[];
  selectedTasks: string[];
  onChangeTask: (task: SelectOption) => void;
  onToggleChecked: (taskId: string) => void;
  onReorderTasks: (args: {
    oldData: SelectOption[];
    newData: SelectOption[];
    startIndex: number;
    finishIndex: number;
  }) => void;
  onRemoveTask: (taskId: string) => void;
  scrollable?: boolean;
}) {
  const readOnly = useReadOnly();
  const [container, setContainer] = useState<HTMLDivElement | null>(null);

  const contextValue = useDragContextValue({
    data: tasks,
    enabled: !readOnly,
    reorderAction: onReorderTasks,
    container,
  });

  return (
    <div
      ref={setContainer}
      className={cn('flex w-full flex-col', scrollable && 'appflowy-scroller max-h-[300px] overflow-y-auto')}
    >
      <DragContext.Provider value={contextValue}>
        {tasks.map((task) => {
          const isSelected = selectedTasks.includes(task.id);

          return (
            <TaskItem
              key={task.id}
              onToggleChecked={onToggleChecked}
              onChange={onChangeTask}
              task={task}
              isSelected={isSelected}
              onRemove={onRemoveTask}
            />
          );
        })}
      </DragContext.Provider>
    </div>
  );
}

export default TaskList;
