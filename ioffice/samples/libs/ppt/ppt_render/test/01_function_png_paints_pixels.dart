import 'package:flutter_test/flutter_test.dart';

void main() {
  test('PNG-PX 1x1 png', () {
    // 红像素 PNG → 中心≠CCCCCC
    fail('spec: 01_function_png_paints_pixels PNG-PX');
  }, skip: 'spec: 01_function_png_paints_pixels');
  test('PNG-SZ 尺寸', () {
    // w/h → 画在 cmd 矩形内
    fail('spec: 01_function_png_paints_pixels PNG-SZ');
  }, skip: 'spec: 01_function_png_paints_pixels');
}
