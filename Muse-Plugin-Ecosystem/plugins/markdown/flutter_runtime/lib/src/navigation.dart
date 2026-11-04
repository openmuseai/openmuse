enum ClientNavigation { local, external, blocked }

ClientNavigation classifyClientNavigation(Uri origin, String raw) {
  final target = Uri.tryParse(raw);
  if (target == null) return ClientNavigation.blocked;
  if (target.toString() == 'about:blank') return ClientNavigation.local;
  if (target.scheme == 'http' || target.scheme == 'https') {
    if (target.origin == origin.origin) return ClientNavigation.local;
    return ClientNavigation.external;
  }
  if (target.scheme == 'mailto') return ClientNavigation.external;
  return ClientNavigation.blocked;
}
