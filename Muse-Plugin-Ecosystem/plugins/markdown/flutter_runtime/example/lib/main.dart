import 'package:flutter/material.dart';
import 'package:openmuse_siyuan_client/openmuse_siyuan_client.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(SiYuanAcceptanceApp(kernel: SiYuanKernel()));
}

class SiYuanAcceptanceApp extends StatelessWidget {
  const SiYuanAcceptanceApp({super.key, required this.kernel});
  final SiYuanKernel kernel;
  @override
  Widget build(BuildContext context) => MaterialApp(
    home: Scaffold(
      appBar: AppBar(title: const Text('SiYuan · local client plugin')),
      body: SiYuanClientView(kernel: kernel),
    ),
  );
}
