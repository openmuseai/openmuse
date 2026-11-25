import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mobile/host/mcp_release_policy.dart';
import 'package:openmuse_mobile/host/bundled_mcp_packages.dart';

void main() {
  test('store builds ignore internal review overrides', () {
    final policy = mcpReviewPolicy(
      debugBuild: false,
      packagesJson: 'invalid',
      remotesJson: 'invalid',
    );
    expect(policy.packages, bundledMcpPackagePins);
    expect(policy.packages.keys, hasLength(1));
    expect(policy.remotes, isEmpty);
  });
  test('internal full-client review requires exact immutable pins', () {
    final policy = mcpReviewPolicy(
      debugBuild: true,
      packagesJson: '{"example/slides@1":"${'a' * 64}"}',
      remotesJson: '{"example/ci@1":"http://127.0.0.1:18803/ci/mcp"}',
    );
    expect(policy.packages['example/slides@1'], 'a' * 64);
    expect(() => policy.packages['other@1'] = 'b' * 64, throwsUnsupportedError);
    for (final input in [
      '{"x@1":"http://remote.example/mcp"}',
      '{"x@1":"file:///private"}',
      '{"x@1":"https://user:password@example.com/mcp"}',
    ]) {
      expect(
        () => mcpReviewPolicy(debugBuild: true, remotesJson: input),
        throwsFormatException,
      );
    }
  });
}
