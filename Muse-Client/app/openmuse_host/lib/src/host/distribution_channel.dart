/// Compiled into the binary. Remote configuration cannot relax Store limits.
const macAppStoreDistribution = bool.fromEnvironment('OPENMUSE_MAC_APP_STORE');

bool hostCommandAllowed(
  String command, {
  bool appStore = macAppStoreDistribution,
}) {
  if (!appStore) return true;
  return !command.startsWith('terminal.') &&
      !command.startsWith('plugin.market.') &&
      command != 'plugin.installFromCatalog';
}
