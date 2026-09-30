import 'package:flutter/material.dart';
import 'contracts.dart';
import 'theme.dart';

final class OpenMuseHostShell extends StatelessWidget {
  const OpenMuseHostShell({super.key, required this.composition});
  final OpenMuseHostComposition composition;

  @override
  Widget build(BuildContext context) => MaterialApp(
    debugShowCheckedModeBanner: false,
    title: 'OpenMuse',
    theme: buildOpenMuseTheme(),
    home: Scaffold(
      appBar: AppBar(
        title: const Text('OpenMuse'),
        actions: [
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16),
            child: Center(
              child: Text(composition.session.accountLabel ?? '未登录'),
            ),
          ),
        ],
      ),
      body: FutureBuilder<List<WorkspaceSummary>>(
        future: composition.workspaceCatalog.listWorkspaces(),
        builder: (context, snapshot) {
          if (!composition.session.signedIn)
            return const Center(child: Text('请登录以访问 Cloud Workspace'));
          if (!snapshot.hasData)
            return const Center(child: CircularProgressIndicator());
          if (snapshot.data!.isEmpty)
            return const Center(child: Text('没有可用的 Workspace'));
          return ListView.separated(
            padding: const EdgeInsets.all(16),
            itemCount: snapshot.data!.length,
            separatorBuilder: (_, _) => const SizedBox(height: 8),
            itemBuilder: (context, index) {
              final workspace = snapshot.data![index];
              return Card(
                child: ListTile(
                  key: ValueKey(workspace.workspaceRef),
                  leading: Icon(
                    workspace.placement == WorkspacePlacement.cloud
                        ? Icons.cloud_outlined
                        : Icons.computer_outlined,
                  ),
                  title: Text(workspace.title),
                  subtitle: Text(
                    workspace.placement == WorkspacePlacement.cloud
                        ? (workspace.runningSessionRef == null
                              ? 'Cloud Workspace'
                              : 'Cloud Workspace · DSH running')
                        : (workspace.runningSessionRef == null
                              ? 'Paired Desktop · 需在线与授权'
                              : 'Paired Desktop · DSH running'),
                  ),
                  trailing: const Icon(Icons.chevron_right),
                  onTap: () => Navigator.of(context).push(
                    MaterialPageRoute<void>(
                      builder: (context) =>
                          composition.workspaceBuilder(context, workspace),
                    ),
                  ),
                ),
              );
            },
          );
        },
      ),
    ),
  );
}
