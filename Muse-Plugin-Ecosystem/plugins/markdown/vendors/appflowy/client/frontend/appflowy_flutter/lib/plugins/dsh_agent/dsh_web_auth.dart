/// DSH Host authenticates the web UI with a per-process launch token.
///
/// Since 0.1.2-alpha.1 only `GET /?token=...` exchanges that token for a
/// signed session cookie. Navigating to `/` first produces
/// "dsh web authentication required; reopen the URL printed by dsh web."
class DshWebAuth {
  DshWebAuth._();

  static const listenHost = '127.0.0.1';
  static const listenPort = 3080;
  static const originUrl = 'http://$listenHost:$listenPort';

  static final _launchUrlPattern = RegExp(r'dsh web:\s*(\S+)');

  /// Full loopback URL printed by `dsh web`, or null when [text] has none.
  ///
  /// Current DSH prints `dsh web: http://127.0.0.1:3080` with no `?token=`.
  /// Older 0.1.2 hosts append a launch token. Both are valid session URLs.
  static String? extractLaunchUrl(String text) {
    final match = _launchUrlPattern.firstMatch(text);
    final raw = match?.group(1);
    if (raw == null) return null;
    final uri = Uri.tryParse(raw);
    if (uri == null || !uri.hasScheme || uri.host.isEmpty) return null;
    return raw;
  }

  /// Per-process launch token from a `dsh web:` log line, if present.
  static String? extractLaunchToken(String text) {
    final url = extractLaunchUrl(text);
    if (url == null) return null;
    return Uri.parse(url).queryParameters['token'];
  }

  /// Origin used by the sidecar readiness probe (no token, so it cannot
  /// steal the cookie the WebView must receive on first navigation).
  static Uri probeUri(String? sessionUrl) {
    final parsed = sessionUrl == null ? null : Uri.tryParse(sessionUrl);
    if (parsed == null || !parsed.hasScheme) {
      return Uri.parse(originUrl);
    }
    return Uri(
      scheme: parsed.scheme,
      host: parsed.host,
      port: parsed.hasPort ? parsed.port : null,
      path: '/',
    );
  }

  /// Ready when the host serves the UI. A 401/403 still needs a launch token
  /// so the WebView does not land on the Host authentication error page.
  static bool isStartupProbeHealthy(int status, String? launchToken) {
    if (status < 200 || status >= 500) return false;
    if (status == 401 || status == 403) {
      return launchToken != null && launchToken.isNotEmpty;
    }
    return true;
  }

  /// PIDs with a TCP LISTEN on loopback [port], parsed from `netstat -ano`.
  static Set<int> listeningPidsFromNetstat(String output, {int port = listenPort}) {
    final pids = <int>{};
    final pattern = RegExp(
      'TCP\\s+(?:127\\.0\\.0\\.1|\\[::1\\]):$port\\s+\\S+\\s+LISTENING\\s+(\\d+)',
      caseSensitive: false,
    );
    for (final match in pattern.allMatches(output)) {
      final pid = int.tryParse(match.group(1) ?? '');
      if (pid != null && pid > 0) pids.add(pid);
    }
    return pids;
  }

  static bool isAddressInUse(String log) {
    return log.contains('EADDRINUSE') ||
        log.contains('address already in use');
  }

  /// Short panel text for a sidecar crash; keep the Node stack in the log file.
  static String summarizeExit(int code, List<String> logTail) {
    final joined = logTail.join('\n');
    if (isAddressInUse(joined)) {
      return 'DSH sidecar exited with $code: $listenHost:$listenPort is already '
          'in use by a leftover node process. Retry after the sidecar reclaims '
          'the port, or close that node.exe and retry.';
    }
    const missingPrefix = 'Cannot find package ';
    for (final line in logTail) {
      final trimmed = line.trim();
      if (trimmed.startsWith(missingPrefix)) {
        return 'DSH sidecar exited with $code: $trimmed';
      }
    }
    for (final line in logTail.reversed) {
      final trimmed = line.trim();
      if (trimmed.startsWith('Error: dsh:')) {
        return 'DSH sidecar exited with $code: $trimmed';
      }
    }
    return logTail.isEmpty
        ? 'DSH sidecar exited with $code'
        : 'DSH sidecar exited with $code\n$joined';
  }
}
