import 'package:flutter/material.dart';
import 'contracts.dart';
import 'theme.dart';

final class OpenMuseHostShell extends StatefulWidget {
  const OpenMuseHostShell({super.key, required this.composition});
  final OpenMuseHostComposition composition;

  @override
  State<OpenMuseHostShell> createState() => _OpenMuseHostShellState();
}

final class _OpenMuseHostShellState extends State<OpenMuseHostShell> {
  late Future<List<WorkspaceSummary>> _workspaces;

  OpenMuseHostComposition get composition => widget.composition;

  @override
  void initState() {
    super.initState();
    _workspaces = composition.workspaceCatalog.listWorkspaces();
  }

  void _reloadWorkspaces() {
    setState(() {
      _workspaces = composition.workspaceCatalog.listWorkspaces();
    });
  }

  @override
  Widget build(BuildContext context) => MaterialApp(
    debugShowCheckedModeBanner: false,
    title: 'OpenMuse',
    theme: buildOpenMuseTheme(),
    home: Scaffold(
      appBar: AppBar(
        title: const Text('OpenMuse'),
        actions: [
          IconButton(
            key: const ValueKey('workspace-refresh'),
            tooltip: '刷新工作区',
            icon: const Icon(Icons.refresh),
            onPressed: _reloadWorkspaces,
          ),
          if (composition.accountDevicesBuilder case final builder?)
            IconButton(
              key: const ValueKey('account-devices-button'),
              tooltip: '账号设备',
              icon: const Icon(Icons.devices_outlined),
              onPressed: () => Navigator.of(
                context,
              ).push<void>(MaterialPageRoute(builder: builder)),
            ),
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16),
            child: Center(
              child: Text(composition.session.accountLabel ?? '未登录'),
            ),
          ),
          if (composition.onSignOut case final signOut?)
            IconButton(
              key: const ValueKey('sign-out-button'),
              tooltip: '退出登录',
              icon: const Icon(Icons.logout),
              onPressed: signOut,
            ),
        ],
      ),
      body: FutureBuilder<List<WorkspaceSummary>>(
        future: _workspaces,
        builder: (context, snapshot) {
          if (!composition.session.signedIn)
            return const Center(child: Text('请登录以访问 Cloud Workspace'));
          if (snapshot.hasError) {
            return Center(
              child: Padding(
                padding: const EdgeInsets.all(24),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    const Icon(Icons.cloud_off_outlined, size: 40),
                    const SizedBox(height: 12),
                    const Text('工作区加载失败'),
                    const SizedBox(height: 6),
                    Text(
                      '${snapshot.error}',
                      key: const ValueKey('workspace-load-error'),
                      textAlign: TextAlign.center,
                    ),
                    const SizedBox(height: 16),
                    OutlinedButton.icon(
                      key: const ValueKey('workspace-retry'),
                      onPressed: _reloadWorkspaces,
                      icon: const Icon(Icons.refresh),
                      label: const Text('重试'),
                    ),
                  ],
                ),
              ),
            );
          }
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
                              ? 'Desktop · 同账号免码连接'
                              : 'Desktop · DSH running'),
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
