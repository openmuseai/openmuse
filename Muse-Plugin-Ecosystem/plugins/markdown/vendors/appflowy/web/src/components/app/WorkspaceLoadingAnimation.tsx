import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';

import logoSvg from '@/assets/icons/logo.svg';
import { useCurrentUser } from '@/components/main/app.hooks';

export function WorkspaceLoadingAnimation() {
  const { t } = useTranslation();
  const currentUser = useCurrentUser();
  const [progress, setProgress] = useState(0);
  const [rotation, setRotation] = useState(0);

  // Determine loading state based on available data
  const isLoadingUser = !currentUser;
  const targetProgress = isLoadingUser ? 25 : 75;

  useEffect(() => {
    const progressInterval = setInterval(() => {
      setProgress((prev) => {
        const diff = targetProgress - prev;

        if (Math.abs(diff) < 1) return targetProgress;
        return prev + diff * 0.1;
      });
    }, 50);

    const rotationInterval = setInterval(() => {
      setRotation((prev) => (prev + 2) % 360);
    }, 50);

    return () => {
      clearInterval(progressInterval);
      clearInterval(rotationInterval);
    };
  }, [targetProgress]);

  return (
    <div className='fixed inset-0 flex items-center justify-center bg-background-primary'>
      <div className='mx-auto flex max-w-md flex-col items-center justify-center px-8'>
        {/* Logo Animation */}
        <div className='relative mb-8'>
          <div 
            className='transition-all duration-1000 ease-out'
            style={{
              transform: `rotate(${rotation}deg) scale(${0.9 + (progress / 100) * 0.2})`,
            }}>
            <div className='relative'>
              <img src={logoSvg} alt='logo' width={37} height={37} />
              {/* Glow effect */}
              <div
                className='absolute inset-0 rounded-full bg-fill-theme-thick opacity-30 blur-xl'
                style={{
                  animation: `pulse 2s ease-in-out infinite`,
                  transform: `scale(${1.2 + (progress / 100) * 0.8})`,
                }}
              />
            </div>
          </div>

          {/* Circular progress ring */}
          <div className='absolute -inset-6 flex items-center justify-center'>
            <svg className='h-28 w-28 -rotate-90 transform' viewBox='0 0 100 100'>
              {/* Background ring */}
              <circle
                cx='50'
                cy='50'
                r='45'
                stroke='currentColor'
                strokeWidth='2'
                fill='none'
                className='text-border-primary opacity-20'
              />
              {/* Progress ring */}
              <circle
                cx='50'
                cy='50'
                r='45'
                stroke='currentColor'
                strokeWidth='2'
                fill='none'
                strokeDasharray={`${2 * Math.PI * 45}`}
                strokeDashoffset={`${2 * Math.PI * 45 * (1 - progress / 100)}`}
                className='text-fill-theme-thick transition-all duration-300 ease-out'
                strokeLinecap='round'
              />
            </svg>
          </div>
        </div>

        {/* Main title */}
        <div className='mb-4 text-center'>
          <h1 className='text-2xl font-bold text-text-primary transition-all duration-500 ease-out'>
            {t('global-loading.installing')}
          </h1>
        </div>

        {/* Simple status text */}
        <div className='mb-6 text-center'>
          <p className='text-sm text-text-secondary transition-all duration-300'>
            {isLoadingUser ? 'Loading user profile...' : 'Loading workspace data...'}
          </p>
        </div>

        {/* Progress percentage */}
        <div className='text-center'>
          <div className='text-xs text-text-tertiary'>
            {Math.round(progress)}%
          </div>
        </div>
      </div>

      {/* Background particles */}
      <div className='pointer-events-none absolute inset-0 overflow-hidden'>
        {[...Array(4)].map((_, i) => (
          <div
            key={i}
            className='absolute h-1 w-1 rounded-full bg-fill-theme-thick opacity-20'
            style={{
              left: `${25 + i * 20}%`,
              top: `${40 + (i % 2) * 20}%`,
              animation: `float 4s ease-in-out infinite ${i * 0.8}s`,
              animationDirection: i % 2 === 0 ? 'normal' : 'reverse',
            }}
          />
        ))}
      </div>
    </div>
  );
}
