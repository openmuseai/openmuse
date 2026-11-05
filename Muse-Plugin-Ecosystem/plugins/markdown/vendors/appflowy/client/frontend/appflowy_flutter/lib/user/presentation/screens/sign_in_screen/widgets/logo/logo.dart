import 'package:appflowy/brand/brand.dart';
import 'package:flutter/material.dart';

class AFLogo extends StatelessWidget {
  const AFLogo({
    super.key,
    this.size = const Size.square(36),
  });

  final Size size;

  @override
  Widget build(BuildContext context) {
    return Image.asset(
      Brand.logoAsset,
      width: size.width,
      height: size.height,
      filterQuality: FilterQuality.high,
    );
  }
}
