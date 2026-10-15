import 'package:flutter/material.dart';

abstract final class MarketColors {
  static const canvas = Color(0xFFF5F4F1);
  static const surface = Color(0xFFFFFFFF);
  static const ink = Color(0xFF191D25);
  static const secondary = Color(0xFF636B7B);
  static const blue = Color(0xFF1765E9);
  static const paleBlue = Color(0xFFE7EFFF);
  static const line = Color(0xFFE9EBF0);
  static const border = line;
}

abstract final class MarketArt {
  static const taskBoard =
      'packages/openmuse_market_ui/assets/task-board-editorial.png';
  static const archify =
      'packages/openmuse_market_ui/assets/archify-editorial.png';
}

ThemeData openMuseMarketTheme() {
  final base = ThemeData.light(useMaterial3: true);
  return base.copyWith(
    scaffoldBackgroundColor: MarketColors.canvas,
    colorScheme: const ColorScheme.light(
      primary: MarketColors.blue,
      surface: MarketColors.surface,
      onSurface: MarketColors.ink,
    ),
    appBarTheme: const AppBarTheme(
      backgroundColor: MarketColors.canvas,
      foregroundColor: MarketColors.ink,
      elevation: 0,
    ),
    textTheme: base.textTheme.apply(
      bodyColor: MarketColors.ink,
      displayColor: MarketColors.ink,
    ),
    inputDecorationTheme: InputDecorationTheme(
      filled: true,
      fillColor: MarketColors.surface,
      border: OutlineInputBorder(
        borderSide: BorderSide.none,
        borderRadius: BorderRadius.circular(16),
      ),
    ),
  );
}

class MarketPlatformBadge extends StatelessWidget {
  const MarketPlatformBadge(this.label, {super.key});
  final String label;

  @override
  Widget build(BuildContext context) => Container(
    padding: const EdgeInsets.symmetric(horizontal: 9, vertical: 5),
    decoration: BoxDecoration(
      color: MarketColors.paleBlue,
      borderRadius: BorderRadius.circular(9),
    ),
    child: Text(
      label,
      style: const TextStyle(
        color: MarketColors.blue,
        fontSize: 11,
        fontWeight: FontWeight.w600,
      ),
    ),
  );
}

class MarketHeroArt extends StatelessWidget {
  const MarketHeroArt({
    super.key,
    required this.asset,
    required this.height,
    required this.child,
    this.onTap,
  });
  final String asset;
  final double height;
  final Widget child;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) => Material(
    color: MarketColors.surface,
    borderRadius: BorderRadius.circular(22),
    clipBehavior: Clip.antiAlias,
    child: InkWell(
      onTap: onTap,
      child: SizedBox(
        height: height,
        width: double.infinity,
        child: Stack(
          fit: StackFit.expand,
          children: [
            Image.asset(asset, fit: BoxFit.cover),
            Container(
              decoration: const BoxDecoration(
                gradient: LinearGradient(
                  colors: [Color(0xDDF7F3EB), Color(0x00F7F3EB)],
                  stops: [0.0, 0.73],
                ),
              ),
            ),
            Padding(padding: const EdgeInsets.all(20), child: child),
          ],
        ),
      ),
    ),
  );
}
