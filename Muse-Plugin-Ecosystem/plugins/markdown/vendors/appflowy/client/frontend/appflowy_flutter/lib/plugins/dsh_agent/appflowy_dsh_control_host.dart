import 'dart:async';

import 'package:appflowy/env/cloud_env.dart';
import 'package:appflowy/mobile/presentation/editor/mobile_editor_screen.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_body_snapshot.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_device_token_service.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_host_plugin_plane.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_workspace_catalog.dart';
import 'package:appflowy/plugins/word/word_snapshot_cache.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/workspace/application/view/view_ext.dart';
import 'package:appflowy/workspace/application/view/view_service.dart';
import 'package:appflowy/workspace/presentation/home/menu/menu_shared_state.dart';
import 'package:appflowy_backend/dispatch/dispatch.dart';
import 'package:appflowy_backend/log.dart';
import 'package:appflowy_backend/protobuf/flowy-document/entities.pb.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pb.dart';
import 'package:flutter/material.dart';
import 'package:muse_appflowy_facets/muse_appflowy_facets.dart';
import 'package:muse_dsh_mobile/muse_dsh_mobile.dart';
import 'package:muse_plugin_facets/muse_plugin_facets.dart';
import 'package:muse_remote_session/facet_transport.dart';
import 'package:muse_remote_session/parent_bridge_adapter.dart';
import 'package:muse_ui_surface_runtime/muse_ui_surface_runtime.dart';
import 'package:nanoid/nanoid.dart';

/// AppFlowy implementation of the DSH control port. Not part of muse_dsh_mobile.
class AppFlowyDshControlHost implements DshMobileControlHost {
  AppFlowyDshControlHost({required this.navigator});

  final NavigatorState Function() navigator;

  @override
  Future<DshControlSession> connect(DshControlSessionRequest request) async {
    if (!request.isCloudAccount) {
      throw const DshControlConnectException('CLOUD_LOGIN_REQUIRED');
    }
    final cloud = getIt<AppFlowyCloudSharedEnv>().appflowyCloudConfig.base_url;
    if (cloud.isEmpty || isLocalAuthEnabled) {
      throw const DshControlConnectException('CLOUD_LOGIN_REQUIRED');
    }
    try {
      final credential = await getIt<DshDeviceTokenService>().loadOrIssue(
        sessionRef: '',
        accountRef: request.accountRef,
        cloudOrigin: Uri.parse(cloud).origin,
        dshOrigin: request.endpoint.origin,
      );
      if (!request.isLive()) {
        throw const DshControlConnectException('STALE_REQUEST');
      }
      return await _AppFlowyDshControlSession.open(
        request: request,
        credential: credential,
        navigator: navigator,
      );
    } on DshControlConnectException {
      rethrow;
    } on DshCredentialException catch (error) {
      throw DshControlConnectException(error.code);
    } on MuseTransportException catch (error) {
      throw DshControlConnectException(error.code, error.status);
    }
  }
}

class _AppFlowyDshControlSession implements DshControlSession {
  _AppFlowyDshControlSession._(this._request);

  final DshControlSessionRequest _request;
  final _feed = MuseSurfaceContextFeed.instance;
  MuseParentBridgeAdapter? _adapter;
  MuseWorkspaceUiFacet? _facet;
  final _plane = DshHostPluginPlane();
  StreamSubscription? _contexts;
  StreamSubscription? _closedSurfaces;
  Timer? _expiry;
  Timer? _selectionDebounce;
  Timer? _contextHeartbeat;
  final _forwarded = <String>{};
  String? _sourceView;
  bool _nativeDocumentVisible = false;
  var _open = true;

  static Future<_AppFlowyDshControlSession> open({
    required DshControlSessionRequest request,
    required DshDeviceCredential credential,
    required NavigatorState Function() navigator,
  }) async {
    final session = _AppFlowyDshControlSession._(request);
    await session._start(credential, navigator);
    return session;
  }

  bool _live() => _open && _request.isLive();

  Future<void> _start(
    DshDeviceCredential credential,
    NavigatorState Function() navigator,
  ) async {
    final inbox =
        MuseIntentInbox(scopeRef: 'workspace.${_request.workspaceRef}');
    final facet = MuseWorkspaceUiFacet(
      workspaceId: _request.workspaceRef,
      title: _request.workspaceTitle,
      instanceRef: 'mobile.${nanoid(20)}',
      navigate: (viewId, blockId) => _navigate(viewId, blockId, navigator),
    );
    facet.register(inbox);
    final adapter = MuseParentBridgeAdapter(
      transport: MuseHttpSseTransport(
        endpoint: dshParentBridgeUri(_request.endpoint),
        headers: {
          'Authorization': 'Bearer ${credential.token}',
          'X-Muse-Device-Id': credential.deviceId,
          'X-Muse-Client': 'mobile',
          'X-Muse-Connection-Id': facet.instanceRef,
        },
      ),
      inbox: inbox,
      workspaceId: _request.workspaceRef,
      workspaceTitle: _request.workspaceTitle,
      deviceId: credential.deviceId,
    );
    _adapter = adapter;
    _facet = facet;
    adapter.onDisconnected = (_) {
      if (_live()) _request.onDisconnected();
    };
    await adapter.connect();
    if (!_live()) return;
    final latest = getIt<MenuSharedState>().latestOpenView;
    _sourceView = latest?.id ?? _sourceView;
    await adapter.contribute(facet.focus(viewId: _sourceView));
    if (!_live()) return;
    await _contributePluginPlane(adapter, latestLayout: latest?.layout);
    if (!_live()) return;
    _contexts = _feed.contributions.listen(_forward);
    _closedSurfaces = _feed.closedSurfaces.listen((ref) {
      if (_forwarded.remove(ref) && _live()) {
        unawaited(
          adapter.closeSurface(ref).catchError((Object _) {
            if (_live()) {
              _request.onDegraded(DshMobileErrorCode.surfaceCloseFailed.message);
            }
          }),
        );
      }
    });
    for (final value in _feed.snapshot(_sourceView ?? '')) {
      _forward(value);
    }
    _contextHeartbeat = Timer.periodic(const Duration(seconds: 60), (_) {
      if (_live()) {
        unawaited(
          () async {
            await adapter.contribute(facet.focus(viewId: _sourceView));
            await _contributePluginPlane(
              adapter,
              latestLayout: getIt<MenuSharedState>().latestOpenView?.layout,
            );
          }()
              .catchError((Object _) {
            if (_live()) {
              _request.onDegraded(DshMobileErrorCode.contextRefreshFailed.message);
            }
          }),
        );
      }
    });
    _expiry = Timer(
        Duration(
          milliseconds: (credential.expiresAt -
                  DateTime.now().millisecondsSinceEpoch -
                  30000)
              .clamp(0, 1 << 31),
        ), () {
      if (_live()) {
        _request.onDegraded(DshMobileErrorCode.credentialExpiring.message);
      }
    });
  }

  Future<void> _contributePluginPlane(
    MuseParentBridgeAdapter adapter, {
    ViewLayoutPB? latestLayout,
  }) async {
    if (!_live()) return;
    final views = await ViewBackendService.getAllViews();
    final items = views.fold(
      (list) => list.items
          .map(
            (view) => DshCatalogViewInput(
              viewId: view.id,
              title: view.name,
              parentViewId: view.parentViewId,
              layout: view.layout.value,
              isSpace: view.isSpace,
            ),
          )
          .toList(),
      (error) {
        Log.warn('DSH catalog views unavailable: $error');
        return const <DshCatalogViewInput>[];
      },
    );
    if (!_live()) return;
    final catalog = _plane.catalog(
      workspaceId: _request.workspaceRef,
      views: items,
      instanceRef: _facet?.instanceRef,
    );
    if (catalog != null) {
      await adapter.contribute(catalog);
    }
    final viewId = _sourceView;
    if (viewId == null || viewId.isEmpty) {
      return;
    }
    final kind = dshBodySnapshotKind(
      layout: latestLayout?.value,
      viewId: viewId,
    );
    if (kind == DshBodySnapshotKind.word) {
      final snapshot = _plane.wordSnapshot(
        workspaceId: _request.workspaceRef,
        viewId: viewId,
        text: WordSnapshotCache.instance.textFor(viewId) ?? '',
        instanceRef: _facet?.instanceRef,
      );
      if (snapshot != null) {
        await adapter.contribute(snapshot);
      }
      return;
    }
    if (kind != DshBodySnapshotKind.markdown) {
      return;
    }
    final document = await DocumentEventGetDocumentText(
      OpenDocumentPayloadPB(documentId: viewId),
    ).send();
    final text = document.fold((pb) => pb.text, (error) {
      Log.warn('DSH snapshot text unavailable: $error');
      return '';
    });
    if (!_live()) return;
    final snapshot = _plane.snapshot(
      workspaceId: _request.workspaceRef,
      viewId: viewId,
      text: text,
      instanceRef: _facet?.instanceRef,
    );
    if (snapshot != null) {
      await adapter.contribute(snapshot);
    }
  }

  void _forward(MuseContextContributionV1 value) {
    final adapter = _adapter;
    if (!_live() ||
        adapter == null ||
        value.scopeRef != _sourceView) {
      return;
    }
    if (value.expiresAt <= DateTime.now().millisecondsSinceEpoch) return;
    if (value.contextType == 'markdown.selection' && !_nativeDocumentVisible) {
      return;
    }
    if (value.contextType != 'markdown.surface' &&
        value.contextType != 'markdown.selection' &&
        value.contextType != 'word.surface' &&
        value.contextType != 'word.selection' &&
        value.contextType != 'word.snapshot') {
      return;
    }
    void send() {
      if (!_live()) return;
      _forwarded.add(value.surfaceInstanceRef);
      unawaited(
        adapter.contribute(value).catchError((Object _) {
          if (_live()) {
            _request.onDegraded(DshMobileErrorCode.contextSyncFailed.message);
          }
        }),
      );
    }

    if (value.contextType == 'markdown.selection') {
      _selectionDebounce?.cancel();
      _selectionDebounce = Timer(const Duration(milliseconds: 200), send);
    } else {
      send();
    }
  }

  Future<MusePresentationIntentStatus> _navigate(
    String viewId,
    String? blockId,
    NavigatorState Function() navigator,
  ) async {
    final adapter = _adapter;
    final facet = _facet;
    if (!_live() || adapter == null || facet == null || _nativeDocumentVisible) {
      return MusePresentationIntentStatus.surfaceClosed;
    }
    if (blockId != null) return MusePresentationIntentStatus.notSupported;
    final view = (await ViewBackendService.getView(viewId)).toNullable();
    if (view == null) return MusePresentationIntentStatus.notFound;
    if (view.layout != ViewLayoutPB.Document) {
      return MusePresentationIntentStatus.notSupported;
    }
    if (!_live()) {
      return MusePresentationIntentStatus.surfaceClosed;
    }
    _sourceView = viewId;
    await adapter.contribute(facet.focus(viewId: viewId));
    if (!_live()) {
      return MusePresentationIntentStatus.surfaceClosed;
    }
    final ready = Completer<void>();
    final readySubscription = _feed.contributions.listen((value) {
      if (value.scopeRef == viewId &&
          value.contextType == 'markdown.surface' &&
          !ready.isCompleted) {
        ready.complete();
      }
    });
    _nativeDocumentVisible = true;
    unawaited(
      navigator()
          .push<void>(
        MaterialPageRoute(
          builder: (_) => MobileDocumentScreen(id: viewId, title: view.name),
        ),
      )
          .whenComplete(() {
        _nativeDocumentVisible = false;
        if (_live()) {
          _request.onDegraded(DshMobileErrorCode.nativeDocumentReturned.message);
        }
      }),
    );
    try {
      await ready.future.timeout(const Duration(seconds: 8));
    } on TimeoutException {
      return MusePresentationIntentStatus.timedOut;
    } finally {
      await readySubscription.cancel();
    }
    if (!_live()) {
      return MusePresentationIntentStatus.surfaceClosed;
    }
    return MusePresentationIntentStatus.applied;
  }

  @override
  Future<void> close() async {
    _open = false;
    _expiry?.cancel();
    _selectionDebounce?.cancel();
    _contextHeartbeat?.cancel();
    _facet?.close();
    _facet = null;
    final adapter = _adapter;
    _adapter = null;
    final contexts = _contexts;
    final closedSurfaces = _closedSurfaces;
    _contexts = null;
    _closedSurfaces = null;
    _forwarded.clear();
    await contexts?.cancel();
    await closedSurfaces?.cancel();
    if (adapter != null) await adapter.close();
  }
}
