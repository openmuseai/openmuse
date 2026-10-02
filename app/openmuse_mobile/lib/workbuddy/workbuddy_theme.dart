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

ThemeData workBuddyTheme() => ThemeData(
  brightness: Brightness.dark,
  useMaterial3: true,
  scaffoldBackgroundColor: WbColors.canvas,
  splashFactory: NoSplash.splashFactory,
  highlightColor: Colors.transparent,
  colorScheme: const ColorScheme.dark(
    surface: WbColors.canvas,
    primary: Colors.white,
  ),
  dividerColor: WbColors.line,
  appBarTheme: const AppBarTheme(
    backgroundColor: WbColors.canvas,
    foregroundColor: WbColors.text,
    elevation: 0,
    systemOverlayStyle: SystemUiOverlayStyle.light,
  ),
);

const wbTitle = TextStyle(
  color: WbColors.text,
  fontSize: 22,
  height: 1.15,
  fontWeight: FontWeight.w600,
  letterSpacing: -0.2,
);

const wbSlogan = TextStyle(
  color: WbColors.text,
  fontSize: 28,
  height: 1.2,
  fontWeight: FontWeight.w600,
  letterSpacing: -0.3,
);

const wbSub = TextStyle(
  color: WbColors.textDim,
  fontSize: 13,
  height: 1.2,
  fontWeight: FontWeight.w400,
);

const wbBody = TextStyle(
  color: Color(0xFFE6E6E6),
  fontSize: 16,
  height: 1.55,
  fontWeight: FontWeight.w400,
);
