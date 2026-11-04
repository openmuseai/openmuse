import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_siyuan_client/openmuse_siyuan_client.dart';

void main() {
  test(
    'Flutter script and regional locale tags resolve to bundled languages',
    () {
      expect(siYuanLanguageTag('zh-Hans'), 'zh-CN');
      expect(siYuanLanguageTag('zh_Hant_HK'), 'zh-TW');
      expect(siYuanLanguageTag('zh-TW'), 'zh-TW');
      expect(siYuanLanguageTag('en-US'), 'en');
      expect(siYuanLanguageTag('ja-JP'), 'ja');
      expect(siYuanLanguageTag('pt-PT'), 'pt-BR');
    },
  );
}
