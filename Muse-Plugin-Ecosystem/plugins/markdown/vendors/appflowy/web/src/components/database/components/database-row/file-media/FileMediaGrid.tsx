import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { GridContextProvider, GridDropZone, GridItem, swap } from 'react-grid-dnd';

import { useReadOnly } from '@/application/database-yjs';
import { FileMediaCellData, FileMediaCellDataItem } from '@/application/database-yjs/cell.type';
import { cn } from '@/lib/utils';

const FileMediaGrid = ({
  items,
  itemWidth = 94,
  itemHeight = 70,
  padding = 4,
  renderItem,
  reorderAction,
}: {
  items: FileMediaCellDataItem[];
  itemWidth?: number;
  itemHeight?: number;
  padding?: number;
  renderItem: (item: FileMediaCellDataItem) => React.ReactNode;
  reorderAction: (args: { newData: FileMediaCellData }) => void;
}) => {
  const readOnly = useReadOnly();
  const [isExpanded, setIsExpanded] = useState(false);
  const [gridItems, setGridItems] = useState<FileMediaCellDataItem[]>(items);
  const stableDataRef = useRef<FileMediaCellDataItem[]>(items);
  const gridRef = useRef<HTMLDivElement | null>(null);
  const [containerWidth, setContainerWidth] = useState(0);

  useEffect(() => {
    stableDataRef.current = items;
  }, [items]);

  const toggleExpand = useCallback(() => {
    setIsExpanded((prev) => !prev);
  }, []);

  // Use ResizeObserver to monitor container width changes
  useEffect(() => {
    if (!gridRef.current) return;

    const resizeObserver = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const newWidth = entry.contentRect.width;

        if (newWidth !== containerWidth) {
          setContainerWidth(newWidth);
        }
      }
    });

    resizeObserver.observe(gridRef.current);

    return () => {
      resizeObserver.disconnect();
    };
  }, [containerWidth]);

  // Update displayed grid items
  const updateGridItems = useCallback(() => {
    if (!gridRef.current || items.length === 0 || containerWidth === 0) {
      return;
    }

    const itemWithPadding = itemWidth + padding * 2;
    const itemsPerRow = Math.floor(containerWidth / itemWithPadding);
    const cols = Math.max(1, itemsPerRow);

    // If expanded, show all items
    if (isExpanded) {
      setGridItems(items);
      return;
    }

    // Calculate maximum number of items in two rows
    const twoRowsCapacity = cols * 2;

    // If total number of items doesn't exceed two rows capacity, show all items
    if (items.length <= twoRowsCapacity) {
      setGridItems(items);
    } else {
      setGridItems(items.slice(0, twoRowsCapacity));
    }
  }, [isExpanded, itemWidth, padding, items, containerWidth]);

  // Update grid items when dependencies change
  useEffect(() => {
    updateGridItems();
  }, [updateGridItems]);

  // Calculate display information
  const displayInfo = useMemo(() => {
    // Calculate capacity when not expanded
    if (!isExpanded) {
      if (items.length > gridItems.length) {
        const visibleCount = gridItems.length - 1;
        const hiddenCount = items.length - visibleCount - 1;

        return { visibleCount, hiddenCount, hasMoreItems: true };
      }
    }

    // Default case: all items are visible or capacity not exceeded
    return {
      visibleCount: items.length,
      hiddenCount: 0,
      hasMoreItems: false,
    };
  }, [isExpanded, items.length, gridItems.length]);

  // Calculate grid configuration
  const gridConfig = useMemo(() => {
    if (containerWidth === 0) return { boxesPerRow: 4, rowHeight: itemHeight + padding * 2 };

    const itemWithPadding = itemWidth + padding * 2;
    const boxesPerRow = Math.floor(containerWidth / itemWithPadding);

    return {
      boxesPerRow: Math.max(1, boxesPerRow),
      rowHeight: itemHeight + padding * 2,
    };
  }, [containerWidth, itemWidth, itemHeight, padding]);

  // Handle drag end
  const onDragEnd = useCallback(
    (sourceId: string, sourceIndex: number, targetIndex: number) => {
      if (readOnly) return;
      const newData = swap(stableDataRef.current, sourceIndex, targetIndex);

      reorderAction({ newData });
      setGridItems((prev) => {
        return swap([...prev], sourceIndex, targetIndex);
      });
    },
    [reorderAction, readOnly]
  );

  // Calculate grid height
  const dropZoneHeight = useMemo(() => {
    const rows = Math.ceil(gridItems.length / gridConfig.boxesPerRow);

    if (isExpanded) {
      return rows * gridConfig.rowHeight;
    }

    if (rows === 0) {
      return 0; // Default height for empty grid
    }

    if (rows === 1) {
      return gridConfig.rowHeight; // Single row case
    }

    return 2 * gridConfig.rowHeight; // Fixed to two rows when not expanded
  }, [isExpanded, gridItems.length, gridConfig.boxesPerRow, gridConfig.rowHeight]);

  const containerStyle = useMemo(() => {
    // Set a fixed width for each item
    const itemTotalWidth = itemWidth + padding * 2;

    // Create a grid with items left-aligned
    return {
      display: 'grid',
      gridTemplateColumns: `repeat(auto-fill, ${itemTotalWidth}px)`,
      gridGap: '0px',
      justifyContent: 'start', // Key property to ensure left alignment
      width: '100%',
      overflow: 'hidden',
    };
  }, [itemWidth, padding]);

  return (
    <div ref={gridRef} className='relative w-full'>
      <GridContextProvider onChange={onDragEnd}>
        <GridDropZone
          id='grid'
          boxesPerRow={gridConfig.boxesPerRow}
          rowHeight={gridConfig.rowHeight}
          style={{
            height: dropZoneHeight,
            padding: 0,
            ...containerStyle,
          }}
        >
          {gridItems.map((item, index) => {
            const isLastVisibleItem = !isExpanded && displayInfo.hasMoreItems && index === displayInfo.visibleCount;

            const GridItemComponent = readOnly ? 'div' : GridItem;

            return (
              <GridItemComponent
                className={cn(readOnly && 'cursor-default select-none')}
                key={item.id}
                style={{
                  minWidth: itemWidth + padding * 2,
                  height: itemHeight + padding * 2,
                  padding: padding,
                  margin: 0,
                  flexGrow: 0,
                }}
              >
                <div className='relative h-full w-full overflow-hidden'>
                  {renderItem(item)}

                  {isLastVisibleItem && (
                    <div
                      className='absolute inset-0 z-10 flex cursor-pointer items-center justify-center overflow-hidden rounded-[6px] bg-surface-overlay transition-colors duration-200 hover:bg-fill-content-visible-hover'
                      onMouseDown={(e) => {
                        e.stopPropagation();
                        e.stopPropagation();
                      }}
                      onClick={(e) => {
                        e.stopPropagation();
                        toggleExpand();
                      }}
                    >
                      <div className='flex flex-col items-center text-sm font-semibold text-text-on-fill'>
                        {`+${displayInfo.hiddenCount}`}
                      </div>
                    </div>
                  )}
                </div>
              </GridItemComponent>
            );
          })}
        </GridDropZone>
      </GridContextProvider>
    </div>
  );
};

export default FileMediaGrid;
