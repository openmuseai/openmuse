import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:muse_dsh_mobile/muse_dsh_mobile.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:webview_flutter_platform_interface/webview_flutter_platform_interface.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  late _WebPlatform platform;
  const channel =
      MethodChannel('dev.fluttercommunity.plus/connectivity_status');

  setUp(() {
    platform = _WebPlatform();
    WebViewPlatform.instance = platform;
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(channel, (_) async => null);
  });
  tearDown(() async {
    TestDefaultBinaryMessengerBinding.instance.defaultBinaryMessenger
        .setMockMethodCallHandler(channel, null);
  });

  DshSessionApi api({
    required List<DshSessionOpen> replies,
    List<String>? seen,
  }) {
    var i = 0;
    return DshSessionApi(
      cloudOrigin: Uri.parse('https://openmuseai.com'),
      accessToken: 'jwt',
      post: (uri, headers, body) async {
        seen?.add('${uri.path} $body');
        if (uri.path.endsWith('/close')) {
          return <String, dynamic>{'code': 0};
        }
        final next = replies[i < replies.length ? i : replies.length - 1];
        i += 1;
        return {
          'data': {
            'sessionRef': next.sessionRef,
            if (next.webUrl != null) 'webUrl': next.webUrl,
            if (next.queuePosition != null) 'queuePosition': next.queuePosition,
            if (next.retryAfterMs != null) 'retryAfterMs': next.retryAfterMs,
            'nodeId': 'local',
          },
        };
      },
    );
  }

  DshMobileCoordinator coordinator({
    DshSessionApi? sessionApi,
    bool requireRemoteSession = false,
    String? accessToken,
    Future<void> Function(Duration duration)? sleep,
  }) {
    return DshMobileCoordinator(
      scope: DshMobileScope(
        workspaceRef: 'ws-1',
        workspaceTitle: 'Docs',
        accountRef: 'account-1',
        isCloudAccount: true,
        isCurrentScope: () => true,
      ),
      sessionApi: sessionApi,
      accessToken: accessToken,
      requireRemoteSession: requireRemoteSession,
      endpoint: DshRemoteConfig.fromWebUrl('https://openmuseai.com/'),
      sleep: sleep ?? (duration) => Future<void>.delayed(duration),
      notify: () {},
    );
  }

  testWidgets('E2-T1 loads session webUrl not the compile-time origin',
      (tester) async {
    await tester.pumpWidget(const SizedBox.shrink());
    const webUrl =
        'https://openmuseai.com/u/abcdabcdabcdabcdabcdabcdabcdabcd/?token=t';
    final c = coordinator(
      sessionApi: api(
        replies: [
          DshSessionOpen.fromJson({
            'sessionRef': 'sess-1',
            'webUrl': webUrl,
          }),
        ],
      ),
      accessToken: 'jwt',
    );
    await c.connect();
    expect(c.surface.controller, isNotNull);
    expect(platform.controllers.single.requests.single.uri.toString(), webUrl);
    await c.dispose();
  });

  testWidgets('E2-T2 stays queued then loads after retryAfterMs', (tester) async {
    await tester.pumpWidget(const SizedBox.shrink());
    const webUrl =
        'https://openmuseai.com/u/abcdabcdabcdabcdabcdabcdabcdabcd/?token=t';
    var slept = Duration.zero;
    final c = coordinator(
      sessionApi: api(
        replies: [
          DshSessionOpen.fromJson({
            'sessionRef': 'sess-q',
            'queuePosition': 1,
            'retryAfterMs': 30,
          }),
          DshSessionOpen.fromJson({
            'sessionRef': 'sess-1',
            'webUrl': webUrl,
          }),
        ],
      ),
      accessToken: 'jwt',
      sleep: (duration) async {
        slept = duration;
      },
    );
    await c.connect();
    expect(slept, const Duration(milliseconds: 30));
    expect(platform.controllers.single.requests.single.uri.toString(), webUrl);
    await c.dispose();
  });

  testWidgets('E2-T3 does not POST session/open without a session API',
      (tester) async {
    await tester.pumpWidget(const SizedBox.shrink());
    final c = coordinator(requireRemoteSession: true);
    await c.connect();
    expect(c.surface.fatalMessage, contains('登录 Cloud'));
    expect(c.surface.controller, isNull);
    expect(platform.controllers, isEmpty);
    await c.dispose();
  });

  testWidgets('E2-T6 closes the pool session when disposed', (tester) async {
    await tester.pumpWidget(const SizedBox.shrink());
    const webUrl =
        'https://openmuseai.com/u/abcdabcdabcdabcdabcdabcdabcdabcd/?token=t';
    final seen = <String>[];
    final c = coordinator(
      sessionApi: api(
        replies: [
          DshSessionOpen.fromJson({
            'sessionRef': 'sess-1',
            'webUrl': webUrl,
          }),
        ],
        seen: seen,
      ),
      accessToken: 'jwt',
    );
    await c.connect();
    expect(seen.single, contains('/api/muse/dsh/session/open'));
    await c.dispose();
    expect(seen.any((row) => row.contains('/close')), isTrue);
  });

  testWidgets('E2-T11 background degrade keeps the WebView', (tester) async {
    await tester.pumpWidget(const SizedBox.shrink());
    const webUrl =
        'https://openmuseai.com/u/abcdabcdabcdabcdabcdabcdabcdabcd/?token=t';
    final c = coordinator(
      sessionApi: api(
        replies: [
          DshSessionOpen.fromJson({
            'sessionRef': 'sess-1',
            'webUrl': webUrl,
          }),
        ],
      ),
      accessToken: 'jwt',
    );
    await c.connect();
    expect(c.surface.controller, isNotNull);
    c.didChangeAppLifecycleState(AppLifecycleState.paused);
    expect(c.surface.controller, isNotNull);
    expect(c.surface.bridgeWarning, contains('后台'));
    expect(c.surface.facetReady, isFalse);
    await c.dispose();
  });
}

class _WebPlatform extends WebViewPlatform {
  final controllers = <_WebController>[];
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
      _Navigation(params);
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
  @override
  Future<void> setOnSSlAuthError(SslAuthErrorCallback callback) async {}
  @override
  Future<void> setOnNavigationRequest(
    NavigationRequestCallback callback,
  ) async {}
  @override
  Future<void> setOnPageStarted(PageEventCallback callback) async {}
  @override
  Future<void> setOnPageFinished(PageEventCallback callback) async {}
  @override
  Future<void> setOnWebResourceError(WebResourceErrorCallback callback) async {}
}
