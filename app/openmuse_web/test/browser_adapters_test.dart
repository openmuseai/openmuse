import 'browser_adapters_stub.dart'
    if (dart.library.js_interop) 'browser_adapters_web.dart'
    as platform;

void main() => platform.runBrowserAdapterTests();
