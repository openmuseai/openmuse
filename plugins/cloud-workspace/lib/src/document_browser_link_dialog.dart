import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:openmuse_mobile_cloud/openmuse_mobile_cloud.dart';

Future<void> showDocumentBrowserLinkDialog(
  BuildContext context, {
  required AppFlowyCloudWorkspaceService service,
  required String workspaceId,
  required String viewId,
  required Uri browserLink,
}) => showDialog<void>(
  context: context,
  builder: (_) => _BrowserLinkDialog(
    service: service,
    workspaceId: workspaceId,
    viewId: viewId,
    link: browserLink,
  ),
);

final class _BrowserLinkDialog extends StatefulWidget {
  const _BrowserLinkDialog({
    required this.service,
    required this.workspaceId,
    required this.viewId,
    required this.link,
  });
  final AppFlowyCloudWorkspaceService service;
  final String workspaceId, viewId;
  final Uri link;
  @override
  State<_BrowserLinkDialog> createState() => _BrowserLinkDialogState();
}

final class _BrowserLinkDialogState extends State<_BrowserLinkDialog> {
  bool? _published;
  bool _busy = false;
  Object? _error;
  @override
  void initState() {
    super.initState();
    _read();
  }

  Future<void> _read() async {
    setState(() {
      _busy = true;
      _error = null;
    });
    try {
      final value = await widget.service.isDocumentPublished(
        widget.workspaceId,
        widget.viewId,
      );
      if (mounted) setState(() => _published = value);
    } catch (error) {
      if (mounted) setState(() => _error = error);
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  Future<void> _change(bool value) async {
    setState(() {
      _busy = true;
      _error = null;
    });
    try {
      if (value) {
        await widget.service.publishDocument(widget.workspaceId, widget.viewId);
      } else {
        await widget.service.unpublishDocument(
          widget.workspaceId,
          widget.viewId,
        );
      }
      // Observe the original state after the acknowledgement; errors stay
      // visible and never imply a confirmed publication or revocation.
      await _read();
    } catch (error) {
      if (mounted) setState(() => _error = error);
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    final zh = Localizations.localeOf(context).languageCode == 'zh';
    return AlertDialog(
      title: Text(zh ? '浏览器只读链接' : 'Read-only browser link'),
      content: SizedBox(
        width: 440,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              zh
                  ? '开启后，获得链接的人无需登录即可浏览最新内容。编辑需要登录并有编辑权限。'
                  : 'Anyone with the link can view live content. Editing requires sign-in and permission.',
            ),
            const SizedBox(height: 12),
            if (_published != null)
              SwitchListTile(
                contentPadding: EdgeInsets.zero,
                key: const ValueKey('document-publication-switch'),
                title: Text(zh ? '允许链接浏览' : 'Allow link viewing'),
                value: _published!,
                onChanged: _busy || _error != null ? null : _change,
              ),
            if (_busy) const LinearProgressIndicator(),
            if (_error != null) ...[
              Text(
                zh
                    ? '无法确认分享状态，请重试。'
                    : 'Could not confirm sharing status. Retry.',
              ),
              TextButton(
                onPressed: _busy ? null : _read,
                child: Text(zh ? '重试' : 'Retry'),
              ),
            ],
            if (_published == true)
              SelectableText(
                widget.link.toString(),
                key: const ValueKey('document-browser-link'),
              ),
          ],
        ),
      ),
      actions: [
        TextButton(
          onPressed: () => Navigator.pop(context),
          child: Text(zh ? '关闭' : 'Close'),
        ),
        FilledButton.icon(
          key: const ValueKey('document-browser-link-copy'),
          onPressed: _busy || _error != null || _published != true
              ? null
              : () async {
                  await Clipboard.setData(
                    ClipboardData(text: widget.link.toString()),
                  );
                },
          icon: const Icon(Icons.copy),
          label: Text(zh ? '复制链接' : 'Copy link'),
        ),
      ],
    );
  }
}
