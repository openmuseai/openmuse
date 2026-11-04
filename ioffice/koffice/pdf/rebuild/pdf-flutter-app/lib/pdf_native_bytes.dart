import 'dart:ffi';

/// Matches KspProductBytes in the versioned product C ABI.
final class PdfNativeBytes extends Struct {
  external Pointer<Uint8> data;

  @Uint64()
  external int size;
}
