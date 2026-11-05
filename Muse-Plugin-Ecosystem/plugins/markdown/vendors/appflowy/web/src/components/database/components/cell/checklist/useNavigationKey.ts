import { useEffect } from 'react';

import { createHotkey, HOT_KEY_NAME } from '@/utils/hotkeys';

export function useNavigationKey({
  onToggleSelectedTask,
  onCreateTask,
  element,
  showCreateTask,
  setShowCreateTask,
  readOnly,
}: {
  onToggleSelectedTask: (taskId: string) => void;
  onCreateTask: () => void;
  element: HTMLElement | null;
  showCreateTask?: boolean;
  setShowCreateTask?: (showCreateTask: boolean) => void;
  readOnly?: boolean;
}) {
  useEffect(() => {
    if (!element || readOnly) {
      return;
    }

    // Add event listeners for keydown and click events
    const handleKeyDown = (event: KeyboardEvent) => {
      const isEnter = createHotkey(HOT_KEY_NAME.ENTER)(event);
      const isToggleChecked = createHotkey(HOT_KEY_NAME.TOGGLE_TODO)(event);

      const isTab = createHotkey(HOT_KEY_NAME.TAB)(event);
      const isEscape = createHotkey(HOT_KEY_NAME.ESCAPE)(event);

      const target = event.target as HTMLElement;

      if (!target || !target.dataset || !target.dataset.taskId) {
        return;
      }

      const taskId = target.dataset.taskId;

      const taskTargets = element.querySelectorAll('[data-task-id]');
      const taskIds = Array.from(taskTargets).map((task) => (task as HTMLInputElement).dataset.taskId);
      const index = taskIds.indexOf(taskId);

      const focusedNextTask = () => {
        if (setShowCreateTask && !showCreateTask) {
          const currentIndex = taskIds.indexOf(taskId);

          if (currentIndex === taskIds.length - 1) {
            setShowCreateTask(true);
            return;
          }
        }

        const nextTaskIndex = index === taskIds.length - 1 ? 0 : index + 1;
        const nextTask = taskTargets[nextTaskIndex] as HTMLElement;

        if (nextTask && nextTask.dataset && nextTask.dataset.taskId) {
          nextTask.focus();
        }
      };

      switch (true) {
        case isEscape:
          if (setShowCreateTask) {
            if (showCreateTask) {
              event.preventDefault();
              event.stopPropagation();
              setShowCreateTask(false);
            }
          }

          break;
        case isEnter:
          event.preventDefault();
          if (taskId === 'create') {
            onCreateTask();
          } else {
            // Handle enter key action for focused next task
            focusedNextTask();
          }

          break;
        case isToggleChecked:
          event.preventDefault();
          if (taskId === 'create') {
            break;
          } else {
            onToggleSelectedTask(taskId);
          }

          break;

        case isTab:
          event.preventDefault();
          if (taskId === 'create') {
            break;
          } else {
            focusedNextTask();
          }

          break;

        default:
          break;
      }
    };

    const handleClick = (event: MouseEvent) => {
      const target = event.target as HTMLElement;

      if (target && target.dataset && target.dataset.taskId) {
        target.focus();
      }
    };

    element.addEventListener('keydown', handleKeyDown);
    element.addEventListener('click', handleClick);

    // Cleanup event listeners on component unmount
    return () => {
      element.removeEventListener('keydown', handleKeyDown);
      element.removeEventListener('click', handleClick);
    };
  }, [onCreateTask, onToggleSelectedTask, element, setShowCreateTask, showCreateTask, readOnly]);
}
