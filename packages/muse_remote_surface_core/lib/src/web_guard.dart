final class RemoteWebGuard {
  static const virtualOrigin = 'https://surface.openmuse.invalid';

  static bool allowsNavigation(Uri uri) {
    if (uri.scheme != 'https' || uri.host.isEmpty || uri.userInfo.isNotEmpty) {
      return false;
    }
    final host = uri.host.toLowerCase();
    if (host == 'localhost' || host == '127.0.0.1' || host == '::1') {
      return false;
    }
    return uri.origin == virtualOrigin &&
        (uri.path == '/' || uri.path.startsWith('/surface/'));
  }

  static bool allowsCookie(String? cookie) => false;
}
