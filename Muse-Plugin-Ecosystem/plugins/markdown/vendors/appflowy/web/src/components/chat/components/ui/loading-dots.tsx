export default function LoadingDots({
  className,
  colors = ['#00b5ff', '#e3006d', '#f7931e'],
  size = 30,
  ...props
}: {
  className?: string;
  size?: number;
  colors?: [string, string, string];
} & React.HTMLAttributes<HTMLDivElement>) {
  return (
    <div className={className} {...props}>
      <div
        style={{
          width: size + 'px',
          aspectRatio: '2',
          background: `
            radial-gradient(circle closest-side, ${colors[0]} 90%, transparent) 0% 50%,
            radial-gradient(circle closest-side, ${colors[1]} 90%, transparent) 50% 50%,
            radial-gradient(circle closest-side, ${colors[2]} 90%, transparent) 100% 50%
          `,
          backgroundSize: 'calc(100%/3) 50%',
          backgroundRepeat: 'no-repeat',
          animation: 'dots-loading 1s infinite linear',
        }}
      />
    </div>
  );
}