/// Web / 无 IO 平台：没有本机 dylib，返回 null（宿主走 golden JSON / wasm 延迟路径）。
String? findFfiLibrary() => null;

bool get isAndroidHost => false;

Future<String?> resolveFfiLibrary() async => null;
