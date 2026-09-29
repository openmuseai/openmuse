import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'login_theme.dart';

enum OpenMuseLoginPage { email, password, passcode }

final class OpenMuseLoginScreen extends StatefulWidget {
  const OpenMuseLoginScreen({
    super.key,
    required this.authentication,
    this.cloudLabel,
    this.onSettings,
    this.termsUri,
    this.privacyUri,
  });

  final OpenMuseAuthenticationController authentication;
  final String? cloudLabel;
  final VoidCallback? onSettings;
  final Uri? termsUri;
  final Uri? privacyUri;

  @override
  State<OpenMuseLoginScreen> createState() => _OpenMuseLoginScreenState();
}

final class _OpenMuseLoginScreenState extends State<OpenMuseLoginScreen> {
  final _emailController = TextEditingController();
  OpenMuseLoginPage _page = OpenMuseLoginPage.email;
  String? _emailError;

  @override
  void dispose() {
    _emailController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => Theme(
    data: OpenMuseLoginTheme.data(),
    child: Builder(
      builder: (context) => switch (_page) {
        OpenMuseLoginPage.email => _EmailPage(
          controller: _emailController,
          errorText: _emailError,
          cloudLabel: widget.cloudLabel,
          onSettings: widget.onSettings,
          termsUri: widget.termsUri,
          privacyUri: widget.privacyUri,
          onContinueWithEmail:
              widget.authentication is OpenMuseEmailCodeAuthenticationController
              ? _continueWithEmail
              : null,
          onContinueWithPassword: _continueWithPassword,
        ),
        OpenMuseLoginPage.password => _PasswordPage(
          authentication: widget.authentication,
          email: _emailController.text.trim(),
          onBack: () => setState(() => _page = OpenMuseLoginPage.email),
        ),
        OpenMuseLoginPage.passcode => _PasscodePage(
          authentication:
              widget.authentication
                  as OpenMuseEmailCodeAuthenticationController,
          listenable: widget.authentication,
          email: _emailController.text.trim(),
          onBack: () => setState(() => _page = OpenMuseLoginPage.email),
        ),
      },
    ),
  );

  void _continueWithPassword() {
    final email = _emailController.text.trim();
    if (!_looksLikeEmail(email)) {
      setState(() => _emailError = 'Please enter a valid email address.');
      return;
    }
    setState(() {
      _emailError = null;
      _page = OpenMuseLoginPage.password;
    });
  }

  void _continueWithEmail() {
    final email = _validatedEmail();
    if (email == null) return;
    final authentication =
        widget.authentication as OpenMuseEmailCodeAuthenticationController;
    setState(() => _page = OpenMuseLoginPage.passcode);
    unawaited(authentication.requestSignInCode(email));
  }

  String? _validatedEmail() {
    final email = _emailController.text.trim();
    if (!_looksLikeEmail(email)) {
      setState(() => _emailError = 'Please enter a valid email address.');
      return null;
    }
    setState(() => _emailError = null);
    return email;
  }
}

final class _EmailPage extends StatelessWidget {
  const _EmailPage({
    required this.controller,
    required this.errorText,
    required this.onContinueWithPassword,
    this.onContinueWithEmail,
    this.cloudLabel,
    this.onSettings,
    this.termsUri,
    this.privacyUri,
  });

  final TextEditingController controller;
  final String? errorText;
  final VoidCallback onContinueWithPassword;
  final VoidCallback? onContinueWithEmail;
  final String? cloudLabel;
  final VoidCallback? onSettings;
  final Uri? termsUri;
  final Uri? privacyUri;

  @override
  Widget build(BuildContext context) => _ResponsiveLoginScaffold(
    bottom: onSettings == null
        ? null
        : TextButton.icon(
            key: const ValueKey('auth.settings'),
            onPressed: onSettings,
            icon: const Icon(Icons.settings_outlined, size: 20),
            label: const Text('Settings'),
          ),
    child: AutofillGroup(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          const _LogoTitle(title: 'Welcome to OpenMuse'),
          if (cloudLabel != null) ...[
            const SizedBox(height: OpenMuseLoginSpacing.l),
            Text(
              'Cloud: $cloudLabel',
              key: const ValueKey('auth.cloud-label'),
              textAlign: TextAlign.center,
              style: Theme.of(context).textTheme.bodySmall,
            ),
          ],
          const SizedBox(height: OpenMuseLoginSpacing.xxl),
          TextField(
            key: const ValueKey('auth.email'),
            controller: controller,
            keyboardType: TextInputType.emailAddress,
            textInputAction: TextInputAction.next,
            autofillHints: const [AutofillHints.email],
            decoration: InputDecoration(
              hintText: 'Please enter your email',
              errorText: errorText,
            ),
            onSubmitted: (_) => onContinueWithPassword(),
          ),
          if (onContinueWithEmail != null) ...[
            const SizedBox(height: OpenMuseLoginSpacing.l),
            _PrimaryButton(
              key: const ValueKey('auth.continue-email'),
              label: 'Continue with email',
              onPressed: onContinueWithEmail,
            ),
          ],
          const SizedBox(height: OpenMuseLoginSpacing.l),
          _OutlinedButton(
            key: const ValueKey('auth.continue-password'),
            label: 'Continue with password',
            onPressed: onContinueWithPassword,
          ),
          const SizedBox(height: OpenMuseLoginSpacing.xxl),
          _Agreement(termsUri: termsUri, privacyUri: privacyUri),
        ],
      ),
    ),
  );
}

final class _PasscodePage extends StatefulWidget {
  const _PasscodePage({
    required this.authentication,
    required this.listenable,
    required this.email,
    required this.onBack,
  });

  final OpenMuseEmailCodeAuthenticationController authentication;
  final OpenMuseAuthenticationController listenable;
  final String email;
  final VoidCallback onBack;

  @override
  State<_PasscodePage> createState() => _PasscodePageState();
}

final class _PasscodePageState extends State<_PasscodePage> {
  final _codeController = TextEditingController();

  @override
  void initState() {
    super.initState();
    widget.listenable.addListener(_changed);
  }

  @override
  void dispose() {
    widget.listenable.removeListener(_changed);
    _codeController.dispose();
    super.dispose();
  }

  void _changed() {
    if (mounted) setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    final snapshot = widget.listenable.snapshot;
    final busy =
        snapshot.phase == OpenMuseAuthenticationPhase.submitting ||
        snapshot.phase == OpenMuseAuthenticationPhase.bootstrapping;
    final error = snapshot.phase == OpenMuseAuthenticationPhase.failure
        ? snapshot.failureMessage
        : null;
    return _ResponsiveLoginScaffold(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          const _LogoTitle(title: 'Check your email'),
          const SizedBox(height: OpenMuseLoginSpacing.l),
          Text(
            'We sent a sign-in link and passcode to ${widget.email}.',
            textAlign: TextAlign.center,
          ),
          const SizedBox(height: OpenMuseLoginSpacing.xxl),
          TextField(
            key: const ValueKey('auth.passcode'),
            controller: _codeController,
            enabled: !busy,
            keyboardType: TextInputType.number,
            textInputAction: TextInputAction.done,
            decoration: InputDecoration(
              hintText: 'Enter passcode',
              errorText: error,
            ),
            onSubmitted: busy ? null : (_) => _submit(),
          ),
          const SizedBox(height: OpenMuseLoginSpacing.xxl),
          _PrimaryButton(
            key: const ValueKey('auth.submit-passcode'),
            label: busy ? 'Verifying…' : 'Continue',
            onPressed: busy ? null : _submit,
            busy: busy,
          ),
          const SizedBox(height: OpenMuseLoginSpacing.l),
          TextButton(
            key: const ValueKey('auth.back'),
            onPressed: busy ? null : widget.onBack,
            child: const Text('Back to login'),
          ),
        ],
      ),
    );
  }

  void _submit() {
    final code = _codeController.text.trim();
    if (code.isEmpty) return;
    widget.authentication.signInWithCode(widget.email, code);
  }
}

final class _PasswordPage extends StatefulWidget {
  const _PasswordPage({
    required this.authentication,
    required this.email,
    required this.onBack,
  });

  final OpenMuseAuthenticationController authentication;
  final String email;
  final VoidCallback onBack;

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
    final snapshot = widget.authentication.snapshot;
    final submitting =
        snapshot.phase == OpenMuseAuthenticationPhase.submitting ||
        snapshot.phase == OpenMuseAuthenticationPhase.bootstrapping;
    final error = snapshot.phase == OpenMuseAuthenticationPhase.failure
        ? snapshot.failureMessage
        : null;
    return _ResponsiveLoginScaffold(
      child: AutofillGroup(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const _LogoTitle(title: 'Enter password'),
            const SizedBox(height: OpenMuseLoginSpacing.xxl),
            Text.rich(
              TextSpan(
                children: [
                  const TextSpan(text: 'Login as'),
                  TextSpan(
                    text: ' ${widget.email}',
                    style: const TextStyle(fontWeight: FontWeight.w600),
                  ),
                ],
              ),
            ),
            const SizedBox(height: OpenMuseLoginSpacing.xxl),
            TextField(
              key: const ValueKey('auth.password'),
              controller: _passwordController,
              focusNode: _passwordFocus,
              enabled: !submitting,
              obscureText: _obscure,
              autofillHints: const [AutofillHints.password],
              textInputAction: TextInputAction.done,
              decoration: InputDecoration(
                hintText: 'Enter password',
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
            const SizedBox(height: OpenMuseLoginSpacing.m),
            const Align(
              alignment: Alignment.centerLeft,
              child: TextButton(
                onPressed: null,
                style: ButtonStyle(
                  padding: WidgetStatePropertyAll(EdgeInsets.zero),
                ),
                child: Text('Forgot password?'),
              ),
            ),
            const SizedBox(height: OpenMuseLoginSpacing.xxl),
            _PrimaryButton(
              key: const ValueKey('auth.submit-password'),
              label: submitting ? 'Verifying…' : 'Continue',
              onPressed: submitting ? null : _submit,
              busy: submitting,
            ),
            const SizedBox(height: OpenMuseLoginSpacing.xxl),
            TextButton(
              key: const ValueKey('auth.back'),
              onPressed: submitting ? null : widget.onBack,
              child: const Text('Back to login'),
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
  Widget build(BuildContext context) => Column(
    mainAxisSize: MainAxisSize.min,
    children: [
      Image.asset(
        'assets/dsh_office_logo.png',
        package: 'openmuse_auth_gotrue',
        width: MediaQuery.sizeOf(context).width >= 600 ? 36 : 40,
        height: MediaQuery.sizeOf(context).width >= 600 ? 36 : 40,
        filterQuality: FilterQuality.high,
      ),
      const SizedBox(height: OpenMuseLoginSpacing.xxl),
      Text(title, style: Theme.of(context).textTheme.headlineSmall),
    ],
  );
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
  const _Agreement({this.termsUri, this.privacyUri});
  final Uri? termsUri;
  final Uri? privacyUri;

  @override
  Widget build(BuildContext context) => Text.rich(
    TextSpan(
      children: [
        const TextSpan(text: 'By continuing, you agree to our '),
        TextSpan(
          text: 'Terms of Service',
          style: TextStyle(
            color: termsUri == null
                ? OpenMuseLoginColors.tertiaryText
                : OpenMuseLoginColors.action,
          ),
        ),
        const TextSpan(text: ' and '),
        TextSpan(
          text: 'Privacy Policy',
          style: TextStyle(
            color: privacyUri == null
                ? OpenMuseLoginColors.tertiaryText
                : OpenMuseLoginColors.action,
          ),
        ),
        const TextSpan(text: '.'),
      ],
      style: Theme.of(context).textTheme.bodySmall,
    ),
    textAlign: TextAlign.center,
  );
}

bool _looksLikeEmail(String value) {
  final at = value.indexOf('@');
  final dot = value.lastIndexOf('.');
  return at > 0 && dot > at + 1 && dot < value.length - 1;
}
