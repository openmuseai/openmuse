/** Public, declarative portal contract. No account or service credentials. */
export const regionCookieName = 'openmuse.region.v1';
export function linkUrl(snapshot, id, locale) {
  const link = snapshot.links[id];
  if (!link || !snapshot.origins[link.originRef]) throw Error('Missing portal link: ' + id);
  const path = link.pathByLocale[locale] ?? link.pathByLocale[link.fallbackLocale];
  if (
    typeof path !== 'string' ||
    !path.startsWith('/') ||
    path.startsWith('//') ||
    /[\\?#\x00-\x20]/.test(path) ||
    path.split('/').some((p) => {
      const decoded = decodeURIComponent(p);
      return ['.', '..'].includes(decoded) || /[/%\\?#\x00-\x20]/.test(decoded);
    })
  )
    throw Error('Unsafe portal link path');
  const target = new URL(path, snapshot.origins[link.originRef]);
  if (target.protocol !== 'https:' || target.username || target.password || target.search || target.hash)
    throw Error('Invalid portal link URL');
  return target.toString();
}
export function validatePortal(snapshot, baseline = snapshot) {
  const portal = snapshot.portal;
  if (!portal && !baseline.portal) return;
  if (!portal || portal.schemaVersion !== 1 || !portal.regions || !portal.regions[portal.defaultRegion])
    throw Error('Invalid portal directory');
  if (baseline.portal && portal.defaultRegion !== baseline.portal.defaultRegion)
    throw Error('Portal default region is installed');
  const regions = Object.entries(portal.regions);
  if (
    !regions.length ||
    regions.length > 8 ||
    Object.keys(portal).some((k) => !['schemaVersion', 'defaultRegion', 'regions'].includes(k))
  )
    throw Error('Invalid portal fields');
  const keys = new Set(Object.keys(baseline.portal?.regions ?? {}));
  if (regions.length !== keys.size) throw Error('Portal region directory is installed');
  for (const [id, region] of regions) {
    const installed = baseline.portal.regions[id];
    if (
      !keys.has(id) ||
      region.dataRegion !== installed.dataRegion ||
      !region.labels ||
      !region.media ||
      !Array.isArray(region.downloads) ||
      region.downloads.length > 32 ||
      Object.keys(region).some(
        (k) => !['dataRegion', 'labels', 'portalLinkId', 'appLinkId', 'media', 'downloads'].includes(k)
      )
    )
      throw Error('Invalid portal region');
    for (const label of Object.values(region.labels))
      if (typeof label !== 'string' || !label.trim() || label.length > 100 || /[<>\x00-\x1f]/.test(label))
        throw Error('Invalid portal label');
    for (const key of ['portalLinkId', 'appLinkId']) {
      for (const locale of snapshot.localization.enabledLocales) {
        const target = new URL(linkUrl(snapshot, region[key], locale));
        const trusted = new URL(linkUrl(baseline, installed[key], locale));
        if (target.origin !== trusted.origin) throw Error('Portal region origin cannot change');
        if (key === 'appLinkId' && target.pathname !== trusted.pathname) throw Error('Portal app mount cannot change');
      }
    }
    if (Object.keys(installed.media).some((slot) => !region.media[slot])) throw Error('Required portal media missing');
    for (const [slot, media] of Object.entries(region.media)) {
      if (installed.media[slot] && media.kind !== installed.media[slot].kind)
        throw Error('Installed media kind cannot change');
      if (
        !['image', 'video'].includes(media.kind) ||
        !media.byLocale?.[media.fallbackLocale] ||
        !snapshot.localization.enabledLocales.includes(media.fallbackLocale) ||
        Object.keys(media).some((k) => !['kind', 'fallbackLocale', 'byLocale'].includes(k))
      )
        throw Error('Invalid portal media');
      for (const [locale, variant] of Object.entries(media.byLocale)) {
        if (
          !snapshot.localization.enabledLocales.includes(locale) ||
          Object.keys(variant).some((k) => !['linkId', 'posterLinkId'].includes(k))
        )
          throw Error('Invalid media language');
        for (const key of ['linkId', ...(media.kind === 'video' ? ['posterLinkId'] : [])]) {
          const url = new URL(linkUrl(snapshot, variant[key], locale));
          if (
            url.protocol !== 'https:' ||
            url.username ||
            url.password ||
            url.search ||
            url.hash ||
            !(
              key === 'posterLinkId' || media.kind === 'image' ? /\.(?:png|jpe?g|webp|avif)$/i : /\.(?:mp4|webm)$/i
            ).test(url.pathname)
          )
            throw Error('Unsupported public media URL');
        }
      }
    }
    const platforms = new Set();
    for (const download of region.downloads) {
      if (
        !/^(macos|windows|linux|android|ios)(-[a-z0-9]+)?$/.test(download.platform) ||
        platforms.has(download.platform) ||
        typeof download.version !== 'string' ||
        !download.version.trim() ||
        download.version.length > 100 ||
        !/^[a-f0-9]{64}$/.test(download.sha256) ||
        Object.keys(download).some((k) => !['platform', 'version', 'sha256', 'linkId'].includes(k))
      )
        throw Error('Invalid regional download');
      platforms.add(download.platform);
      for (const locale of snapshot.localization.enabledLocales) {
        const url = new URL(linkUrl(snapshot, download.linkId, locale));
        if (url.protocol !== 'https:' || url.username || url.password || url.search || url.hash)
          throw Error('Invalid download URL');
      }
    }
  }
}
export function resolvePortalRegion(snapshot, desired, deploymentRegion) {
  const directory = snapshot.portal?.regions;
  if (!directory) return null;
  if (desired && Object.hasOwn(directory, desired)) return desired;
  if (deploymentRegion) {
    if (!Object.hasOwn(directory, deploymentRegion)) throw Error('Unknown portal deployment region');
    return deploymentRegion;
  }
  return snapshot.portal.defaultRegion;
}
export function portalPresentation(snapshot, locale, regionId) {
  if (!snapshot.portal) return null;
  const region = snapshot.portal.regions[regionId];
  if (!region) throw Error('Portal region unavailable');
  return {
    regionId,
    dataRegion: region.dataRegion,
    regions: Object.entries(snapshot.portal.regions).map(([id, value]) => ({
      id,
      label: value.labels[locale] ?? value.labels[snapshot.localization.defaultLocale] ?? Object.values(value.labels)[0],
    })),
    portalUrl: linkUrl(snapshot, region.portalLinkId, locale),
    webApplication: linkUrl(snapshot, region.appLinkId, locale),
    media: Object.fromEntries(
      Object.entries(region.media).map(([slot, value]) => {
        const variant = value.byLocale[locale] ?? value.byLocale[value.fallbackLocale];
        const localMedia = (id) => {
          const target = new URL(linkUrl(snapshot, id, locale));
          return snapshot.links[id].originRef === snapshot.links[region.portalLinkId].originRef
            ? target.pathname
            : target.toString();
        };
        return [
          slot,
          {
            kind: value.kind,
            src: localMedia(variant.linkId),
            ...(variant.posterLinkId ? { poster: localMedia(variant.posterLinkId) } : {}),
          },
        ];
      })
    ),
    downloads: region.downloads.map((value) => ({ ...value, url: linkUrl(snapshot, value.linkId, locale) })),
  };
}
