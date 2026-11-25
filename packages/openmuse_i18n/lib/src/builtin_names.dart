import 'package:flutter/widgets.dart';
import 'controller.dart';

/// Translate only Host-owned plugin identities; third-party metadata is data.
String builtinPluginDisplayName(
  BuildContext context,
  String id,
  String fallback,
) {
  final s = stringsOf(context);
  return switch (id) {
    'com.openmuse.auth.gotrue' => s.builtinAuth(),
    'com.openmuse.workspace.cloud' => s.builtinCloud(),
    'com.openmuse.workspace.paired.mobile' => s.builtinPaired(),
    'com.openmuse.remote-workbench' => s.builtinRemote(),
    'com.openmuse.mobile.scheduled-tasks' => s.builtinScheduled(),
    'com.openmuse.mobile.workspace-files' => s.builtinFiles(),
    'com.openmuse.dsh-mobile-agent' => s.builtinAgent(),
    'com.openmuse.speech.input' => s.builtinSpeech(),
    'com.openmuse.mobile-plugin-market' => s.builtinMarket(),
    _ => fallback,
  };
}
