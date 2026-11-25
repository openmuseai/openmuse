import 'package:flutter/material.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';

/// A read-only upgrade notice; retained installations have no launch action.
final class LegacyPluginNotice extends StatelessWidget {
  const LegacyPluginNotice({
    super.key,
    required this.store,
    required this.child,
  });

  final MobilePluginStore? store;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    final strings = stringsOf(context);
    final store = this.store;
    if (store == null || store.allowDownloadedPlugins) return child;
    return ListenableBuilder(
      listenable: store,
      builder: (context, _) {
        if (store.quarantinedPluginIds.isEmpty &&
            !store.hasUnreadableLegacyState) {
          return child;
        }
        return Column(
          children: [
            Material(
              color: Theme.of(context).colorScheme.surfaceContainerHighest,
              child: SafeArea(
                bottom: false,
                child: Padding(
                  padding: const EdgeInsets.all(12),
                  child: Text(
                    store.hasUnreadableLegacyState
                        ? strings.format('legacy_plugins.unreadable')
                        : strings.format('legacy_plugins.retained'),
                    key: const ValueKey('legacy-plugin-notice'),
                  ),
                ),
              ),
            ),
            Expanded(child: child),
          ],
        );
      },
    );
  }
}
