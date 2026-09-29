enum MobileRenderer { text, markdown, image, pdf, desktopFallback }

final class ResourceHandle {
  const ResourceHandle({
    required this.resourceRef,
    required this.revision,
    required this.audience,
    required this.generation,
    required this.expiresAtMs,
    required this.size,
    required this.mediaType,
  });
  final String resourceRef, revision, audience, mediaType;
  final int generation, expiresAtMs, size;
}

abstract interface class ResourceRangePort {
  Future<List<int>> read(ResourceHandle handle, int start, int endExclusive);
}

final class MobileResourceClient {
  const MobileResourceClient(this.port, {this.maxTextBytes = 1024 * 1024});
  final ResourceRangePort port;
  final int maxTextBytes;

  MobileRenderer renderer(ResourceHandle handle) {
    if (handle.mediaType == 'text/markdown') return MobileRenderer.markdown;
    if (handle.mediaType.startsWith('text/')) return MobileRenderer.text;
    if (handle.mediaType.startsWith('image/')) return MobileRenderer.image;
    if (handle.mediaType == 'application/pdf') return MobileRenderer.pdf;
    return MobileRenderer.desktopFallback;
  }

  Future<List<int>> readRange(
    ResourceHandle handle, {
    required String audience,
    required int generation,
    required int nowMs,
    required int start,
    required int endExclusive,
  }) async {
    if (audience != handle.audience ||
        generation != handle.generation ||
        nowMs >= handle.expiresAtMs ||
        start < 0 ||
        endExclusive <= start ||
        endExclusive > handle.size ||
        endExclusive - start > maxTextBytes) {
      throw StateError('resource handle denied');
    }
    return port.read(handle, start, endExclusive);
  }

  Map<String, Object> agentContext(ResourceHandle handle, String excerpt) => {
    'resourceRef': handle.resourceRef,
    'revision': handle.revision,
    'excerpt': excerpt.substring(0, excerpt.length.clamp(0, 4096)),
  };
}

final class NativeCapabilityHandle {
  const NativeCapabilityHandle(this.handleRef, this.audience, this.expiresAtMs);
  final String handleRef, audience;
  final int expiresAtMs;
}
