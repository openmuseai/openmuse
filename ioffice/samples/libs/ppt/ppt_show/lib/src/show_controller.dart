import 'package:flutter/foundation.dart';
import 'package:ppt_render/ppt_render.dart';

/// Host-facing slide session: open/layout + chrome state.
class PptShowController extends ChangeNotifier {
  PptShowController({PptCoreEngine? engine})
      : engine = engine ?? PptCoreEngine();

  final PptCoreEngine engine;

  RenderEnvelope? envelope;
  Uint8List? _pptx;
  bool get hasDocument => _pptx != null;
  String documentName = 'Presentation';
  String status = 'idle';
  double zoom = 1.0;
  int pageIndex = 0;
  bool _alive = true;

  @override
  void dispose() {
    _alive = false;
    super.dispose();
  }

  void _notify() {
    if (!_alive) return;
    notifyListeners();
  }

  void report(String message) {
    status = message;
    _notify();
  }

  Future<void> init() async {
    status = 'init FFI…';
    _notify();
    try {
      await engine.initFfi();
      status = engine.ffiReady ? 'Ready' : engine.source;
    } catch (e) {
      status = 'FFI init error: $e';
    }
    _notify();
  }

  Future<void> openBytes(Uint8List bytes, {required String name}) async {
    if (!engine.ffiReady) {
      await engine.initFfi();
    }
    if (!engine.ffiReady) {
      documentName = name;
      envelope = null;
      _pptx = bytes;
      status =
          'FFI not ready — cannot parse $name. ${engine.source}. Run samples/ppt-render/scripts/build-macos.sh';
      _notify();
      return;
    }
    documentName = name;
    _pptx = bytes;
    status = 'PptxReader.open…';
    _notify();
    try {
      envelope = engine.renderPptx(bytes);
      pageIndex = envelope!.selectedPage.clamp(0, envelope!.pages.length - 1);
      if (envelope!.ok == false) {
        status = 'error: ${envelope!.error ?? "render failed"}';
      } else {
        status =
            '${envelope!.pageCount} pages · ${currentPage.commands.length} cmds · ${envelope!.pipeline}';
      }
    } catch (e) {
      envelope = null;
      status = 'error: $e';
    }
    _notify();
  }

  Future<void> renderHelloFixture({String name = 'hello fixture'}) async {
    documentName = name;
    _pptx = null;
    if (!engine.ffiReady) {
      status = 'FFI not ready: ${engine.source}';
      _notify();
      return;
    }
    try {
      envelope = engine.renderHello();
      pageIndex = 0;
      status =
          '${envelope!.pageCount} pages · ${currentPage.commands.length} cmds · ${envelope!.pipeline}';
    } catch (e) {
      envelope = null;
      status = 'error: $e';
    }
    _notify();
  }

  Future<void> showGolden(RenderEnvelope env, {required String name}) async {
    documentName = name;
    envelope = env;
    pageIndex = env.selectedPage.clamp(0, env.pages.length - 1);
    _pptx = null;
    status = '${env.commands.length} cmds · golden JSON';
    _notify();
  }

  void setZoom(double z) {
    zoom = z.clamp(0.25, 4.0);
    _notify();
  }

  RenderPage get currentPage => envelope!.pageAt(pageIndex);

  void setPage(int index) {
    final env = envelope;
    if (env == null || env.pages.isEmpty) return;
    pageIndex = index.clamp(0, env.pages.length - 1);
    status =
        'Page ${pageIndex + 1}/${env.pageCount} · ${currentPage.commands.length} cmds';
    _notify();
  }

  void previousPage() => setPage(pageIndex - 1);

  void nextPage() => setPage(pageIndex + 1);
}
