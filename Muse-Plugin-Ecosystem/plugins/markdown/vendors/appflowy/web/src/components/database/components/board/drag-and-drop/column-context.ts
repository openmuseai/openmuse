import { createContext, useContext } from 'react';

export type ColumnDragContextProps = {
  columnId: string;
  getCardIndex: (userId: string) => number;
  getNumCards: () => number;
};

export const ColumnDragContext = createContext<ColumnDragContextProps | null>(null);

export function useColumnDragContext(): ColumnDragContextProps {
  const value = useContext(ColumnDragContext);

  if (!value) {
    throw new Error('useColumnDragContext must be used within a ColumnProvider');
  }

  return value;
}
