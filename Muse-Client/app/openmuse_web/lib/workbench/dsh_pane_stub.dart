import 'package:flutter/material.dart';

final class OpenMuseDshPane extends StatelessWidget {
  const OpenMuseDshPane({super.key, this.bootstrapPath = '/dsh/'});

  final String bootstrapPath;

  @override
  Widget build(BuildContext context) =>
      const Center(child: Text('DSH 面板需要浏览器运行环境。'));
}
