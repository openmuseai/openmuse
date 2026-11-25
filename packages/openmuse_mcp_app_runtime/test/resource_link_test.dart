import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_mcp_app_runtime/src/resource_link.dart';

void main() {
  String link(String scope) => ResourceReference(
    identity: ResourceIdentity(
      scope: scope, source: 'installed-cloud-source', workspace: 'workspace',
      mount: 'pages', resource: 'document',
    ),
  ).toUri().toString();
  test('App links resolve only resource locators in the current account', () {
    expect(checkedAppResourceLink(link('account-a'), 'account-a').identity.resource, 'document');
    expect(() => checkedAppResourceLink(link('account-b'), 'account-a'), throwsStateError);
    expect(() => checkedAppResourceLink(link('signed-out-local'), 'signed-out-local'), throwsStateError);
    for (final value in ['https://attacker.invalid/', 'file:///etc/passwd', 'javascript:alert(1)', null]) {
      expect(() => checkedAppResourceLink(value, 'account-a'), throwsA(anything));
    }
  });
  test('Link support is opt-in and never loosens the App network sandbox', () {
    final standard = buildMcpAppHostDocument(appHtml: '<p>App</p>', arguments: {}, platform: 'desktop');
    final links = buildMcpAppHostDocument(appHtml: '<p>App</p>', arguments: {}, platform: 'desktop', resourceLinksAvailable: true);
    expect(standard, contains("false&&message.method==='ui/open-link'"));
    expect(links, contains("true&&message.method==='ui/open-link'"));
    expect(links, contains('sandbox="allow-scripts allow-downloads"'));
    expect(restrictMcpAppDocument('<p>App</p>'), contains("connect-src 'none'"));
  });
}
