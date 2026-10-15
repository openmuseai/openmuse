import 'package:flutter/widgets.dart';

import '../l10n/openmuse_auth_localizations.dart';

export '../l10n/openmuse_auth_localizations.dart';

/// Resolves the auth localizations for [context].
///
/// The OpenMuse apps register [OpenMuseAuthLocalizations.delegate], but the
/// sign-in page is also rendered by hosts and widget tests that pump it on
/// their own. Fall back to the template locale instead of failing when the
/// delegate is absent.
OpenMuseAuthLocalizations openMuseAuthLocalizations(BuildContext context) =>
    OpenMuseAuthLocalizations.of(context) ??
    lookupOpenMuseAuthLocalizations(const Locale('en'));

/// App-wide UI language. Login pages default to Chinese and can switch to
/// English without leaving the screen.
final class OpenMuseUiLocale extends ChangeNotifier {
  OpenMuseUiLocale({Locale initial = const Locale('zh')}) : locale = initial;

  Locale locale;

  void select(Locale value) {
    if (locale == value) return;
    locale = value;
    notifyListeners();
  }
}

final class OpenMuseUiLocaleScope extends InheritedNotifier<OpenMuseUiLocale> {
  const OpenMuseUiLocaleScope({
    super.key,
    required OpenMuseUiLocale controller,
    required super.child,
  }) : super(notifier: controller);

  static OpenMuseUiLocale? maybeOf(BuildContext context) => context
      .dependOnInheritedWidgetOfExactType<OpenMuseUiLocaleScope>()
      ?.notifier;
}

/// Maps an authentication failure code to a localized message.
///
/// The controller has no [BuildContext], so it reports a stable code together
/// with an English fallback. Prefer the localized message and keep the fallback
/// for codes this version does not recognize.
String? openMuseAuthFailureMessage(
  BuildContext context, {
  required String? code,
  required String? fallback,
}) {
  final localizations = openMuseAuthLocalizations(context);
  final message = switch (code) {
    'invalid_credentials' => localizations.errorInvalidCredentials,
    'invalid_response' => localizations.errorInvalidResponse,
    'session_expired' => localizations.errorSessionExpired,
    'network' => localizations.errorNetwork,
    'server' => localizations.errorServer,
    'bootstrap' => localizations.errorBootstrap,
    'invalid_configuration' => localizations.errorInvalidConfiguration,
    'invalid_code' => localizations.errorInvalidCode,
    'account_exists' => localizations.errorAccountExists,
    'weak_password' => localizations.errorWeakPassword,
    'rate_limited' => localizations.errorRateLimited,
    _ => null,
  };
  return message ?? fallback;
}
