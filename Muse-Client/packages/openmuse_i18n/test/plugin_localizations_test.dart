import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';

void main() {
  test(
    'plugin language context follows the Host and exports safe data without changing user content',
    () {
      final locale = OpenMuseLocaleController(initialTag: 'en');
      final plugin = OpenMusePluginLocalizationAdapter(locale);
      var updates = 0;
      void changed() => updates++;
      plugin.addListener(changed);
      expect(plugin.message('common.save'), 'Save');
      locale.selectTag('zh-Hans');
      expect(updates, 1);
      expect(plugin.message('common.save'), '保存');
      final data = plugin.namespaceData('common');
      expect(data['locale'], 'zh-Hans');
      expect(data['direction'], 'ltr');
      expect(() => plugin.namespaceData('../other'), throwsFormatException);
      plugin.removeListener(changed);
      locale.dispose();
    },
  );
}
