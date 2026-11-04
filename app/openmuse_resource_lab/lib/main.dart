import 'dart:async';
import 'package:openmuse_dsh_plugin/openmuse_dsh_plugin.dart'
    show DshPopupRouteObserver;
// The acceptance host deliberately exercises the production desktop WebView.
// ignore: implementation_imports
import 'package:openmuse_dsh_plugin/src/dsh_web_view.dart' show DshWebView;
import 'dart:convert';
import 'dart:io';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:muse_dsh_conversation_core/muse_dsh_conversation_core.dart';
import 'package:muse_dsh_conversation_flutter/muse_dsh_conversation_flutter.dart';
import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_resource_center_host/openmuse_resource_center_host.dart';
import 'package:openmuse_resource_notes/openmuse_resource_notes.dart';
import 'package:openmuse_workspace_paired/openmuse_workspace_paired.dart';

// Only the development acceptance target consumes these test coordination hooks.
NativeResourceCenter? resourceLabCenter;
final resourceLabCapture = GlobalKey();
bool resourceLabReviewReady = false;
bool resourceLabReviewReleased = false;
Future<void> Function()? closeResourceLab;

Future<void> main() async {
  if (kReleaseMode) throw StateError('The acceptance lab is development-only.');
  WidgetsFlutterBinding.ensureInitialized();
  final file = File(
    Platform.environment['OPENMUSE_RESOURCE_QA_INPUT'] ??
        '/tmp/openmuse-resource-center-qa/runtime.json',
  );
  final input = Map<String, Object?>.from(
    jsonDecode(await file.readAsString()) as Map,
  );
  final registry = OpenMusePluginRegistry(
    context: OpenMusePluginContext(executeHostCommand: (a, b) async => null),
  );
  registry.install(OpenMuseResourceNotesPlugin());
  await registry.activate('com.openmuse.resource-notes');
  final center = await NativeResourceCenter.open(
    scope: input['account'] as String,
    deviceRef: 'resource-qa-pc',
    deviceLabel: 'Studio Mac',
    registry: registry,
    workspace: 'openmuse.local.default',
    workspaceLabel: 'Release workspace',
    storageRoot: Directory('${file.parent.path}/pc-resources'),
  );
  resourceLabCenter = center;
  final workspace = Directory(input['workspace'] as String);
  await Directory('${workspace.path}/Research').create(recursive: true);
  await File(
    '${workspace.path}/Research/Remote brief.md',
  ).writeAsString('# Remote brief\n\nA document stored on the PC.');
  await center.mountDirectories({
    workspace.path: 'Project files',
  }, workspaceLabel: 'Release workspace');
  final gateway = PairedDesktopGateway(
    currentAccountRef: () => center.hub.scope,
    validateToken: (token) async {
      if (token != input['accessToken']) throw const ResourceFailure('DENIED');
      return center.hub.scope;
    },
    dshEndpoint: () async => Uri.parse(input['dshEndpoint'] as String),
    nativeApiToken: input['bridgeToken'] as String,
    workspaceRef: 'openmuse.local.default',
    workspaceTitle: 'Release workspace',
    deviceRef: 'resource-qa-pc',
    deviceName: 'Studio Mac',
    bindAddress: InternetAddress.anyIPv4,
    port: input['gatewayPort'] as int,
    fixedPairingCode: input['pairingCode'] as String,
    resources: (request) async {
      try {
        return await center.rpc.dispatch(
          request.body,
          ResourcePeer(
            scope: request.accountRef,
            deviceRef: request.deviceRef,
            workspaceRef: request.workspaceRef,
            expiresAtMs: request.expiresAtMs,
          ),
        );
      } on ResourceFailure catch (e) {
        return {'error': e.code};
      }
    },
  );
  await gateway.start();
  gateway.armPairing();
  final key = GlobalKey<LabState>();
  final control = await HttpServer.bind(
    InternetAddress.anyIPv4,
    input['controlPort'] as int,
  );
  control.listen((request) async {
    request.response.headers.contentType = ContentType.json;
    if (request.headers.value('authorization') !=
        'Bearer ${input['accessToken']}') {
      request.response.statusCode = 403;
      await request.response.close();
      return;
    }
    try {
      Object? result;
      switch (request.uri.path) {
        case '/review-gate':
          result = {'released': resourceLabReviewReleased};
        case '/release-review':
          resourceLabReviewReleased = true;
          result = {'released': true};
        case '/state':
          final sessions = await key.currentState?.client.listSessions();
          result = {
            'paired': gateway.running,
            'sources': center.hub.providers
                .map((p) => p.source.toJson())
                .toList(),
            'sessions': sessions
                ?.map(
                  (s) => {
                    'id': s.sessionId,
                    'running': s.running,
                    'headSeq': s.headSeq,
                  },
                )
                .toList(),
            'projectionCount':
                await Directory(
                  '${center.root.path}/execution-projections',
                ).exists()
                ? await Directory(
                    '${center.root.path}/execution-projections',
                  ).list().length
                : 0,
          };
        case '/clear-projections':
          await center.execution.projections.clear();
          final fixtureRoot = Directory(
            '${center.root.path}/execution-projections',
          );
          if (await fixtureRoot.exists()) {
            await fixtureRoot.delete(recursive: true);
          }
          result = {'cleared': true};
        case '/arm-pairing':
          resourceLabReviewReleased = false;
          resourceLabReviewReady = false;
          gateway.armPairing();
          result = {'armed': true};
        case '/conversation':
          await key.currentState?.showLatestConversation();
          resourceLabReviewReady = true;
          result = {'shown': true};
        default:
          request.response.statusCode = 404;
          result = {'error': 'NOT_FOUND'};
      }
      request.response.write(jsonEncode(result));
    } on Object {
      request.response.statusCode = 400;
      request.response.write('{"error":"CONTROL_FAILED"}');
    }
    await request.response.close();
  });
  closeResourceLab = () async {
    resourceLabReviewReleased = true;
    await key.currentState?.closeConversation();
    await control.close(force: true);
    await gateway.stop();
    await center.close();
  };
  runApp(Lab(key: key, center: center, input: input));
}

final class Lab extends StatefulWidget {
  const Lab({super.key, required this.center, required this.input});
  final NativeResourceCenter center;
  final Map<String, Object?> input;
  @override
  State<Lab> createState() => LabState();
}

final class LabState extends State<Lab> {
  final navigator = GlobalKey<NavigatorState>();
  late final endpoint = Uri.parse(widget.input['dshEndpoint'] as String);
  late final client = DshNativeGatewayClient(
    origin: Uri(
      scheme: endpoint.scheme,
      host: endpoint.host,
      port: endpoint.port,
    ),
    bootstrapPath: endpoint.hasQuery
        ? '${endpoint.path}?${endpoint.query}'
        : endpoint.path,
    allowInsecureLoopback: true,
    headers: {'x-openmuse-bridge-token': widget.input['bridgeToken'] as String},
  );
  DshConversationController? conversation;
  final requestedSession = ValueNotifier<String?>(null);
  int selected = 0;
  String? failure;
  Future<void> showLatestConversation() async {
    try {
      await client.initialize();
      final sessions = await client.listSessions();
      if (sessions.isEmpty) return;
      final latest = sessions.first;
      requestedSession.value = latest.sessionId;
      if (conversation?.store.snapshot.sessionId != latest.sessionId) {
        await conversation?.dispose();
        final sessionClient = DshNativeGatewayClient(
          origin: Uri(
            scheme: endpoint.scheme,
            host: endpoint.host,
            port: endpoint.port,
          ),
          bootstrapPath: '${endpoint.path}?${endpoint.query}',
          allowInsecureLoopback: true,
          headers: {
            'x-openmuse-bridge-token': widget.input['bridgeToken'] as String,
          },
        );
        await sessionClient.initialize();
        conversation = DshConversationController(client: sessionClient);
        await conversation!.start(latest.sessionId, running: latest.running);
      }
      if (mounted) setState(() => selected = 1);
    } on Object catch (error) {
      if (mounted) setState(() => failure = error.runtimeType.toString());
    }
  }

  Future<void> closeConversation() async {
    final active = conversation;
    conversation = null;
    await active?.dispose();
    client.close();
  }

  @override
  void dispose() {
    requestedSession.dispose();
    unawaited(closeConversation());
    super.dispose();
  }

  @override
  Widget build(BuildContext context) => MaterialApp(
    navigatorKey: navigator,
    navigatorObservers: [DshPopupRouteObserver.instance],
    debugShowCheckedModeBanner: false,
    locale: const Locale('en'),
    supportedLocales: const [Locale('en'), Locale('zh')],
    localizationsDelegates: [OpenMuseStringsDelegate(OpenMuseCatalog.builtIn)],
    theme: ThemeData(
      useMaterial3: true,
      colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xff2467d6)),
      scaffoldBackgroundColor: const Color(0xfff5f5f3),
    ),
    builder: (context, child) => ResourceLinkRouting(
      open: (uri) async {
        final context = navigator.currentContext!;
        try {
          await openResourcePage(
            context,
            widget.center.controller,
            ResourceReference.fromUri(uri),
          );
        } on Object catch (error) {
          if (context.mounted) showResourceError(context, error);
        }
      },
      child: RepaintBoundary(key: resourceLabCapture, child: child!),
    ),
    home: Scaffold(
      body: SafeArea(
        child: Row(
          children: [
            NavigationRail(
              selectedIndex: selected,
              onDestinationSelected: (index) {
                if (index == 1) {
                  unawaited(showLatestConversation());
                } else {
                  setState(() => selected = index);
                }
              },
              labelType: NavigationRailLabelType.all,
              destinations: const [
                NavigationRailDestination(
                  icon: Icon(Icons.home_outlined),
                  label: Text('Home'),
                ),
                NavigationRailDestination(
                  icon: Icon(Icons.chat_bubble_outline),
                  label: Text('Conversation'),
                ),
                NavigationRailDestination(
                  icon: Icon(Icons.web_asset),
                  label: Text('Desktop web'),
                ),
              ],
            ),
            const VerticalDivider(width: 1),
            Expanded(
              child: selected == 2
                  ? DshWebView(
                      url: endpoint,
                      activeMountPath: widget.input['workspace'] as String,
                      requestedSession: requestedSession,
                      reloadToken: 0,
                      onActivateWorkspace: (_) async {},
                      onOpenResource: (_) async {},
                    )
                  : selected == 0
                  ? ResourceCenterView(
                      controller: widget.center.controller,
                      title: 'Resource center',
                    )
                  : conversation == null
                  ? Center(
                      child: Text(
                        failure ?? 'Open a conversation on the phone.',
                      ),
                    )
                  : DshConversationSurface(
                      store: conversation!.store,
                      onSend: (text, {mode = 'queue'}) =>
                          conversation!.send(text, mode: mode),
                      onCancel: () => conversation!.cancel(),
                      onFallbackRequested: () {},
                      showComposer: false,
                    ),
            ),
          ],
        ),
      ),
    ),
  );
}
