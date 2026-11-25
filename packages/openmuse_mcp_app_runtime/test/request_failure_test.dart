import 'dart:async';
import 'dart:io';
import 'package:flutter_test/flutter_test.dart';
import 'package:openmuse_mcp_app_runtime/src/request_failure.dart';

void main() {
  test('classifies known failures without rendering transport details', () {
    expect(appRequestFailureCode(TimeoutException('secret URL')), 'APP_REQUEST_TIMEOUT');
    expect(appRequestFailureCode(const SocketException('secret host')), 'APP_NETWORK_UNAVAILABLE');
    expect(appRequestFailureCode(const HttpException('PAIRED_APP_HTTP_502')), 'PAIRED_APP_HTTP_502');
    expect(appRequestFailureCode(StateError('APP_SESSION_REVOKED')), 'APP_SESSION_REVOKED');
    expect(appRequestFailureCode(const FormatException('secret response')), 'APP_INVALID_RESPONSE');
  });
  test('untrusted error messages never cross into the App frame', () {
    for (final error in <Object>[
      StateError('APP_SECRET https://private/?token=secret'),
      const HttpException('PAIRED_APP_HTTP_502?token=secret'),
      Exception('secret body'),
    ]) {
      expect(appRequestFailureCode(error), 'APP_REQUEST_FAILED');
    }
  });
}
