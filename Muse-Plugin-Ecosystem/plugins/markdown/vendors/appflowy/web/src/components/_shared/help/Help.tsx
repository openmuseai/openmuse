import { Button, Divider, Portal, Tooltip } from '@mui/material';
import Box from '@mui/material/Box';
import { PopoverProps } from '@mui/material/Popover';
import * as React from 'react';
import { useContext } from 'react';
import { useTranslation } from 'react-i18next';

import { ReactComponent as BugIcon } from '@/assets/icons/bug.svg';
import { ReactComponent as FeedbackIcon } from '@/assets/icons/feedback.svg';
import { ReactComponent as SupportIcon } from '@/assets/icons/help.svg';
import { ReactComponent as DocumentationIcon } from '@/assets/icons/help_&_documentation.svg';
import { ReactComponent as MoonIcon } from '@/assets/icons/moon.svg';
import { ReactComponent as WhatsNewIcon } from '@/assets/icons/star.svg';
import { ReactComponent as SunIcon } from '@/assets/icons/sun.svg';
import { notify } from '@/components/_shared/notify';
import { Popover } from '@/components/_shared/popover';
import { ThemeModeContext } from '@/components/main/useAppThemeMode';
import { brand, isConfiguredUrl } from '@/config/brand';
import { copyTextToClipboard } from '@/utils/copy';

const popoverProps: Partial<PopoverProps> = {
  anchorOrigin: {
    vertical: 'top',
    horizontal: 'right',
  },
  transformOrigin: {
    vertical: 'bottom',
    horizontal: 'right',
  },
};

export default function Help() {
  const ref = React.useRef<HTMLDivElement | null>(null);
  const [open, setOpen] = React.useState(false);
  const { t } = useTranslation();
  const { isDark, setDark } = useContext(ThemeModeContext) || {};

  return (
    <Portal>
      <Box className={'fixed bottom-6 right-6'} sx={{ transform: 'translateZ(0px)', flexGrow: 1 }}>
        <Tooltip title={t('questionBubble.help')}>
          <div ref={ref} onClick={() => setOpen(!open)} className={'py-2'}>
            <div
              className={
                'flex h-9 w-9 cursor-pointer items-center justify-center rounded-full border border-border-primary bg-surface-primary shadow-md hover:bg-surface-primary-hover'
              }
            >
              <SupportIcon className={'h-5 w-5'} />
            </div>
          </div>
        </Tooltip>
        <Popover {...popoverProps} open={open} anchorEl={ref.current} onClose={() => setOpen(false)}>
          <div className={'flex h-fit w-[240px] flex-col gap-1 p-2'}>
            <Button
              color={'inherit'}
              variant={'text'}
              className={'justify-start'}
              startIcon={isDark ? <SunIcon /> : <MoonIcon />}
              onClick={() => setDark?.(!isDark)}
            >
              {isDark ? t('settings.appearance.themeMode.light') : t('settings.appearance.themeMode.dark')}
            </Button>
            {isConfiguredUrl(brand.whatsNewUrl) && (
              <Button
                component={'a'}
                target='_blank'
                href={brand.whatsNewUrl}
                className={'justify-start'}
                color={'inherit'}
                startIcon={<WhatsNewIcon />}
                variant={'text'}
              >
                {t('questionBubble.whatsNew')}
              </Button>
            )}
            {isConfiguredUrl(brand.docsUrl) && (
              <Button
                component={'a'}
                href={brand.docsUrl}
                className={'justify-start'}
                target='_blank'
                color={'inherit'}
                startIcon={<DocumentationIcon />}
                variant={'text'}
              >
                {t('questionBubble.documentation')}
              </Button>
            )}
            {isConfiguredUrl(brand.discordUrl) && (
              <Button
                component={'a'}
                href={brand.discordUrl}
                className={'justify-start'}
                target='_blank'
                color={'inherit'}
                startIcon={<SupportIcon />}
                variant={'text'}
              >
                {t('questionBubble.help')}
              </Button>
            )}
            <Button
              onClick={() => {
                const info = {
                  platform: 'web',
                  url: window.location.href,
                  userAgent: navigator.userAgent,
                };

                void copyTextToClipboard(JSON.stringify(info, null, 2));
                notify.success(t('questionBubble.debug.success'));
              }}
              className={'justify-start'}
              color={'inherit'}
              startIcon={<BugIcon />}
              variant={'text'}
            >
              {t('questionBubble.debug.name')}
            </Button>
            {isConfiguredUrl(brand.githubIssuesUrl) && (
              <Button
                component={'a'}
                target='_blank'
                href={brand.githubIssuesUrl}
                className={'justify-start'}
                color={'inherit'}
                startIcon={<FeedbackIcon />}
                variant={'text'}
              >
                {t('questionBubble.feedback')}
              </Button>
            )}

            {(isConfiguredUrl(brand.forumUrl) ||
              isConfiguredUrl(brand.twitterHandle) ||
              isConfiguredUrl(brand.redditUrl)) && <Divider />}
            {isConfiguredUrl(brand.forumUrl) && (
              <Button
                size={'small'}
                target='_blank'
                component={'a'}
                href={brand.forumUrl}
                className={'justify-start text-text-secondary'}
                color={'inherit'}
                variant={'text'}
              >
                Community Forum
              </Button>
            )}
            {isConfiguredUrl(brand.twitterHandle) && (
              <Button
                size={'small'}
                component={'a'}
                target='_blank'
                href={`https://x.com/${brand.twitterHandle}`}
                className={'justify-start text-text-secondary'}
                color={'inherit'}
                variant={'text'}
              >
                Twitter - @{brand.twitterHandle}
              </Button>
            )}
            {isConfiguredUrl(brand.redditUrl) && (
              <Button
                size={'small'}
                component={'a'}
                target='_blank'
                href={brand.redditUrl}
                className={'justify-start text-text-secondary'}
                color={'inherit'}
                variant={'text'}
              >
                Reddit
              </Button>
            )}
          </div>
        </Popover>
      </Box>
    </Portal>
  );
}
