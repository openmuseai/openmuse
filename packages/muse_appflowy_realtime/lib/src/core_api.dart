abstract interface class RealtimeWireCore {
  Map<String, dynamic> call(Map<String, dynamic> request);
}

final class RealtimeFailure implements Exception {
  const RealtimeFailure(this.code);
  final String code;
  @override
  String toString() => 'RealtimeFailure($code)';
}
