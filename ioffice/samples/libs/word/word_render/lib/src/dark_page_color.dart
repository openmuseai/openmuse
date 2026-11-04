import 'package:flutter/painting.dart';

/// GenOffice screen-only color transform. Authored document colors stay intact.
Color darkPageColor(Color color) {
  final argb = color.toARGB32();
  final r = ((argb >> 16) & 0xff) / 255.0;
  final g = ((argb >> 8) & 0xff) / 255.0;
  final b = (argb & 0xff) / 255.0;
  int channel(double value) =>
      ((0x1e / 255.0 + value.clamp(0.0, 1.0) * (1 - 0x1e / 255.0)) * 255)
          .round();
  return Color.fromARGB(
    (argb >> 24) & 0xff,
    channel(0.333 * r - 0.667 * g - 0.667 * b + 1),
    channel(-0.667 * r + 0.333 * g - 0.667 * b + 1),
    channel(-0.667 * r - 0.667 * g + 0.333 * b + 1),
  );
}
