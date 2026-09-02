import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_mobile/muse_dsh_mobile.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:webview_flutter_platform_interface/webview_flutter_platform_interface.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  late _WebPlatform platform;
  late _FakeHost host;
  var currentScope = true;
  const channel =
      MethodChannel('dev.fluttercommunity.plus/connectivity_status');

  setUp(() {
    platform = _WebPlatform();
    WebViewPlatform.instance = platform;
    host = _FakeHost();
    currentScope = true;
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(channel, (_) async => null);
  });
  tearDown(() async {
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(channel, null);
  });

  Future<void> open(WidgetTester tester, {bool cloudAccount = true}) async {
    host.pending = Completer<DshControlSession>();
    await tester.pumpWidget(
      MaterialApp(
        home: DshMobileShellPage(
          scope: DshMobileScope(
            workspaceRef: 'workspace.test',
            workspaceTitle: 'Test workspace',
            accountRef: 'account.test',
            isCloudAccount: cloudAccount,
            isCurrentScope: () => currentScope,
          ),
          controlHost: host,
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
    'official chat is independent of the optional control host',
    () {
      testWidgets('loads before control connect completes, with empty headers',
          (tester) async {
        await open(tester);
        expect(host.calls, 1);
        expect(find.byType(WebViewWidget), findsOneWidget);
        expect(find.byType(CircularProgressIndicator), findsNothing);
        expect(
          platform.controllers.single.requests.single.uri,
          DshRemoteConfig.fromEnvironment()!.publicUri,
        );
        expect(platform.controllers.single.requests.single.headers, isEmpty);
        expect(find.textContaining('正在尝试工作区联动'), findsOneWidget);
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
          host.pending.completeError(DshControlConnectException(code));
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

      testWidgets('guest loads chat without connecting the control host',
          (tester) async {
        await open(tester, cloudAccount: false);
        expect(host.calls, 0);
        expect(find.byType(WebViewWidget), findsOneWidget);
        expect(find.textContaining('工作区联动未连接'), findsOneWidget);
        await close(tester);
      });

      testWidgets(
          'background revokes pending control without removing chat',
          (tester) async {
        await open(tester);
        tester.binding.handleAppLifecycleStateChanged(AppLifecycleState.paused);
        await tester.pump();
        host.pending.complete(_IdleSession());
        tester.binding
            .handleAppLifecycleStateChanged(AppLifecycleState.resumed);
        await tester.pumpAndSettle();
        expect(find.byType(WebViewWidget), findsOneWidget);
        expect(find.textContaining('应用已进入后台'), findsOneWidget);
        expect(find.textContaining('工作区联动已连接'), findsNothing);
        await close(tester);
      });

      testWidgets('scope change closes chat even after control failure',
          (tester) async {
        await open(tester);
        host.pending.completeError(
          const DshControlConnectException('CLOUD_DEVICE_TOKEN_REJECTED'),
        );
        await tester.pumpAndSettle();
        currentScope = false;
        await tester.pump(const Duration(seconds: 1));
        await tester.pumpAndSettle();
        expect(find.byType(WebViewWidget), findsNothing);
        expect(find.textContaining('账号或工作区已改变'), findsOneWidget);
        await close(tester);
      });

      testWidgets('refresh ignores control errors from the previous generation',
          (tester) async {
        await open(tester);
        final previous = host.pending;
        host.pending = Completer<DshControlSession>();
        await tester.tap(find.byTooltip('重新连接'));
        await tester.pumpAndSettle();
        expect(host.calls, 2);
        previous.completeError(const DshControlConnectException('STALE_REQUEST'));
        await tester.pumpAndSettle();
        expect(find.textContaining('STALE_REQUEST'), findsNothing);
        expect(find.byType(WebViewWidget), findsOneWidget);
        expect(platform.controllers, hasLength(2));
        host.pending
            .completeError(const DshControlConnectException('CURRENT_REQUEST'));
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
        ? 'Run with --dart-define=MUSE_DSH_PUBLIC_URL=https://dsh.openmuseai.com/'
        : false,
  );
}

class _FakeHost implements DshMobileControlHost {
  late Completer<DshControlSession> pending;
  var calls = 0;

  @override
  Future<DshControlSession> connect(DshControlSessionRequest request) {
    if (!request.isCloudAccount) {
      throw const DshControlConnectException('CLOUD_LOGIN_REQUIRED');
    }
    calls++;
    return pending.future;
  }
}

class _IdleSession implements DshControlSession {
  @override
  Future<void> close() async {}
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
  Future<void> addJavaScriptChannel(JavaScriptChannelParams params) async {}
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
