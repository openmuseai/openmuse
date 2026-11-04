import 'package:flutter/material.dart';

abstract final class OpenMuseLoginColors {
  static const primaryText = Color(0xff21232a);
  static const secondaryText = Color(0xff6f748c);
  static const tertiaryText = Color(0xff989eb7);
  static const border = Color(0xffe4e8f5);
  static const action = Color(0xff00b5ff);
  static const actionHover = Color(0xff0092d6);
  static const error = Color(0xffe71d32);
  static const background = Color(0xffffffff);
}

abstract final class OpenMuseLoginSpacing {
  static const xs = 4.0;
  static const s = 6.0;
  static const m = 8.0;
  static const l = 12.0;
  static const xl = 16.0;
  static const xxl = 20.0;
}

abstract final class OpenMuseLoginTheme {
  static ThemeData data() => ThemeData(
    useMaterial3: true,
    scaffoldBackgroundColor: OpenMuseLoginColors.background,
    colorScheme: ColorScheme.fromSeed(
      seedColor: OpenMuseLoginColors.action,
      brightness: Brightness.light,
      primary: OpenMuseLoginColors.action,
      error: OpenMuseLoginColors.error,
      surface: OpenMuseLoginColors.background,
    ),
    textTheme: const TextTheme(
      headlineSmall: TextStyle(
        color: OpenMuseLoginColors.primaryText,
        fontSize: 20,
        fontWeight: FontWeight.w600,
        height: 1.3,
      ),
      bodyMedium: TextStyle(
        color: OpenMuseLoginColors.primaryText,
        fontSize: 14,
        height: 1.4,
      ),
      bodySmall: TextStyle(
        color: OpenMuseLoginColors.secondaryText,
        fontSize: 12,
        height: 1.4,
      ),
    ),
    inputDecorationTheme: const InputDecorationTheme(
      filled: true,
      fillColor: OpenMuseLoginColors.background,
      contentPadding: EdgeInsets.symmetric(horizontal: 8, vertical: 10),
      border: OutlineInputBorder(
        borderRadius: BorderRadius.all(Radius.circular(10)),
        borderSide: BorderSide(color: OpenMuseLoginColors.border),
      ),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.all(Radius.circular(10)),
        borderSide: BorderSide(color: OpenMuseLoginColors.border),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.all(Radius.circular(10)),
        borderSide: BorderSide(color: OpenMuseLoginColors.action, width: 1.5),
      ),
      errorBorder: OutlineInputBorder(
        borderRadius: BorderRadius.all(Radius.circular(10)),
        borderSide: BorderSide(color: OpenMuseLoginColors.error),
      ),
    ),
  );
}
