import 'package:flutter/material.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'contracts.dart';
import 'theme.dart';

final class OpenMuseHostShell extends StatefulWidget {
  const OpenMuseHostShell({
    super.key,
    required this.composition,
    this.localeController,
  });
  final OpenMuseLocaleController? localeController;
  final OpenMuseHostComposition composition;

  @override
  State<OpenMuseHostShell> createState() => _OpenMuseHostShellState();
}

final class _OpenMuseHostShellState extends State<OpenMuseHostShell> {
  late Future<List<WorkspaceSummary>> _workspaces;
  late final _locale = widget.localeController ?? OpenMuseLocaleController();

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
  void dispose() {
    if (widget.localeController == null) _locale.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => ListenableBuilder(
    listenable: _locale,
    builder: (context, _) => OpenMuseLocaleScope(
      controller: _locale,
      child: MaterialApp(
        locale: _locale.locale,
        supportedLocales: _locale.supportedLocales,
        localizationsDelegates: [
          OpenMuseStringsDelegate(_locale.catalog),
          ...GlobalMaterialLocalizations.delegates,
        ],
        debugShowCheckedModeBanner: false,
        title: 'OpenMuse',
        theme: buildOpenMuseTheme(),
        home: Scaffold(
          appBar: AppBar(
            title: const Text('OpenMuse'),
            actions: [
              IconButton(
                key: const ValueKey('workspace-refresh'),
                tooltip: _locale.strings.shellRefreshWorkspaces(),
                icon: const Icon(Icons.refresh),
                onPressed: _reloadWorkspaces,
              ),
              if (composition.accountDevicesBuilder case final builder?)
                IconButton(
                  key: const ValueKey('account-devices-button'),
                  tooltip: _locale.strings.shellAccountDevices(),
                  icon: const Icon(Icons.devices_outlined),
                  onPressed: () => Navigator.of(
                    context,
                  ).push<void>(MaterialPageRoute(builder: builder)),
                ),
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 16),
                child: Center(
                  child: Text(
                    composition.session.accountLabel ??
                        _locale.strings.shellNotSignedIn(),
                  ),
                ),
              ),
              if (composition.onSignOut case final signOut?)
                IconButton(
                  key: const ValueKey('sign-out-button'),
                  tooltip: _locale.strings.shellSignOut(),
                  icon: const Icon(Icons.logout),
                  onPressed: signOut,
                ),
            ],
          ),
          body: FutureBuilder<List<WorkspaceSummary>>(
            future: _workspaces,
            builder: (context, snapshot) {
              if (!composition.session.signedIn)
                return Center(child: Text(_locale.strings.shellSignInCloud()));
              if (snapshot.hasError) {
                return Center(
                  child: Padding(
                    padding: const EdgeInsets.all(24),
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        const Icon(Icons.cloud_off_outlined, size: 40),
                        const SizedBox(height: 12),
                        Text(_locale.strings.shellWorkspacesFailed()),
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
                          label: Text(_locale.strings.commonRetry()),
                        ),
                      ],
                    ),
                  ),
                );
              }
              if (!snapshot.hasData)
                return const Center(child: CircularProgressIndicator());
              if (snapshot.data!.isEmpty)
                return Center(child: Text(_locale.strings.shellNoWorkspaces()));
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
                                  ? _locale.strings.shellDesktopDirect()
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
      ),
    ),
  );
}
