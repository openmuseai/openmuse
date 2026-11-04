import 'support/device_result.dart';
import 'dart:io';
import 'package:path_provider/path_provider.dart';
import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:openmuse_mobile/main.dart' as mobile;
import 'package:openmuse_mobile/workbuddy/workbuddy_shell.dart';
import 'package:openmuse_mobile/mobile_plugins/mobile_workspace_mirror_client.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue.dart';
import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:openmuse_mobile/native_dsh_page.dart';

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  recordDeviceResult(binding);
  testWidgets(
    'real mobile boot: explicit US selection, locale, draft, legal and capability gates',
    (tester) async {
      final source =
          jsonDecode(
                await rootBundle.loadString(
                  'packages/openmuse_runtime_config/assets/releases/overseas.json',
                ),
              )
              as Map<String, dynamic>;
      final us = (source['regions'] as List).first as Map;
      final snapshot = us['bundle']['snapshot'] as Map;
      // Isolate acceptance preferences and authentication from the owner's account.
      snapshot['deployment']['id'] = 'global-us-device-acceptance';
      snapshot['scope']['variantId'] = 'device-acceptance-v2';
      snapshot['authentication']['sessionStorageKey'] =
          'openmuse.auth.device-acceptance.v1';
      source['id'] = 'overseas-device-acceptance';
      final release = InstalledRelease.parse(jsonEncode(source));
      // Each run starts signed out; a failed prior run may have left this
      // acceptance-only session behind. Account restoration is a separate
      // transition and must not race the locale/state-retention assertions.
      final testBundle = release.regions
          .singleWhere((r) => r.id == 'us')
          .bundle!;
      await const FlutterSecureValueStore().delete(
        testBundle.snapshot.authSessionStorageKey,
      );
      final regionStore = MemoryConfigStore();
      await tester.pumpWidget(
        OpenMuseRegionBootstrap(
          loadRelease: () async => release,
          store: (_) async => regionStore,
          boot: mobile.bootOpenMuseMobile,
        ),
      );
      await _wait(tester, find.byKey(const ValueKey('region-us')));
      expect(
        tester
            .widget<ListTile>(find.byKey(const ValueKey('region-eu')))
            .enabled,
        isFalse,
      );
      expect(
        tester
            .widget<ListTile>(find.byKey(const ValueKey('region-apac')))
            .enabled,
        isFalse,
      );
      expect(
        tester
            .widget<FilledButton>(find.byKey(const ValueKey('region-continue')))
            .onPressed,
        isNull,
      );
      await tester.tap(find.byKey(const ValueKey('region-us')));
      await tester.pump();
      await tester.tap(find.byKey(const ValueKey('region-continue')));
      await _wait(tester, find.byKey(const ValueKey('legal-checkbox')));
      expect(find.byType(mobile.OpenMuseMobileApplication), findsNothing);
      await tester.tap(find.byKey(const ValueKey('legal-checkbox')));
      await tester.pump();
      await tester.tap(find.byKey(const ValueKey('legal-accept')));
      await _wait(tester, find.byType(mobile.OpenMuseMobileApplication));
      final app = tester.widget<mobile.OpenMuseMobileApplication>(
        find.byType(mobile.OpenMuseMobileApplication),
      );
      expect(
        app.runtime,
        isNotNull,
        reason: 'Native plugins must receive the bootstrapped runtime',
      );
      expect(
        app.officeEngine,
        isNull,
        reason: 'Store application must not load unready native engines',
      );
      expect(app.runtime!.network.json['scope']['dataRegion'], 'us');
      final locale = app.localeController!;
      expect(
        locale.tag,
        'en',
        reason: 'The overseas installation starts in English',
      );
      await _wait(tester, find.byKey(const ValueKey('wb-tab-Tasks')));
      await tester.tap(find.byKey(const ValueKey('wb-tab-Tasks')));
      await _wait(tester, find.byKey(const ValueKey('wb-composer')));
      final composer = find.byKey(const ValueKey('wb-composer'));
      // Capture the actual TextInput client id in profile/AOT tests too. The
      // unregistered tester's -1 client shortcut only works in debug mode.
      tester.testTextInput.register();
      try {
        await tester.enterText(composer, 'Device draft 原始内容');
        FocusManager.instance.primaryFocus?.unfocus();
        await tester.pumpAndSettle();
      } finally {
        tester.testTextInput.unregister();
      }
      final editor = tester.widget<TextField>(composer).controller;
      expect(editor!.text, 'Device draft 原始内容');
      final shellState = tester.state(find.byType(WorkBuddyShell));
      locale.selectTag('zh-Hans');
      await tester.pumpAndSettle();
      expect(locale.tag, 'zh-Hans');
      expect(tester.state(find.byType(WorkBuddyShell)), same(shellState));
      expect(tester.widget<TextField>(composer).controller, same(editor));
      expect(editor.text, 'Device draft 原始内容');
      locale.selectTag('en');
      await locale.saved;
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('wb-tab-Account')));
      await _wait(tester, find.byKey(const ValueKey('my-language')));
      expect(find.byKey(const ValueKey('my-app-filing-number')), findsNothing);
      await tester.ensureVisible(find.byKey(const ValueKey('my-about')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const ValueKey('my-about')));
      await tester.pumpAndSettle();
      expect(find.text('Privacy Policy'), findsOneWidget);
      expect(find.text('Terms of Service'), findsOneWidget);
      expect(tester.takeException(), isNull);
      if (const bool.fromEnvironment('OPENMUSE_ACCEPTANCE_LIVE_ACCOUNT')) {
        Navigator.of(tester.element(find.text('Privacy Policy'))).pop();
        await tester.pumpAndSettle();
        await _liveAccount(tester, app);
      }
      debugPrint(
        'GLOBALIZATION_DEVICE_ACCEPTANCE_PASS region=us locale=en,zh-Hans draft=retained runtime=injected office=absent',
      );
      await tester.pumpWidget(const SizedBox.shrink());
      await tester.pumpAndSettle();
    },
  );
}

Future<void> _wait(WidgetTester tester, Finder finder) async {
  final deadline = DateTime.now().add(const Duration(seconds: 60));
  while (DateTime.now().isBefore(deadline)) {
    await tester.pump();
    if (finder.evaluate().isNotEmpty) return;
    await Future<void>.delayed(const Duration(milliseconds: 100));
  }
  expect(finder, findsWidgets);
}

Future<void> _liveAccount(
  WidgetTester tester,
  mobile.OpenMuseMobileApplication app,
) async {
  final support = await getApplicationSupportDirectory();
  final credentialsFile = File('${support.path}/acceptance-credentials.json');
  for (var i = 0; i < 120 && !await credentialsFile.exists(); i++) {
    await Future<void>.delayed(const Duration(milliseconds: 500));
  }
  expect(
    await credentialsFile.exists(),
    isTrue,
    reason: 'Supply credentials through the test app private directory',
  );
  final credentials = jsonDecode(await credentialsFile.readAsString()) as Map;
  await credentialsFile.delete();
  final auth = app.authenticationPlugin.authentication;
  await auth.signInWithPassword(
    credentials['email'] as String,
    credentials['password'] as String,
  );
  expect(
    auth.snapshot.isAuthenticated,
    isTrue,
    reason: 'US account must authenticate and bootstrap the cloud workspace',
  );
  final workspaces = await app.cloudWorkspacePlugin.service.listWorkspaces();
  expect(workspaces, isNotEmpty);
  final directory = app.pairedDesktopPlugin.controller.directory!;
  await directory.reconcile();
  for (
    var i = 0;
    i < 120 &&
        (directory.snapshot.loading || directory.snapshot.devices.isEmpty);
    i++
  ) {
    await tester.pump(const Duration(milliseconds: 250));
  }
  expect(directory.snapshot.devices, isNotEmpty);
  expect(directory.snapshot.failureMessage, isNull);
  debugPrint(
    'LIVE_ACCOUNT_DEVICE_ACCEPTANCE_PASS signIn=true workspaces=${workspaces.length} deviceDirectory=true',
  );
  if (const bool.fromEnvironment('OPENMUSE_ACCEPTANCE_LIVE_DESKTOP')) {
    final paired = app.pairedDesktopPlugin.controller;
    final linux = directory.snapshot.devices.where(
      (device) =>
          device.online &&
          device.platform.toLowerCase() == 'linux' &&
          device.supportsPairedDesktop,
    );
    expect(linux, isNotEmpty, reason: 'The US Linux desktop must be online');
    expect(
      await paired.connectDevice(linux.first),
      isTrue,
      reason: paired.snapshot.failureMessage,
    );
    final mirror = MobileWorkspaceMirrorClient(paired.snapshot.connection!);
    try {
      expect((await mirror.mounts())['mounts'] as List, isNotEmpty);
      const marker = 'OpenMuse-User-Guide.md';
      final found = (await mirror.search(marker))['entries'] as List;
      final entry = found.cast<Map>().singleWhere((e) => e['name'] == marker);
      final file = await mirror.readResource(entry['resourceRef'] as String);
      expect(
        utf8.decode(file['bytes'] as List<int>),
        await rootBundle.loadString(
          'packages/openmuse_resource_center_host/assets/welcome/$marker',
        ),
      );
      debugPrint(
        'LIVE_DESKTOP_DEVICE_ACCEPTANCE_PASS platform=${Platform.operatingSystem} accountGrant=true mounts=true linuxFileRead=true',
      );
    } finally {
      mirror.close();
    }
    if (const bool.fromEnvironment('OPENMUSE_ACCEPTANCE_LIVE_AI')) {
      await _liveAi(tester, app);
    }
  }
  if (const bool.fromEnvironment('OPENMUSE_ACCEPTANCE_DELETE_ACCOUNT')) {
    await tester.pumpAndSettle();
    if (!directory.snapshot.devices.any(
      (device) => device.supportsPairedDesktop,
    )) {
      await _wait(tester, find.text('Connect Desktop'));
    }
    // First sign-in may offer Desktop selection. Dismiss that optional sheet
    // before navigating to Account, just as the user can dismiss it with Back.
    Navigator.of(
      tester.element(find.byType(WorkBuddyShell)),
    ).popUntil((route) => route.isFirst);
    await tester.pumpAndSettle();
    final accountTab = find.byKey(const ValueKey('wb-tab-Account'));
    await tester.tap(accountTab);
    await tester.pumpAndSettle();
    await _wait(tester, find.byKey(const ValueKey('my-language')));
    await tester.tap(find.byKey(const ValueKey('my-account')));
    await tester.pumpAndSettle();
    final deletion = find.byKey(const ValueKey('my-delete-account'));
    await tester.scrollUntilVisible(
      deletion,
      250,
      scrollable: find.byType(Scrollable).last,
    );
    await tester.pumpAndSettle();
    await tester.tap(deletion);
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const ValueKey('confirm-delete-account')));
    for (var i = 0; i < 120 && auth.snapshot.isAuthenticated; i++) {
      await tester.pump(const Duration(milliseconds: 500));
    }
    expect(
      auth.snapshot.isAuthenticated,
      isFalse,
      reason:
          'Account deletion must be confirmed by the server and clear the app session',
    );
    debugPrint(
      'LIVE_ACCOUNT_DELETION_DEVICE_ACCEPTANCE_PASS serverConfirmed=true signedOut=true',
    );
  } else {
    await auth.signOut();
  }
}

/// Exercises the production native conversation and permission UI against the
/// review account's Desktop. Never selects or writes a provider/model/key.
Future<void> _liveAi(
  WidgetTester tester,
  mobile.OpenMuseMobileApplication app,
) async {
  final descriptor =
      app.pairedDesktopPlugin.controller.snapshot.connection!.session;
  final client = DshNativeGatewayClient(
    origin: Uri.parse(descriptor.origin),
    bootstrapPath: descriptor.path,
    allowInsecureLoopback: descriptor.allowInsecureLoopback,
    allowInsecurePrivateNetworkForTesting:
        descriptor.allowInsecurePrivateNetworkForTesting,
  );
  await client.initialize();
  final workspaces = await client.listWorkspaces();
  expect(workspaces, isNotEmpty);
  final created = await client.createSession(
    workspaceId: workspaces.first.workspaceId,
  );
  final options = await client.sessionOptions(created.sessionId);
  expect(
    options.defaultModel.provider,
    'opencode-go',
    reason: 'Use the provider configured by the owner without changing it',
  );
  final selected = options.models.singleWhere(
    (m) =>
        m.provider == options.defaultModel.provider &&
        m.id == options.defaultModel.model,
  );
  final handle = NativeDshSessionHandle();
  final navigation = Navigator.of(tester.element(find.byType(WorkBuddyShell)));
  navigation.popUntil((route) => route.isFirst);
  var declined = false;
  const marker = 'OPENMUSE_LAUNCH_LINK_OK';
  const prompt =
      'Reply with only OPENMUSE_LAUNCH_LINK_OK. Do not use any tools or modify files.';
  final route = navigation.push<void>(
    MaterialPageRoute(
      builder: (_) => NativeDshPage(
        session: descriptor,
        workspaceTitle: workspaces.first.title,
        requestedSessionId: created.sessionId,
        handle: handle,
        initialPrompt: prompt,
        onInitialPromptDeclined: (_) => declined = true,
      ),
    ),
  );
  await _wait(tester, find.byKey(const ValueKey('ai-sharing-allow')));
  expect(find.textContaining(selected.providerName), findsWidgets);
  expect(find.textContaining(selected.name), findsWidgets);
  await tester.tap(find.widgetWithText(TextButton, 'Cancel'));
  await tester.pump();
  await Future<void>.delayed(const Duration(milliseconds: 200));
  expect(declined, isTrue);
  expect(
    handle.snapshot!.rows.where(
      (r) =>
          r.kind == DshConversationRowKind.user ||
          r.kind == DshConversationRowKind.optimisticUser,
    ),
    isEmpty,
    reason: 'Declining sharing must not append or submit a prompt',
  );
  final afterCancel = (await client.listSessions()).singleWhere(
    (s) => s.sessionId == created.sessionId,
  );
  expect(
    afterCancel.blank,
    isTrue,
    reason: 'The remote journal must remain blank after cancellation',
  );
  final sending = handle.send(prompt);
  await _wait(tester, find.byKey(const ValueKey('ai-sharing-allow')));
  await tester.tap(find.byKey(const ValueKey('ai-sharing-allow')));
  await tester.pump();
  await sending;
  final deadline = DateTime.now().add(const Duration(minutes: 3));
  while (DateTime.now().isBefore(deadline)) {
    await tester.pump();
    if (handle.snapshot!.rows.any(
      (r) =>
          r.kind == DshConversationRowKind.assistant && r.text.contains(marker),
    )) {
      break;
    }
    await Future<void>.delayed(const Duration(milliseconds: 200));
  }
  expect(
    handle.snapshot!.rows.any(
      (r) =>
          r.kind == DshConversationRowKind.assistant && r.text.contains(marker),
    ),
    isTrue,
    reason: 'A real assistant response must arrive through the paired Desktop',
  );
  debugPrint(
    'LIVE_AI_DEVICE_ACCEPTANCE_PASS provider=opencode-go cancel=remoteBlank allow=assistantReply modelUnchanged=true',
  );
  final assistant = handle.snapshot!.rows.lastWhere(
    (r) =>
        r.kind == DshConversationRowKind.assistant && r.text.contains(marker),
  );
  final report = find.byKey(ValueKey('dsh-native.report:${assistant.key}'));
  await tester.ensureVisible(report);
  await tester.tap(report);
  await _wait(tester, find.byKey(const ValueKey('ai-report-submit')));
  await tester.tap(find.widgetWithText(TextButton, 'Cancel'));
  await tester.pumpAndSettle();
  await tester.tap(report);
  await _wait(tester, find.byKey(const ValueKey('ai-report-reason')));
  await tester.tap(find.byKey(const ValueKey('ai-report-reason')));
  await tester.pumpAndSettle();
  await tester.tap(find.text('Other').last);
  await tester.pumpAndSettle();
  final comment = find.byKey(const ValueKey('ai-report-comment'));
  await tester.ensureVisible(comment);
  tester.testTextInput.register();
  try {
    await tester.enterText(
      comment,
      'OpenMuse native report acceptance - harmless QA response.',
    );
    FocusManager.instance.primaryFocus?.unfocus();
    await tester.pumpAndSettle();
  } finally {
    tester.testTextInput.unregister();
  }
  await tester.ensureVisible(find.byKey(const ValueKey('ai-report-submit')));
  await tester.tap(find.byKey(const ValueKey('ai-report-submit')));
  await _wait(tester, find.text('Report submitted for safety review.'));
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  binding.reportData ??= {};
  binding.reportData!['milestones'] = {
    'region': 'us',
    'accountLogin': true,
    'desktopAccountGrant': true,
    'desktopGuideBytesMatch': true,
    'sharingCancelRemoteBlank': true,
    'provider': 'opencode-go',
    'sharingAllowAssistantReply': true,
    'reportCancel': true,
    'reportServerReceipt': true,
    'providerConfigurationChanged': false,
  };
  navigation.pop();
  // A route's push future completes when pop starts, before the reverse
  // transition unmounts NativeDshPage and detaches its caller-owned handle.
  await tester.pumpAndSettle();
  await route;
  handle.dispose();
  client.close();
}
