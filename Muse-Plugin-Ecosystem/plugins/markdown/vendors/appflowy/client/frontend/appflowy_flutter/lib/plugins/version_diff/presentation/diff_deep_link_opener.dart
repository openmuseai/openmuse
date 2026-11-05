import 'package:appflowy/plugins/resource_surface/resource_open_request.dart';
import 'package:appflowy/plugins/version_diff/application/text_version_diff_service.dart';
import 'package:appflowy/plugins/version_diff/presentation/text_diff_plugin.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace/application/tabs/tabs_bloc.dart';
import 'package:flutter/material.dart';

final class MuseDiffDeepLinkOpener {
  const MuseDiffDeepLinkOpener._();

  static Future<void> open({
    required BuildContext context,
    required String path,
    required String sessionCwd,
    required String comparisonId,
    String? changeId,
  }) async {
    try {
      final resource = await MuseLocalResourceRouter().resolve(
        MuseResourceOpenRequest(
          path: path,
          sessionCwd: sessionCwd,
          origin: MuseResourceOpenOrigin.dshConversation,
        ),
      );
      final document = await getIt<MuseTextVersionDiffService>().openComparison(
        resource.file,
        comparisonId,
        initialChangeId: changeId,
      );
      if (!context.mounted) return;
      if (document == null) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(content: Text('Comparison 不存在或版本内容已不可用')),
        );
        return;
      }
      getIt<TabsBloc>().openExternalPlugin(MuseTextDiffPlugin(document));
    } on Object catch (error) {
      if (!context.mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Unable to open comparison: $error')),
      );
    }
  }
}
