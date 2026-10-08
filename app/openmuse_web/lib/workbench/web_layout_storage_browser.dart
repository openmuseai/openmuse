import 'package:web/web.dart' as web;

String _key(String scope) =>
    'openmuse.workbench.v2.${Uri.encodeComponent(scope)}';

String? readWebLayout(String scope) {
  try {
    return web.window.localStorage.getItem(_key(scope));
  } on Object {
    return null;
  }
}

void writeWebLayout(String scope, String value) {
  try {
    web.window.localStorage.setItem(_key(scope), value);
  } on Object {
    // Storage may be disabled by the browser; layout remains usable in memory.
  }
}
