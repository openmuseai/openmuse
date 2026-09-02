# `@muse/dsh-mobile-surface`

Host-side DSH plugin. Injects Mobile layout CSS and a layout adapter that only walks from `[data-shell-overlay]` upward.

Web and desktop stay on the official DSH layout. The adapter is inert unless `window.MuseNativeCapability` exists (AppFlowy WebView). It then sets `data-muse-surface="mobile"` for CSS. A phone browser on the public DSH URL has no channel and stays official.

Mobile chrome: header hamburger / new-session call the official sidebar controls (`打开侧边栏` / `收起侧边栏` / `新建会话`). Token/timing rows under the composer and turn-tail clocks are hidden; copy/like actions stay.

Flag: `MUSE_DSH_MOBILE_SURFACE_ENABLED` (default on). Does not import Flutter or `@muse/dsh-appflowy` Host modules.
