import 'dart:async';
import 'package:flutter/material.dart';
import 'package:openmuse_auth_gotrue/openmuse_auth_gotrue_web.dart';
import 'package:openmuse_runtime_config/openmuse_runtime_config.dart';
import 'package:openmuse_plugin_sdk/openmuse_plugin_sdk.dart';
import 'package:openmuse_smart_document/openmuse_smart_document.dart';
import 'package:openmuse_smart_document_plugins/public_document.dart';
import 'web_session_page.dart';
import 'package:web/web.dart' as web;

/// A link opens a read-only original document, including for signed-in users.
/// Only the explicit Edit action enters the authenticated resource/plugin path.
final class DocumentLinkPage extends StatefulWidget {
  const DocumentLinkPage({
    super.key,
    required this.target,
    required this.authentication,
    this.config,
    this.initialEditing = false,
  });
  final DocumentBrowserTarget target;
  final GoTrueAuthenticationController authentication;
  final RuntimeConfig? config;
  final bool initialEditing;
  @override
  State<DocumentLinkPage> createState() => _DocumentLinkPageState();
}

final class _DocumentLinkPageState extends State<DocumentLinkPage> {
  SmartDocumentController? _controller;
  DocumentMediaSession? _media;
  PublicDocumentReader? _reader;
  bool _loading = true;
  Object? _failure;
  int _epoch = 0;
  @override
  void initState() {
    super.initState();
    if (!widget.initialEditing) unawaited(_open());
  }

  Future<void> _open() async {
    final epoch = ++_epoch;
    final previous = _controller;
    _controller = null;
    previous?.invalidateAuthority();
    _media?.dispose();
    _media = null;
    _reader?.close();
    setState(() {
      _loading = true;
      _failure = null;
    });
    final reader = PublicDocumentReader(
      origin:
          widget.config?.webServiceUri(ServiceId.cloud, Uri.base) ??
          Uri.parse(Uri.base.origin),
      target: widget.target,
      onNameChanged: () {
        if (mounted && epoch == _epoch) setState(() {});
      },
      onUnavailable: () {
        if (!mounted || epoch != _epoch) return;
        _controller?.invalidateAuthority();
        setState(
          () => _failure = const SmartDocumentFailure(
            'PUBLIC_DOCUMENT_UNAVAILABLE',
          ),
        );
      },
    );
    _reader = reader;
    try {
      await previous?.close();
      final core = await loadDocumentCore();
      if (!mounted || epoch != _epoch) return;
      final controller = await SmartDocumentController.open(
        core: core,
        store: TransientDocumentStore(),
        objectId: widget.target.viewId,
        provider: reader,
        editable: false,
        syncInterval: const Duration(seconds: 2),
      );
      final media = DocumentMediaSession(
        controller: controller,
        repository: PublicDocumentAssets(reader),
        draftStore: TransientDocumentStore(),
        networkReader: createDocumentNetworkImageReader(),
      );
      if (!mounted || epoch != _epoch) {
        media.dispose();
        await controller.close();
        return;
      }
      setState(() {
        _controller = controller;
        _media = media;
      });
    } catch (error) {
      reader.close();
      if (mounted && epoch == _epoch) setState(() => _failure = error);
    } finally {
      if (!mounted || epoch != _epoch) {
        reader.close();
      } else {
        setState(() => _loading = false);
      }
    }
  }

  Future<void> _edit() async {
    ++_epoch;
    final controller = _controller;
    _controller = null;
    _media?.dispose();
    _media = null;
    _reader?.close();
    _reader = null;
    await controller?.close();
    if (!mounted) return;
    // Reload at the identity-only edit route so the original installed-region
    // and consent gates run before constructing an authenticated editor.
    web.window.history.replaceState(
      null,
      '',
      Uri.base.replace(fragment: '${widget.target.route}/edit').toString(),
    );
    web.window.location.reload();
  }

  @override
  void dispose() {
    ++_epoch;
    _controller?.invalidateAuthority();
    _media?.dispose();
    _reader?.close();
    unawaited(_controller?.close());
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final zh = Localizations.localeOf(context).languageCode == 'zh';
    if (widget.initialEditing) {
      return AnimatedBuilder(
        animation: widget.authentication,
        builder: (context, _) {
          final auth = widget.authentication;
          if (auth.snapshot.isAuthenticated) {
            return WebSessionPage(
              authentication: auth,
              config: widget.config,
              initialDocument: (
                workspaceId: widget.target.workspaceId,
                viewId: widget.target.viewId,
              ),
            );
          }
          if (auth.snapshot.phase == OpenMuseAuthenticationPhase.restoring) {
            return const Scaffold(
              body: Center(child: CircularProgressIndicator()),
            );
          }
          return OpenMuseLoginScreen(
            authentication: auth,
            cloudLabel: zh ? '登录以编辑智能文档' : 'Sign in to edit SmartDocument',
            termsUri: widget.config?.linkUri(
              LinkId.terms,
              locale: zh ? 'zh-Hans' : 'en',
            ),
            privacyUri: widget.config?.linkUri(
              LinkId.privacy,
              locale: zh ? 'zh-Hans' : 'en',
            ),
          );
        },
      );
    }
    final controller = _controller;
    return Scaffold(
      appBar: AppBar(
        automaticallyImplyLeading: false,
        title: Text(zh ? '智能文档 · 只读浏览' : 'SmartDocument · Read only'),
        actions: [
          TextButton.icon(
            key: const ValueKey('public-document-edit'),
            onPressed: _edit,
            icon: const Icon(Icons.edit_outlined),
            label: Text(zh ? '编辑' : 'Edit'),
          ),
        ],
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : _failure != null || controller == null
          ? Center(
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  Text(
                    zh
                        ? '此文档未开启分享或已停止分享。'
                        : 'This document is not shared or sharing has ended.',
                  ),
                  TextButton(
                    onPressed: _open,
                    child: Text(zh ? '重试' : 'Retry'),
                  ),
                ],
              ),
            )
          : SmartDocumentView(
              controller: controller,
              title: _reader!.name,
              media: _media,
              statusLabel: zh ? '实时内容 · 只读' : 'Live content · Read only',
            ),
    );
  }
}
