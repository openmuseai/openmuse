import { CircularProgress } from '@mui/material';
import Typography from '@mui/material/Typography';
import debounce from 'lodash-es/debounce';
import React, { useCallback, useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { createApi } from 'unsplash-js';

import { Input } from '@/components/ui/input';

const unsplash = createApi({
  accessKey: '',
});

const SEARCH_DEBOUNCE_TIME = 500;

export function Unsplash({ onDone, onEscape }: { onDone?: (value: string) => void; onEscape?: () => void }) {
  const { t } = useTranslation();

  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string>('');
  const [photos, setPhotos] = useState<
    {
      thumb: string;
      full: string;
      alt: string | null;
      id: string;
      user: {
        name: string;
        link: string;
      };
    }[]
  >([]);
  const [searchValue, setSearchValue] = useState('');

  const handleChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;

    setSearchValue(value);
  }, []);

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      if (e.key === 'Escape') {
        onEscape?.();
      }
    },
    [onEscape]
  );

  const loadPhotos = useCallback(async (searchValue: string) => {
    const pages = 1;
    const perPage = 18;
    const requests = Array.from({ length: pages }, (_, i) =>
      searchValue
        ? unsplash.search.getPhotos({
            query: searchValue,
            perPage,
            page: i + 1,
          })
        : unsplash.photos.list({ perPage, page: i + 1 })
    );

    setError('');
    setLoading(true);

    const results = await Promise.all(requests);

    setLoading(false);

    const photos = results.flatMap((result) => {
      if (result.errors) {
        setError(result.errors[0]);
        return [];
      }

      return result.response.results.map((photo) => ({
        id: photo.id,
        thumb: photo.urls.thumb,
        full: photo.urls.full,
        alt: photo.alt_description,
        user: {
          name: photo.user.name,
          link: photo.user.links.html,
        },
      }));
    });

    setPhotos(photos);

    return photos;
  }, []);

  const debounceSearchPhotos = useMemo(() => {
    return debounce(loadPhotos, SEARCH_DEBOUNCE_TIME);
  }, [loadPhotos]);

  useEffect(() => {
    void debounceSearchPhotos(searchValue);
    return () => {
      debounceSearchPhotos.cancel();
    };
  }, [debounceSearchPhotos, searchValue]);

  return (
    <div tabIndex={0} onKeyDown={handleKeyDown} className={'flex h-fit max-w-[600px] flex-col gap-2'}>
      <Input
        variant={'default'}
        autoFocus
        onKeyDown={handleKeyDown}
        spellCheck={false}
        onChange={handleChange}
        value={searchValue}
        className={'w-full'}
        placeholder={t('document.imageBlock.searchForAnImage')}
      />

      {loading ? (
        <div className={'flex h-[120px] w-full items-center justify-center gap-2 text-xs'}>
          <CircularProgress size={24} />
          <div className={'text-xs text-text-secondary'}>{t('editor.loading')}</div>
        </div>
      ) : error ? (
        <Typography className={'flex h-[120px] w-full items-center justify-center gap-2 text-xs text-function-error'}>
          {error}
        </Typography>
      ) : (
        <div className={'flex flex-col gap-4'}>
          {photos.length > 0 ? (
            <>
              <div className={`grid w-full grid-cols-3 gap-2`}>
                {photos.map((photo) => (
                  <div key={photo.id + photo.full} className={'relative pt-[56.25%]'}>
                    <img
                      onClick={() => {
                        onDone?.(photo.full);
                      }}
                      src={photo.thumb}
                      alt={photo.alt ?? ''}
                      className={`absolute left-0 top-0 h-full w-[128px] cursor-pointer rounded object-cover transition-opacity hover:opacity-80`}
                    />
                  </div>
                ))}
              </div>
              <Typography className={'w-full text-center text-xs text-text-secondary'}>
                {t('findAndReplace.searchMore')}
              </Typography>
            </>
          ) : (
            <Typography
              className={'flex h-[120px] w-full items-center justify-center gap-2 text-xs text-text-secondary'}
            >
              {t('findAndReplace.noResult')}
            </Typography>
          )}
        </div>
      )}
    </div>
  );
}
