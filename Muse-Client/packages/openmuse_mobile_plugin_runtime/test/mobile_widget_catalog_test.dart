import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile_plugin_runtime/openmuse_mobile_plugin_runtime.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  test('workbench accepts only its shared read-only A2UI profile', () {
    final surface = MobileWidgetCatalog.surface();
    addTearDown(surface.dispose);
    expect(
      () => MobileWidgetCatalog.apply(surface, [
        {
          'version': 'v0.9',
          'createSurface': {
            'surfaceId': 'main',
            'catalogId': MobileWidgetCatalog.catalogId,
          },
        },
        {
          'version': 'v0.9',
          'updateComponents': {
            'surfaceId': 'main',
            'components': [
              {'id': 'root', 'component': 'Text', 'text': '预览'},
            ],
          },
        },
      ]),
      returnsNormally,
    );
    expect(
      () => MobileWidgetCatalog.apply(surface, [
        {
          'version': 'v0.9',
          'updateComponents': {
            'surfaceId': 'main',
            'components': [
              {'id': 'input', 'component': 'TextField', 'label': '禁止输入'},
            ],
          },
        },
      ]),
      throwsFormatException,
    );
  });
}
