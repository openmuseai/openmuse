import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:url_launcher/url_launcher.dart';

import 'auth_localizations.dart';
import 'login_theme.dart';

enum OpenMuseLoginPage {
  email,
  password,
  passcode,
  signUp,
  signUpCode,
  recoveryCode,
  resetPassword,
}

final class OpenMuseLoginScreen extends StatefulWidget {
  const OpenMuseLoginScreen({
    super.key,
    required this.authentication,
    this.cloudLabel,
    this.onSettings,
    this.onAnonymous,
    this.termsUri,
    this.privacyUri,
    this.onOpenLegalUri,
  });

  final OpenMuseAuthenticationController authentication;
  final String? cloudLabel;
  final VoidCallback? onSettings;
  final VoidCallback? onAnonymous;
  final Uri? termsUri;
  final Uri? privacyUri;
  final void Function(Uri uri)? onOpenLegalUri;

  @override
  State<OpenMuseLoginScreen> createState() => _OpenMuseLoginScreenState();
}

final class _OpenMuseLoginScreenState extends State<OpenMuseLoginScreen> {
  final _emailController = TextEditingController();
  OpenMuseLoginPage _page = OpenMuseLoginPage.email;

  /// Kept as a flag rather than a message so the error follows the active
  /// locale instead of freezing the language it was raised in.
  bool _emailInvalid = false;

  @override
  void dispose() {
    _emailController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => AnimatedBuilder(
    animation: widget.authentication,
    builder: (context, _) => Theme(
      data: OpenMuseLoginTheme.data(),
      child: Builder(
        builder: (context) {
          final l10n = openMuseAuthLocalizations(context);
          return switch (_page) {
            OpenMuseLoginPage.email => _EmailPage(
              controller: _emailController,
              errorText: _emailInvalid ? l10n.invalidEmail : null,
              cloudLabel: widget.cloudLabel,
              onSettings: widget.onSettings,
              onAnonymous: widget.onAnonymous,
              termsUri: widget.termsUri,
              privacyUri: widget.privacyUri,
              onOpenLegalUri: widget.onOpenLegalUri,
              onContinueWithEmail:
                  widget.authentication
                      is OpenMuseEmailCodeAuthenticationController
                  ? _continueWithEmail
                  : null,
              onContinueWithPassword: _continueWithPassword,
              onSignUp:
                  widget.authentication
                      is OpenMuseAccountAuthenticationController
                  ? () => setState(() => _page = OpenMuseLoginPage.signUp)
                  : null,
              failure: _page == OpenMuseLoginPage.email
                  ? _failure(context)
                  : null,
            ),
            OpenMuseLoginPage.password => _PasswordPage(
              authentication: widget.authentication,
              email: _emailController.text.trim(),
              onBack: _backToEmail,
              onForgotPassword:
                  widget.authentication
                      is OpenMuseAccountAuthenticationController
                  ? _requestRecovery
                  : null,
            ),
            OpenMuseLoginPage.passcode => _PasscodePage(
              listenable: widget.authentication,
              email: _emailController.text.trim(),
              onBack: _backToEmail,
              onVerify: (code) =>
                  (widget.authentication
                          as OpenMuseEmailCodeAuthenticationController)
                      .signInWithCode(_emailController.text.trim(), code),
              onResend: () =>
                  (widget.authentication
                          as OpenMuseEmailCodeAuthenticationController)
                      .requestSignInCode(_emailController.text.trim()),
            ),
            OpenMuseLoginPage.signUp => _SignUpPage(
              authentication:
                  widget.authentication
                      as OpenMuseAccountAuthenticationController,
              listenable: widget.authentication,
              emailController: _emailController,
              onBack: _backToEmail,
              onCodeSent: () =>
                  setState(() => _page = OpenMuseLoginPage.signUpCode),
            ),
            OpenMuseLoginPage.signUpCode => _PasscodePage(
              listenable: widget.authentication,
              email: _emailController.text.trim(),
              onBack: _backToEmail,
              onVerify: (code) =>
                  (widget.authentication
                          as OpenMuseAccountAuthenticationController)
                      .verifySignUpCode(_emailController.text.trim(), code),
              onResend: () =>
                  (widget.authentication
                          as OpenMuseAccountAuthenticationController)
                      .resendSignUpCode(_emailController.text.trim()),
              description: l10n.signUpCodeSent(_emailController.text.trim()),
            ),
            OpenMuseLoginPage.recoveryCode => _PasscodePage(
              listenable: widget.authentication,
              email: _emailController.text.trim(),
              onBack: _backToEmail,
              onVerify: _verifyRecovery,
              onResend: () =>
                  (widget.authentication
                          as OpenMuseAccountAuthenticationController)
                      .requestPasswordRecovery(_emailController.text.trim()),
              description: l10n.recoveryCodeSent(_emailController.text.trim()),
            ),
            OpenMuseLoginPage.resetPassword => _ResetPasswordPage(
              authentication:
                  widget.authentication
                      as OpenMuseAccountAuthenticationController,
              listenable: widget.authentication,
              onBack: _backToEmail,
            ),
          };
        },
      ),
    ),
  );

  String? _failure(BuildContext context) {
    final snapshot = widget.authentication.snapshot;
    if (snapshot.phase != OpenMuseAuthenticationPhase.failure) return null;
    return openMuseAuthFailureMessage(
      context,
      code: snapshot.failureCode,
      fallback: snapshot.failureMessage,
    );
  }

  void _backToEmail() {
    if (widget.authentication is OpenMuseAccountAuthenticationController) {
      (widget.authentication as OpenMuseAccountAuthenticationController)
          .cancelPendingFlow();
    }
    setState(() => _page = OpenMuseLoginPage.email);
  }

  Future<void> _requestRecovery() async {
    final account =
        widget.authentication as OpenMuseAccountAuthenticationController;
    await account.requestPasswordRecovery(_emailController.text.trim());
    if (mounted &&
        widget.authentication.snapshot.phase ==
            OpenMuseAuthenticationPhase.awaitingPasscode) {
      setState(() => _page = OpenMuseLoginPage.recoveryCode);
    }
  }

  Future<void> _verifyRecovery(String code) async {
    final account =
        widget.authentication as OpenMuseAccountAuthenticationController;
    await account.verifyRecoveryCode(_emailController.text.trim(), code);
    if (mounted &&
        widget.authentication.snapshot.phase ==
            OpenMuseAuthenticationPhase.awaitingPasswordReset) {
      setState(() => _page = OpenMuseLoginPage.resetPassword);
    }
  }

  void _continueWithPassword() {
    final email = _emailController.text.trim();
    if (!_looksLikeEmail(email)) {
      setState(() => _emailInvalid = true);
      return;
    }
    setState(() {
      _emailInvalid = false;
      _page = OpenMuseLoginPage.password;
    });
  }

  Future<void> _continueWithEmail() async {
    final email = _validatedEmail();
    if (email == null) return;
    final authentication =
        widget.authentication as OpenMuseEmailCodeAuthenticationController;
    await authentication.requestSignInCode(email);
    if (mounted &&
        widget.authentication.snapshot.phase ==
            OpenMuseAuthenticationPhase.awaitingPasscode) {
      setState(() => _page = OpenMuseLoginPage.passcode);
    }
  }

  String? _validatedEmail() {
    final email = _emailController.text.trim();
    if (!_looksLikeEmail(email)) {
      setState(() => _emailInvalid = true);
      return null;
    }
    setState(() => _emailInvalid = false);
    return email;
  }
}

final class _EmailPage extends StatelessWidget {
  const _EmailPage({
    required this.controller,
    required this.errorText,
    required this.onContinueWithPassword,
    this.onSignUp,
    this.failure,
    this.onContinueWithEmail,
    this.cloudLabel,
    this.onSettings,
    this.onAnonymous,
    this.termsUri,
    this.privacyUri,
    this.onOpenLegalUri,
  });

  final TextEditingController controller;
  final String? errorText;
  final VoidCallback onContinueWithPassword;
  final VoidCallback? onSignUp;
  final String? failure;
  final VoidCallback? onContinueWithEmail;
  final String? cloudLabel;
  final VoidCallback? onSettings;
  final VoidCallback? onAnonymous;
  final Uri? termsUri;
  final Uri? privacyUri;
  final void Function(Uri uri)? onOpenLegalUri;

  @override
  Widget build(BuildContext context) {
    final l10n = openMuseAuthLocalizations(context);
    return _ResponsiveLoginScaffold(
      bottom: _loginFooter(
        context,
        l10n: l10n,
        onSettings: onSettings,
        onAnonymous: onAnonymous,
      ),
      child: AutofillGroup(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            _LogoTitle(title: l10n.welcomeTitle),
            if (cloudLabel != null) ...[
              const SizedBox(height: OpenMuseLoginSpacing.l),
              Text(
                l10n.cloudLabel(cloudLabel!),
                key: const ValueKey('auth.cloud-label'),
                textAlign: TextAlign.center,
                style: Theme.of(context).textTheme.bodySmall,
              ),
            ],
            const SizedBox(height: OpenMuseLoginSpacing.xxl),
            _FocusHint(
              builder: (node, focused) => TextField(
                key: const ValueKey('auth.email'),
                controller: controller,
                focusNode: node,
                keyboardType: TextInputType.emailAddress,
                textInputAction: TextInputAction.next,
                autofillHints: const [AutofillHints.email],
                decoration: InputDecoration(
                  hintText: focused ? null : l10n.emailHint,
                  errorText: errorText,
                ),
                onSubmitted: (_) => onContinueWithPassword(),
              ),
            ),
            if (onContinueWithEmail != null) ...[
              const SizedBox(height: OpenMuseLoginSpacing.l),
              _PrimaryButton(
                key: const ValueKey('auth.continue-email'),
                label: l10n.continueWithEmail,
                onPressed: onContinueWithEmail,
              ),
            ],
            const SizedBox(height: OpenMuseLoginSpacing.l),
            _OutlinedButton(
              key: const ValueKey('auth.continue-password'),
              label: l10n.continueWithPassword,
              onPressed: onContinueWithPassword,
            ),
            if (onSignUp != null) ...[
              const SizedBox(height: OpenMuseLoginSpacing.m),
              TextButton(
                key: const ValueKey('auth.sign-up'),
                onPressed: onSignUp,
                child: Text(l10n.createAccount),
              ),
            ],
            if (failure != null) ...[
              const SizedBox(height: OpenMuseLoginSpacing.m),
              Text(failure!, style: const TextStyle(color: Colors.red)),
            ],
            const SizedBox(height: OpenMuseLoginSpacing.xxl),
            _Agreement(
              termsUri: termsUri,
              privacyUri: privacyUri,
              onOpen: onOpenLegalUri,
            ),
          ],
        ),
      ),
    );
  }
}

final class _PasscodePage extends StatefulWidget {
  const _PasscodePage({
    required this.listenable,
    required this.email,
    required this.onBack,
    required this.onResend,
    required this.onVerify,
    this.description,
  });

  final OpenMuseAuthenticationController listenable;
  final String email;
  final VoidCallback onBack;
  final Future<void> Function() onResend;
  final Future<void> Function(String) onVerify;
  final String? description;

  @override
  State<_PasscodePage> createState() => _PasscodePageState();
}

final class _PasscodePageState extends State<_PasscodePage> {
  final _codeController = TextEditingController();
  Timer? _resendTimer;
  int _resendSeconds = 30;

  @override
  void initState() {
    super.initState();
    widget.listenable.addListener(_changed);
    _startResendTimer();
  }

  @override
  void dispose() {
    widget.listenable.removeListener(_changed);
    _resendTimer?.cancel();
    _codeController.dispose();
    super.dispose();
  }

  void _changed() {
    if (mounted) setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    final l10n = openMuseAuthLocalizations(context);
    final snapshot = widget.listenable.snapshot;
    final busy =
        snapshot.phase == OpenMuseAuthenticationPhase.submitting ||
        snapshot.phase == OpenMuseAuthenticationPhase.bootstrapping;
    final error = snapshot.phase == OpenMuseAuthenticationPhase.failure
        ? openMuseAuthFailureMessage(
            context,
            code: snapshot.failureCode,
            fallback: snapshot.failureMessage,
          )
        : null;
    return _ResponsiveLoginScaffold(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          _LogoTitle(title: l10n.checkYourEmailTitle),
          const SizedBox(height: OpenMuseLoginSpacing.l),
          Text(
            widget.description ?? l10n.signInCodeSent(widget.email),
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: OpenMuseLoginSpacing.xxl),
          _FocusHint(
            builder: (node, focused) => TextField(
              key: const ValueKey('auth.passcode'),
              controller: _codeController,
              focusNode: node,
              enabled: !busy,
              keyboardType: TextInputType.number,
              inputFormatters: [
                FilteringTextInputFormatter.digitsOnly,
                LengthLimitingTextInputFormatter(8),
              ],
              textInputAction: TextInputAction.done,
              decoration: InputDecoration(
                hintText: focused ? null : l10n.passcodeHint,
                errorText: error,
              ),
              onSubmitted: busy ? null : (_) => _submit(),
            ),
          ),
          const SizedBox(height: OpenMuseLoginSpacing.xxl),
          _PrimaryButton(
            key: const ValueKey('auth.submit-passcode'),
            label: busy ? l10n.verifying : l10n.continueAction,
            onPressed: busy ? null : _submit,
            busy: busy,
          ),
          const SizedBox(height: OpenMuseLoginSpacing.m),
          TextButton(
            key: const ValueKey('auth.resend-code'),
            onPressed: busy || _resendSeconds > 0 ? null : _resend,
            child: Text(
              _resendSeconds > 0
                  ? l10n.resendCodeCountdown(_resendSeconds)
                  : l10n.resendCode,
            ),
          ),
          const SizedBox(height: OpenMuseLoginSpacing.l),
          TextButton(
            key: const ValueKey('auth.back'),
            onPressed: busy ? null : widget.onBack,
            child: Text(l10n.backToLogin),
          ),
        ],
      ),
    );
  }

  void _submit() {
    final code = _codeController.text.trim();
    if (code.isEmpty) return;
    unawaited(widget.onVerify(code));
  }

  void _startResendTimer() {
    _resendTimer?.cancel();
    _resendSeconds = 30;
    _resendTimer = Timer.periodic(const Duration(seconds: 1), (timer) {
      if (!mounted) {
        timer.cancel();
        return;
      }
      setState(() => _resendSeconds--);
      if (_resendSeconds <= 0) timer.cancel();
    });
  }

  Future<void> _resend() async {
    await widget.onResend();
    if (mounted &&
        widget.listenable.snapshot.phase ==
            OpenMuseAuthenticationPhase.awaitingPasscode) {
      setState(_startResendTimer);
    }
  }
}

final class _PasswordPage extends StatefulWidget {
  const _PasswordPage({
    required this.authentication,
    required this.email,
    required this.onBack,
    this.onForgotPassword,
  });

  final OpenMuseAuthenticationController authentication;
  final String email;
  final VoidCallback onBack;
  final Future<void> Function()? onForgotPassword;

  @override
  State<_PasswordPage> createState() => _PasswordPageState();
}

final class _PasswordPageState extends State<_PasswordPage> {
  final _passwordController = TextEditingController();
  final _passwordFocus = FocusNode();
  bool _obscure = true;

  @override
  void initState() {
    super.initState();
    widget.authentication.addListener(_onAuthenticationChanged);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) _passwordFocus.requestFocus();
    });
  }

  @override
  void dispose() {
    widget.authentication.removeListener(_onAuthenticationChanged);
    _passwordController.dispose();
    _passwordFocus.dispose();
    super.dispose();
  }

  void _onAuthenticationChanged() {
    if (mounted) setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    final l10n = openMuseAuthLocalizations(context);
    final snapshot = widget.authentication.snapshot;
    final submitting =
        snapshot.phase == OpenMuseAuthenticationPhase.submitting ||
        snapshot.phase == OpenMuseAuthenticationPhase.bootstrapping;
    final error = snapshot.phase == OpenMuseAuthenticationPhase.failure
        ? openMuseAuthFailureMessage(
            context,
            code: snapshot.failureCode,
            fallback: snapshot.failureMessage,
          )
        : null;
    return _ResponsiveLoginScaffold(
      child: AutofillGroup(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            _LogoTitle(title: l10n.enterPasswordTitle),
            const SizedBox(height: OpenMuseLoginSpacing.xxl),
            Text.rich(
              TextSpan(
                children: [
                  TextSpan(text: l10n.loginAsLabel),
                  TextSpan(
                    text: ' ${widget.email}',
                    style: const TextStyle(fontWeight: FontWeight.w600),
                  ),
                ],
              ),
            ),
            const SizedBox(height: OpenMuseLoginSpacing.xxl),
            _FocusHint(
              focusNode: _passwordFocus,
              builder: (node, focused) => TextField(
                key: const ValueKey('auth.password'),
                controller: _passwordController,
                focusNode: node,
                enabled: !submitting,
                obscureText: _obscure,
                autofillHints: const [AutofillHints.password],
                textInputAction: TextInputAction.done,
                decoration: InputDecoration(
                  hintText: focused ? null : l10n.passwordHint,
                  errorText: error,
                  suffixIcon: IconButton(
                    key: const ValueKey('auth.password-visibility'),
                    onPressed: () => setState(() => _obscure = !_obscure),
                    icon: Icon(
                      _obscure
                          ? Icons.visibility_outlined
                          : Icons.visibility_off_outlined,
                      size: 20,
                    ),
                  ),
                ),
                onSubmitted: submitting ? null : (_) => _submit(),
              ),
            ),
            const SizedBox(height: OpenMuseLoginSpacing.m),
            Align(
              alignment: Alignment.centerLeft,
              child: TextButton(
                onPressed: submitting ? null : widget.onForgotPassword,
                style: const ButtonStyle(
                  padding: WidgetStatePropertyAll(EdgeInsets.zero),
                ),
                child: Text(l10n.forgotPassword),
              ),
            ),
            const SizedBox(height: OpenMuseLoginSpacing.xxl),
            _PrimaryButton(
              key: const ValueKey('auth.submit-password'),
              label: submitting ? l10n.verifying : l10n.continueAction,
              onPressed: submitting ? null : _submit,
              busy: submitting,
            ),
            const SizedBox(height: OpenMuseLoginSpacing.xxl),
            TextButton(
              key: const ValueKey('auth.back'),
              onPressed: submitting ? null : widget.onBack,
              child: Text(l10n.backToLogin),
            ),
          ],
        ),
      ),
    );
  }

  void _submit() {
    if (_passwordController.text.isEmpty) return;
    TextInput.finishAutofillContext();
    widget.authentication.signInWithPassword(
      widget.email,
      _passwordController.text,
    );
  }
}

final class _SignUpPage extends StatelessWidget {
  const _SignUpPage({
    required this.authentication,
    required this.listenable,
    required this.emailController,
    required this.onBack,
    required this.onCodeSent,
  });

  final OpenMuseAccountAuthenticationController authentication;
  final OpenMuseAuthenticationController listenable;
  final TextEditingController emailController;
  final VoidCallback onBack;
  final VoidCallback onCodeSent;

  @override
  Widget build(BuildContext context) => _PasswordFormPage(
    title: openMuseAuthLocalizations(context).createAccountTitle,
    emailController: emailController,
    listenable: listenable,
    onBack: onBack,
    onSubmit: (password) async {
      await authentication.signUp(emailController.text.trim(), password);
      if (listenable.snapshot.phase ==
          OpenMuseAuthenticationPhase.awaitingPasscode) {
        onCodeSent();
      }
    },
  );
}

final class _ResetPasswordPage extends StatelessWidget {
  const _ResetPasswordPage({
    required this.authentication,
    required this.listenable,
    required this.onBack,
  });

  final OpenMuseAccountAuthenticationController authentication;
  final OpenMuseAuthenticationController listenable;
  final VoidCallback onBack;

  @override
  Widget build(BuildContext context) => _PasswordFormPage(
    title: openMuseAuthLocalizations(context).resetPasswordTitle,
    listenable: listenable,
    onBack: onBack,
    onSubmit: authentication.resetPassword,
  );
}

final class _PasswordFormPage extends StatefulWidget {
  const _PasswordFormPage({
    required this.title,
    required this.listenable,
    required this.onBack,
    required this.onSubmit,
    this.emailController,
  });

  final String title;
  final TextEditingController? emailController;
  final OpenMuseAuthenticationController listenable;
  final VoidCallback onBack;
  final Future<void> Function(String) onSubmit;

  @override
  State<_PasswordFormPage> createState() => _PasswordFormPageState();
}

final class _PasswordFormPageState extends State<_PasswordFormPage> {
  final _password = TextEditingController();
  final _confirmation = TextEditingController();
  String? _validation;
  bool _emailInvalid = false;
  bool _obscure = true;

  @override
  void dispose() {
    _password.dispose();
    _confirmation.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final l10n = openMuseAuthLocalizations(context);
    final snapshot = widget.listenable.snapshot;
    final busy =
        snapshot.phase == OpenMuseAuthenticationPhase.submitting ||
        snapshot.phase == OpenMuseAuthenticationPhase.bootstrapping;
    final failure = snapshot.phase == OpenMuseAuthenticationPhase.failure
        ? openMuseAuthFailureMessage(
            context,
            code: snapshot.failureCode,
            fallback: snapshot.failureMessage,
          )
        : null;
    return _ResponsiveLoginScaffold(
      child: AutofillGroup(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            _LogoTitle(title: widget.title),
            if (widget.emailController != null) ...[
              const SizedBox(height: OpenMuseLoginSpacing.xxl),
              _FocusHint(
                builder: (node, focused) => TextField(
                  key: const ValueKey('auth.signup-email'),
                  controller: widget.emailController,
                  focusNode: node,
                  enabled: !busy,
                  keyboardType: TextInputType.emailAddress,
                  textInputAction: TextInputAction.next,
                  autofillHints: const [AutofillHints.email],
                  decoration: InputDecoration(
                    hintText: focused ? null : l10n.emailHint,
                    errorText: _emailInvalid ? l10n.invalidEmail : null,
                  ),
                ),
              ),
            ],
            const SizedBox(height: OpenMuseLoginSpacing.l),
            _FocusHint(
              builder: (node, focused) => TextField(
                key: const ValueKey('auth.new-password'),
                controller: _password,
                focusNode: node,
                enabled: !busy,
                obscureText: _obscure,
                autofillHints: const [AutofillHints.newPassword],
                decoration: InputDecoration(
                  hintText: focused ? null : l10n.newPassword,
                  suffixIcon: IconButton(
                    onPressed: () => setState(() => _obscure = !_obscure),
                    icon: Icon(
                      _obscure
                          ? Icons.visibility_outlined
                          : Icons.visibility_off_outlined,
                    ),
                  ),
                ),
              ),
            ),
            const SizedBox(height: OpenMuseLoginSpacing.l),
            _FocusHint(
              builder: (node, focused) => TextField(
                key: const ValueKey('auth.confirm-password'),
                controller: _confirmation,
                focusNode: node,
                enabled: !busy,
                obscureText: _obscure,
                autofillHints: const [AutofillHints.newPassword],
                textInputAction: TextInputAction.done,
                decoration: InputDecoration(
                  hintText: focused ? null : l10n.confirmPassword,
                  errorText: _validation ?? failure,
                ),
                onSubmitted: busy ? null : (_) => _submit(),
              ),
            ),
            const SizedBox(height: OpenMuseLoginSpacing.xxl),
            _PrimaryButton(
              key: const ValueKey('auth.submit-new-password'),
              label: busy
                  ? l10n.verifying
                  : widget.emailController == null
                  ? l10n.setNewPassword
                  : l10n.createAccount,
              onPressed: busy ? null : _submit,
              busy: busy,
            ),
            const SizedBox(height: OpenMuseLoginSpacing.l),
            TextButton(
              key: const ValueKey('auth.back'),
              onPressed: busy ? null : widget.onBack,
              child: Text(l10n.backToLogin),
            ),
          ],
        ),
      ),
    );
  }

  void _submit() {
    final l10n = openMuseAuthLocalizations(context);
    if (widget.emailController != null &&
        !_looksLikeEmail(widget.emailController!.text.trim())) {
      setState(() => _emailInvalid = true);
      return;
    }
    _emailInvalid = false;
    if (_password.text.length < 8) {
      setState(() => _validation = l10n.passwordTooShort);
      return;
    }
    if (_password.text != _confirmation.text) {
      setState(() => _validation = l10n.passwordMismatch);
      return;
    }
    setState(() => _validation = null);
    TextInput.finishAutofillContext();
    unawaited(widget.onSubmit(_password.text));
  }
}

final class _ResponsiveLoginScaffold extends StatelessWidget {
  const _ResponsiveLoginScaffold({required this.child, this.bottom});

  final Widget child;
  final Widget? bottom;

  @override
  Widget build(BuildContext context) => Scaffold(
    resizeToAvoidBottomInset: true,
    backgroundColor: OpenMuseLoginColors.background,
    body: SafeArea(
      child: LayoutBuilder(
        builder: (context, constraints) {
          final mobile = constraints.maxWidth < 600;
          final horizontal = mobile ? 40.0 : 24.0;
          return SingleChildScrollView(
            keyboardDismissBehavior: ScrollViewKeyboardDismissBehavior.onDrag,
            padding: EdgeInsets.symmetric(horizontal: horizontal, vertical: 38),
            child: ConstrainedBox(
              constraints: BoxConstraints(
                minHeight: (constraints.maxHeight - 76).clamp(
                  0,
                  double.infinity,
                ),
              ),
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Center(
                    child: ConstrainedBox(
                      constraints: const BoxConstraints(maxWidth: 320),
                      child: child,
                    ),
                  ),
                  if (bottom != null) ...[const SizedBox(height: 24), bottom!],
                ],
              ),
            ),
          );
        },
      ),
    ),
  );
}

final class _LogoTitle extends StatelessWidget {
  const _LogoTitle({required this.title});
  final String title;

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        const _OpenMuseLoginMark(),
        const SizedBox(height: OpenMuseLoginSpacing.xxl),
        Text(title, style: Theme.of(context).textTheme.headlineSmall),
      ],
    );
  }
}

final class _OpenMuseLoginMark extends StatelessWidget {
  const _OpenMuseLoginMark();

  @override
  Widget build(BuildContext context) => Container(
    key: const ValueKey('auth.brand'),
    width: 40,
    height: 40,
    decoration: BoxDecoration(
      color: const Color(0xffe8ebff),
      borderRadius: BorderRadius.circular(12),
    ),
    child: const Icon(Icons.auto_awesome, color: Color(0xff4c63d9), size: 22),
  );
}

final class _LanguageSwitch extends StatelessWidget {
  const _LanguageSwitch({
    required this.selected,
    required this.onSelect,
    required this.options,
  });

  final List<LocaleOption> options;

  final String selected;
  final ValueChanged<String> onSelect;

  @override
  Widget build(BuildContext context) => Row(
    mainAxisSize: MainAxisSize.min,
    children: [
      for (final option in options) ...[
        if (option != options.first) const Text(' / '),
        _chip(label: option.label, language: option.tag, uiKey: option.uiKey),
      ],
    ],
  );

  Widget _chip({
    required String label,
    required String language,
    required String uiKey,
  }) {
    final active = selected == language;
    return TextButton(
      key: ValueKey('auth.locale-$uiKey'),
      onPressed: active ? null : () => onSelect(language),
      style: TextButton.styleFrom(
        foregroundColor: const Color(0xff1a73e8),
        disabledForegroundColor: const Color(0xff1a1a1a),
        padding: const EdgeInsets.symmetric(horizontal: 2),
        minimumSize: Size.zero,
        tapTargetSize: MaterialTapTargetSize.shrinkWrap,
        visualDensity: VisualDensity.compact,
      ),
      child: Text(
        label,
        style: TextStyle(
          fontSize: 12,
          height: 1.1,
          fontWeight: active ? FontWeight.w700 : FontWeight.w500,
        ),
      ),
    );
  }
}

final class _PrimaryButton extends StatelessWidget {
  const _PrimaryButton({
    super.key,
    required this.label,
    required this.onPressed,
    this.busy = false,
  });
  final String label;
  final VoidCallback? onPressed;
  final bool busy;

  @override
  Widget build(BuildContext context) => SizedBox(
    width: double.infinity,
    height: 44,
    child: FilledButton(
      onPressed: onPressed,
      style: FilledButton.styleFrom(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
      ),
      child: busy
          ? const SizedBox.square(
              dimension: 18,
              child: CircularProgressIndicator(
                strokeWidth: 2,
                color: Colors.white,
              ),
            )
          : Text(label),
    ),
  );
}

final class _OutlinedButton extends StatelessWidget {
  const _OutlinedButton({
    super.key,
    required this.label,
    required this.onPressed,
  });
  final String label;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) => SizedBox(
    width: double.infinity,
    height: 44,
    child: OutlinedButton(
      onPressed: onPressed,
      style: OutlinedButton.styleFrom(
        foregroundColor: OpenMuseLoginColors.primaryText,
        side: const BorderSide(color: OpenMuseLoginColors.border),
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
      ),
      child: Text(label),
    ),
  );
}

final class _Agreement extends StatelessWidget {
  const _Agreement({this.termsUri, this.privacyUri, this.onOpen});
  final Uri? termsUri;
  final Uri? privacyUri;
  final void Function(Uri uri)? onOpen;

  @override
  Widget build(BuildContext context) {
    final l10n = openMuseAuthLocalizations(context);
    final style = Theme.of(context).textTheme.bodySmall;
    return Text.rich(
      TextSpan(
        children: [
          TextSpan(text: l10n.agreementPrefix),
          WidgetSpan(
            alignment: PlaceholderAlignment.baseline,
            baseline: TextBaseline.alphabetic,
            child: _LegalLink(
              label: l10n.termsOfService,
              uri: termsUri,
              linkKey: const ValueKey('auth.terms'),
              style: style,
              onOpen: onOpen,
            ),
          ),
          TextSpan(text: l10n.agreementConjunction),
          WidgetSpan(
            alignment: PlaceholderAlignment.baseline,
            baseline: TextBaseline.alphabetic,
            child: _LegalLink(
              label: l10n.privacyPolicy,
              uri: privacyUri,
              linkKey: const ValueKey('auth.privacy'),
              style: style,
              onOpen: onOpen,
            ),
          ),
          TextSpan(text: l10n.agreementSuffix),
        ],
        style: style,
      ),
      textAlign: TextAlign.center,
    );
  }
}

final class _LegalLink extends StatelessWidget {
  const _LegalLink({
    required this.label,
    required this.uri,
    required this.linkKey,
    required this.style,
    required this.onOpen,
  });

  final String label;
  final Uri? uri;
  final Key linkKey;
  final TextStyle? style;
  final void Function(Uri uri)? onOpen;

  @override
  Widget build(BuildContext context) {
    final linked = uri != null;
    return Semantics(
      button: linked,
      link: linked,
      child: GestureDetector(
        key: linkKey,
        onTap: linked ? () => _open(uri!) : null,
        child: Text(
          label,
          style: style?.copyWith(
            color: linked
                ? OpenMuseLoginColors.action
                : OpenMuseLoginColors.tertiaryText,
          ),
        ),
      ),
    );
  }

  void _open(Uri uri) {
    final opener = onOpen;
    if (opener != null) {
      opener(uri);
      return;
    }
    unawaited(launchUrl(uri, mode: LaunchMode.externalApplication));
  }
}

Widget _loginFooter(
  BuildContext context, {
  required OpenMuseAuthLocalizations l10n,
  required VoidCallback? onSettings,
  required VoidCallback? onAnonymous,
}) {
  final controller = OpenMuseUiLocaleScope.maybeOf(context);
  final language = _LanguageSwitch(
    options: controller?.options ?? OpenMuseCatalog.builtIn.locales,
    selected:
        controller?.tag ??
        OpenMuseCatalog.builtIn.resolve([
          Localizations.localeOf(context).toLanguageTag(),
        ]),
    onSelect: (value) => controller?.selectTag(value),
  );
  final settings = onSettings == null
      ? null
      : TextButton.icon(
          key: const ValueKey('auth.settings'),
          onPressed: onSettings,
          icon: const Icon(Icons.settings_outlined, size: 20),
          label: Text(l10n.settings),
        );
  final anonymous = onAnonymous == null
      ? null
      : TextButton.icon(
          key: const ValueKey('auth.anonymous'),
          onPressed: onAnonymous,
          icon: const Icon(Icons.person_outline, size: 20),
          label: Text(l10n.anonymousMode),
        );
  return Row(
    mainAxisSize: MainAxisSize.min,
    children: [
      ?settings,
      if (settings != null && anonymous != null) const SizedBox(width: 12),
      ?anonymous,
      if (settings != null || anonymous != null) const SizedBox(width: 8),
      language,
    ],
  );
}

bool _looksLikeEmail(String value) {
  final at = value.indexOf('@');
  final dot = value.lastIndexOf('.');
  return at > 0 && dot > at + 1 && dot < value.length - 1;
}

/// Hides [hint] text while the field is focused so the caret is not covered.
final class _FocusHint extends StatefulWidget {
  const _FocusHint({this.focusNode, required this.builder});

  final FocusNode? focusNode;
  final Widget Function(FocusNode node, bool focused) builder;

  @override
  State<_FocusHint> createState() => _FocusHintState();
}

final class _FocusHintState extends State<_FocusHint> {
  FocusNode? _owned;
  late bool _focused;

  FocusNode get _node => widget.focusNode ?? _owned!;

  @override
  void initState() {
    super.initState();
    if (widget.focusNode == null) _owned = FocusNode();
    _focused = _node.hasFocus;
    _node.addListener(_onFocus);
  }

  @override
  void didUpdateWidget(covariant _FocusHint oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.focusNode == widget.focusNode) return;
    (oldWidget.focusNode ?? _owned)?.removeListener(_onFocus);
    if (widget.focusNode == null) {
      _owned ??= FocusNode();
    } else {
      _owned?.dispose();
      _owned = null;
    }
    _focused = _node.hasFocus;
    _node.addListener(_onFocus);
  }

  void _onFocus() {
    final focused = _node.hasFocus;
    if (focused == _focused || !mounted) return;
    setState(() => _focused = focused);
  }

  @override
  void dispose() {
    _node.removeListener(_onFocus);
    _owned?.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => widget.builder(_node, _focused);
}
