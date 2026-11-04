import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

abstract final class WbColors {
  static const canvas = Color(0xFF191C21);
  static const drawer = Color(0xFF1F1F1F);
  static const sheet = Color(0xFF242830);
  static const card = Color(0xFF2C2C2C);
  static const chip = Color(0xFF252525);
  static const bubble = Color(0xFF323639);
  static const line = Color(0xFF2A2A2A);
  static const border = Color(0xFF5A5A5A);
  static const borderSoft = Color(0xFF3A3A3A);
  static const text = Color(0xFFEDF0F5);
  static const textDim = Color(0xFF8E8E8E);
  static const textMuted = Color(0xFFA3ADBB);
  static const avatar = Color(0xFF0FA780);
}

ThemeData workBuddyTheme({Brightness brightness = Brightness.dark}) =>
    brightness == Brightness.light
    ? ThemeData(
        brightness: Brightness.light,
        useMaterial3: true,
        scaffoldBackgroundColor: const Color(0xFFF6F7F9),
        colorScheme: const ColorScheme.light(
          primary: Color(0xFF3867C8),
          onPrimary: Colors.white,
          primaryContainer: Color(0xFFEDF2FC),
          onPrimaryContainer: Color(0xFF3867C8),
          secondary: Color(0xFF3867C8),
          onSecondary: Colors.white,
          secondaryContainer: Color(0xFFEDF2FC),
          onSecondaryContainer: Color(0xFF3867C8),
          surface: Colors.white,
          onSurface: Color(0xFF20252D),
          onSurfaceVariant: Color(0xFF737E8D),
          surfaceContainer: Color(0xFFECEFF3),
          surfaceContainerHigh: Color(0xFFE7E9ED),
          outline: Color(0xFF8D96A4),
          outlineVariant: Color(0xFFE5E7EB),
        ),
        appBarTheme: const AppBarTheme(
          backgroundColor: Color(0xFFF6F7F9),
          foregroundColor: Color(0xFF20252D),
          elevation: 0,
          systemOverlayStyle: SystemUiOverlayStyle.dark,
        ),
      )
    : ThemeData(
        brightness: Brightness.dark,
        useMaterial3: true,
        scaffoldBackgroundColor: const Color(0xFF191C21),
        splashFactory: NoSplash.splashFactory,
        highlightColor: Colors.transparent,
        colorScheme: const ColorScheme.dark(
          surface: Color(0xFF242830),
          primary: Color(0xFF8FAFF5),
          onPrimary: Color(0xFF101D35),
          primaryContainer: Color(0xFF2B3A55),
          onPrimaryContainer: Color(0xFFB9CEFF),
          secondary: Color(0xFF8FAFF5),
          onSecondary: Color(0xFF101D35),
          secondaryContainer: Color(0xFF2B3A55),
          onSecondaryContainer: Color(0xFFB9CEFF),
          onSurface: Color(0xFFEDF0F5),
          onSurfaceVariant: Color(0xFFA3ADBB),
          surfaceContainer: Color(0xFF242830),
          surfaceContainerHigh: Color(0xFF2C2C2C),
          outline: Color(0xFF686868),
          outlineVariant: Color(0xFF383838),
        ),
        dividerColor: WbColors.line,
        appBarTheme: const AppBarTheme(
          backgroundColor: Color(0xFF191C21),
          foregroundColor: Color(0xFFEDF0F5),
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

const wbSub = TextStyle(fontSize: 13, height: 1.2, fontWeight: FontWeight.w400);

const wbBody = TextStyle(
  fontSize: 16,
  height: 1.55,
  fontWeight: FontWeight.w400,
);
