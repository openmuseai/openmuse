import { ReactComponent as ToggleListIcon } from '@/assets/icons/toggle_list.svg';

function OutlineIcon({
  isExpanded,
  setIsExpanded,
  level,
}: {
  isExpanded: boolean;
  setIsExpanded: (isExpanded: boolean) => void;
  level: number;
}) {
  if (isExpanded) {
    return (
      <button
        data-testid="outline-toggle-collapse"
        style={{
          paddingLeft: 1.125 * level + 'em',
        }}
        onClick={(e) => {
          e.stopPropagation();
          setIsExpanded(false);
        }}
        className={'opacity-50 hover:opacity-100'}
      >
        <ToggleListIcon className={'rotate-90 transform rounded-[2px] hover:bg-fill-content-hover'} />
      </button>
    );
  }

  return (
    <button
      data-testid="outline-toggle-expand"
      style={{
        paddingLeft: 1.125 * level + 'em',
      }}
      className={'opacity-50 hover:opacity-100'}
      onClick={(e) => {
        e.stopPropagation();
        setIsExpanded(true);
      }}
    >
      <ToggleListIcon className={'rounded-[2px] hover:bg-fill-content-hover'} />
    </button>
  );
}

export default OutlineIcon;
