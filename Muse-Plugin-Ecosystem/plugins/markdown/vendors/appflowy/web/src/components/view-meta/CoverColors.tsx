import { ColorEnum, colorMap, GradientEnum, gradientMap } from '@/utils/color';

import { ColorTile } from '../_shared/color-picker';

const colors = [
  [ColorEnum.Tint1, colorMap[ColorEnum.Tint1]],
  [ColorEnum.Tint2, colorMap[ColorEnum.Tint2]],
  [ColorEnum.Tint3, colorMap[ColorEnum.Tint3]],
  [ColorEnum.Tint4, colorMap[ColorEnum.Tint4]],
  [ColorEnum.Tint5, colorMap[ColorEnum.Tint5]],
  [ColorEnum.Tint6, colorMap[ColorEnum.Tint6]],
  [ColorEnum.Tint7, colorMap[ColorEnum.Tint7]],
  [ColorEnum.Tint8, colorMap[ColorEnum.Tint8]],
  [ColorEnum.Tint9, colorMap[ColorEnum.Tint9]],
  [ColorEnum.Tint10, colorMap[ColorEnum.Tint10]],
];

const gradients = [
  [GradientEnum.gradient1, gradientMap[GradientEnum.gradient1]],
  [GradientEnum.gradient2, gradientMap[GradientEnum.gradient2]],
  [GradientEnum.gradient3, gradientMap[GradientEnum.gradient3]],
  [GradientEnum.gradient4, gradientMap[GradientEnum.gradient4]],
  [GradientEnum.gradient5, gradientMap[GradientEnum.gradient5]],
  [GradientEnum.gradient6, gradientMap[GradientEnum.gradient6]],
  [GradientEnum.gradient7, gradientMap[GradientEnum.gradient7]],
  [GradientEnum.gradient8, gradientMap[GradientEnum.gradient8]],
  [GradientEnum.gradient9, gradientMap[GradientEnum.gradient9]],
  [GradientEnum.gradient10, gradientMap[GradientEnum.gradient10]],
];

function Colors({
  isPro,
  selectedColor,
  onDone,
}: {
  isPro: boolean;
  selectedColor?: string;
  onDone?: (value: string) => void;
}) {
  return (
    <div className='flex flex-col gap-2'>
      <div className={'flex justify-between'}>
        {colors.map(([name, value]) => (
          <ColorTile key={name} value={value} active={selectedColor === name} onClick={() => onDone?.(name)} />
        ))}
      </div>
      {isPro && (
        <div className={'flex justify-between'}>
          {gradients.map(([name, value]) => (
            <ColorTile key={name} value={value} active={selectedColor === name} onClick={() => onDone?.(name)} />
          ))}
        </div>
      )}
    </div>
  );
}

export default Colors;
