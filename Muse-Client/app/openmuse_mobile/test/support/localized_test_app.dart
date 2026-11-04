import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';

/// Existing behavior assertions use the installed Chinese locale explicitly.
Widget localizedTestApp({
  Widget? home,
  ThemeData? theme,
  TransitionBuilder? builder,
  GlobalKey<NavigatorState>? navigatorKey,
}) {
  final locale = OpenMuseLocaleController(initialTag: 'zh-Hans');
  addTearDown(locale.dispose);
  return OpenMuseLocaleScope(
    controller: locale,
    child: MaterialApp(
      locale: locale.locale,
      supportedLocales: locale.supportedLocales,
      localizationsDelegates: [
        OpenMuseStringsDelegate(locale.catalog),
        ...GlobalMaterialLocalizations.delegates,
      ],
      theme: theme,
      builder: builder,
      navigatorKey: navigatorKey,
      home: home,
    ),
  );
}
