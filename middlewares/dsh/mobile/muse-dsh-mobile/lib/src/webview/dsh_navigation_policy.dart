import 'package:muse_dsh_mobile/src/dsh_remote_config.dart';

class DshNavigationPolicy {
  const DshNavigationPolicy(this.config);

  final DshRemoteConfig config;

  static const blockedSchemes = {
    'file',
    'content',
    'intent',
    'javascript',
    'data',
    'about',
  };

  bool allows(Uri uri) {
    if (blockedSchemes.contains(uri.scheme)) return false;
    if (uri.scheme == 'blob') return false;
    return config.allows(uri);
  }
}
