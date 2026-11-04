import '../mobile_build_locale.dart';

import 'package:flutter/material.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_registry_provider/openmuse_registry_provider.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:url_launcher/url_launcher.dart';

import '../speech_model_manager.dart';
import '../workbuddy/workbuddy_models.dart';

Future<void> _openExternal(BuildContext context, Uri uri) async {
  if (await launchUrl(uri, mode: LaunchMode.externalApplication)) return;
  if (context.mounted) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(
          stringsOf(context).mobileUnableToOpenThisLinkRightNowC8600c59(),
        ),
      ),
    );
  }
}

final class MobileMyPage extends StatefulWidget {
  const MobileMyPage({
    super.key,
    required this.themeMode,
    required this.onThemeMode,
    required this.workspaces,
    required this.deviceName,
    this.authentication,
    this.onSignIn,
    this.onSignOut,
    this.onDeleteAccount,
    this.onDevices,
    this.onPlugins,
    this.registryProviders,
    this.speechModels,
  });
  final ThemeMode themeMode;
  final ValueChanged<ThemeMode> onThemeMode;
  final List<WbWorkspace> workspaces;
  final String deviceName;
  final OpenMuseAuthenticationController? authentication;
  final VoidCallback? onSignIn;
  final VoidCallback? onSignOut;
  final Future<void> Function()? onDeleteAccount;
  final VoidCallback? onDevices;
  final VoidCallback? onPlugins;
  final McpRegistryProviders? registryProviders;
  final SpeechModelManager? speechModels;
  @override
  State<MobileMyPage> createState() => _MobileMyPageState();
}

final class _MobileMyPageState extends State<MobileMyPage> {
  late final _package = PackageInfo.fromPlatform();
  @override
  Widget build(BuildContext context) {
    final s = stringsOf(context);
    final colors = Theme.of(context).colorScheme;
    final identity = widget.authentication?.snapshot.identity;
    final config = RuntimeConfigScope.maybeOf(context);
    final filing = config?.appFilingNumber;
    final locale = OpenMuseLocaleScope.maybeOf(context);
    return Scaffold(
      body: ListView(
        padding: const EdgeInsets.fromLTRB(20, 18, 20, 28),
        children: [
          Padding(
            padding: const EdgeInsets.only(bottom: 24),
            child: Text(
              s.mobileAccountA10e8da3(),
              textAlign: TextAlign.center,
              style: const TextStyle(
                fontSize: 22,
                fontWeight: FontWeight.w600,
                letterSpacing: -.4,
              ),
            ),
          ),
          SettingsGroup(
            children: [
              SettingsRow(
                key: const ValueKey('my-account'),
                leading: _AccountAvatar(email: identity?.email, size: 44),
                title: identity?.email ?? s.mobileNotSignedIn3c4f1f91(),
                subtitle: identity == null
                    ? s.mobileSignInToSyncDevicesAndPlugins2fba3876()
                    : s.mobileAccountSettings(),
                onTap: identity == null
                    ? widget.onSignIn
                    : () => Navigator.of(context).push<void>(
                        MaterialPageRoute(
                          builder: (_) => _AccountDetailsPage(
                            email: identity.email,
                            onSignOut: widget.onSignOut,
                            onDelete: widget.onDeleteAccount,
                          ),
                        ),
                      ),
              ),
            ],
          ),
          _SectionCaption(s.mobileAppsServices()),
          SettingsGroup(
            children: [
              SettingsRow(
                key: const ValueKey('my-plugins'),
                icon: Icons.widgets_outlined,
                title: s.marketMyPlugins(),
                subtitle: s.mobilePluginsSummary(),
                onTap: widget.onPlugins,
              ),

              SettingsRow(
                key: const ValueKey('my-speech-models'),
                icon: Icons.mic_none_outlined,
                title: s.mobileModels(),
                onTap: () => Navigator.of(context).push<void>(
                  MaterialPageRoute(
                    builder: (_) =>
                        MobileSpeechModelsPage(manager: widget.speechModels),
                  ),
                ),
              ),
              SettingsRow(
                key: const ValueKey('my-store-provider'),
                icon: Icons.storefront_outlined,
                title: s.mobileStoreProviderTitle(),
                onTap: () => Navigator.of(context).push<void>(
                  MaterialPageRoute(
                    builder: (_) =>
                        _StoreProviderPage(providers: widget.registryProviders),
                  ),
                ),
              ),
            ],
          ),
          _SectionCaption(s.mobilePreferences()),
          SettingsGroup(
            children: [
              Padding(
                padding: const EdgeInsets.fromLTRB(16, 16, 16, 14),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        Icon(
                          Icons.brightness_6_outlined,
                          size: 21,
                          color: colors.onSurfaceVariant,
                        ),
                        const SizedBox(width: 14),
                        Expanded(
                          child: Text(
                            s.mobileAppearance76acb029(),
                            style: const TextStyle(fontSize: 16),
                          ),
                        ),
                      ],
                    ),
                    const SizedBox(height: 14),
                    SizedBox(
                      width: double.infinity,
                      child: SegmentedButton<ThemeMode>(
                        key: const ValueKey('my-theme-mode'),
                        segments: [
                          ButtonSegment(
                            value: ThemeMode.system,
                            label: _fit(s.mobileSystem9a0c51db()),
                          ),
                          ButtonSegment(
                            value: ThemeMode.light,
                            label: _fit(s.mobileLight3f812d90()),
                          ),
                          ButtonSegment(
                            value: ThemeMode.dark,
                            label: _fit(s.mobileDark30112ded()),
                          ),
                        ],
                        selected: {widget.themeMode},
                        showSelectedIcon: false,
                        onSelectionChanged: (value) =>
                            widget.onThemeMode(value.single),
                        style: ButtonStyle(
                          side: WidgetStatePropertyAll(
                            BorderSide(color: colors.outlineVariant),
                          ),
                          backgroundColor: WidgetStateProperty.resolveWith(
                            (states) => states.contains(WidgetState.selected)
                                ? colors.primary
                                : colors.surfaceContainer,
                          ),
                          foregroundColor: WidgetStateProperty.resolveWith(
                            (states) => states.contains(WidgetState.selected)
                                ? colors.onPrimary
                                : colors.onSurfaceVariant,
                          ),
                          textStyle: const WidgetStatePropertyAll(
                            TextStyle(
                              fontSize: 13,
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                          visualDensity: VisualDensity.compact,
                        ),
                      ),
                    ),
                  ],
                ),
              ),
              SettingsRow(
                key: const ValueKey('my-language'),
                icon: Icons.language_outlined,
                title: s.mobileLanguageA8ae4ccd(),
                value: locale?.followsSystem == true
                    ? s.commonFollowSystem()
                    : OpenMuseCatalog.builtIn
                          .option(locale?.tag ?? s.locale)
                          .label,
                onTap: locale == null
                    ? null
                    : () => Navigator.of(context).push<void>(
                        MaterialPageRoute(
                          builder: (_) => const _LanguagePage(),
                        ),
                      ),
              ),
            ],
          ),
          const SizedBox(height: 20),
          SettingsGroup(
            children: [
              SettingsRow(
                key: const ValueKey('my-about'),
                icon: Icons.info_outline,
                title: s.mobileAboutOpenmuse86b0c36e(),
                onTap: () => Navigator.of(context).push<void>(
                  MaterialPageRoute(
                    builder: (_) => _MobileAboutPage(package: _package),
                  ),
                ),
              ),
            ],
          ),
          FutureBuilder<PackageInfo>(
            future: _package,
            builder: (context, result) => Padding(
              key: const ValueKey('my-app-version'),
              padding: const EdgeInsets.only(top: 14),
              child: Text(
                '${s.mobileVersion()} ${_version(result.data)}',
                textAlign: TextAlign.center,
                style: TextStyle(fontSize: 12, color: colors.onSurfaceVariant),
              ),
            ),
          ),
          if (filing != null)
            Center(
              child: TextButton(
                key: const ValueKey('my-app-filing-number'),
                onPressed: () =>
                    _openExternal(context, config!.linkUri(LinkId.filing)),
                child: Text(s.mobileFilingNumber(number: filing)),
              ),
            ),
        ],
      ),
    );
  }

  Widget _fit(String text) => FittedBox(
    fit: BoxFit.scaleDown,
    child: Text(
      text,
      maxLines: 1,
      style: TextStyle(
        fontFamily: Theme.of(context).textTheme.labelLarge?.fontFamily,
      ),
    ),
  );
}

String _version(PackageInfo? info) =>
    info == null ? '—' : '${info.version} (${info.buildNumber})';

final class SettingsGroup extends StatelessWidget {
  const SettingsGroup({super.key, required this.children});
  final List<Widget> children;
  @override
  Widget build(BuildContext context) => Material(
    color: Theme.of(context).colorScheme.surface,
    borderRadius: BorderRadius.circular(18),
    clipBehavior: Clip.antiAlias,
    child: Column(
      children: [
        for (var i = 0; i < children.length; i++) ...[
          if (i > 0)
            Divider(
              height: 1,
              indent: 51,
              color: Theme.of(
                context,
              ).colorScheme.outlineVariant.withValues(alpha: .55),
            ),
          children[i],
        ],
      ],
    ),
  );
}

final class SettingsRow extends StatelessWidget {
  const SettingsRow({
    super.key,
    this.icon,
    this.leading,
    required this.title,
    this.subtitle,
    this.value,
    this.onTap,
    this.trailing,
  });
  final IconData? icon;
  final Widget? leading;
  final String title;
  final String? subtitle;
  final String? value;
  final VoidCallback? onTap;
  final Widget? trailing;
  @override
  Widget build(BuildContext context) {
    final c = Theme.of(context).colorScheme;
    return InkWell(
      onTap: onTap,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 17),
        child: Row(
          children: [
            if (leading != null || icon != null) ...[
              leading ?? Icon(icon, color: c.onSurfaceVariant, size: 21),
              const SizedBox(width: 14),
            ],
            Expanded(
              flex: 5,
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    title,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(
                      fontSize: 16,
                      fontWeight: FontWeight.w500,
                      color: c.onSurface,
                    ),
                  ),
                  if (subtitle != null) ...[
                    const SizedBox(height: 4),
                    Text(
                      subtitle!,
                      maxLines: 2,
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(
                        fontSize: 13,
                        height: 1.4,
                        color: c.onSurfaceVariant,
                      ),
                    ),
                  ],
                ],
              ),
            ),
            if (value != null) ...[
              const SizedBox(width: 12),
              Expanded(
                flex: 4,
                child: Text(
                  value!,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  textAlign: TextAlign.end,
                  style: TextStyle(fontSize: 14, color: c.onSurfaceVariant),
                ),
              ),
            ],
            if (trailing != null)
              trailing!
            else if (onTap != null) ...[
              const SizedBox(width: 8),
              Icon(
                Icons.chevron_right_rounded,
                color: c.onSurfaceVariant,
                size: 20,
              ),
            ],
          ],
        ),
      ),
    );
  }
}

final class _SectionCaption extends StatelessWidget {
  const _SectionCaption(this.text);
  final String text;
  @override
  Widget build(BuildContext context) => Padding(
    padding: const EdgeInsets.fromLTRB(4, 23, 4, 10),
    child: Text(
      text,
      style: TextStyle(
        fontSize: 12,
        fontWeight: FontWeight.w600,
        color: Theme.of(context).colorScheme.onSurfaceVariant,
      ),
    ),
  );
}

final class _AccountAvatar extends StatelessWidget {
  const _AccountAvatar({this.email, this.size = 80});
  final String? email;
  final double size;
  @override
  Widget build(BuildContext context) {
    final c = Theme.of(context).colorScheme;
    return CircleAvatar(
      radius: size / 2,
      backgroundColor: c.primary.withValues(alpha: .12),
      child: email?.isNotEmpty == true
          ? Text(
              email!.substring(0, 1).toUpperCase(),
              style: TextStyle(
                fontSize: size * .38,
                fontWeight: FontWeight.w600,
                color: c.primary,
              ),
            )
          : Icon(
              Icons.person_outline_rounded,
              color: c.primary,
              size: size * .45,
            ),
    );
  }
}

final class MobileSettingsPage extends StatelessWidget {
  const MobileSettingsPage({
    super.key,
    required this.title,
    required this.children,
    this.footer,
  });
  final String title;
  final List<Widget> children;
  final Widget? footer;
  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(
      leading: BackButton(
        key: const ValueKey('settings-back'),
        onPressed: () => Navigator.of(context).maybePop(),
      ),
      title: Text(
        title,
        style: const TextStyle(fontSize: 18, fontWeight: FontWeight.w600),
      ),
      centerTitle: true,
    ),
    bottomNavigationBar: footer == null
        ? null
        : SafeArea(
            top: false,
            child: Padding(
              padding: const EdgeInsets.fromLTRB(20, 4, 20, 20),
              child: footer,
            ),
          ),
    body: SafeArea(
      top: false,
      child: ListView(
        padding: const EdgeInsets.fromLTRB(20, 20, 20, 32),
        children: children,
      ),
    ),
  );
}

final class _AccountDetailsPage extends StatefulWidget {
  const _AccountDetailsPage({
    required this.email,
    this.onSignOut,
    this.onDelete,
  });
  final String? email;
  final VoidCallback? onSignOut;
  final Future<void> Function()? onDelete;
  @override
  State<_AccountDetailsPage> createState() => _AccountDetailsPageState();
}

final class _AccountDetailsPageState extends State<_AccountDetailsPage> {
  bool busy = false;
  @override
  Widget build(BuildContext context) {
    final s = stringsOf(context);
    final c = Theme.of(context).colorScheme;
    return MobileSettingsPage(
      title: s.mobileAccountSettings(),
      footer: widget.onDelete == null
          ? null
          : Center(
              heightFactor: 1,
              child: TextButton(
                key: const ValueKey('my-delete-account'),
                onPressed: busy ? null : _delete,
                style: TextButton.styleFrom(foregroundColor: c.error),
                child: Text(s.mobileDeleteAccount13c11365()),
              ),
            ),
      children: [
        Padding(
          padding: const EdgeInsets.only(top: 16, bottom: 32),
          child: Center(child: _AccountAvatar(email: widget.email)),
        ),
        SettingsGroup(
          children: [
            SettingsRow(
              icon: Icons.alternate_email_rounded,
              title: s.mobileEmailAddress(),
              subtitle: widget.email,
            ),
          ],
        ),
        const SizedBox(height: 28),
        SizedBox(
          width: double.infinity,
          child: OutlinedButton(
            key: const ValueKey('my-sign-out'),
            onPressed: busy || widget.onSignOut == null
                ? null
                : () {
                    Navigator.of(context).pop();
                    widget.onSignOut!();
                  },
            style: OutlinedButton.styleFrom(
              minimumSize: const Size.fromHeight(50),
              foregroundColor: c.onSurface,
              backgroundColor: c.surface,
              shape: RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(16),
              ),
              side: BorderSide.none,
            ),
            child: Text(s.mobileSignOutB67e6932()),
          ),
        ),
      ],
    );
  }

  Future<void> _delete() async {
    final s = stringsOf(context);
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (dialog) => AlertDialog(
        title: Text(s.mobilePermanentlyDeleteYourAccount4524c47f()),
        content: Text(s.mobileThisDeletesYourOpenmuseAccountTheWorDaf1a68b()),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(dialog, false),
            child: Text(s.mobileCancelCda6010b()),
          ),
          TextButton(
            key: const ValueKey('confirm-delete-account'),
            onPressed: () => Navigator.pop(dialog, true),
            style: TextButton.styleFrom(
              foregroundColor: Theme.of(context).colorScheme.error,
            ),
            child: Text(s.mobileDeletePermanently0cc7ee72()),
          ),
        ],
      ),
    );
    if (confirmed != true || !mounted) return;
    setState(() => busy = true);
    try {
      await widget.onDelete!();
      if (mounted) Navigator.of(context).pop();
    } catch (error) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              s.mobileAccountDeletionDidNotComplete2bb8b0dd(value1: error),
            ),
          ),
        );
      }
    } finally {
      if (mounted) setState(() => busy = false);
    }
  }
}

final class _LanguagePage extends StatelessWidget {
  const _LanguagePage();
  @override
  Widget build(BuildContext context) {
    final locale = OpenMuseLocaleScope.maybeOf(context)!;
    final s = stringsOf(context);
    final options = [
      ('system', s.commonFollowSystem()),
      for (final option in locale.options) (option.tag, option.label),
    ];
    return MobileSettingsPage(
      title: s.mobileLanguageSelection(),
      children: [
        SettingsGroup(
          children: [
            for (final option in options)
              SettingsRow(
                title: option.$2,
                trailing:
                    (locale.followsSystem ? 'system' : locale.tag) == option.$1
                    ? Icon(
                        Icons.check_rounded,
                        color: Theme.of(context).colorScheme.primary,
                      )
                    : const SizedBox(width: 24),
                onTap: () {
                  if (option.$1 == 'system') {
                    locale.followSystem();
                  } else {
                    locale.selectTag(option.$1);
                  }
                  Navigator.of(context).pop();
                },
              ),
          ],
        ),
      ],
    );
  }
}

final class MobileSpeechModelsPage extends StatelessWidget {
  const MobileSpeechModelsPage({super.key, this.manager});
  final SpeechModelManager? manager;
  @override
  Widget build(BuildContext context) {
    final model = manager;
    if (model == null) {
      return MobileSettingsPage(
        title: stringsOf(context).mobileModels(),
        children: [Text(stringsOf(context).mobileModelUnavailable())],
      );
    }
    return ListenableBuilder(
      listenable: model,
      builder: (context, _) {
        final s = stringsOf(context);
        final c = Theme.of(context).colorScheme;
        return MobileSettingsPage(
          title: s.mobileModels(),
          children: [
            SettingsGroup(
              children: [
                Padding(
                  padding: const EdgeInsets.all(20),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Icon(
                        Icons.graphic_eq_rounded,
                        size: 28,
                        color: c.primary,
                      ),
                      const SizedBox(height: 18),
                      Text(
                        s.mobileSpeechModelName(),
                        style: TextStyle(
                          fontSize: 20,
                          fontWeight: FontWeight.w600,
                          color: c.onSurface,
                        ),
                      ),
                      const SizedBox(height: 8),
                      Text(
                        s.mobileModelDescription(),
                        style: TextStyle(
                          fontSize: 14,
                          height: 1.5,
                          color: c.onSurfaceVariant,
                        ),
                      ),
                      const SizedBox(height: 20),
                      Row(
                        children: [
                          Icon(
                            model.installed
                                ? Icons.check_circle_outline
                                : Icons.download_rounded,
                            size: 17,
                            color: c.primary,
                          ),
                          const SizedBox(width: 8),
                          Flexible(
                            child: Text(
                              model.busy
                                  ? s.mobileModelDownloading()
                                  : model.installed
                                  ? s.mobileModelInstalled()
                                  : s.mobileModelNotInstalled(),
                              style: const TextStyle(fontSize: 14),
                            ),
                          ),
                        ],
                      ),
                      const SizedBox(height: 7),
                      Text(
                        s.mobileModelStorage(),
                        style: TextStyle(
                          fontSize: 12,
                          color: c.onSurfaceVariant,
                        ),
                      ),
                      if (model.busy) ...[
                        const SizedBox(height: 18),
                        LinearProgressIndicator(
                          value: model.removing ? null : model.progress,
                          borderRadius: BorderRadius.circular(3),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          '${(model.progress * 100).round()}%',
                          style: TextStyle(
                            fontSize: 12,
                            color: c.onSurfaceVariant,
                          ),
                        ),
                      ],
                      if (model.failure != null) ...[
                        const SizedBox(height: 16),
                        Text(
                          model.failure == 'cancelled'
                              ? s.mobileModelDownloadCancelled()
                              : model.failure == 'busy'
                              ? s.mobileModelBusy()
                              : s.mobileModelDownloadFailed(),
                          style: TextStyle(
                            color: model.failure == 'cancelled'
                                ? c.onSurfaceVariant
                                : c.error,
                            fontSize: 13,
                            height: 1.4,
                          ),
                        ),
                      ],
                      const SizedBox(height: 22),
                      SizedBox(
                        width: double.infinity,
                        child: model.busy
                            ? OutlinedButton(
                                onPressed: model.removing ? null : model.cancel,
                                child: Text(s.mobileCancelDownload()),
                              )
                            : model.installed
                            ? OutlinedButton(
                                key: const ValueKey('speech-model-remove'),
                                onPressed: model.remove,
                                child: Text(s.mobileModelRemove()),
                              )
                            : FilledButton.icon(
                                key: const ValueKey('speech-model-install'),
                                onPressed: model.install,
                                icon: const Icon(
                                  Icons.download_rounded,
                                  size: 18,
                                ),
                                label: Text(s.mobileInstallModel()),
                              ),
                      ),
                    ],
                  ),
                ),
              ],
            ),
          ],
        );
      },
    );
  }
}

final class _StoreProviderPage extends StatefulWidget {
  const _StoreProviderPage({this.providers});
  final McpRegistryProviders? providers;
  @override
  State<_StoreProviderPage> createState() => _StoreProviderPageState();
}

final class _StoreProviderPageState extends State<_StoreProviderPage> {
  final input = TextEditingController();
  bool busy = false;
  String? failure;
  @override
  void dispose() {
    input.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final s = stringsOf(context);
    final providers = widget.providers;
    return MobileSettingsPage(
      title: s.mobileStoreProviderTitle(),
      children: [
        Text(
          s.mobileStoreProviderDescription(),
          style: TextStyle(
            fontSize: 14,
            height: 1.5,
            color: Theme.of(context).colorScheme.onSurfaceVariant,
          ),
        ),
        const SizedBox(height: 20),
        if (providers != null)
          SettingsGroup(
            children: [
              for (final uri in providers.origins)
                SettingsRow(
                  title: uri.host,
                  subtitle: uri.toString(),
                  trailing: providers.selected == uri
                      ? Icon(
                          Icons.check_rounded,
                          color: Theme.of(context).colorScheme.primary,
                        )
                      : null,
                  onTap: busy ? null : () => _select(uri.toString()),
                ),
            ],
          ),
        const SizedBox(height: 28),
        Text(
          s.marketRegistryAddress(),
          style: const TextStyle(fontSize: 13, fontWeight: FontWeight.w600),
        ),
        const SizedBox(height: 8),
        TextField(
          controller: input,
          enabled: !busy && providers != null,
          keyboardType: TextInputType.url,
          autocorrect: false,
          decoration: const InputDecoration(
            hintText: 'https://',
            border: OutlineInputBorder(),
          ),
        ),
        if (failure != null)
          Padding(
            padding: const EdgeInsets.only(top: 8),
            child: Text(
              failure!,
              style: TextStyle(
                color: Theme.of(context).colorScheme.error,
                fontSize: 13,
              ),
            ),
          ),
        const SizedBox(height: 16),
        FilledButton(
          onPressed: busy || providers == null
              ? null
              : () => _select(input.text),
          child: Text(s.mobileSave()),
        ),
      ],
    );
  }

  Future<void> _select(String value) async {
    setState(() {
      busy = true;
      failure = null;
    });
    try {
      await widget.providers!.select(value);
      if (mounted) setState(() {});
    } catch (_) {
      if (mounted) {
        setState(() => failure = stringsOf(context).mobileInvalidProvider());
      }
    } finally {
      if (mounted) setState(() => busy = false);
    }
  }
}

final class _MobileAboutPage extends StatelessWidget {
  const _MobileAboutPage({required this.package});
  final Future<PackageInfo> package;
  @override
  Widget build(BuildContext context) {
    final s = stringsOf(context);
    final config = RuntimeConfigScope.maybeOf(context);
    final filing = config?.appFilingNumber;
    final locale = Localizations.localeOf(context).toLanguageTag();
    return MobileSettingsPage(
      title: s.mobileAboutOpenmuse86b0c36e(),
      children: [
        const Text(
          'OpenMuse',
          style: TextStyle(
            fontSize: 26,
            fontWeight: FontWeight.w600,
            letterSpacing: -.6,
          ),
        ),
        const SizedBox(height: 6),
        FutureBuilder<PackageInfo>(
          future: package,
          builder: (_, result) => Text(
            '${s.mobileVersion()} ${_version(result.data)}',
            style: TextStyle(
              color: Theme.of(context).colorScheme.onSurfaceVariant,
              fontSize: 13,
            ),
          ),
        ),
        const SizedBox(height: 18),
        Text(
          s.mobileOperatedByDongguanChengxuanElectronic4bdff712(),
          style: const TextStyle(fontSize: 13, height: 1.5),
        ),
        if (filing != null)
          Text(
            s.mobileFilingNumber(number: filing),
            key: const ValueKey('app-filing-number'),
          ),
        const SizedBox(height: 28),
        SettingsGroup(
          children: [
            SettingsRow(
              title: s.mobilePrivacyPolicy9233e2fb(),
              icon: Icons.privacy_tip_outlined,
              onTap: config == null
                  ? null
                  : () => _openExternal(
                      context,
                      config.linkUri(LinkId.privacy, locale: locale),
                    ),
            ),
            SettingsRow(
              title: s.mobileTermsOfServiceEd11dce1(),
              icon: Icons.description_outlined,
              onTap: config == null
                  ? null
                  : () => _openExternal(
                      context,
                      config.linkUri(LinkId.terms, locale: locale),
                    ),
            ),
            SettingsRow(
              title: s.mobileContactSupportE159c0ac(),
              icon: Icons.mail_outline,
              subtitle: config?.supportEmail,
              onTap: config == null
                  ? null
                  : () => _openExternal(context, config.supportUri),
            ),
            SettingsRow(
              title: s.mobileAccountDeletionInformation85b039b1(),
              icon: Icons.person_remove_outlined,
              onTap: config == null
                  ? null
                  : () => _openExternal(
                      context,
                      config.linkUri(LinkId.accountDeletion, locale: locale),
                    ),
            ),
          ],
        ),
      ],
    );
  }
}
