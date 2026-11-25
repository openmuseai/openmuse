import 'dart:io';
import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_market/openmuse_mobile_plugin_market.dart';
import 'package:openmuse_mobile/host/bundled_mcp_packages.dart';

class _Asset extends CachingAssetBundle {
  int loads = 0;
  bool corrupt = false;
  @override
  Future<ByteData> load(String key) async {
    expect(key, bundledSlidesAsset);
    loads++;
    final bytes = await File(key).readAsBytes();
    if (corrupt) bytes[bytes.length - 1] ^= 1;
    return ByteData.sublistView(bytes);
  }
}

class _NoNetwork extends HttpOverrides {
  int requests = 0;
  @override
  HttpClient createHttpClient(SecurityContext? context) => _Client(this);
}

class _Client implements HttpClient {
  _Client(this.owner);
  final _NoNetwork owner;
  @override
  set connectionTimeout(Duration? value) {}
  @override
  void close({bool force = false}) {}
  @override
  dynamic noSuchMethod(Invocation invocation) {
    owner.requests++;
    throw StateError('Network is forbidden during bundled installation');
  }
}

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();
  late Directory support;
  late RemoteMcpMarket market;
  late _Asset assets;
  setUp(() async {
    support = await Directory.systemTemp.createTemp('bundled-mcp-test-');
    market = RemoteMcpMarket(
      Uri.parse('https://unreachable.invalid'),
      File('${support.path}/receipts.json'),
      allowedPackages: bundledMcpPackagePins,
    );
    assets = _Asset();
  });
  tearDown(() => support.delete(recursive: true));

  test(
    'exact bundled presentation installs, survives restart and repairs offline with zero network',
    () async {
      final network = _NoNetwork();
      await HttpOverrides.runWithHttpOverrides(() async {
        await installBundledMcpPackages(market, assets: assets);
        expect(assets.loads, 1);
        final entry = (await market.installedEntries()).single;
        expect(entry.packageSha256, bundledSlidesEntry.packageSha256);
        expect(entry.packageUri?.scheme, 'asset');
        expect(market.hasOfflineUi(entry), isTrue);
        expect(await market.mcpAppDocument(entry), isNotNull);
        expect(
          await market.resourceMediaTypes(entry),
          contains(
            'application/vnd.openxmlformats-officedocument.presentationml.presentation',
          ),
        );
        final restored = RemoteMcpMarket(
          market.origin,
          market.receiptFile,
          allowedPackages: bundledMcpPackagePins,
        );
        await installBundledMcpPackages(restored, assets: assets);
        expect(assets.loads, 1);
        await market.packageDirectory(entry).delete(recursive: true);
        await installBundledMcpPackages(restored, assets: assets);
        expect(assets.loads, 2);
        expect(restored.hasOfflineUi(entry), isTrue);
        expect(network.requests, 0);
      }, network);
    },
  );

  test(
    'a modified application asset is rejected without any installation receipt',
    () async {
      assets.corrupt = true;
      await expectLater(
        installBundledMcpPackages(market, assets: assets),
        throwsFormatException,
      );
      expect(await market.installed(), isEmpty);
      expect(market.hasOfflineUi(bundledSlidesEntry), isFalse);
    },
  );

  test(
    'an asset identity without explicit local bytes never becomes a network download',
    () async {
      final network = _NoNetwork();
      await HttpOverrides.runWithHttpOverrides(() async {
        await expectLater(
          market.install(bundledSlidesEntry),
          throwsFormatException,
        );
        expect(network.requests, 0);
        expect(await market.installed(), isEmpty);
      }, network);
    },
  );

  test(
    'an internal candidate override does not silently install the shipping package',
    () async {
      final review = RemoteMcpMarket(
        market.origin,
        market.receiptFile,
        allowedPackages: {bundledMcpPackagePins.keys.single: '0' * 64},
      );
      await installBundledMcpPackages(review, assets: assets);
      expect(assets.loads, 0);
      expect(await review.installed(), isEmpty);
    },
  );
}
