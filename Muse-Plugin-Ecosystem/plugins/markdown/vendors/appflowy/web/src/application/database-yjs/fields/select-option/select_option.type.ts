import { Filter } from '@/application/database-yjs';

export enum SelectOptionColor {
  OptionColor1 = 'Purple',
  OptionColor2 = 'Pink',
  OptionColor3 = 'LightPink',
  OptionColor4 = 'Orange',
  OptionColor5 = 'Yellow',
  OptionColor6 = 'Lime',
  OptionColor7 = 'Green',
  OptionColor8 = 'Aqua',
  OptionColor9 = 'Blue',
  OptionColor10 = 'Cream',
  OptionColor11 = 'Mint',
  OptionColor12 = 'Sky',
  OptionColor13 = 'Lilac',
  OptionColor14 = 'Pearl',
  OptionColor15 = 'Sunset',
  OptionColor16 = 'Coral',
  OptionColor17 = 'Sapphire',
  OptionColor18 = 'Moss',
  OptionColor19 = 'Sand',
  OptionColor20 = 'Charcoal',
}

export enum SelectOptionFilterCondition {
  OptionIs = 0,
  OptionIsNot = 1,
  OptionContains = 2,
  OptionDoesNotContain = 3,
  OptionIsEmpty = 4,
  OptionIsNotEmpty = 5,
}

export interface SelectOptionFilter extends Filter {
  condition: SelectOptionFilterCondition;
  optionIds: string[];
}

export interface SelectOption {
  id: string;
  name: string;
  color: SelectOptionColor;
}

export interface SelectTypeOption {
  disable_color: boolean;
  options: SelectOption[];
}
