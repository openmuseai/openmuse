import { useTranslation } from 'react-i18next';

import { MentionablePerson, MentionPersonRole } from '@/application/types';
import { ReactComponent as AtIcon } from '@/assets/icons/invite_user.svg';
import { cn } from '@/lib/utils';

import { PersonAvatar } from './PersonAvatar';

interface PersonSuggestionItemProps {
  suggestion: { type: 'user' | 'email'; data: MentionablePerson | string };
  isHovered: boolean;
  onClick: () => void;
  onMouseEnter: () => void;
}

export function PersonSuggestionItem({
  suggestion,
  isHovered,
  onClick,
  onMouseEnter,
}: PersonSuggestionItemProps) {
  const { t } = useTranslation();
  
  if (suggestion.type === 'user' && typeof suggestion.data !== 'string') {
    const person = suggestion.data;
    const isGuest = person.role === MentionPersonRole.Guest;

    return (
      <div
        className={cn(
          'flex w-full items-center gap-2 rounded-300 px-2 py-1.5 cursor-pointer',
          'hover:bg-fill-content-hover',
          isHovered && 'bg-fill-content-hover'
        )}
        onMouseEnter={onMouseEnter}
        onClick={onClick}
      >
        <div className='flex w-full flex-row items-center gap-2 overflow-hidden'>
          <PersonAvatar avatarUrl={person.avatar_url || ''} name={person.name} />
          <div className='flex w-full flex-1 flex-col gap-0.5 overflow-hidden'>
            <div className='flex items-center gap-2'>
              <div className='truncate text-sm text-text-primary'>{person.name}</div>
              {isGuest && (
                <span className='rounded-full bg-fill-warning-light px-2 py-[1px] text-xs text-text-warning-on-fill'>
                  {t('shareAction.guest')}
                </span>
              )}
            </div>
            <div className='truncate whitespace-nowrap text-xs text-text-secondary'>{person.email}</div>
          </div>
        </div>
      </div>
    );
  }

  // Email suggestion
  return (
    <div
      className={cn(
        'flex items-center gap-2 px-2 py-1.5 rounded-300 cursor-pointer text-sm',
        'hover:bg-fill-content-hover',
        isHovered && 'bg-fill-content-hover'
      )}
      onMouseEnter={onMouseEnter}
      onClick={onClick}
    >
      <AtIcon className="w-5 h-5 text-text-primary" />
      <div className="text-text-primary">{suggestion.data as string}</div>
    </div>
  );
}