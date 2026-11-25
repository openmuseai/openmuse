import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';

import 'package:flutter/material.dart';
import 'package:openmuse_i18n/openmuse_i18n.dart';

String _reportId() {
  final random = Random.secure();
  final bytes = List.generate(16, (_) => random.nextInt(256));
  bytes[6] = (bytes[6] & 15) | 64;
  bytes[8] = (bytes[8] & 63) | 128;
  final hex = bytes.map((b) => b.toRadixString(16).padLeft(2, '0')).join();
  return '${hex.substring(0, 8)}-${hex.substring(8, 12)}-${hex.substring(12, 16)}-${hex.substring(16, 20)}-${hex.substring(20)}';
}

String reportExcerpt(String value) =>
    String.fromCharCodes(value.runes.take(6000));

final class AiContentReport {
  AiContentReport({
    String? id,
    required this.category,
    required String response,
    required this.comment,
    required this.provider,
    required this.model,
  }) : id = id ?? _reportId(),
       response = reportExcerpt(response);
  final String id, category, response, comment, provider, model;
  Map<String, Object> toJson() => {
    'id': id,
    'category': category,
    'response': response,
    'comment': comment,
    'provider': provider,
    'model': model,
  };
}

final class AiReportFailure implements Exception {
  const AiReportFailure({this.signIn = false, this.limited = false});
  final bool signIn, limited;
}

final class AiContentReportClient {
  AiContentReportClient({
    required this.origin,
    required this.accessToken,
    required this.currentAccount,
    this.networkGuard,
    this.allowInsecureLoopback = false,
  }) {
    final loopback = origin.host == '127.0.0.1' || origin.host == 'localhost';
    if (origin.host.isEmpty ||
        origin.userInfo.isNotEmpty ||
        origin.hasQuery ||
        origin.hasFragment ||
        (origin.path.isNotEmpty && origin.path != '/') ||
        (origin.scheme != 'https' &&
            !(allowInsecureLoopback && loopback && origin.scheme == 'http'))) {
      throw const FormatException('Invalid report service origin');
    }
  }
  final Uri origin;
  final Future<String?> Function() accessToken;
  final String? Function() currentAccount;
  final VoidCallback? networkGuard;
  final bool allowInsecureLoopback;

  Future<String> submit(
    AiContentReport report, {
    required String account,
  }) async {
    final result = await _request('POST', account, report.toJson());
    if (result['accepted'] != true || result['id'] != report.id) {
      throw const AiReportFailure();
    }
    return report.id;
  }

  Future<void> eraseForAccount({required String account}) async {
    final result = await _request('DELETE', account, null);
    if (result['deleted'] != true) throw const AiReportFailure();
  }

  Future<Map<String, dynamic>> _request(
    String method,
    String account,
    Map<String, Object>? body,
  ) async {
    final token = await accessToken();
    if (token == null || token.isEmpty || currentAccount() != account) {
      throw const AiReportFailure(signIn: true);
    }
    networkGuard?.call();
    final client = HttpClient()
      ..connectionTimeout = const Duration(seconds: 10);
    try {
      final request = await client
          .openUrl(method, origin.resolve('/v1/content-reports'))
          .timeout(const Duration(seconds: 15));
      request.followRedirects = false;
      networkGuard?.call();
      if (currentAccount() != account) {
        throw const AiReportFailure(signIn: true);
      }
      request.headers.set(HttpHeaders.authorizationHeader, 'Bearer $token');
      request.headers.contentType = ContentType.json;
      if (body != null) request.write(jsonEncode(body));
      final response = await request.close().timeout(
        const Duration(seconds: 20),
      );
      if (response.statusCode == 401 || response.statusCode == 403) {
        throw const AiReportFailure(signIn: true);
      }
      if (response.statusCode == 429) {
        throw const AiReportFailure(limited: true);
      }
      if (response.statusCode != (method == 'POST' ? 201 : 200)) {
        throw const AiReportFailure();
      }
      final bytes = <int>[];
      await for (final chunk in response.timeout(const Duration(seconds: 10))) {
        bytes.addAll(chunk);
        if (bytes.length > 4096) throw const AiReportFailure();
      }
      final result = jsonDecode(utf8.decode(bytes));
      if (result is! Map<String, dynamic>) throw const AiReportFailure();
      return result;
    } on AiReportFailure {
      rethrow;
    } on Object {
      // Never display a token, origin URL or untrusted server error body.
      throw const AiReportFailure();
    } finally {
      client.close(force: true);
    }
  }
}

final class AiContentReportScope extends InheritedWidget {
  const AiContentReportScope({
    super.key,
    required this.client,
    required super.child,
  });
  final AiContentReportClient client;
  static AiContentReportClient? maybeOf(BuildContext context) => context
      .dependOnInheritedWidgetOfExactType<AiContentReportScope>()
      ?.client;
  @override
  bool updateShouldNotify(AiContentReportScope oldWidget) =>
      client != oldWidget.client;
}

Future<void> showAiContentReport({
  required BuildContext context,
  required AiContentReportClient client,
  required String response,
  required String provider,
  required String model,
}) async {
  final account = client.currentAccount();
  if (account == null) return;
  final result = await showDialog<String>(
    context: context,
    barrierDismissible: false,
    builder: (_) => AiContentReportDialog(
      client: client,
      account: account,
      response: response,
      provider: provider,
      model: model,
    ),
  );
  if (context.mounted && result != null) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text(stringsOf(context).commonAiReportReceived())),
    );
  }
}

final class AiContentReportDialog extends StatefulWidget {
  const AiContentReportDialog({
    super.key,
    required this.client,
    required this.account,
    required this.response,
    required this.provider,
    required this.model,
  });
  final AiContentReportClient client;
  final String account, response, provider, model;
  @override
  State<AiContentReportDialog> createState() => _AiContentReportDialogState();
}

final class _AiContentReportDialogState extends State<AiContentReportDialog> {
  final _comment = TextEditingController();
  String _category = 'harmful';
  AiContentReport? _lastAttempt;
  bool _sending = false;
  AiReportFailure? _failure;
  @override
  void dispose() {
    _comment.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    final previous = _lastAttempt;
    final report = AiContentReport(
      id: previous?.category == _category && previous?.comment == _comment.text
          ? previous?.id
          : null,
      category: _category,
      comment: _comment.text,
      response: widget.response,
      provider: widget.provider,
      model: widget.model,
    );
    setState(() {
      _sending = true;
      _failure = null;
      _lastAttempt = report;
    });
    try {
      final reference = await widget.client.submit(
        report,
        account: widget.account,
      );
      if (mounted) Navigator.pop(context, reference);
    } on Object catch (error) {
      if (mounted) {
        setState(() {
          _sending = false;
          _failure = error is AiReportFailure ? error : const AiReportFailure();
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final strings = stringsOf(context);
    final colors = Theme.of(context).colorScheme;
    return PopScope(
      canPop: !_sending,
      child: AlertDialog(
        title: Text(strings.commonAiReportTitle()),
        content: SizedBox(
          width: 480,
          child: SingleChildScrollView(
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Text(strings.commonAiReportDisclosure()),
                const SizedBox(height: 16),
                ConstrainedBox(
                  constraints: const BoxConstraints(maxHeight: 130),
                  child: SingleChildScrollView(
                    child: SelectableText(
                      reportExcerpt(widget.response),
                      style: Theme.of(context).textTheme.bodySmall,
                    ),
                  ),
                ),
                const SizedBox(height: 16),
                DropdownButtonFormField<String>(
              key: const ValueKey('ai-report-reason'),
                  initialValue: _category,
                  decoration: InputDecoration(
                    labelText: strings.commonAiReportReason(),
                  ),
                  isExpanded: true,
                  items: [
                    DropdownMenuItem(
                      value: 'harmful',
                      child: Text(strings.commonAiReportHarmful()),
                    ),
                    DropdownMenuItem(
                      value: 'misleading',
                      child: Text(strings.commonAiReportMisleading()),
                    ),
                    DropdownMenuItem(
                      value: 'privacy',
                      child: Text(strings.commonAiReportPrivacy()),
                    ),
                    DropdownMenuItem(
                      value: 'other',
                      child: Text(strings.commonAiReportOther()),
                    ),
                  ],
                  onChanged: _sending
                      ? null
                      : (value) => setState(() => _category = value!),
                ),
                const SizedBox(height: 12),
                TextField(
              key: const ValueKey('ai-report-comment'),
                  controller: _comment,
                  enabled: !_sending,
                  maxLength: 500,
                  maxLines: 3,
                  decoration: InputDecoration(
                    labelText: strings.commonAiReportComment(),
                  ),
                ),
                if (_failure case final error?)
                  Semantics(
                    liveRegion: true,
                    child: Text(
                      error.signIn
                          ? strings.commonAiReportSignIn()
                          : error.limited
                          ? strings.commonAiReportLimit()
                          : strings.commonAiReportFailed(),
                      style: TextStyle(color: colors.error),
                    ),
                  ),
              ],
            ),
          ),
        ),
        actions: [
          TextButton(
            onPressed: _sending ? null : () => Navigator.pop(context),
            child: Text(strings.commonAiSharingCancel()),
          ),
          FilledButton(
            key: const ValueKey('ai-report-submit'),
            onPressed: _sending ? null : _submit,
            child: Text(
              _sending
                  ? strings.commonAiReportSending()
                  : strings.commonAiReportSubmit(),
            ),
          ),
        ],
      ),
    );
  }
}
