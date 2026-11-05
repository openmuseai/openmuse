// User-visible product brand.
//
// Display names and URLs come from brandValues, generated from
// brand/config.yaml by scripts/apply-brand.py.
// Edit the YAML (not this file) when the English or Chinese name changes.
import { brandValues } from '@/config/brand.generated';

export const brand = {
  productName: brandValues.productName,
  productNameZh: brandValues.productNameZh,
  siteUrl: brandValues.siteUrl,
  appPath: brandValues.appPath,
  termsUrl: brandValues.termsUrl,
  privacyUrl: brandValues.privacyUrl,
  downloadUrl: brandValues.downloadUrl,
  githubUrl: brandValues.githubUrl,
  githubIssuesUrl: brandValues.githubIssuesUrl,
  twitterHandle: brandValues.twitterHandle,
  discordUrl: brandValues.discordUrl,
  docsUrl: brandValues.docsUrl,
  whatsNewUrl: brandValues.whatsNewUrl,
  templatesUrl: brandValues.templatesUrl,
  forumUrl: brandValues.forumUrl,
  redditUrl: brandValues.redditUrl,
  desktopScheme: brandValues.desktopScheme,
  legacyDesktopScheme: brandValues.legacyDesktopScheme,
  description: brandValues.description,
} as const;

export function isConfiguredUrl(url: string | undefined | null): boolean {
  return typeof url === 'string' && url.trim().length > 0;
}

export function localizedProductName(language = navigator.language): string {
  if (language.toLowerCase().startsWith('zh')) {
    return brand.productNameZh;
  }
  return brand.productName;
}

/** `openmuse://open-page?...` from a path+query without a leading slash. */
export function desktopAppUrl(pathAndQuery: string): string {
  const scheme = brand.desktopScheme.endsWith('://')
    ? brand.desktopScheme
    : `${brand.desktopScheme}://`;
  return `${scheme}${pathAndQuery.replace(/^\//, '')}`;
}
