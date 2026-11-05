import 'package:appflowy/ai/service/appflowy_ai_service.dart';
import 'package:appflowy/core/config/kv.dart';
import 'package:appflowy/core/network_monitor.dart';
import 'package:appflowy/env/cloud_env.dart';
import 'package:appflowy/mobile/presentation/search/view_ancestor_cache.dart';
import 'package:appflowy/plugins/document/application/prelude.dart';
import 'package:appflowy/plugins/document/presentation/editor_plugins/copy_and_paste/clipboard_service.dart';
import 'package:appflowy/plugins/trash/application/prelude.dart';
import 'package:appflowy/shared/appflowy_cache_manager.dart';
import 'package:appflowy/shared/custom_image_cache_manager.dart';
import 'package:appflowy/shared/easy_localiation_service.dart';
import 'package:appflowy/startup/startup.dart';
import 'package:appflowy/startup/tasks/appflowy_cloud_task.dart';
import 'package:appflowy/user/application/auth/af_cloud_auth_service.dart';
import 'package:appflowy/user/application/auth/auth_service.dart';
import 'package:appflowy/user/application/prelude.dart';
import 'package:appflowy/user/application/reminder/reminder_bloc.dart';
import 'package:appflowy/user/application/user_listener.dart';
import 'package:appflowy/user/presentation/router.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_agent_controller.dart';
import 'package:appflowy/plugins/resource_surface/engine_registry.dart';
import 'package:appflowy/plugins/resource_surface/engines/register.dart';
import 'package:appflowy/plugins/resource_surface/helix/helix_language_servers.dart';
import 'package:appflowy/plugins/resource_surface/helix/helix_settings.dart';
import 'package:appflowy/plugins/resource_surface/muse_presentation_channel.dart';
import 'package:appflowy/plugins/resource_surface/resource_open_defaults.dart';
import 'package:appflowy/plugins/resource_surface/resource_tab_action_registry.dart';
import 'package:appflowy/plugins/resource_surface/resource_tab_actions.dart';
import 'package:appflowy/plugins/version_diff/application/text_version_diff_service.dart';
import 'package:appflowy/plugins/version_diff/application/image_overlay_diff_service.dart';
import 'package:appflowy/plugins/version_diff/application/text_version_repository.dart';
import 'package:appflowy/plugins/version_diff/domain/diff_workbench_contract.dart';
import 'package:appflowy/plugins/version_diff/domain/version_diff_contract.dart';
import 'package:appflowy/plugins/version_diff/generic/binary_metadata_diff_provider.dart';
import 'package:appflowy/plugins/version_diff/image/image_overlay_diff_provider.dart';
import 'package:appflowy/plugins/version_diff/presentation/resource_version_pane.dart';
import 'package:appflowy/plugins/version_diff/text/text_diff_provider.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_sidecar.dart';
import 'package:appflowy/plugins/dsh_agent/dsh_device_token_service.dart';
import 'package:appflowy/workspace/application/action_navigation/action_navigation_bloc.dart';
import 'package:appflowy/workspace/application/edit_panel/edit_panel_bloc.dart';
import 'package:appflowy/workspace/application/favorite/favorite_bloc.dart';
import 'package:appflowy/workspace/application/recent/cached_recent_service.dart';
import 'package:appflowy/workspace/application/settings/appearance/base_appearance.dart';
import 'package:appflowy/workspace/application/settings/appearance/desktop_appearance.dart';
import 'package:appflowy/workspace/application/settings/appearance/mobile_appearance.dart';
import 'package:appflowy/workspace/application/settings/prelude.dart';
import 'package:appflowy/workspace/application/sidebar/rename_view/rename_view_bloc.dart';
import 'package:appflowy/workspace/application/subscription_success_listenable/subscription_success_listenable.dart';
import 'package:appflowy/workspace/application/tabs/tabs_bloc.dart';
import 'package:appflowy/workspace/application/user/prelude.dart';
import 'package:appflowy/workspace/application/view/prelude.dart';
import 'package:appflowy/workspace/application/workspace/prelude.dart';
import 'package:appflowy/workspace/presentation/home/menu/menu_shared_state.dart';
import 'package:appflowy/workspace_platform/application/workspace_controller.dart';
import 'package:appflowy/workspace_platform/infrastructure/local_workspace_provider.dart';
import 'package:appflowy/workspace_platform/infrastructure/workspace_persistence.dart';
import 'package:appflowy/workspace_platform/infrastructure/workspace_provider.dart';
import 'package:appflowy_backend/log.dart';
import 'package:appflowy_backend/protobuf/flowy-folder/view.pb.dart';
import 'package:appflowy_backend/protobuf/flowy-user/protobuf.dart';
import 'package:appflowy_popover/appflowy_popover.dart';
import 'package:flowy_infra/file_picker/file_picker_impl.dart';
import 'package:flowy_infra/file_picker/file_picker_service.dart';
import 'package:fluttertoast/fluttertoast.dart';
import 'package:get_it/get_it.dart';
import 'package:universal_platform/universal_platform.dart';

class DependencyResolver {
  static Future<void> resolve(
    GetIt getIt,
    IntegrationMode mode,
  ) async {
    // getIt.registerFactory<KeyValueStorage>(() => RustKeyValue());
    getIt.registerFactory<KeyValueStorage>(() => DartKeyValue());

    await _resolveCloudDeps(getIt);
    _resolveUserDeps(getIt, mode);
    _resolveHomeDeps(getIt);
    _resolveFolderDeps(getIt);
    _resolveCommonService(getIt, mode);

    // Install the Host→Flutter presentation dispatch channel once for the
    // lifetime of the app: until this succeeds the Rust
    // `muse.resource-presentation` provider fails closed with
    // `SURFACE_UNAVAILABLE`, so a click in the DSH panel can open nothing.
    MusePresentationChannelHost.instance.install();
  }
}

Future<void> _resolveCloudDeps(GetIt getIt) async {
  final env = await AppFlowyCloudSharedEnv.fromEnv();
  Log.info("cloud setting: $env");
  getIt.registerFactory<AppFlowyCloudSharedEnv>(() => env);
  getIt.registerFactory<AIRepository>(() => AppFlowyAIService());

  if (isAppFlowyCloudEnabled) {
    getIt.registerSingleton(
      AppFlowyCloudDeepLink(),
      dispose: (obj) async {
        await obj.dispose();
      },
    );
  }
}

void _resolveCommonService(
  GetIt getIt,
  IntegrationMode mode,
) async {
  getIt.registerFactory<FilePickerService>(() => FilePicker());

  getIt.registerFactory<ApplicationDataStorage>(
    () => mode.isTest ? MockApplicationDataStorage() : ApplicationDataStorage(),
  );

  getIt.registerFactory<ClipboardService>(
    () => ClipboardService(),
  );

  // theme
  getIt.registerFactory<BaseAppearance>(
    () => UniversalPlatform.isMobile ? MobileAppearance() : DesktopAppearance(),
  );

  getIt.registerFactory<FlowyCacheManager>(
    () => FlowyCacheManager()
      ..registerCache(TemporaryDirectoryCache())
      ..registerCache(CustomImageCacheManager())
      ..registerCache(FeatureFlagCache()),
  );

  getIt.registerSingleton<EasyLocalizationService>(EasyLocalizationService());
}

void _resolveUserDeps(GetIt getIt, IntegrationMode mode) {
  switch (currentCloudType()) {
    case AuthenticatorType.local:
      getIt.registerFactory<AuthService>(
        () => BackendAuthService(
          AuthTypePB.Local,
        ),
      );
      break;
    case AuthenticatorType.appflowyCloud:
    case AuthenticatorType.appflowyCloudSelfHost:
    case AuthenticatorType.appflowyCloudDevelop:
      getIt.registerFactory<AuthService>(() => AppFlowyCloudAuthService());
      break;
  }

  getIt.registerFactory<AuthRouter>(() => AuthRouter());

  getIt.registerFactory<SignInBloc>(
    () => SignInBloc(getIt<AuthService>()),
  );
  getIt.registerFactory<SignUpBloc>(
    () => SignUpBloc(getIt<AuthService>()),
  );

  getIt.registerFactory<SplashRouter>(() => SplashRouter());
  getIt.registerFactory<EditPanelBloc>(() => EditPanelBloc());
  getIt.registerFactory<SplashBloc>(() => SplashBloc());
  getIt.registerLazySingleton<NetworkListener>(() => NetworkListener());
  getIt.registerLazySingleton<CachedRecentService>(() => CachedRecentService());
  getIt.registerLazySingleton<ViewAncestorCache>(() => ViewAncestorCache());
  getIt.registerLazySingleton<SubscriptionSuccessListenable>(
    () => SubscriptionSuccessListenable(),
  );
}

void _resolveHomeDeps(GetIt getIt) {
  getIt.registerSingleton(FToast());

  getIt.registerSingleton(MenuSharedState());

  getIt.registerFactoryParam<UserListener, UserProfilePB, void>(
    (user, _) => UserListener(userProfile: user),
  );

  // share
  getIt.registerFactoryParam<ShareBloc, ViewPB, void>(
    (view, _) => ShareBloc(view: view),
  );

  getIt.registerSingleton<ActionNavigationBloc>(ActionNavigationBloc());

  getIt.registerLazySingleton<TabsBloc>(() => TabsBloc());

  getIt.registerSingleton<ReminderBloc>(ReminderBloc());

  getIt.registerSingleton<RenameViewBloc>(RenameViewBloc(PopoverController()));

  getIt.registerLazySingleton<DshAgentController>(() => DshAgentController());
  final workspaceProviders = MuseWorkspaceProviderRegistry()
    ..register(MuseLocalWorkspaceProvider());
  getIt.registerSingleton<MuseWorkspaceProviderRegistry>(workspaceProviders);
  getIt.registerLazySingleton<MuseWorkspacePersistence>(
    MuseWorkspacePersistence.new,
  );
  getIt.registerLazySingleton<MuseWorkspaceController>(
    () => MuseWorkspaceController(
      providers: getIt<MuseWorkspaceProviderRegistry>(),
      persistence: getIt<MuseWorkspacePersistence>(),
    ),
    dispose: (controller) => controller.dispose(),
  );
  getIt.registerLazySingleton<MuseTextVersionRepository>(
    MuseTextVersionRepository.new,
  );
  final textDiffProvider = MuseTextDiffProvider();
  final imageDiffProvider = MuseImageOverlayDiffProvider();
  final diffProviders = MuseDiffProviderRegistry()
    ..register(textDiffProvider)
    ..register(imageDiffProvider);
  final semanticDiffProviders = MuseSemanticDiffProviderRegistry()
    ..register(MuseBinaryMetadataDiffProvider())
    ..register(imageDiffProvider)
    ..register(textDiffProvider);
  final diffRenderers = MuseDiffRendererRegistry()
    ..register(
      MuseDiffRendererManifest(
        id: 'muse.diff-viewer.text.v2',
        supportedChangeSchemas: {'muse.diff.changeset.v1'},
        modes: {MuseDiffViewMode.sideBySide, MuseDiffViewMode.unified},
        capabilities: const MuseDiffRendererCapabilities(
          selection: true,
          copy: true,
          search: true,
          syncScroll: true,
          alignment: true,
          folding: true,
          overview: true,
        ),
      ),
    )
    ..register(
      MuseDiffRendererManifest(
        id: 'muse.diff-viewer.image-overlay.v1',
        supportedChangeSchemas: {'muse.diff.image-overlay.changeset.v1'},
        modes: {MuseDiffViewMode.overlay, MuseDiffViewMode.sideBySide},
        capabilities: const MuseDiffRendererCapabilities(
          selection: false,
          copy: false,
          search: false,
        ),
      ),
    );
  getIt.registerSingleton<MuseTextDiffProvider>(textDiffProvider);
  getIt.registerSingleton<MuseImageOverlayDiffProvider>(imageDiffProvider);
  getIt.registerSingleton<MuseDiffProviderRegistry>(diffProviders);
  getIt.registerSingleton<MuseSemanticDiffProviderRegistry>(
    semanticDiffProviders,
  );
  getIt.registerSingleton<MuseDiffRendererRegistry>(diffRenderers);
  getIt.registerLazySingleton<MuseResourceVersionPaneRegistry>(
    MuseResourceVersionPaneRegistry.new,
  );
  getIt.registerLazySingleton<MuseTextVersionDiffService>(
    () => MuseTextVersionDiffService(
      repository: getIt<MuseTextVersionRepository>(),
      provider: getIt<MuseTextDiffProvider>(),
    ),
  );
  getIt.registerLazySingleton<MuseImageOverlayDiffService>(
    () => MuseImageOverlayDiffService(
      repository: getIt<MuseTextVersionRepository>(),
      provider: getIt<MuseImageOverlayDiffProvider>(),
    ),
  );
  final resourceEngines = builtinResourceEngineRegistry();
  getIt.registerSingleton<MuseResourceEngineRegistry>(resourceEngines);
  getIt.registerLazySingleton<MuseResourceOpenDefaults>(
    MuseResourceOpenDefaults.new,
  );
  final resourceTabActions = MuseResourceTabActionRegistry();
  registerDefaultMuseResourceTabActions(resourceTabActions);
  getIt.registerSingleton<MuseResourceTabActionRegistry>(resourceTabActions);
  getIt.registerLazySingleton<HelixLanguageServerInstaller>(
    HelixLanguageServerInstaller.new,
  );
  getIt.registerLazySingleton<HelixSettingsController>(
    () => HelixSettingsController(getIt<HelixLanguageServerInstaller>()),
  );
  getIt.registerLazySingleton<DshSidecar>(
    () => DshSidecar(getIt<DshAgentController>()),
    dispose: (sidecar) async {
      await sidecar.stop();
    },
  );
  getIt.registerLazySingleton<DshDeviceTokenService>(
    DshDeviceTokenService.new,
  );
}

void _resolveFolderDeps(GetIt getIt) {
  // Workspace
  getIt.registerFactoryParam<WorkspaceListener, UserProfilePB, String>(
    (user, workspaceId) =>
        WorkspaceListener(user: user, workspaceId: workspaceId),
  );

  getIt.registerFactoryParam<ViewBloc, ViewPB, void>(
    (view, _) => ViewBloc(
      view: view,
    ),
  );

  // User
  getIt.registerFactoryParam<SettingsUserViewBloc, UserProfilePB, void>(
    (user, _) => SettingsUserViewBloc(user),
  );

  // Trash
  getIt.registerLazySingleton<TrashService>(() => TrashService());
  getIt.registerLazySingleton<TrashListener>(() => TrashListener());
  getIt.registerFactory<TrashBloc>(
    () => TrashBloc(),
  );

  // Favorite
  getIt.registerFactory<FavoriteBloc>(() => FavoriteBloc());
}
