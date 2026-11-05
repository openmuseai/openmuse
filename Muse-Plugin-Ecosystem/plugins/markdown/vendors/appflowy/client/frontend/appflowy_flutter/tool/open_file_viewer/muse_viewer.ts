import { createViewer } from '../../../../../../../vendors/open-file-viewer/packages/core/src/viewer.ts';
import type { FileViewer, PreviewPlugin } from '../../../../../../../vendors/open-file-viewer/packages/core/src/types.ts';
import '../../../../../../../vendors/open-file-viewer/packages/core/src/style.css';

declare global {
  interface Window {
    MuseViewer: { open(input: MuseOpenInput): Promise<void> };
    MuseViewerBridge?: { postMessage(value: string): void };
    // Windows embeds the viewer in Edge WebView2 instead of a Flutter web view:
    // there is no JavaScript channel, messages travel through the WebView2 host
    // object (`chrome.webview`) in both directions.
    chrome?: {
      webview?: {
        postMessage(value: unknown): void;
        addEventListener?(
          type: 'message',
          listener: (event: { data: MuseOpenInput }) => void,
        ): void;
      };
    };
  }
}

interface MuseOpenInput {
  name: string;
  mime: string;
  base64?: string;
  url?: string;
  line?: number;
  requestId?: string;
}

const root = document.querySelector<HTMLElement>('#viewer');
if (!root) throw new Error('viewer root missing');
let viewer: FileViewer | undefined;
let activeKind: ViewerKind | undefined;
let openGeneration = 0;

const hostView = window.chrome?.webview;

const reply = (value: Record<string, unknown>) => {
  const payload = JSON.stringify(value);
  if (window.MuseViewerBridge) {
    window.MuseViewerBridge.postMessage(payload);
    return;
  }
  hostView?.postMessage(payload);
};

const decode = (value: string): Uint8Array => {
  const raw = atob(value);
  const bytes = new Uint8Array(raw.length);
  for (let i = 0; i < raw.length; i += 1) bytes[i] = raw.charCodeAt(i);
  return bytes;
};

type ViewerKind = 'image' | 'pdf' | 'text' | 'audio' | 'video' | 'office' | 'archive' | 'epub' | 'xps';

const extensionOf = (name: string): string => name.split(/[?#]/, 1)[0]?.split('.').pop()?.toLowerCase() || '';

const viewerKind = (input: MuseOpenInput): ViewerKind => {
  const extension = extensionOf(input.name);
  if (input.mime.startsWith('image/') || ['avif', 'bmp', 'gif', 'ico', 'jpeg', 'jpg', 'png', 'svg', 'tif', 'tiff', 'webp'].includes(extension)) return 'image';
  if (input.mime === 'application/pdf' || extension === 'pdf') return 'pdf';
  if (input.mime.startsWith('audio/') || ['aac', 'aiff', 'flac', 'm4a', 'mp3', 'ogg', 'wav'].includes(extension)) return 'audio';
  if (input.mime.startsWith('video/') || ['3gp', 'avi', 'm4v', 'mkv', 'mov', 'mp4', 'webm'].includes(extension)) return 'video';
  if (['doc', 'docm', 'docx', 'odp', 'ods', 'odt', 'ppt', 'pptx', 'rtf', 'xls', 'xlsx'].includes(extension)) return 'office';
  if (extension === 'epub') return 'epub';
  if (extension === 'xps') return 'xps';
  if (['zip', 'rar', '7z', 'tar', 'gz'].includes(extension)) return 'archive';
  return 'text';
};

const pdfOptions = {
  workerSrc: './pdf.worker.mjs',
  cMapUrl: './cmaps/',
  standardFontDataUrl: './standard_fonts/',
  webFallbackScripts: 'never' as const,
  useSystemFonts: true,
};

const pluginsFor = async (kind: ViewerKind): Promise<PreviewPlugin[]> => {
  switch (kind) {
    case 'image': return [(await import('../../../../../../../vendors/open-file-viewer/packages/core/src/plugins/image.ts')).imagePlugin()];
    case 'pdf': return [(await import('../../../../../../../vendors/open-file-viewer/packages/core/src/plugins/pdf.ts')).pdfPlugin(pdfOptions)];
    case 'audio': return [(await import('../../../../../../../vendors/open-file-viewer/packages/core/src/plugins/audio.ts')).audioPlugin()];
    case 'video': return [(await import('../../../../../../../vendors/open-file-viewer/packages/core/src/plugins/video.ts')).videoPlugin()];
    case 'office': return [(await import('../../../../../../../vendors/open-file-viewer/packages/core/src/plugins/office.ts')).officePlugin({ pdf: pdfOptions })];
    case 'archive': return [(await import('../../../../../../../vendors/open-file-viewer/packages/core/src/plugins/archive.ts')).archivePlugin()];
    case 'epub': return [(await import('../../../../../../../vendors/open-file-viewer/packages/core/src/plugins/epub.ts')).epubPlugin()];
    case 'xps': return [(await import('../../../../../../../vendors/open-file-viewer/packages/core/src/plugins/xps.ts')).xpsPlugin()];
    case 'text': return [(await import('../../../../../../../vendors/open-file-viewer/packages/core/src/plugins/text.ts')).textPlugin()];
  }
};

window.MuseViewer = {
  async open(input) {
    const generation = ++openGeneration;
    try {
      reply({ type: 'viewer.resource-accepted', requestId: input.requestId });
      const source = input.url ?? (input.base64 !== undefined
        ? new File([decode(input.base64)], input.name, { type: input.mime })
        : undefined);
      if (source === undefined) throw new Error('viewer resource has no transport');
      const kind = viewerKind(input);
      if (viewer && activeKind === kind) {
        await viewer.reload(source);
        if (generation !== openGeneration) return;
        if (input.line !== undefined) viewer.goToPage(input.line);
        return;
      }
      const plugins = await pluginsFor(kind);
      if (generation !== openGeneration) return;
      viewer?.destroy();
      activeKind = kind;
      viewer = createViewer({
        container: root,
        file: source,
        fileName: input.name,
        mimeType: input.mime,
        width: '100%',
        height: '100%',
        fit: 'contain',
        theme: 'dark',
        toolbar: true,
        initialPage: input.line,
        plugins,
        onLoad() { reply({ type: 'viewer.ready', engine: 'open-file-viewer', requestId: input.requestId }); },
        onError(error) { reply({ type: 'viewer.error', message: error.message, requestId: input.requestId }); },
      });
    } catch (error) {
      reply({ type: 'viewer.error', message: error instanceof Error ? error.message : String(error) });
    }
  },
};

window.addEventListener('beforeunload', () => viewer?.destroy());

hostView?.addEventListener?.('message', (event) => {
  void window.MuseViewer.open(event.data);
});

reply({ type: 'viewer.shell-ready', engine: 'open-file-viewer' });
