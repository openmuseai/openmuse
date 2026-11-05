import 'dart:async';

import 'package:appflowy/env/backend_env.dart';
import 'package:appflowy/env/cloud_env.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_device_token_service.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_mobile_agent_page.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_mobile/muse_dsh_mobile.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:webview_flutter_platform_interface/webview_flutter_platform_interface.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  late _WebPlatform platform;
  late _Credentials credentials;
  var currentScope = true;
  const channel =
      MethodChannel('dev.fluttercommunity.plus/connectivity_status');

  setUp(() {
    platform = _WebPlatform();
    WebViewPlatform.instance = platform;
    credentials = _Credentials();
    currentScope = true;
    getIt.registerSingleton<DshDeviceTokenService>(credentials);
    getIt.registerSingleton<AppFlowyCloudSharedEnv>(
      AppFlowyCloudSharedEnv(
        authenticatorType: AuthenticatorType.appflowyCloud,
        appflowyCloudConfig: AppFlowyCloudConfiguration(
          base_url: 'https://cloud.invalid',
          ws_base_url: 'wss://cloud.invalid/ws/v1',
          gotrue_url: 'https://cloud.invalid/gotrue',
          enable_sync_trace: false,
          base_web_domain: 'cloud.invalid',
        ),
      ),
    );
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(channel, (_) async => null);
  });
  tearDown(() async {
    await getIt.reset();
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(channel, null);
  });

  Future<void> open(
    WidgetTester tester, {
    bool cloudAccount = true,
    bool withSession = true,
  }) async {
    credentials.result = Completer<DshDeviceCredential>();
    DshSessionApi? api;
    if (withSession) {
      final origin = DshRemoteConfig.fromEnvironment()!.publicUri.origin;
      api = DshSessionApi(
        cloudOrigin: Uri.parse(origin),
        accessToken: 'jwt',
        post: (uri, headers, body) async => {
          'data': {
            'sessionRef': 'session.1',
            'webUrl':
                '$origin/u/abcdabcdabcdabcdabcdabcdabcdabcd/?token=t',
            'nodeId': 'local',
          },
        },
      );
    }
    await tester.pumpWidget(
      MaterialApp(
        home: DshMobileAgentPage(
          workspaceId: 'workspace.test',
          workspaceTitle: 'Test workspace',
          accountRef: 'account.test',
          isCloudAccount: cloudAccount,
          isCurrentScope: () => currentScope,
          sessionApi: api,
          accessToken: withSession ? 'jwt' : null,
        ),
      ),
    );
    await tester.pumpAndSettle();
  }

  Future<void> close(WidgetTester tester) async {
    await tester.pumpWidget(const SizedBox.shrink());
    await tester.pumpAndSettle();
  }

  group(
    'official chat is independent of the optional native bridge',
    () {
      testWidgets(
          'loads before credential request completes, with no JS bridge or auth header',
          (tester) async {
        await open(tester);
        expect(credentials.calls, 1);
        expect(find.byType(WebViewWidget), findsOneWidget);
        expect(find.byType(CircularProgressIndicator), findsNothing);
        expect(
          platform.controllers.single.requests.single.uri,
          Uri.parse(
            '${DshRemoteConfig.fromEnvironment()!.publicUri.origin}/u/abcdabcdabcdabcdabcdabcdabcdabcd/?token=t',
          ),
        );
        expect(platform.controllers.single.requests.single.headers, isEmpty);
        expect(find.textContaining('正在尝试工作区联动'), findsOneWidget);
        expect(platform.controllers.single.channels, ['MuseNativeCapability']);
        expect(
          platform.controllers.single.requests.single.headers,
          isEmpty,
        );
        await close(tester);
      });

      for (final code in [
        'CLOUD_DEVICE_TOKEN_REJECTED',
        'CLOUD_DEVICE_TOKEN_UNAVAILABLE',
      ]) {
        testWidgets('$code retains the same chat WebView', (tester) async {
          await open(tester);
          final controller = tester
              .widget<WebViewWidget>(find.byType(WebViewWidget))
              .platform
              .params
              .controller;
          credentials.result.completeError(DshCredentialException(code));
          await tester.pumpAndSettle();
          expect(find.textContaining('工作区联动未连接'), findsOneWidget);
          expect(find.textContaining(code), findsOneWidget);
          expect(
            tester
                .widget<WebViewWidget>(find.byType(WebViewWidget))
                .platform
                .params
                .controller,
            same(controller),
          );
          platform.navigation.finished('https://dsh.openmuseai.com/');
          await tester.pump();
          expect(find.textContaining('页面已加载'), findsOneWidget);
          expect(platform.controllers.single.requests, hasLength(1));
          await close(tester);
        });
      }

      testWidgets('guest without a session token does not load compile-time /dsh/',
          (tester) async {
        await open(tester, cloudAccount: false, withSession: false);
        expect(credentials.calls, 0);
        expect(find.byType(WebViewWidget), findsNothing);
        expect(find.textContaining('登录 Cloud 后才能启动远程 Agent'), findsOneWidget);
        await close(tester);
      });

      testWidgets(
          'background revokes pending bridge without removing chat or accepting late credentials',
          (tester) async {
        await open(tester);
        tester.binding.handleAppLifecycleStateChanged(AppLifecycleState.paused);
        await tester.pump();
        credentials.result.complete(
          const DshDeviceCredential(
            token: 'test.signature',
            expiresAt: 9999999999999,
            deviceId: 'test',
            kid: 'test',
          ),
        );
        tester.binding
            .handleAppLifecycleStateChanged(AppLifecycleState.resumed);
        await tester.pumpAndSettle();
        expect(find.byType(WebViewWidget), findsOneWidget);
        expect(find.textContaining('应用已进入后台'), findsOneWidget);
        expect(find.textContaining('工作区联动已连接'), findsNothing);
        await close(tester);
      });

      testWidgets('scope change closes chat even after credential failure',
          (tester) async {
        await open(tester);
        credentials.result.completeError(
          const DshCredentialException('CLOUD_DEVICE_TOKEN_REJECTED'),
        );
        await tester.pumpAndSettle();
        currentScope = false;
        await tester.pump(const Duration(seconds: 1));
        await tester.pumpAndSettle();
        expect(find.byType(WebViewWidget), findsNothing);
        expect(find.textContaining('账号或工作区已改变'), findsOneWidget);
        await close(tester);
      });

      testWidgets(
          'refresh ignores credentials from the previous page generation',
          (tester) async {
        await open(tester);
        final previous = credentials.result;
        credentials.result = Completer<DshDeviceCredential>();
        await tester.tap(find.byTooltip('重新连接'));
        await tester.pumpAndSettle();
        expect(credentials.calls, 2);
        previous.completeError(const DshCredentialException('STALE_REQUEST'));
        await tester.pumpAndSettle();
        expect(find.textContaining('STALE_REQUEST'), findsNothing);
        expect(find.byType(WebViewWidget), findsOneWidget);
        expect(platform.controllers, hasLength(2));
        credentials.result
            .completeError(const DshCredentialException('CURRENT_REQUEST'));
        await tester.pumpAndSettle();
        expect(find.textContaining('CURRENT_REQUEST'), findsOneWidget);
        await close(tester);
      });

      testWidgets('retains HTTPS origin navigation boundary in chat-only mode',
          (tester) async {
        await open(tester, cloudAccount: false);
        for (final url in [
          'https://foreign.invalid/',
          'http://dsh.openmuseai.com/',
          'https://dsh.openmuseai.com.evil.invalid/',
        ]) {
          expect(
            await platform.navigation
                .navigate(NavigationRequest(url: url, isMainFrame: true)),
            NavigationDecision.prevent,
          );
        }
        expect(
          await platform.navigation.navigate(
            NavigationRequest(
              url: DshRemoteConfig.fromEnvironment()!.publicUri.toString(),
              isMainFrame: true,
            ),
          ),
          NavigationDecision.navigate,
        );
        await close(tester);
      });

      testWidgets(
          'invalid TLS is cancelled and reported instead of a blank page',
          (tester) async {
        await open(tester, cloudAccount: false);
        final error = _SslError();
        platform.navigation.ssl(error);
        await tester.pumpAndSettle();
        expect(error.cancelled, isTrue);
        expect(error.proceeded, isFalse);
        expect(find.byType(WebViewWidget), findsNothing);
        expect(find.textContaining('HTTPS 证书未通过手机验证'), findsOneWidget);
        await close(tester);
      });

      testWidgets(
          'main-frame failure remains fatal, subresource failure does not',
          (tester) async {
        await open(tester, cloudAccount: false);
        platform.navigation.error(
          const WebResourceError(
            errorCode: -1,
            description: 'resource unavailable',
            isForMainFrame: false,
          ),
        );
        await tester.pump();
        expect(find.byType(WebViewWidget), findsOneWidget);
        platform.navigation.error(
          const WebResourceError(
            errorCode: -1,
            description: 'page unavailable',
            isForMainFrame: true,
          ),
        );
        await tester.pumpAndSettle();
        expect(find.byType(WebViewWidget), findsNothing);
        expect(find.textContaining('DSH 页面加载失败'), findsOneWidget);
        await close(tester);
      });
    },
    skip: DshRemoteConfig.fromEnvironment() == null
        ? 'Run with --dart-define-from-file=config/mobile/openmuse.json'
        : false,
  );
}

class _Credentials extends DshDeviceTokenService {
  late Completer<DshDeviceCredential> result;
  var calls = 0;
  @override
  Future<DshDeviceCredential> loadOrIssue({
    required String accountRef,
    required String cloudOrigin,
    required String dshOrigin,
    required String sessionRef,
  }) {
    calls++;
    return result.future;
  }
}

class _WebPlatform extends WebViewPlatform {
  final controllers = <_WebController>[];
  late _Navigation navigation;
  @override
  PlatformWebViewController createPlatformWebViewController(
    PlatformWebViewControllerCreationParams params,
  ) {
    final controller = _WebController(params);
    controllers.add(controller);
    return controller;
  }

  @override
  PlatformNavigationDelegate createPlatformNavigationDelegate(
    PlatformNavigationDelegateCreationParams params,
  ) =>
      navigation = _Navigation(params);
  @override
  PlatformWebViewWidget createPlatformWebViewWidget(
    PlatformWebViewWidgetCreationParams params,
  ) =>
      _WebWidget(params);
}

class _WebController extends PlatformWebViewController {
  _WebController(super.params) : super.implementation();
  final requests = <LoadRequestParams>[];
  final channels = <String>[];
  @override
  Future<void> setJavaScriptMode(JavaScriptMode javaScriptMode) async {}
  @override
  Future<void> setPlatformNavigationDelegate(
    PlatformNavigationDelegate handler,
  ) async {}
  @override
  Future<void> loadRequest(LoadRequestParams params) async =>
      requests.add(params);
  @override
  Future<void> addJavaScriptChannel(JavaScriptChannelParams params) async =>
      channels.add(params.name);
  @override
  Future<void> runJavaScript(String javaScript) async {}
  @override
  Future<String?> currentUrl() async =>
      requests.isEmpty ? null : requests.last.uri.toString();
}

class _WebWidget extends PlatformWebViewWidget {
  _WebWidget(super.params) : super.implementation();
  @override
  Widget build(BuildContext context) => const SizedBox.expand();
}

class _Navigation extends PlatformNavigationDelegate {
  _Navigation(super.params) : super.implementation();
  late NavigationRequestCallback navigate;
  late PageEventCallback finished;
  late WebResourceErrorCallback error;
  late SslAuthErrorCallback ssl;
  @override
  Future<void> setOnSSlAuthError(SslAuthErrorCallback callback) async =>
      ssl = callback;
  @override
  Future<void> setOnNavigationRequest(
    NavigationRequestCallback callback,
  ) async =>
      navigate = callback;
  @override
  Future<void> setOnPageStarted(PageEventCallback callback) async {}
  @override
  Future<void> setOnPageFinished(PageEventCallback callback) async =>
      finished = callback;
  @override
  Future<void> setOnWebResourceError(WebResourceErrorCallback callback) async =>
      error = callback;
}

class _SslError extends PlatformSslAuthError {
  _SslError() : super(certificate: null, description: 'Untrusted certificate');
  var cancelled = false;
  var proceeded = false;
  @override
  Future<void> cancel() async => cancelled = true;
  @override
  Future<void> proceed() async => proceeded = true;
}
