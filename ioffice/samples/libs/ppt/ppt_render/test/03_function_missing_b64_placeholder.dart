import 'package:flutter_test/flutter_test.dart';

void main() {
  test('NB-EMPTY 无 b64', () {
    // DrawImage → 不 crash 且占位
    fail('spec: 03_function_missing_b64_placeholder NB-EMPTY');
  }, skip: 'spec: 03_function_missing_b64_placeholder');
  test('NB-BAD 坏 b64', () {
    // !!!! → 占位不 crash
    fail('spec: 03_function_missing_b64_placeholder NB-BAD');
  }, skip: 'spec: 03_function_missing_b64_placeholder');
}
