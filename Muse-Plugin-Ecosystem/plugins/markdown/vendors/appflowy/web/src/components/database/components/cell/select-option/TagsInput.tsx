import { cva, type VariantProps } from 'class-variance-authority';
import { forwardRef, useState, useRef, useEffect, KeyboardEvent } from 'react';
import * as React from 'react';

import { SelectOptionColor } from '@/application/database-yjs';
import { ReactComponent as CloseIcon } from '@/assets/icons/close.svg';
import { cn } from '@/lib/utils';

const tagColorClasses = {
  [SelectOptionColor.OptionColor1]: 'bg-tag-fill-01-light text-tag-text-01-light',
  [SelectOptionColor.OptionColor2]: 'bg-tag-fill-02-light text-tag-text-02-light',
  [SelectOptionColor.OptionColor3]: 'bg-tag-fill-03-light text-tag-text-03-light',
  [SelectOptionColor.OptionColor4]: 'bg-tag-fill-04-light text-tag-text-04-light',
  [SelectOptionColor.OptionColor5]: 'bg-tag-fill-05-light text-tag-text-05-light',
  [SelectOptionColor.OptionColor6]: 'bg-tag-fill-06-light text-tag-text-06-light',
  [SelectOptionColor.OptionColor7]: 'bg-tag-fill-07-light text-tag-text-07-light',
  [SelectOptionColor.OptionColor8]: 'bg-tag-fill-08-light text-tag-text-08-light',
  [SelectOptionColor.OptionColor9]: 'bg-tag-fill-09-light text-tag-text-09-light',
  [SelectOptionColor.OptionColor10]: 'bg-tag-fill-10-light text-tag-text-10-light',
  [SelectOptionColor.OptionColor11]: 'bg-tag-fill-01-thick text-tag-text-01-thick',
  [SelectOptionColor.OptionColor12]: 'bg-tag-fill-02-thick text-tag-text-02-thick',
  [SelectOptionColor.OptionColor13]: 'bg-tag-fill-03-thick text-tag-text-03-thick',
  [SelectOptionColor.OptionColor14]: 'bg-tag-fill-04-thick text-tag-text-04-thick',
  [SelectOptionColor.OptionColor15]: 'bg-tag-fill-05-thick text-tag-text-05-thick',
  [SelectOptionColor.OptionColor16]: 'bg-tag-fill-06-thick text-tag-text-06-thick',
  [SelectOptionColor.OptionColor17]: 'bg-tag-fill-07-thick text-tag-text-07-thick',
  [SelectOptionColor.OptionColor18]: 'bg-tag-fill-08-thick text-tag-text-08-thick',
  [SelectOptionColor.OptionColor19]: 'bg-tag-fill-09-thick text-tag-text-09-thick',
  [SelectOptionColor.OptionColor20]: 'bg-tag-fill-10-thick text-tag-text-10-thick',
};

export interface Tag {
  id: string;
  text: string;
  color?: SelectOptionColor;
}

// Base input styles that apply to all variants and sizes
const baseInputStyles = cn(
  // Text and placeholder styling
  'text-text-primary placeholder:text-text-tertiary',

  // Selection styling
  'selection:bg-fill-theme-thick selection:text-text-on-fill focus:caret-fill-theme-thick',

  'bg-fill-content',

  // Layout
  'flex min-w-[100px]',

  // Typography
  'text-sm',

  // Effects
  'outline-none',

  // File input styling
  'file:inline-flex file:border-0 file:bg-fill-content file:text-sm file:font-medium',

  // Disabled state
  'disabled:pointer-events-none disabled:cursor-not-allowed'
);

const tagInputVariants = cva('flex items-center gap-1 overflow-x-auto scrollbar-hide', {
  variants: {
    variant: {
      // Default variant with focus styles
      default:
        'border-border-primary border data-[focused=true]:border-border-theme-thick data-[focused=true]:ring-border-theme-thick data-[focused=true]:ring-[0.5px] disabled:border-border-primary disabled:bg-fill-primary-hover disabled:text-text-tertiary hover:border-border-primary-hover',
    },
    size: {
      // Small size input
      sm: 'h-8 px-2 rounded-300',

      // Medium size input (default)
      md: 'h-10 px-2 rounded-400',
    },
  },
  defaultVariants: {
    variant: 'default',
    size: 'sm',
  },
});

// Tag component styles
const tagStyles = cva(
  'flex items-center h-5 py-[1px] max-w-[120px] rounded-[6px] px-1 text-text-primary text-sm gap-1 whitespace-nowrap',
  {
    variants: {
      color: tagColorClasses,
    },
    defaultVariants: {
      color: SelectOptionColor.OptionColor1,
    },
  }
);

// Tag component
interface TagComponentProps {
  tag: Tag;
  onRemove: (id: string) => void;
}

const TagComponent = ({ tag, onRemove }: TagComponentProps) => {
  return (
    <div className={cn(tagStyles({ color: tag.color || SelectOptionColor.OptionColor9 }))}>
      <span className={'flex-1 truncate text-[0.75rem] leading-[1.5]'}>{tag.text}</span>
      <button
        type='button'
        tabIndex={-1}
        className='flex h-[14px] w-[14px] items-start justify-center rounded-full p-0.5 hover:bg-fill-content-hover focus:outline-none focus:ring-1 focus:ring-offset-1'
        onClick={() => onRemove(tag.id)}
      >
        <CloseIcon className='h-[10px] w-[10px]' />
      </button>
    </div>
  );
};

export interface TagInputProps
  extends Omit<React.InputHTMLAttributes<HTMLInputElement>, 'size' | 'variant' | 'value' | 'onChange'>,
    VariantProps<typeof tagInputVariants> {
  multiple?: boolean;
  tags: Tag[];
  onTagsChange: (tags: Tag[]) => void;
  inputProps?: React.InputHTMLAttributes<HTMLInputElement>;
  inputRef?: React.Ref<HTMLInputElement>;
  onInputChange?: (value: string) => void;
  inputValue?: string;
}

const TagsInput = forwardRef<HTMLDivElement, TagInputProps>(
  (
    {
      className,
      variant,
      size,
      multiple = false,
      tags,
      onTagsChange,
      inputProps,
      inputRef,
      onInputChange,
      inputValue = '',
      ...props
    },
    ref
  ) => {
    const [focused, setFocused] = useState(false);
    const [internalInputValue, setInternalInputValue] = useState(inputValue);
    const containerRef = useRef<HTMLDivElement>(null);
    const inputElementRef = useRef<HTMLInputElement>(null);

    // Handle the forwarded ref
    const resolvedInputRef = (inputRef as React.RefObject<HTMLInputElement>) || inputElementRef;

    useEffect(() => {
      setInternalInputValue(inputValue);
    }, [inputValue]);

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
      const value = e.target.value;

      setInternalInputValue(value);
      onInputChange?.(value);
    };

    const handleRemoveTag = (id: string) => {
      const newTags = tags.filter((tag) => tag.id !== id);

      onTagsChange(newTags);
    };

    const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
      // Handle a backspace key when input is empty to remove the last tag
      if (e.key === 'Backspace' && internalInputValue === '' && tags.length > 0) {
        const newTags = [...tags];

        newTags.pop();
        onTagsChange(newTags);
      }

      // Handle Enter key to add a new tag
      if (e.key === 'Enter' && internalInputValue.trim() !== '') {
        e.preventDefault();

        // For single tag mode, replace the existing tag
        if (!multiple) {
          onTagsChange([
            {
              id: Date.now().toString(),
              text: internalInputValue.trim(),
            },
          ]);
        } else {
          // For multiple tags, add to the existing ones
          onTagsChange([
            ...tags,
            {
              id: Date.now().toString(),
              text: internalInputValue.trim(),
            },
          ]);
        }

        setInternalInputValue('');
        onInputChange?.('');
      }
    };

    const focusInput = () => {
      resolvedInputRef.current?.focus();
    };

    return (
      <div
        ref={ref}
        data-slot='tag-input'
        className={cn(tagInputVariants({ variant, size }), className)}
        data-focused={focused}
        onClick={focusInput}
      >
        <div ref={containerRef} className={'appflowy-hidden-scroller h-full w-full overflow-x-auto'}>
          <div className={'flex h-full flex-grow flex-nowrap items-center gap-1'}>
            {/* Render tags */}
            {tags.map((tag) => (
              <TagComponent key={tag.id} tag={tag} onRemove={handleRemoveTag} />
            ))}

            {/* Input field */}
            <input
              ref={resolvedInputRef}
              type='text'
              className={cn(
                'min-w-[100px] flex-1',
                baseInputStyles,
                // Invalid state styling (applied via aria-invalid attribute)
                'aria-invalid:ring-border-error-thick aria-invalid:border-border-error-thick',
                inputProps?.className
              )}
              onFocus={(e) => {
                setFocused(true);
                props?.onFocus?.(e);
              }}
              onKeyDown={handleKeyDown}
              onBlur={(e) => {
                setFocused(false);
                props?.onBlur?.(e);
              }}
              value={internalInputValue}
              onChange={handleInputChange}
              {...props}
              {...inputProps}
            />
          </div>
        </div>
      </div>
    );
  }
);

export { TagsInput };
