import { useContext } from 'react';
import { useParams } from 'react-router-dom';

import { AuthInternalContext } from '@/components/app/contexts/AuthInternalContext';
import { AppNavigationContext } from '@/components/app/contexts/AppNavigationContext';
import { AppOperationsContext } from '@/components/app/contexts/AppOperationsContext';
import { AppOutlineContext } from '@/components/app/contexts/AppOutlineContext';

/** Safe outside AppProvider so the DSH panel can mount in isolated tests. */
export function useDshWorkspaceId(): string | undefined {
  const fromAuth = useContext(AuthInternalContext)?.currentWorkspaceId;
  const fromRoute = useParams<{ workspaceId?: string }>().workspaceId?.trim();
  return fromAuth || fromRoute || undefined;
}

export function useDshWorkspaceTitle(): string | undefined {
  const auth = useContext(AuthInternalContext);
  const id = auth?.currentWorkspaceId;
  const info = auth?.userWorkspaceInfo;
  if (!id) return info?.selectedWorkspace?.name;
  return info?.workspaces.find((item) => item.id === id)?.name ?? info?.selectedWorkspace?.name;
}

export function useDshViewId(): string | undefined {
  return useContext(AppNavigationContext)?.viewId;
}

export function useDshOutline() {
  return useContext(AppOutlineContext)?.outline;
}

export function useDshToView() {
  return useContext(AppOperationsContext)?.toView;
}
