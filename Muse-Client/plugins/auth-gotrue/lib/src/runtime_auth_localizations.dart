// GENERATED adapter from the auth ARB message contract.
import 'package:openmuse_i18n/openmuse_i18n.dart';
import '../l10n/openmuse_auth_localizations.dart';

final class RuntimeAuthLocalizations extends OpenMuseAuthLocalizations {
  RuntimeAuthLocalizations(this.strings)
    : super(localeFromTag(strings.locale).languageCode);
  final OpenMuseStrings strings;
  @override
  String get welcomeTitle => strings.authWelcomeTitle();
  @override
  String cloudLabel(String label) => strings.authCloudLabel(label: label);
  @override
  String get emailHint => strings.authEmailHint();
  @override
  String get continueWithEmail => strings.authContinueWithEmail();
  @override
  String get continueWithPassword => strings.authContinueWithPassword();
  @override
  String get invalidEmail => strings.authInvalidEmail();
  @override
  String get checkYourEmailTitle => strings.authCheckYourEmailTitle();
  @override
  String signInCodeSent(String email) =>
      strings.authSignInCodeSent(email: email);
  @override
  String get passcodeHint => strings.authPasscodeHint();
  @override
  String get verifying => strings.authVerifying();
  @override
  String get continueAction => strings.authContinueAction();
  @override
  String get backToLogin => strings.authBackToLogin();
  @override
  String get enterPasswordTitle => strings.authEnterPasswordTitle();
  @override
  String get passwordHint => strings.authPasswordHint();
  @override
  String get loginAsLabel => strings.authLoginAsLabel();
  @override
  String get forgotPassword => strings.authForgotPassword();
  @override
  String get createAccount => strings.authCreateAccount();
  @override
  String get createAccountTitle => strings.authCreateAccountTitle();
  @override
  String signUpCodeSent(String email) =>
      strings.authSignUpCodeSent(email: email);
  @override
  String recoveryCodeSent(String email) =>
      strings.authRecoveryCodeSent(email: email);
  @override
  String get resendCode => strings.authResendCode();
  @override
  String resendCodeCountdown(int seconds) =>
      strings.authResendCodeCountdown(seconds: seconds);
  @override
  String get resetPasswordTitle => strings.authResetPasswordTitle();
  @override
  String get newPassword => strings.authNewPassword();
  @override
  String get confirmPassword => strings.authConfirmPassword();
  @override
  String get setNewPassword => strings.authSetNewPassword();
  @override
  String get passwordTooShort => strings.authPasswordTooShort();
  @override
  String get passwordMismatch => strings.authPasswordMismatch();
  @override
  String get agreementPrefix => strings.authAgreementPrefix();
  @override
  String get termsOfService => strings.authTermsOfService();
  @override
  String get agreementConjunction => strings.authAgreementConjunction();
  @override
  String get privacyPolicy => strings.authPrivacyPolicy();
  @override
  String get agreementSuffix => strings.authAgreementSuffix();
  @override
  String get settings => strings.authSettings();
  @override
  String get anonymousMode => strings.authAnonymousMode();
  @override
  String get accountSectionTitle => strings.authAccountSectionTitle();
  @override
  String get notSignedIn => strings.authNotSignedIn();
  @override
  String get signOut => strings.authSignOut();
  @override
  String get errorInvalidCredentials => strings.authErrorInvalidCredentials();
  @override
  String get errorInvalidResponse => strings.authErrorInvalidResponse();
  @override
  String get errorSessionExpired => strings.authErrorSessionExpired();
  @override
  String get errorNetwork => strings.authErrorNetwork();
  @override
  String get errorServer => strings.authErrorServer();
  @override
  String get errorBootstrap => strings.authErrorBootstrap();
  @override
  String get errorInvalidConfiguration =>
      strings.authErrorInvalidConfiguration();
  @override
  String get errorInvalidCode => strings.authErrorInvalidCode();
  @override
  String get errorAccountExists => strings.authErrorAccountExists();
  @override
  String get errorWeakPassword => strings.authErrorWeakPassword();
  @override
  String get errorRateLimited => strings.authErrorRateLimited();
}
