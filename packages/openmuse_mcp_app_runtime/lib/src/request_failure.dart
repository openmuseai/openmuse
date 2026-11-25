import 'dart:async';
import 'dart:io';

// Transport exception messages may contain URLs, tokens or response bodies.
String appRequestFailureCode(Object error) {
  if (error is TimeoutException) return 'APP_REQUEST_TIMEOUT';
  if (error is SocketException) return 'APP_NETWORK_UNAVAILABLE';
  if (error is HttpException &&
      RegExp(r'^PAIRED_APP_HTTP_[0-9]{3}$').hasMatch(error.message)) {
    return error.message;
  }
  if (error is StateError &&
      RegExp(r'^(APP_[A-Z_]+|VIEW_REVOKED)$').hasMatch('${error.message}')) {
    return '${error.message}';
  }
  if (error is FormatException) return 'APP_INVALID_RESPONSE';
  return 'APP_REQUEST_FAILED';
}
