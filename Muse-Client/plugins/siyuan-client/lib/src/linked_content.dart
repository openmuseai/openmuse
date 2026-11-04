import 'package:mime/mime.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';

ResourceSeed linkedDocumentContent(String name, List<int> bytes) =>
    ResourceSeed(
      bytes: bytes,
      // OOXML files have ZIP magic. Preserve their document media type so Word,
      // Excel and presentation handlers can be selected by the shared registry.
      mediaType:
          lookupMimeType(name) ??
          lookupMimeType('', headerBytes: bytes.take(64).toList()) ??
          'application/octet-stream',
    );
