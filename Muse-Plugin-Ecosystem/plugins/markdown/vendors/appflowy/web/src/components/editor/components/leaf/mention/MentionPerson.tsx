import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';

import { MentionType } from '@/application/types';
import { useEditorContext } from '@/components/editor/EditorContext';

export function MentionPerson({ personId, person_name }: { type: MentionType; personId: string; person_name?: string }) {
  const [isDeleted, setIsDeleted] = useState(false);
  const { t } = useTranslation();
  const [name, setName] = useState(person_name);
  const { getMentionUser } = useEditorContext();

  useEffect(() => {
    const fetchUser = async () => {
      try {
        const user = await getMentionUser?.(personId);

        if (user) {
          setName(user.name);
        } else {
          setIsDeleted(true);
        }
      } catch (error) {
        setName(person_name || 'Anonymous');
      }
    };

    void fetchUser();
  }, [getMentionUser, personId, person_name]);

  return (
    <span contentEditable={false} data-mention-id={personId} className='mention-person'>
      <span className='mr-0.5 text-text-tertiary'>@</span>
      <span className='text-text-secondary'>{isDeleted ? t('deleted') : name}</span>
    </span>
  );
}
