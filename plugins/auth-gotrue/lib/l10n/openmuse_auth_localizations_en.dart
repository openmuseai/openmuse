// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'openmuse_auth_localizations.dart';

// ignore_for_file: type=lint

/// The translations for English (`en`).
class OpenMuseAuthLocalizationsEn extends OpenMuseAuthLocalizations {
  OpenMuseAuthLocalizationsEn([String locale = 'en']) : super(locale);

  @override
  String get welcomeTitle => 'Welcome to OpenMuse';

  @override
  String cloudLabel(String label) {
    return 'Cloud: $label';
  }

  @override
  String get emailHint => 'Please enter your email';

  @override
  String get continueWithEmail => 'Continue with email';

  @override
  String get continueWithPassword => 'Continue with password';

  @override
  String get invalidEmail => 'Please enter a valid email address.';

  @override
  String get checkYourEmailTitle => 'Check your email';

  @override
  String signInCodeSent(String email) {
    return 'We sent a sign-in code to $email.';
  }

  @override
  String get passcodeHint => 'Enter passcode';

  @override
  String get verifying => 'Verifying…';

  @override
  String get continueAction => 'Continue';

  @override
  String get backToLogin => 'Back to login';

  @override
  String get enterPasswordTitle => 'Enter password';

  @override
  String get passwordHint => 'Enter password';

  @override
  String get loginAsLabel => 'Login as';

  @override
  String get forgotPassword => 'Forgot password?';

  @override
  String get createAccount => 'Create account';

  @override
  String get createAccountTitle => 'Create your account';

  @override
  String signUpCodeSent(String email) {
    return 'Enter the verification code sent to $email to confirm your account.';
  }

  @override
  String recoveryCodeSent(String email) {
    return 'Enter the recovery code sent to $email.';
  }

  @override
  String get resendCode => 'Resend code';

  @override
  String resendCodeCountdown(int seconds) {
    return 'Resend code in ${seconds}s';
  }

  @override
  String get resetPasswordTitle => 'Reset your password';

  @override
  String get newPassword => 'New password';

  @override
  String get confirmPassword => 'Confirm password';

  @override
  String get setNewPassword => 'Set new password';

  @override
  String get passwordTooShort => 'Use at least 8 characters.';

  @override
  String get passwordMismatch => 'Passwords do not match.';

  @override
  String get agreementPrefix => 'By continuing, you agree to our ';

  @override
  String get termsOfService => 'Terms of Service';

  @override
  String get agreementConjunction => ' and ';

  @override
  String get privacyPolicy => 'Privacy Policy';

  @override
  String get agreementSuffix => '.';

  @override
  String get settings => 'Settings';

  @override
  String get anonymousMode => 'Anonymous mode';

  @override
  String get accountSectionTitle => 'Account';

  @override
  String get notSignedIn => 'Not signed in';

  @override
  String get signOut => 'Sign out';

  @override
  String get errorInvalidCredentials => 'The email or password is incorrect.';

  @override
  String get errorInvalidResponse =>
      'The authentication service returned an unexpected response.';

  @override
  String get errorSessionExpired =>
      'Your session has expired. Please sign in again.';

  @override
  String get errorNetwork =>
      'Cannot reach the service. Check your connection and try again.';

  @override
  String get errorServer =>
      'The service could not complete the request. Try again later.';

  @override
  String get errorBootstrap =>
      'The account workspace could not be initialized.';

  @override
  String get errorInvalidConfiguration =>
      'The service endpoint is not configured correctly.';

  @override
  String get errorInvalidCode => 'The code is invalid or has expired.';

  @override
  String get errorAccountExists =>
      'An account already exists for this email. Sign in instead.';

  @override
  String get errorWeakPassword => 'Please choose a stronger password.';

  @override
  String get errorRateLimited =>
      'Too many requests. Please wait and try again.';
}
