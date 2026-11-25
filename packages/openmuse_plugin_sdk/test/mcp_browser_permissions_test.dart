import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

void main() {
  test(
    'browser grants require explicit supported resource permissions and a separate origin',
    () {
      final p = McpBrowserPermissions.fromJson({
        'microphone': {},
        'clipboardWrite': {},
        'camera': {},
      });
      expect(p.toJson().keys, ['microphone', 'clipboardWrite']);
      expect(
        () => McpBrowserPermissions.fromJson({'microphone': true}),
        throwsFormatException,
      );
      expect(
        () => buildMcpAppHostDocument(
          appUrl: 'https://host.test/editor',
          hostOrigin: 'https://host.test',
          arguments: {},
          platform: 'mobile',
          browserPermissions: p,
        ),
        throwsArgumentError,
      );
      expect(
        () => buildMcpAppHostDocument(
          appHtml: '<p>unisolated</p>',
          hostOrigin: 'https://host.test',
          arguments: {},
          platform: 'mobile',
          browserPermissions: p,
        ),
        throwsArgumentError,
      );
      final html = buildMcpAppHostDocument(
        appUrl: 'https://view.test/editor',
        hostOrigin: 'https://host.test',
        arguments: {},
        platform: 'mobile',
        browserPermissions: p,
      );
      expect(html, contains('allow-same-origin'));
      expect(html, contains('allow="microphone; clipboard-write"'));
      expect(html, isNot(contains('allow-modals')));
      final original = buildMcpAppHostDocument(
        appHtml: '<p>View</p>',
        arguments: {},
        platform: 'desktop',
      );
      expect(original, isNot(contains('allow-same-origin')));
    },
  );
  test('PNG PDF and Save As never replace the opened resource', () {
    Map<String, Object?> request(String name, String mime) => {
      'method': 'ui/download-file',
      'params': {
        'contents': [
          {
            'resource': {'uri': 'file:///$name', 'mimeType': mime},
          },
        ],
      },
    };
    expect(
      isOriginalMcpDownload(
        request('slides.pptx', 'application/pptx'),
        'slides.pptx',
        'application/pptx',
      ),
      true,
    );
    expect(
      isOriginalMcpDownload(
        request('slides.pptx', 'image/png'),
        'slides.pptx',
        'application/pptx',
      ),
      false,
    );
    expect(
      isOriginalMcpDownload(
        request('slides-copy.pptx', 'application/pptx'),
        'slides.pptx',
        'application/pptx',
      ),
      false,
    );
    expect(
      isOriginalMcpDownload(
        request('slides.pdf', 'application/pdf'),
        'slides.pptx',
        'application/pptx',
      ),
      false,
    );
  });
}
