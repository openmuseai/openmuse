import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_mobile/workbuddy/workbuddy_theme.dart';

final class DesignHost extends StatelessWidget {
  const DesignHost({
    super.key,
    required this.locale,
    required this.brightness,
    required this.home,
    this.captureKey,
    this.textScale = 1,
    this.fontFamily,
  });
  final OpenMuseLocaleController locale;
  final Brightness brightness;
  final Widget home;
  final Key? captureKey;
  final double textScale;
  final String? fontFamily;
  @override
  Widget build(BuildContext context) => RepaintBoundary(
    key: captureKey,
    child: OpenMuseLocaleScope(
      controller: locale,
      child: ListenableBuilder(
        listenable: locale,
        builder: (context, _) => MaterialApp(
          debugShowCheckedModeBanner: false,
          locale: locale.locale,
          supportedLocales: locale.supportedLocales,
          localizationsDelegates: [
            OpenMuseStringsDelegate(locale.catalog),
            ...GlobalMaterialLocalizations.delegates,
          ],
          theme: workBuddyTheme(brightness: brightness).copyWith(
            textTheme: workBuddyTheme(
              brightness: brightness,
            ).textTheme.apply(fontFamily: fontFamily),
          ),
          builder: (context, child) => MediaQuery(
            data: MediaQuery.of(
              context,
            ).copyWith(textScaler: TextScaler.linear(textScale)),
            child: child!,
          ),
          home: ColoredBox(
            color: workBuddyTheme(
              brightness: brightness,
            ).scaffoldBackgroundColor,
            child: SafeArea(child: home),
          ),
        ),
      ),
    ),
  );
}

final class DesignAuthentication extends ChangeNotifier
    implements OpenMuseAuthenticationController {
  @override
  OpenMuseAuthenticationSnapshot get snapshot =>
      const OpenMuseAuthenticationSnapshot(
        phase: OpenMuseAuthenticationPhase.authenticated,
        identity: OpenMuseAuthenticatedIdentity(
          subject: 'design-fixture',
          email: 'design@openmuse.example',
        ),
      );
  @override
  Future<String?> accessToken({bool forceRefresh = false}) async => null;
  @override
  Future<void> restore() async {}
  @override
  Future<void> signInWithPassword(String email, String password) async {}
  @override
  Future<void> signOut() async {}
}
