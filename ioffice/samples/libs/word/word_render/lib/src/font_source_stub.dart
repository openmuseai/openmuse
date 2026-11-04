import 'dart:typed_data';

class HostFontFile {
  const HostFontFile(this.id, this.bytes);
  final String id;
  final Uint8List bytes;
}

Uint8List? tryLoadHostFontBytes() => null;

List<HostFontFile> tryLoadHostFonts({bool includeCjk = false}) => const [];
