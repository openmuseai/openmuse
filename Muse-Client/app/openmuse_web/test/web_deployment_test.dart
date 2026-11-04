import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:openmuse_web/auth/web_deployment.dart';

InstalledRelease fixture(String region) {
  final market = region == 'cn' ? 'china' : 'overseas';
  final data =
      jsonDecode(
            File(
              '../../packages/openmuse_runtime_config/assets/releases/$market.json',
            ).readAsStringSync(),
          )
          as Map<String, dynamic>;
  data['id'] = 'official-web-$region';
  data['selection'] = 'fixed';
  data['fixedRegion'] = region;
  data['regions'] = (data['regions'] as List)
      .where((r) => r['id'] == region)
      .toList();
  return InstalledRelease.parse(jsonEncode(data));
}

void main() {
  test(
    'US Web binds to the US origin and /app mount before any adapters start',
    () {
      final release = fixture('us');
      void check(String page) => validateWebDeployment(
        release,
        Uri.parse(page),
        region: 'us',
        origin: 'https://us.openmuseai.com',
        basePath: '/app/',
      );
      check('https://us.openmuseai.com/app/?lang=zh-Hans&region=cn');
      check('http://127.0.0.1:4174/app/');
      for (final invalid in [
        'https://app.openmuseai.com/app/',
        'https://openmuseai.com/app/',
        'https://us.openmuseai.com/',
        'http://us.openmuseai.com/app/',
      ]) {
        expect(() => check(invalid), throwsFormatException);
      }
      expect(
        () => validateWebDeployment(
          release,
          Uri.parse('https://us.openmuseai.com/app/'),
          region: 'cn',
          origin: 'https://us.openmuseai.com',
          basePath: '/app/',
        ),
        throwsFormatException,
      );
    },
  );
  test(
    'domestic Web is fixed independently from UI language and query parameters',
    () {
      final release = fixture('cn');
      validateWebDeployment(
        release,
        Uri.parse('https://app.openmuseai.com/?lang=en&region=us'),
        region: 'cn',
        origin: 'https://app.openmuseai.com',
        basePath: '/',
      );
      expect(release.fixedRegion, 'cn');
      expect(
        () => validateWebDeployment(
          release,
          Uri.parse('https://us.openmuseai.com/app/'),
          region: 'cn',
          origin: 'https://app.openmuseai.com',
          basePath: '/',
        ),
        throwsFormatException,
      );
    },
  );
}
