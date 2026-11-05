import React, { useCallback, useEffect, useMemo, useState } from 'react';

import DshAgentPanel from '@/components/dsh-agent/DshAgentPanel';
import { getDshAgentUrl } from '@/components/dsh-agent/dsh-origin';

const DEFAULT_WIDTH = 420;
const STORAGE_KEY_OPEN = 'appflowy.dshAgent.open';
const STORAGE_KEY_WIDTH = 'appflowy.dshAgent.width';

function readStoredBool(key: string, fallback: boolean): boolean {
  try {
    const raw = localStorage.getItem(key);
    if (raw === null) return fallback;
    return raw === '1' || raw === 'true';
  } catch {
    return fallback;
  }
}

function readStoredWidth(fallback: number): number {
  try {
    const raw = localStorage.getItem(STORAGE_KEY_WIDTH);
    const n = raw ? Number(raw) : NaN;
    return Number.isFinite(n) && n >= 280 ? n : fallback;
  } catch {
    return fallback;
  }
}

export { getDshAgentUrl } from '@/components/dsh-agent/dsh-origin';

export const DshAgentContext = React.createContext<
  | {
      open: boolean;
      width: number;
      url: string;
      setOpen: (open: boolean) => void;
      toggle: () => void;
      setWidth: (width: number) => void;
    }
  | undefined
>(undefined);

export function useDshAgentContext() {
  const ctx = React.useContext(DshAgentContext);
  if (!ctx) {
    throw new Error('useDshAgentContext must be used within a DshAgentProvider');
  }
  return ctx;
}

export function useDshAgentContextOptional() {
  return React.useContext(DshAgentContext);
}

export function DshAgentProvider({ children }: { children: React.ReactNode }) {
  const url = useMemo(() => getDshAgentUrl(), []);
  const [open, setOpenState] = useState(() => readStoredBool(STORAGE_KEY_OPEN, false));
  const [width, setWidthState] = useState(() => readStoredWidth(DEFAULT_WIDTH));

  const setOpen = useCallback((next: boolean) => {
    setOpenState(next);
    try {
      localStorage.setItem(STORAGE_KEY_OPEN, next ? '1' : '0');
    } catch {
      /* ignore */
    }
  }, []);

  const toggle = useCallback(() => {
    setOpen(!open);
  }, [open, setOpen]);

  const setWidth = useCallback((next: number) => {
    setWidthState(next);
    try {
      localStorage.setItem(STORAGE_KEY_WIDTH, String(next));
    } catch {
      /* ignore */
    }
  }, []);

  useEffect(() => {
    const params = new URLSearchParams(window.location.search);
    if (params.get('dsh') === '1') {
      setOpen(true);
    }
  }, [setOpen]);

  const value = useMemo(
    () => ({ open, width, url, setOpen, toggle, setWidth }),
    [open, width, url, setOpen, toggle, setWidth]
  );

  return (
    <DshAgentContext.Provider value={value}>
      {children}
      <DshAgentPanel />
    </DshAgentContext.Provider>
  );
}
