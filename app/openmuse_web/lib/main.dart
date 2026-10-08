import 'dart:async';

import 'package:flutter/material.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue_web.dart';
import 'package:openmuse_host_shell/openmuse_host_shell.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

import 'auth/browser_auth.dart';
import 'workbench/web_session_page.dart';

void main() => runApp(const OpenMuseWebApp());

final class OpenMuseWebApp extends StatefulWidget {
  const OpenMuseWebApp({super.key, this.authentication});

  /// Injection point for widget and integration tests.
  final GoTrueAuthenticationController? authentication;

  @override
  State<OpenMuseWebApp> createState() => _OpenMuseWebAppState();
}

final class _OpenMuseWebAppState extends State<OpenMuseWebApp> {
  BrowserGoTrueProvider? _provider;
  late final GoTrueAuthenticationController _authentication;

  @override
  void initState() {
    super.initState();
    _provider = widget.authentication == null ? BrowserGoTrueProvider() : null;
    _authentication =
        widget.authentication ??
        GoTrueAuthenticationController(
          provider: _provider!,
          store: const BrowserAuthSessionStore(),
        );
    unawaited(_authentication.restore());
  }

  @override
  void dispose() {
    _provider?.close();
    if (widget.authentication == null) _authentication.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => MaterialApp(
    debugShowCheckedModeBanner: false,
    title: 'OpenMuse',
    theme: buildOpenMuseTheme(),
    home: AnimatedBuilder(
      animation: _authentication,
      builder: (context, _) {
        final snapshot = _authentication.snapshot;
        if (snapshot.isAuthenticated) {
          return WebSessionPage(authentication: _authentication);
        }
        if (snapshot.phase == OpenMuseAuthenticationPhase.restoring) {
          return const Scaffold(
            body: Center(child: CircularProgressIndicator()),
          );
        }
        return OpenMuseLoginScreen(
          authentication: _authentication,
          cloudLabel: 'openmuseai.com',
        );
      },
    ),
  );
}
