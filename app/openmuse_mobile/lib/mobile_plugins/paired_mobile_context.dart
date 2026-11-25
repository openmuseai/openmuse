import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

import '../workbuddy/workbuddy_models.dart';

/// Host-supplied context for first-party paired capabilities. New plugins can
/// opt in without adding plugin-ID conditionals to the Shell.
abstract interface class PairedMobileContextContributor {
  void updatePairedContext({
    required PairedDesktopConnection? connection,
    required List<WbWorkspace> workspaces,
    required String? selectedWorkspaceId,
  });
}
