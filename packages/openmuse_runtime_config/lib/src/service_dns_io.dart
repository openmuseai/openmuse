import 'dart:async';
import 'dart:convert';
import 'dart:io';

/// Process-scoped recovery for installed HTTPS service origins when system DNS
/// fails. TLS still uses the original URL/SNI and default certificate checks.
/// Resolver requests contain only the public hostname, never account data.
void installServiceDnsFallback({
  required Set<Uri> origins,
  required Uri resolver,
}) {
  final hosts = origins
      .where((u) => u.scheme == 'https' && u.userInfo.isEmpty)
      .map((u) => u.host)
      .toSet();
  if (resolver.scheme != 'https' ||
      resolver.userInfo.isNotEmpty ||
      resolver.hasQuery ||
      resolver.hasFragment) {
    throw ArgumentError('A trusted HTTPS DNS endpoint is required');
  }
  final current = HttpOverrides.current;
  if (current is ServiceDnsOverrides) {
    current
      ..hosts = hosts
      ..resolver = resolver;
  } else if (current == null) {
    // Never replace a test/embedding application's own network override.
    HttpOverrides.global = ServiceDnsOverrides(
      hosts: hosts,
      resolver: resolver,
    );
  }
}

List<InternetAddress> decodeServiceDnsAnswers(String body) {
  if (body.length > 16384) throw const FormatException('DNS response limit');
  final value = jsonDecode(body);
  if (value is! Map || value['Status'] != 0 || value['Answer'] is! List) {
    throw const FormatException('DNS resolution unavailable');
  }
  final addresses = <InternetAddress>[];
  for (final answer in value['Answer'] as List) {
    if (answer is Map && answer['type'] == 1 && answer['data'] is String) {
      final address = InternetAddress.tryParse(answer['data'] as String);
      if (address?.type == InternetAddressType.IPv4 &&
          !address!.isLoopback &&
          !address.isMulticast) {
        addresses.add(address);
      }
    }
  }
  if (addresses.isEmpty) throw const FormatException('DNS address unavailable');
  return addresses;
}

final class ServiceDnsOverrides extends HttpOverrides {
  ServiceDnsOverrides({required this.hosts, required this.resolver});
  Set<String> hosts;
  Uri resolver;
  final _cache = <String, ({DateTime until, InternetAddress address})>{};
  Future<InternetAddress> _resolve(
    String host,
    SecurityContext? context,
  ) async {
    final cached = _cache[host];
    if (cached != null && cached.until.isAfter(DateTime.now())) {
      return cached.address;
    }
    final client = super.createHttpClient(context);
    try {
      final request = await client
          .getUrl(
            resolver.replace(queryParameters: {'name': host, 'type': 'A'}),
          )
          .timeout(const Duration(seconds: 5));
      request.followRedirects = false;
      final response = await request.close().timeout(
        const Duration(seconds: 5),
      );
      if (response.statusCode != 200) {
        throw const SocketException('DNS resolution unavailable');
      }
      final chunks = <int>[];
      await for (final chunk in response.timeout(const Duration(seconds: 5))) {
        if (chunks.length + chunk.length > 16384) {
          throw const SocketException('DNS response limit');
        }
        chunks.addAll(chunk);
      }
      final address = decodeServiceDnsAnswers(utf8.decode(chunks)).first;
      _cache[host] = (
        until: DateTime.now().add(const Duration(seconds: 60)),
        address: address,
      );
      return address;
    } finally {
      client.close(force: true);
    }
  }

  @override
  HttpClient createHttpClient(SecurityContext? context) {
    final client = super.createHttpClient(context);
    client.connectionFactory = (uri, proxyHost, proxyPort) async {
      final secureScheme = uri.scheme == 'https' || uri.scheme == 'wss';
      // WebSocket.connect can pass an HTTPS URI with port 0 after converting
      // a WSS URL with no explicit port. Match HttpClient's default-port rule.
      final port = uri.port != 0
          ? uri.port
          : secureScheme
          ? 443
          : uri.scheme == 'ws'
          ? 80
          : uri.port;
      if (proxyHost != null || !secureScheme || !hosts.contains(uri.host)) {
        if (proxyHost == null && secureScheme) {
          return SecureSocket.startConnect(uri.host, port, context: context);
        }
        return Socket.startConnect(proxyHost ?? uri.host, proxyPort ?? port);
      }
      InternetAddress? fallback;
      try {
        await InternetAddress.lookup(uri.host);
      } on SocketException {
        fallback = await _resolve(uri.host, context);
      }
      if (fallback == null) {
        return SecureSocket.startConnect(uri.host, port, context: context);
      }
      // A connectionFactory replaces HttpClient's TLS setup. Complete TLS with
      // the original hostname before returning any socket to the HTTP layer.
      final task = await Socket.startConnect(fallback, port);
      Socket? raw;
      var canceled = false;
      final secure = () async {
        try {
          raw = await task.socket;
          if (canceled) throw const SocketException('Connection canceled');
          return await SecureSocket.secure(
            raw!,
            host: uri.host,
            context: context,
          );
        } on Object {
          raw?.destroy();
          rethrow;
        }
      }();
      return ConnectionTask.fromSocket(secure, () {
        canceled = true;
        task.cancel();
        raw?.destroy();
      });
    };
    return client;
  }
}
