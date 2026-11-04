import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';
import 'package:test/test.dart';

final class MemoryRanges implements ResourceRangePort {
  MemoryRanges(this.bytes);
  final List<int> bytes;
  @override
  Future<List<int>> read(
    ResourceHandle handle,
    int start,
    int endExclusive,
  ) async => bytes.sublist(start, endExclusive);
}

void main() {
  const handle = ResourceHandle(
    resourceRef: 'r',
    revision: 'v1',
    audience: 'viewer',
    generation: 2,
    expiresAtMs: 100,
    size: 10,
    mediaType: 'text/markdown',
  );
  test('handle audience revision generation expiry fail closed', () async {
    final client = MobileResourceClient(
      MemoryRanges(List.generate(10, (i) => i)),
    );
    expect(
      await client.readRange(
        handle,
        audience: 'viewer',
        generation: 2,
        nowMs: 1,
        start: 0,
        endExclusive: 5,
      ),
      [0, 1, 2, 3, 4],
    );
    for (final action in [
      () => client.readRange(
        handle,
        audience: 'other',
        generation: 2,
        nowMs: 1,
        start: 0,
        endExclusive: 5,
      ),
      () => client.readRange(
        handle,
        audience: 'viewer',
        generation: 1,
        nowMs: 1,
        start: 0,
        endExclusive: 5,
      ),
      () => client.readRange(
        handle,
        audience: 'viewer',
        generation: 2,
        nowMs: 100,
        start: 0,
        endExclusive: 5,
      ),
    ]) {
      expect(action, throwsStateError);
    }
  });
  test('large files require bounded ranges and unsupported gets fallback', () {
    final client = MobileResourceClient(MemoryRanges([]), maxTextBytes: 4);
    expect(
      () => client.readRange(
        handle,
        audience: 'viewer',
        generation: 2,
        nowMs: 1,
        start: 0,
        endExclusive: 5,
      ),
      throwsStateError,
    );
    expect(
      client.renderer(
        const ResourceHandle(
          resourceRef: 'x',
          revision: '1',
          audience: 'v',
          generation: 1,
          expiresAtMs: 2,
          size: 1,
          mediaType: 'application/octet-stream',
        ),
      ),
      MobileRenderer.desktopFallback,
    );
  });
}
