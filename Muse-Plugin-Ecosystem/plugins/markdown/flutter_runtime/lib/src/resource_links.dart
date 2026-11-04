import 'dart:io';
import 'dart:typed_data';

enum ResourceLinkKind { asset, localFile, external, resource, blocked }

final class ResourceLinkTarget {
  const ResourceLinkTarget(this.kind, this.uri);
  final ResourceLinkKind kind;
  final Uri uri;
}

ResourceLinkTarget resolveResourceLink(Uri origin, String raw) {
  final blocked = ResourceLinkTarget(ResourceLinkKind.blocked, Uri());
  if (raw.isEmpty ||
      raw.length > 4096 ||
      raw.contains('\\') ||
      raw.contains(RegExp(r'[\x00-\x1f]')))
    return blocked;
  final uri = Uri.tryParse(raw);
  if (uri == null) return blocked;
  if (uri.scheme == 'openmuse-resource')
    return ResourceLinkTarget(ResourceLinkKind.resource, uri);
  if (uri.scheme == 'file' ||
      raw.startsWith('/storage/') ||
      raw.startsWith('/sdcard/'))
    return ResourceLinkTarget(
      ResourceLinkKind.localFile,
      uri.scheme == 'file' ? uri : Uri.file(raw),
    );
  if (uri.scheme == 'content' || uri.scheme == 'mailto')
    return ResourceLinkTarget(ResourceLinkKind.external, uri);
  final resolved = origin.resolveUri(uri);
  if (resolved.scheme != 'http' && resolved.scheme != 'https') return blocked;
  if (resolved.origin != origin.origin)
    return ResourceLinkTarget(ResourceLinkKind.external, resolved);
  // Decode before checking: an encoded path must not escape the asset folder.
  final path = Uri.decodeComponent(uri.path);
  if ((path.startsWith('assets/') || path.startsWith('/assets/')) &&
      !path.split('/').any((p) => p == '..' || p == '.') &&
      !path.contains('\\') &&
      resolved.path.startsWith('/assets/')) {
    return ResourceLinkTarget(ResourceLinkKind.asset, resolved);
  }
  return blocked;
}

/// Reads only an authenticated asset served by this plugin's loopback kernel.
Future<Uint8List> readResourceLinkAsset(
  Uri origin,
  Uri uri,
  Map<String, String> headers,
) async {
  if (resolveResourceLink(origin, uri.toString()).kind !=
      ResourceLinkKind.asset)
    throw const FormatException('Invalid linked asset');
  final client = HttpClient()..connectionTimeout = const Duration(seconds: 15);
  const limit = 64 * 1024 * 1024;
  try {
    final request = await client.getUrl(uri);
    request.followRedirects = false;
    headers.forEach(request.headers.set);
    final response = await request.close().timeout(const Duration(seconds: 30));
    if (response.statusCode != 200 || response.contentLength > limit)
      throw const FileSystemException('Linked attachment is unavailable');
    final bytes = BytesBuilder(copy: false);
    await for (final part in response.timeout(const Duration(seconds: 30))) {
      if (bytes.length + part.length > limit)
        throw const FileSystemException('Linked attachment exceeds 64 MiB');
      bytes.add(part);
    }
    return bytes.takeBytes();
  } finally {
    client.close(force: true);
  }
}
