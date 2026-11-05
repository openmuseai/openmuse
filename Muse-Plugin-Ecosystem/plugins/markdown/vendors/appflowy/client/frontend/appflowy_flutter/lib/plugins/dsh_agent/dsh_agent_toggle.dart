import 'package:appflowy/generated/flowy_svgs.g.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_agent_controller.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_sidecar.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:flowy_infra_ui/style_widget/icon_button.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class DshAgentToggle extends StatelessWidget {
  const DshAgentToggle({super.key});

  @override
  Widget build(BuildContext context) {
    final controller = context.watch<DshAgentController>();
    return FlowyIconButton(
      tooltipText:
          controller.open ? 'Close DeepSeek Agent' : 'Open DeepSeek Agent',
      width: 24,
      icon: FlowySvg(
        FlowySvgs.m_home_ai_chat_icon_m,
        size: const Size.square(16),
        color: controller.open
            ? Theme.of(context).colorScheme.primary
            : Theme.of(context).iconTheme.color,
      ),
      onPressed: () async {
        final next = !controller.open;
        if (!next) {
          controller.setOpen(false);
          return;
        }
        // Mark launching before the panel mounts so the WebView does not
        // hit 127.0.0.1:3080 while the sidecar is still starting.
        controller.setLaunching(true);
        controller.setOpen(true);
        try {
          await getIt<DshSidecar>().ensureStarted();
        } catch (_) {
          // Panel shows the sidecar error; keep the shell open so the user can retry.
        }
      },
    );
  }
}
