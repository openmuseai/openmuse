import 'package:muse_dsh_conversation_protocol/muse_dsh_conversation_protocol.dart'
    show isPrivateNativeTransportHost;
import 'mobile_build_locale.dart';

import 'package:flutter/material.dart';
import 'package:openmuse_mobile_core/openmuse_mobile_core.dart';

Uri resolveRemoteDshUri(DshSessionDescriptor session) {
  final origin = Uri.tryParse(session.origin);
  if (origin == null ||
      origin.host.isEmpty ||
      origin.userInfo.isNotEmpty ||
      origin.hasQuery ||
      origin.hasFragment ||
      (origin.path.isNotEmpty && origin.path != '/')) {
    throw const FormatException('invalid Remote DSH origin');
  }
  final loopback =
      origin.host == 'localhost' ||
      origin.host == '127.0.0.1' ||
      origin.host == '::1' ||
      origin.host == '10.0.2.2';
  final privateNetwork = isPrivateNativeTransportHost(origin.host);
  if (origin.scheme != 'https' &&
      !(session.allowInsecureLoopback && origin.scheme == 'http' && loopback) &&
      !(session.allowInsecurePrivateNetworkForTesting &&
          origin.scheme == 'http' &&
          privateNetwork)) {
    throw const FormatException('insecure Remote DSH origin');
  }
  if ((!session.path.startsWith('/session/') &&
          !session.path.startsWith('/u/')) ||
      session.path.contains('..') ||
      session.path.contains('#')) {
    throw const FormatException('invalid Remote DSH path');
  }
  final target = origin.resolve(session.path);
  if (!isAllowedRemoteDshNavigation(target, origin)) {
    throw const FormatException('cross-origin Remote DSH URL');
  }
  return target;
}

bool isAllowedRemoteDshNavigation(Uri target, Uri origin) =>
    target.scheme == origin.scheme &&
    target.host == origin.host &&
    target.port == origin.port &&
    target.userInfo.isEmpty;

/// Legacy web sessions cannot prove their script recipients or send approvals.
/// Retain the route identity without initializing a network-capable WebView.
final class RemoteDshPage extends StatelessWidget {
  const RemoteDshPage({
    this.strings,
    super.key,
    required this.session,
    required this.workspaceTitle,
  });
  final DshSessionDescriptor session;
  final String workspaceTitle;
  final OpenMuseStrings? strings;

  @override
  Widget build(BuildContext context) => Scaffold(
    appBar: AppBar(title: Text('$workspaceTitle · Agent')),
    body: Center(
      child: SingleChildScrollView(
        padding: const EdgeInsets.all(24),
        child: Text(
          (strings ?? stringsOf(context)).format('privacy.remote_unverified'),
          textAlign: TextAlign.center,
        ),
      ),
    ),
  );
}
