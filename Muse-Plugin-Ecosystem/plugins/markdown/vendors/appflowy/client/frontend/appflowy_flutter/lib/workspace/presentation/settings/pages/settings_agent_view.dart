import 'package:appflowy/plugins/dsh_agent/dsh_agent_controller.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_sidecar.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace/presentation/settings/shared/setting_list_tile.dart';
import 'package:appflowy/workspace/presentation/settings/shared/settings_body.dart';
import 'package:appflowy/workspace/presentation/settings/shared/settings_category.dart';
import 'package:appflowy/workspace/presentation/widgets/toggle/toggle.dart';
import 'package:flutter/material.dart';

class SettingsAgentView extends StatelessWidget {
  const SettingsAgentView({super.key});

  @override
  Widget build(BuildContext context) {
    final controller = getIt<DshAgentController>();
    return AnimatedBuilder(
      animation: controller,
      builder: (context, _) {
        return SettingsBody(
          title: 'Agent',
          description: '控制主编辑区旁的 DeepSeek Agent 面板。默认开启。',
          children: [
            SettingsCategory(
              title: '面板',
              children: [
                SettingListTile(
                  label: '显示 Agent',
                  hint: '关闭后主编辑区不再显示 Agent 面板，可随时在此重新打开。',
                  trailing: [
                    Toggle(
                      value: controller.open,
                      onChanged: (enabled) => _setOpen(controller, enabled),
                    ),
                  ],
                ),
              ],
            ),
          ],
        );
      },
    );
  }

  Future<void> _setOpen(DshAgentController controller, bool enabled) async {
    if (!enabled) {
      controller.setOpen(false);
      return;
    }
    controller.setLaunching(true);
    controller.setOpen(true);
    try {
      await getIt<DshSidecar>().ensureStarted();
    } catch (_) {}
  }
}
