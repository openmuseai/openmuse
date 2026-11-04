import 'dsh_navigation_stub.dart'
    if (dart.library.js_interop) 'dsh_navigation_web.dart'
    as platform;

void openDshInCurrentTab() => platform.openDshInCurrentTab();
