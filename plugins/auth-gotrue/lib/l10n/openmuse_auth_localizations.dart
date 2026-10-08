import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:intl/intl.dart' as intl;

import 'openmuse_auth_localizations_en.dart';
import 'openmuse_auth_localizations_zh.dart';

// ignore_for_file: type=lint

/// Callers can lookup localized strings with an instance of OpenMuseAuthLocalizations
/// returned by `OpenMuseAuthLocalizations.of(context)`.
///
/// Applications need to include `OpenMuseAuthLocalizations.delegate()` in their app's
/// `localizationDelegates` list, and the locales they support in the app's
/// `supportedLocales` list. For example:
///
/// ```dart
/// import 'l10n/openmuse_auth_localizations.dart';
///
/// return MaterialApp(
///   localizationsDelegates: OpenMuseAuthLocalizations.localizationsDelegates,
///   supportedLocales: OpenMuseAuthLocalizations.supportedLocales,
///   home: MyApplicationHome(),
/// );
/// ```
///
/// ## Update pubspec.yaml
///
/// Please make sure to update your pubspec.yaml to include the following
/// packages:
///
/// ```yaml
/// dependencies:
///   # Internationalization support.
///   flutter_localizations:
///     sdk: flutter
///   intl: any # Use the pinned version from flutter_localizations
///
///   # Rest of dependencies
/// ```
///
/// ## iOS Applications
///
/// iOS applications define key application metadata, including supported
/// locales, in an Info.plist file that is built into the application bundle.
/// To configure the locales supported by your app, you’ll need to edit this
/// file.
///
/// First, open your project’s ios/Runner.xcworkspace Xcode workspace file.
/// Then, in the Project Navigator, open the Info.plist file under the Runner
/// project’s Runner folder.
///
/// Next, select the Information Property List item, select Add Item from the
/// Editor menu, then select Localizations from the pop-up menu.
///
/// Select and expand the newly-created Localizations item then, for each
/// locale your application supports, add a new item and select the locale
/// you wish to add from the pop-up menu in the Value field. This list should
/// be consistent with the languages listed in the OpenMuseAuthLocalizations.supportedLocales
/// property.
abstract class OpenMuseAuthLocalizations {
  OpenMuseAuthLocalizations(String locale)
    : localeName = intl.Intl.canonicalizedLocale(locale.toString());

  final String localeName;

  static OpenMuseAuthLocalizations? of(BuildContext context) {
    return Localizations.of<OpenMuseAuthLocalizations>(
      context,
      OpenMuseAuthLocalizations,
    );
  }

  static const LocalizationsDelegate<OpenMuseAuthLocalizations> delegate =
      _OpenMuseAuthLocalizationsDelegate();

  /// A list of this localizations delegate along with the default localizations
  /// delegates.
  ///
  /// Returns a list of localizations delegates containing this delegate along with
  /// GlobalMaterialLocalizations.delegate, GlobalCupertinoLocalizations.delegate,
  /// and GlobalWidgetsLocalizations.delegate.
  ///
  /// Additional delegates can be added by appending to this list in
  /// MaterialApp. This list does not have to be used at all if a custom list
  /// of delegates is preferred or required.
  static const List<LocalizationsDelegate<dynamic>> localizationsDelegates =
      <LocalizationsDelegate<dynamic>>[
        delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
      ];

  /// A list of this localizations delegate's supported locales.
  static const List<Locale> supportedLocales = <Locale>[
    Locale('en'),
    Locale('zh'),
  ];

  /// Headline on the email step of the sign-in page.
  ///
  /// In en, this message translates to:
  /// **'Welcome to OpenMuse'**
  String get welcomeTitle;

  /// Shows which cloud endpoint the sign-in page will authenticate against.
  ///
  /// In en, this message translates to:
  /// **'Cloud: {label}'**
  String cloudLabel(String label);

  /// Placeholder inside the email field.
  ///
  /// In en, this message translates to:
  /// **'Please enter your email'**
  String get emailHint;

  /// Primary button that requests a one-time sign-in code by email.
  ///
  /// In en, this message translates to:
  /// **'Continue with email'**
  String get continueWithEmail;

  /// Secondary button that switches to the password step.
  ///
  /// In en, this message translates to:
  /// **'Continue with password'**
  String get continueWithPassword;

  /// Validation error shown under the email field.
  ///
  /// In en, this message translates to:
  /// **'Please enter a valid email address.'**
  String get invalidEmail;

  /// Headline on the passcode step.
  ///
  /// In en, this message translates to:
  /// **'Check your email'**
  String get checkYourEmailTitle;

  /// Explains where the one-time code was sent.
  ///
  /// In en, this message translates to:
  /// **'We sent a sign-in code to {email}.'**
  String signInCodeSent(String email);

  /// Placeholder inside the one-time code field.
  ///
  /// In en, this message translates to:
  /// **'Enter passcode'**
  String get passcodeHint;

  /// Label of the submit button while a request is in flight.
  ///
  /// In en, this message translates to:
  /// **'Verifying…'**
  String get verifying;

  /// Label of the primary submit button when idle.
  ///
  /// In en, this message translates to:
  /// **'Continue'**
  String get continueAction;

  /// Returns from the password or passcode step to the email step.
  ///
  /// In en, this message translates to:
  /// **'Back to login'**
  String get backToLogin;

  /// Headline on the password step.
  ///
  /// In en, this message translates to:
  /// **'Enter password'**
  String get enterPasswordTitle;

  /// Placeholder inside the password field.
  ///
  /// In en, this message translates to:
  /// **'Enter password'**
  String get passwordHint;

  /// Precedes the email address on the password step.
  ///
  /// In en, this message translates to:
  /// **'Login as'**
  String get loginAsLabel;

  /// Opens email code password recovery.
  ///
  /// In en, this message translates to:
  /// **'Forgot password?'**
  String get forgotPassword;

  /// No description provided for @createAccount.
  ///
  /// In en, this message translates to:
  /// **'Create account'**
  String get createAccount;

  /// No description provided for @createAccountTitle.
  ///
  /// In en, this message translates to:
  /// **'Create your account'**
  String get createAccountTitle;

  /// No description provided for @signUpCodeSent.
  ///
  /// In en, this message translates to:
  /// **'Enter the verification code sent to {email} to confirm your account.'**
  String signUpCodeSent(String email);

  /// No description provided for @recoveryCodeSent.
  ///
  /// In en, this message translates to:
  /// **'Enter the recovery code sent to {email}.'**
  String recoveryCodeSent(String email);

  /// No description provided for @resendCode.
  ///
  /// In en, this message translates to:
  /// **'Resend code'**
  String get resendCode;

  /// No description provided for @resendCodeCountdown.
  ///
  /// In en, this message translates to:
  /// **'Resend code in {seconds}s'**
  String resendCodeCountdown(int seconds);

  /// No description provided for @resetPasswordTitle.
  ///
  /// In en, this message translates to:
  /// **'Reset your password'**
  String get resetPasswordTitle;

  /// No description provided for @newPassword.
  ///
  /// In en, this message translates to:
  /// **'New password'**
  String get newPassword;

  /// No description provided for @confirmPassword.
  ///
  /// In en, this message translates to:
  /// **'Confirm password'**
  String get confirmPassword;

  /// No description provided for @setNewPassword.
  ///
  /// In en, this message translates to:
  /// **'Set new password'**
  String get setNewPassword;

  /// No description provided for @passwordTooShort.
  ///
  /// In en, this message translates to:
  /// **'Use at least 8 characters.'**
  String get passwordTooShort;

  /// No description provided for @passwordMismatch.
  ///
  /// In en, this message translates to:
  /// **'Passwords do not match.'**
  String get passwordMismatch;

  /// Leading text of the terms sentence. Keep the trailing space.
  ///
  /// In en, this message translates to:
  /// **'By continuing, you agree to our '**
  String get agreementPrefix;

  /// Linked segment of the terms sentence.
  ///
  /// In en, this message translates to:
  /// **'Terms of Service'**
  String get termsOfService;

  /// Joins the two linked segments. Keep the surrounding spaces.
  ///
  /// In en, this message translates to:
  /// **' and '**
  String get agreementConjunction;

  /// Second linked segment of the terms sentence.
  ///
  /// In en, this message translates to:
  /// **'Privacy Policy'**
  String get privacyPolicy;

  /// Ends the terms sentence.
  ///
  /// In en, this message translates to:
  /// **'.'**
  String get agreementSuffix;

  /// Footer button that opens host settings.
  ///
  /// In en, this message translates to:
  /// **'Settings'**
  String get settings;

  /// Footer button that skips sign-in.
  ///
  /// In en, this message translates to:
  /// **'Anonymous mode'**
  String get anonymousMode;

  /// Heading of the account section in host settings.
  ///
  /// In en, this message translates to:
  /// **'Account'**
  String get accountSectionTitle;

  /// Shown in host settings when there is no identity.
  ///
  /// In en, this message translates to:
  /// **'Not signed in'**
  String get notSignedIn;

  /// Signs the current identity out from host settings.
  ///
  /// In en, this message translates to:
  /// **'Sign out'**
  String get signOut;

  /// Failure message for the invalid_credentials code.
  ///
  /// In en, this message translates to:
  /// **'The email or password is incorrect.'**
  String get errorInvalidCredentials;

  /// Failure message for the invalid_response code.
  ///
  /// In en, this message translates to:
  /// **'The authentication service returned an unexpected response.'**
  String get errorInvalidResponse;

  /// Failure message for the session_expired code.
  ///
  /// In en, this message translates to:
  /// **'Your session has expired. Please sign in again.'**
  String get errorSessionExpired;

  /// Failure message for the network code.
  ///
  /// In en, this message translates to:
  /// **'Cannot reach the service. Check your connection and try again.'**
  String get errorNetwork;

  /// Failure message for the server code.
  ///
  /// In en, this message translates to:
  /// **'The service could not complete the request. Try again later.'**
  String get errorServer;

  /// Failure message for the bootstrap code.
  ///
  /// In en, this message translates to:
  /// **'The account workspace could not be initialized.'**
  String get errorBootstrap;

  /// Failure message for the invalid_configuration code.
  ///
  /// In en, this message translates to:
  /// **'The service endpoint is not configured correctly.'**
  String get errorInvalidConfiguration;

  /// No description provided for @errorInvalidCode.
  ///
  /// In en, this message translates to:
  /// **'The code is invalid or has expired.'**
  String get errorInvalidCode;

  /// No description provided for @errorAccountExists.
  ///
  /// In en, this message translates to:
  /// **'An account already exists for this email. Sign in instead.'**
  String get errorAccountExists;

  /// No description provided for @errorWeakPassword.
  ///
  /// In en, this message translates to:
  /// **'Please choose a stronger password.'**
  String get errorWeakPassword;

  /// No description provided for @errorRateLimited.
  ///
  /// In en, this message translates to:
  /// **'Too many requests. Please wait and try again.'**
  String get errorRateLimited;
}

class _OpenMuseAuthLocalizationsDelegate
    extends LocalizationsDelegate<OpenMuseAuthLocalizations> {
  const _OpenMuseAuthLocalizationsDelegate();

  @override
  Future<OpenMuseAuthLocalizations> load(Locale locale) {
    return SynchronousFuture<OpenMuseAuthLocalizations>(
      lookupOpenMuseAuthLocalizations(locale),
    );
  }

  @override
  bool isSupported(Locale locale) =>
      <String>['en', 'zh'].contains(locale.languageCode);

  @override
  bool shouldReload(_OpenMuseAuthLocalizationsDelegate old) => false;
}

OpenMuseAuthLocalizations lookupOpenMuseAuthLocalizations(Locale locale) {
  // Lookup logic when only language code is specified.
  switch (locale.languageCode) {
    case 'en':
      return OpenMuseAuthLocalizationsEn();
    case 'zh':
      return OpenMuseAuthLocalizationsZh();
  }

  throw FlutterError(
    'OpenMuseAuthLocalizations.delegate failed to load unsupported locale "$locale". This is likely '
    'an issue with the localizations generation tool. Please file an issue '
    'on GitHub with a reproducible sample app and the gen-l10n configuration '
    'that was used.',
  );
}
