import { useMemo } from 'react';

import { getTypeOptions } from '@/application/database-yjs';
import { useFieldSelector } from '@/application/database-yjs/selector';

export function useFieldTypeOption (fieldId: string) {
  const { field, clock } = useFieldSelector(fieldId);

  return useMemo(() => {
    return getTypeOptions(field);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [field, clock]);
}
