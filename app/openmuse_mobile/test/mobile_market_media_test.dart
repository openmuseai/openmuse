import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_market/src/market_discover_view.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';

void main() {
  testWidgets('media tiles identify GIF and explain a blocked video URL', (
    tester,
  ) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: SizedBox(
            height: 220,
            child: MarketMediaView(
              media: MobileMarketMedia(
                type: 'gif',
                uri:
                    'asset:packages/openmuse_market_ui/assets/task-board-editorial.png',
              ),
            ),
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();
    expect(find.text('动图'), findsOneWidget);

    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: SizedBox(
            height: 220,
            child: MarketMediaView(
              media: MobileMarketMedia(
                type: 'video',
                uri: 'http://example.invalid/video.mp4',
              ),
            ),
          ),
        ),
      ),
    );
    await tester.tap(find.byTooltip('播放视频'));
    await tester.pump();
    expect(find.text('视频地址不可用'), findsOneWidget);
  });
}
