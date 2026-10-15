import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

abstract final class WbColors {
  static const canvas = Color(0xFF1A1A1A);
  static const drawer = Color(0xFF1F1F1F);
  static const sheet = Color(0xFF242424);
  static const card = Color(0xFF2C2C2C);
  static const chip = Color(0xFF252525);
  static const bubble = Color(0xFF323639);
  static const line = Color(0xFF2A2A2A);
  static const border = Color(0xFF5A5A5A);
  static const borderSoft = Color(0xFF3A3A3A);
  static const text = Color(0xFFF2F2F2);
  static const textDim = Color(0xFF8E8E8E);
  static const textMuted = Color(0xFFA3A3A3);
  static const avatar = Color(0xFF0FA780);
}

ThemeData workBuddyTheme({Brightness brightness = Brightness.dark}) =>
    brightness == Brightness.light
    ? ThemeData(
        brightness: Brightness.light,
        useMaterial3: true,
        scaffoldBackgroundColor: const Color(0xFFF5F4F1),
        colorScheme: const ColorScheme.light(
          primary: Color(0xFF1765E9),
          surface: Colors.white,
          onSurface: Color(0xFF191D25),
          onSurfaceVariant: Color(0xFF667080),
          surfaceContainer: Color(0xFFEDEBE8),
          surfaceContainerHigh: Color(0xFFE7E9ED),
          outline: Color(0xFF8D96A4),
          outlineVariant: Color(0xFFE5E7EB),
        ),
        appBarTheme: const AppBarTheme(
          backgroundColor: Color(0xFFF5F4F1),
          foregroundColor: Color(0xFF191D25),
          elevation: 0,
          systemOverlayStyle: SystemUiOverlayStyle.dark,
        ),
      )
    : ThemeData(
        brightness: Brightness.dark,
        useMaterial3: true,
        scaffoldBackgroundColor: const Color(0xFF171B22),
        splashFactory: NoSplash.splashFactory,
        highlightColor: Colors.transparent,
        colorScheme: const ColorScheme.dark(
          surface: Color(0xFF242A34),
          primary: Color(0xFF5B92FF),
          onSurface: Color(0xFFF1F3F6),
          onSurfaceVariant: Color(0xFFA9B3C1),
          surfaceContainer: Color(0xFF242A34),
          surfaceContainerHigh: Color(0xFF323A46),
          outline: Color(0xFF637083),
          outlineVariant: Color(0xFF3A4351),
        ),
        dividerColor: WbColors.line,
        appBarTheme: const AppBarTheme(
          backgroundColor: Color(0xFF171B22),
          foregroundColor: Color(0xFFF1F3F6),
          elevation: 0,
          systemOverlayStyle: SystemUiOverlayStyle.light,
        ),
      );

const wbTitle = TextStyle(
  fontSize: 22,
  height: 1.15,
  fontWeight: FontWeight.w600,
  letterSpacing: -0.2,
);

const wbSlogan = TextStyle(
  fontSize: 28,
  height: 1.2,
  fontWeight: FontWeight.w600,
  letterSpacing: -0.3,
);

const wbSub = TextStyle(
  fontSize: 13,
  height: 1.2,
  fontWeight: FontWeight.w400,
);

const wbBody = TextStyle(
  fontSize: 16,
  height: 1.55,
  fontWeight: FontWeight.w400,
);
