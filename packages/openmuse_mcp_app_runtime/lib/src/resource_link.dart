import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

/// Link text is a locator, never an authorization grant. The installed resource
/// source and live cloud account authorize discovery and content again.
ResourceReference checkedAppResourceLink(Object? value, String account) {
  if (value is! String) throw StateError('APP_RESOURCE_LINK_INVALID');
  final reference = ResourceReference.fromUri(value);
  if (account == 'signed-out-local' || reference.identity.scope != account) {
    throw StateError('APP_RESOURCE_ACCOUNT_MISMATCH');
  }
  if (reference.retained != null) {
    throw StateError('APP_RESOURCE_RETAINED_LINK_UNSUPPORTED');
  }
  return reference;
}
