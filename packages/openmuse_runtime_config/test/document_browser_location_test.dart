import 'dart:convert';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';

void main() {
  Map<String, dynamic> snapshot(String region) =>
      (jsonDecode(File('assets/deployments/$region.json').readAsStringSync())
              as Map)['snapshot']
          as Map<String, dynamic>;
  test('document browser follows the installed region and mount', () {
    expect(
      RuntimeConfig.fromJson(snapshot('china')).documentBrowserUri.toString(),
      'https://app.openmuseai.com/',
    );
    expect(
      RuntimeConfig.fromJson(
        snapshot('global-us'),
      ).documentBrowserUri.toString(),
      'https://us.openmuseai.com/app/',
    );
    final legacy = snapshot('global-us');
    (legacy['web'] as Map).remove('documentBaseUrl');
    expect(RuntimeConfig.fromJson(legacy).documentBrowserUri, isNull);
  });
  test(
    'document browser location cannot carry credentials, redirects or an insecure host',
    () {
      for (final invalid in [
        'http://example.com/app/',
        'https://user:password@example.com/',
        'https://example.com/?host=other',
        'https://example.com/#token',
        'https://example.com/app',
      ]) {
        final value = snapshot('global-us');
        (value['web'] as Map)['documentBaseUrl'] = invalid;
        expect(() => RuntimeConfig.fromJson(value), throwsFormatException);
      }
    },
  );
}
