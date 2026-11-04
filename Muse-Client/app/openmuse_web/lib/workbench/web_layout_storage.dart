import 'web_layout_storage_stub.dart'
    if (dart.library.js_interop) 'web_layout_storage_browser.dart'
    as platform;

String? readWebLayout(String scope) => platform.readWebLayout(scope);
void writeWebLayout(String scope, String value) =>
    platform.writeWebLayout(scope, value);
