// GENERATED from assets/catalog.json; edit the registry.
import 'catalog.dart';

final class OpenMuseStrings {
  OpenMuseStrings(this._catalog, this._locale);
  factory OpenMuseStrings.forLocale(String tag, {OpenMuseCatalog? catalog}) {
    final value = catalog ?? OpenMuseCatalog.builtIn;
    return OpenMuseStrings(() => value, () => value.resolve([tag]));
  }
  static final fallback = OpenMuseStrings.forLocale(
    OpenMuseCatalog.builtIn.fallbackLocale,
  );
  final OpenMuseCatalog Function() _catalog;
  final String Function() _locale;
  String get locale => _locale();
  Map<String, Object?> namespaceData(String namespace) =>
      _catalog().namespaceData(locale, namespace);
  String format(String id, [Map<String, Object?> arguments = const {}]) =>
      _catalog().format(locale, id, arguments);
  String adminAction() => format("admin.action");
  String adminActions() => format("admin.actions");
  String adminActiveSequence() => format("admin.activeSequence");
  String adminActor() => format("admin.actor");
  String adminApplyRollout() => format("admin.applyRollout");
  String adminApprove() => format("admin.approve");
  String adminApproveStep() => format("admin.approveStep");
  String adminApproved() => format("admin.approved");
  String adminAttachPack() => format("admin.attachPack");
  String adminAttached() => format("admin.attached");
  String adminAudit() => format("admin.audit");
  String adminAuditDescription() => format("admin.auditDescription");
  String adminAuditTitle() => format("admin.auditTitle");
  String adminChangeNote() => format("admin.changeNote");
  String adminConfiguration() => format("admin.configuration");
  String adminConnect() => format("admin.connect");
  String adminDefaultLocale() => format("admin.defaultLocale");
  String adminDemoNotice() => format("admin.demoNotice");
  String adminDetail() => format("admin.detail");
  String adminDraft() => format("admin.draft");
  String adminEditStep() => format("admin.editStep");
  String adminEditorDescription() => format("admin.editorDescription");
  String adminEditorTitle() => format("admin.editorTitle");
  String adminEmpty() => format("admin.empty");
  String adminFeatureFlags() => format("admin.featureFlags");
  String adminHashBound() => format("admin.hashBound");
  String adminId() => format("admin.id");
  String adminImmutableRelease() => format("admin.immutableRelease");
  String adminIndependentReview() => format("admin.independentReview");
  String adminLanguage() => format("admin.language");
  String adminLanguages() => format("admin.languages");
  String adminLifetime() => format("admin.lifetime");
  String adminLoadTemplate() => format("admin.loadTemplate");
  String adminLocalService() => format("admin.localService");
  String adminMessageData() => format("admin.messageData");
  String adminMessages() => format("admin.messages");
  String adminMessagesDescription() => format("admin.messagesDescription");
  String adminMessagesTitle() => format("admin.messagesTitle");
  String adminNamespace() => format("admin.namespace");
  String adminNeedDraft() => format("admin.needDraft");
  String adminNewDraft() => format("admin.newDraft");
  String adminNoDiff() => format("admin.noDiff");
  String adminOpen() => format("admin.open");
  String adminOverview() => format("admin.overview");
  String adminOverviewDescription() => format("admin.overviewDescription");
  String adminOverviewTitle() => format("admin.overviewTitle");
  String adminPendingChanges() => format("admin.pendingChanges");
  String adminPlatform() => format("admin.platform");
  String adminPluginMarketEnabled() => format("admin.pluginMarketEnabled");
  String adminPortalApplied() => format("admin.portalApplied");
  String adminPortalApply() => format("admin.portalApply");
  String adminPortalDescription() => format("admin.portalDescription");
  String adminPortalDownloads() => format("admin.portalDownloads");
  String adminPortalDownloadsHelp() => format("admin.portalDownloadsHelp");
  String adminPortalEntrypoints() => format("admin.portalEntrypoints");
  String adminPortalInvalidUrl() => format("admin.portalInvalidUrl");
  String adminPortalKind() => format("admin.portalKind");
  String adminPortalMediaUrl() => format("admin.portalMediaUrl");
  String adminPortalPosterUrl() => format("admin.portalPosterUrl");
  String adminPortalPreview() => format("admin.portalPreview");
  String adminPortalRegion() => format("admin.portalRegion");
  String adminPortalSlot() => format("admin.portalSlot");
  String adminPortalTitle() => format("admin.portalTitle");
  String adminPreviewOnly() => format("admin.previewOnly");
  String adminPublicCohorts() => format("admin.publicCohorts");
  String adminPublish() => format("admin.publish");
  String adminPublishStep() => format("admin.publishStep");
  String adminPublished() => format("admin.published");
  String adminPublishedAt() => format("admin.publishedAt");
  String adminPublishedState() => format("admin.publishedState");
  String adminRecentDrafts() => format("admin.recentDrafts");
  String adminRefresh() => format("admin.refresh");
  String adminRefreshIntervalSeconds() =>
      format("admin.refreshIntervalSeconds");
  String adminReleases() => format("admin.releases");
  String adminReleasesDescription() => format("admin.releasesDescription");
  String adminReleasesTitle() => format("admin.releasesTitle");
  String adminReview() => format("admin.review");
  String adminReviewDescription() => format("admin.reviewDescription");
  String adminRollback() => format("admin.rollback");
  String adminRollout() => format("admin.rollout");
  String adminRolloutApplied() => format("admin.rolloutApplied");
  String adminRolloutStep() => format("admin.rolloutStep");
  String adminSave() => format("admin.save");
  String adminSaved() => format("admin.saved");
  String adminScope() => format("admin.scope");
  String adminSequence() => format("admin.sequence");
  String adminSimulate() => format("admin.simulate");
  String adminSimulateTitle() => format("admin.simulateTitle");
  String adminSnapshotData() => format("admin.snapshotData");
  String adminState() => format("admin.state");
  String adminSubmit() => format("admin.submit");
  String adminSubmitted() => format("admin.submitted");
  String adminSubmittedState() => format("admin.submittedState");
  String adminToken() => format("admin.token");
  String adminUnsaved() => format("admin.unsaved");
  String adminUpdateChannel() => format("admin.updateChannel");
  String adminUpdatePromptsEnabled() => format("admin.updatePromptsEnabled");
  String adminUpdated() => format("admin.updated");
  String adminValid() => format("admin.valid");
  String adminValidate() => format("admin.validate");
  String adminValidateStep() => format("admin.validateStep");
  String adminWorkspace() => format("admin.workspace");
  String authAccountSectionTitle() => format("auth.accountSectionTitle");
  String authAgreementConjunction() => format("auth.agreementConjunction");
  String authAgreementPrefix() => format("auth.agreementPrefix");
  String authAgreementSuffix() => format("auth.agreementSuffix");
  String authAnonymousMode() => format("auth.anonymousMode");
  String authBackToLogin() => format("auth.backToLogin");
  String authCheckYourEmailTitle() => format("auth.checkYourEmailTitle");
  String authCloudLabel({required String label}) =>
      format("auth.cloudLabel", {"label": label});
  String authConfirmPassword() => format("auth.confirmPassword");
  String authContinueAction() => format("auth.continueAction");
  String authContinueWithEmail() => format("auth.continueWithEmail");
  String authContinueWithPassword() => format("auth.continueWithPassword");
  String authCreateAccount() => format("auth.createAccount");
  String authCreateAccountTitle() => format("auth.createAccountTitle");
  String authEmailHint() => format("auth.emailHint");
  String authEnterPasswordTitle() => format("auth.enterPasswordTitle");
  String authErrorAccountExists() => format("auth.errorAccountExists");
  String authErrorBootstrap() => format("auth.errorBootstrap");
  String authErrorInvalidCode() => format("auth.errorInvalidCode");
  String authErrorInvalidConfiguration() =>
      format("auth.errorInvalidConfiguration");
  String authErrorInvalidCredentials() =>
      format("auth.errorInvalidCredentials");
  String authErrorInvalidResponse() => format("auth.errorInvalidResponse");
  String authErrorNetwork() => format("auth.errorNetwork");
  String authErrorRateLimited() => format("auth.errorRateLimited");
  String authErrorServer() => format("auth.errorServer");
  String authErrorSessionExpired() => format("auth.errorSessionExpired");
  String authErrorWeakPassword() => format("auth.errorWeakPassword");
  String authForgotPassword() => format("auth.forgotPassword");
  String authInvalidEmail() => format("auth.invalidEmail");
  String authLoginAsLabel() => format("auth.loginAsLabel");
  String authNewPassword() => format("auth.newPassword");
  String authNotSignedIn() => format("auth.notSignedIn");
  String authPasscodeHint() => format("auth.passcodeHint");
  String authPasswordHint() => format("auth.passwordHint");
  String authPasswordMismatch() => format("auth.passwordMismatch");
  String authPasswordTooShort() => format("auth.passwordTooShort");
  String authPrivacyPolicy() => format("auth.privacyPolicy");
  String authRecoveryCodeSent({required String email}) =>
      format("auth.recoveryCodeSent", {"email": email});
  String authResendCode() => format("auth.resendCode");
  String authResendCodeCountdown({required int seconds}) =>
      format("auth.resendCodeCountdown", {"seconds": seconds});
  String authResetPasswordTitle() => format("auth.resetPasswordTitle");
  String authSetNewPassword() => format("auth.setNewPassword");
  String authSettings() => format("auth.settings");
  String authSignInCodeSent({required String email}) =>
      format("auth.signInCodeSent", {"email": email});
  String authSignOut() => format("auth.signOut");
  String authSignUpCodeSent({required String email}) =>
      format("auth.signUpCodeSent", {"email": email});
  String authTermsOfService() => format("auth.termsOfService");
  String authVerifying() => format("auth.verifying");
  String authWelcomeTitle() => format("auth.welcomeTitle");
  String builtinAgent() => format("builtin.agent");
  String builtinAuth() => format("builtin.auth");
  String builtinCloud() => format("builtin.cloud");
  String builtinFiles() => format("builtin.files");
  String builtinMarket() => format("builtin.market");
  String builtinPaired() => format("builtin.paired");
  String builtinRemote() => format("builtin.remote");
  String builtinScheduled() => format("builtin.scheduled");
  String builtinSpeech() => format("builtin.speech");
  String cliCloseTerminal() => format("cli.closeTerminal");
  String cliMinimizeTerminal() => format("cli.minimizeTerminal");
  String cliNewTerminal() => format("cli.newTerminal");
  String cliTerminal() => format("cli.terminal");
  String cloudConnect() => format("cloud.connect");
  String cloudDshUnavailable() => format("cloud.dshUnavailable");
  String cloudEmpty() => format("cloud.empty");
  String cloudRefresh() => format("cloud.refresh");
  String cloudReuse() => format("cloud.reuse");
  String cloudRunning({required Object? devices}) =>
      format("cloud.running", {"devices": devices});
  String cloudTitle() => format("cloud.title");
  String cloudUnavailable() => format("cloud.unavailable");
  String cloudWorkspace() => format("cloud.workspace");
  String commonAiReportAttachments() => format("common.aiReportAttachments");
  String commonAiReportComment() => format("common.aiReportComment");
  String commonAiReportDeleteFailed() => format("common.aiReportDeleteFailed");
  String commonAiReportDisclosure() => format("common.aiReportDisclosure");
  String commonAiReportFailed() => format("common.aiReportFailed");
  String commonAiReportHarmful() => format("common.aiReportHarmful");
  String commonAiReportLimit() => format("common.aiReportLimit");
  String commonAiReportMisleading() => format("common.aiReportMisleading");
  String commonAiReportOther() => format("common.aiReportOther");
  String commonAiReportPrivacy() => format("common.aiReportPrivacy");
  String commonAiReportReason() => format("common.aiReportReason");
  String commonAiReportReceived() => format("common.aiReportReceived");
  String commonAiReportSending() => format("common.aiReportSending");
  String commonAiReportSignIn() => format("common.aiReportSignIn");
  String commonAiReportSubmit() => format("common.aiReportSubmit");
  String commonAiReportTitle() => format("common.aiReportTitle");
  String commonAiSharingAllow() => format("common.aiSharingAllow");
  String commonAiSharingBody({
    required Object? provider,
    required Object? model,
  }) => format("common.aiSharingBody", {"provider": provider, "model": model});
  String commonAiSharingCancel() => format("common.aiSharingCancel");
  String commonAiSharingTitle() => format("common.aiSharingTitle");
  String commonAiSharingUnknown() => format("common.aiSharingUnknown");
  String commonCancel() => format("common.cancel");
  String commonConfigExpired() => format("common.configExpired");
  String commonConfigPending() => format("common.configPending");
  String commonConfigUnavailable() => format("common.configUnavailable");
  String commonDesktopSetupGuide() => format("common.desktopSetupGuide");
  String commonDesktopUpdateNeeded() => format("common.desktopUpdateNeeded");
  String commonFeatureUnavailable() => format("common.featureUnavailable");
  String commonFollowSystem() => format("common.followSystem");
  String commonLanguage() => format("common.language");
  String commonLanguageDownloadFailed() =>
      format("common.languageDownloadFailed");
  String commonLanguageDownloading() => format("common.languageDownloading");
  String commonLegalAccept() => format("common.legalAccept");
  String commonLegalAcknowledge() => format("common.legalAcknowledge");
  String commonLegalChangeRegion() => format("common.legalChangeRegion");
  String commonLegalContinuing() => format("common.legalContinuing");
  String commonLegalDecision() => format("common.legalDecision");
  String commonLegalDecline() => format("common.legalDecline");
  String commonLegalDescription() => format("common.legalDescription");
  String commonLegalDevices() => format("common.legalDevices");
  String commonLegalFailed() => format("common.legalFailed");
  String commonLegalIntro() => format("common.legalIntro");
  String commonLegalLocal() => format("common.legalLocal");
  String commonLegalPermissions() => format("common.legalPermissions");
  String commonLegalPrivacy() => format("common.legalPrivacy");
  String commonLegalPrivacyDetail() => format("common.legalPrivacyDetail");
  String commonLegalRegion() => format("common.legalRegion");
  String commonLegalTerms() => format("common.legalTerms");
  String commonLegalTermsDetail() => format("common.legalTermsDetail");
  String commonLegalTitle() => format("common.legalTitle");
  String commonLegalWorkspace() => format("common.legalWorkspace");
  String commonOffline() => format("common.offline");
  String commonRegionContinue() => format("common.regionContinue");
  String commonRegionDescription() => format("common.regionDescription");
  String commonRegionFailed() => format("common.regionFailed");
  String commonRegionLoading() => format("common.regionLoading");
  String commonRegionTitle() => format("common.regionTitle");
  String commonRegionUnavailable() => format("common.regionUnavailable");
  String commonRetry() => format("common.retry");
  String commonRouteUnavailable() => format("common.routeUnavailable");
  String commonSave() => format("common.save");
  String commonServiceRegion() => format("common.serviceRegion");
  String dshActiveCount({required Object? count}) =>
      format("dsh.activeCount", {"count": count});
  String dshAddContent() => format("dsh.addContent");
  String dshAllFiles({required Object? count}) =>
      format("dsh.allFiles", {"count": count});
  String dshAnswerRequired() => format("dsh.answerRequired");
  String dshAnswerResult() => format("dsh.answerResult");
  String dshAnswerSent() => format("dsh.answerSent");
  String dshAnswered() => format("dsh.answered");
  String dshArtifacts() => format("dsh.artifacts");
  String dshBinary() => format("dsh.binary");
  String dshBinaryUnavailable() => format("dsh.binaryUnavailable");
  String dshChangedFile() => format("dsh.changedFile");
  String dshChanges() => format("dsh.changes");
  String dshChangesUnavailable() => format("dsh.changesUnavailable");
  String dshChat() => format("dsh.chat");
  String dshCollapse() => format("dsh.collapse");
  String dshCommands() => format("dsh.commands");
  String dshCompatibility() => format("dsh.compatibility");
  String dshCompletedCount({required Object? count}) =>
      format("dsh.completedCount", {"count": count});
  String dshComposer() => format("dsh.composer");
  String dshConnecting() => format("dsh.connecting");
  String dshContinue() => format("dsh.continue");
  String dshConversation() => format("dsh.conversation");
  String dshDefaultOpen() => format("dsh.defaultOpen");
  String dshDeletedFile() => format("dsh.deletedFile");
  String dshEditFailed() => format("dsh.editFailed");
  String dshEditFiles() => format("dsh.editFiles");
  String dshEdited() => format("dsh.edited");
  String dshEditing() => format("dsh.editing");
  String dshExplore() => format("dsh.explore");
  String dshFailed() => format("dsh.failed");
  String dshFetchWeb() => format("dsh.fetchWeb");
  String dshFile() => format("dsh.file");
  String dshFullFrontend() => format("dsh.fullFrontend");
  String dshGoal() => format("dsh.goal");
  String dshInput() => format("dsh.input");
  String dshMoreFileActions({required Object? name}) =>
      format("dsh.moreFileActions", {"name": name});
  String dshNewFile() => format("dsh.newFile");
  String dshNextFile() => format("dsh.nextFile");
  String dshOpenDesktop() => format("dsh.openDesktop");
  String dshOpenError() => format("dsh.openError");
  String dshOpenRetry() => format("dsh.openRetry");
  String dshOpenedDefault() => format("dsh.openedDefault");
  String dshOpenedDesktop() => format("dsh.openedDesktop");
  String dshOpening() => format("dsh.opening");
  String dshOtherAnswer() => format("dsh.otherAnswer");
  String dshOutput() => format("dsh.output");
  String dshPendingCount({required Object? count}) =>
      format("dsh.pendingCount", {"count": count});
  String dshPresentFiles() => format("dsh.presentFiles");
  String dshPreview() => format("dsh.preview");
  String dshPreviewTitle() => format("dsh.previewTitle");
  String dshPreviousFile() => format("dsh.previousFile");
  String dshRan() => format("dsh.ran");
  String dshRead() => format("dsh.read");
  String dshReadFailed() => format("dsh.readFailed");
  String dshReadFiles() => format("dsh.readFiles");
  String dshReading() => format("dsh.reading");
  String dshReasoning() => format("dsh.reasoning");
  String dshReconnecting() => format("dsh.reconnecting");
  String dshRetry() => format("dsh.retry");
  String dshReveal() => format("dsh.reveal");
  String dshRevealFailed() => format("dsh.revealFailed");
  String dshRevealed() => format("dsh.revealed");
  String dshRevealing() => format("dsh.revealing");
  String dshRunFailed() => format("dsh.runFailed");
  String dshRunning() => format("dsh.running");
  String dshSearchFiles() => format("dsh.searchFiles");
  String dshSearchWeb() => format("dsh.searchWeb");
  String dshSelect() => format("dsh.select");
  String dshSelectModel() => format("dsh.selectModel");
  String dshSend() => format("dsh.send");
  String dshSendFailed({required Object? detail}) =>
      format("dsh.sendFailed", {"detail": detail});
  String dshSending() => format("dsh.sending");
  String dshSharedSession() => format("dsh.sharedSession");
  String dshSizeUnavailable() => format("dsh.sizeUnavailable");
  String dshSkip() => format("dsh.skip");
  String dshSkipped() => format("dsh.skipped");
  String dshSplitDiff() => format("dsh.splitDiff");
  String dshStandardMode() => format("dsh.standardMode");
  String dshStop() => format("dsh.stop");
  String dshSubmitted() => format("dsh.submitted");
  String dshSummaryUnavailable() => format("dsh.summaryUnavailable");
  String dshTaskProgress() => format("dsh.taskProgress");
  String dshTasks() => format("dsh.tasks");
  String dshThinking() => format("dsh.thinking");
  String dshTooLarge() => format("dsh.tooLarge");
  String dshToolCall() => format("dsh.toolCall");
  String dshToolsSummary({required Object? tools}) =>
      format("dsh.toolsSummary", {"tools": tools});
  String dshTrail() => format("dsh.trail");
  String dshTurnSteps({required Object? turns, required Object? steps}) =>
      format("dsh.turnSteps", {"turns": turns, "steps": steps});
  String dshUnifiedDiff() => format("dsh.unifiedDiff");
  String dshUnsupported() => format("dsh.unsupported");
  String dshUnsupportedNames({required Object? names}) =>
      format("dsh.unsupportedNames", {"names": names});
  String dshUpdatedFiles({required Object? count}) =>
      format("dsh.updatedFiles", {"count": count});
  String dshWaitConnection() => format("dsh.waitConnection");
  String dshWaiting() => format("dsh.waiting");
  String dshWebFallback() => format("dsh.webFallback");
  String dshWorkspaceEdits() => format("dsh.workspaceEdits");
  String dshWriteFailed() => format("dsh.writeFailed");
  String dshWriteFiles() => format("dsh.writeFiles");
  String dshWriting() => format("dsh.writing");
  String dshWritten() => format("dsh.written");
  String filesCount({required Object? count}) =>
      format("files.count", {"count": count});
  String htmlBack() => format("html.back");
  String htmlBrowseAuthorizedDesktopWorkspaces() =>
      format("html.browse_authorized_desktop_workspaces");
  String htmlCancel() => format("html.cancel");
  String htmlChooseAFolder() => format("html.choose_a_folder");
  String htmlConnectAndSelectADesktopWorkspaceFirst() =>
      format("html.connect_and_select_a_desktop_workspace_first");
  String htmlCreating() => format("html.creating");
  String htmlElapsed({required Object? seconds}) =>
      format("html.elapsed", {"seconds": seconds});
  String htmlEmptyFolder() => format("html.empty_folder");
  String htmlEnterAMessage() => format("html.enter_a_message");
  String htmlFile() => format("html.file");
  String htmlFileCount({required Object? count}) =>
      format("html.fileCount", {"count": count});
  String htmlFolder() => format("html.folder");
  String htmlFolders() => format("html.folders");
  String htmlItems() => format("html.items");
  String htmlLoading() => format("html.loading");
  String htmlLoadingNamed({required Object? name}) =>
      format("html.loadingNamed", {"name": name});
  String htmlLoadingFolders() => format("html.loading_folders");
  String htmlLoadingScheduledTasks() => format("html.loading_scheduled_tasks");
  String htmlLoadingWorkspaceFolders() =>
      format("html.loading_workspace_folders");
  String htmlMessage() => format("html.message");
  String htmlMoreResultsAvailable() => format("html.more_results_available");
  String htmlNoFoldersAreAvailableOnDesktop() =>
      format("html.no_folders_are_available_on_desktop");
  String htmlNoScheduledTasks() => format("html.no_scheduled_tasks");
  String htmlPreparingToOpenTheFile() =>
      format("html.preparing_to_open_the_file");
  String htmlReopen() => format("html.reopen");
  String htmlResultCount({required Object? count}) =>
      format("html.resultCount", {"count": count});
  String htmlResults() => format("html.results");
  String htmlS() => format("html.s");
  String htmlScheduleTask() => format("html.schedule_task");
  String htmlScheduledTasks() => format("html.scheduled_tasks");
  String htmlScheduledTime() => format("html.scheduled_time");
  String htmlSearch() => format("html.search");
  String htmlSearchResultCount({
    required Object? count,
    required Object? truncated,
  }) => format("html.searchResultCount", {
    "count": count,
    "truncated": truncated,
  });
  String htmlSearchFilenames() => format("html.search_filenames");
  String htmlSearching() => format("html.searching");
  String htmlSendAMessageToAWorkspaceAtAScheduledTime() =>
      format("html.send_a_message_to_a_workspace_at_a_scheduled_time");
  String htmlTaskCount({required Object? count}) =>
      format("html.taskCount", {"count": count});
  String htmlTaskState({required Object? state}) =>
      format("html.taskState", {"state": state});
  String htmlTasks() => format("html.tasks");
  String htmlTheEditorHasClosedSelectReopenToOpenItAgain() =>
      format("html.the_editor_has_closed_select_reopen_to_open_it_again");
  String htmlTotal() => format("html.total");
  String htmlWorkspace() => format("html.workspace");
  String htmlWorkspaceFiles() => format("html.workspace_files");
  String marketA2uiPage() => format("market.a2uiPage");
  String marketA2uiState() => format("market.a2uiState");
  String marketBuiltInDisabled() => format("market.builtInDisabled");
  String marketBuiltInReady() => format("market.builtInReady");
  String marketBuiltInTools() => format("market.builtInTools");
  String marketCancel() => format("market.cancel");
  String marketCategoryA11y({required Object? category}) =>
      format("market.categoryA11y", {"category": category});
  String marketCategoryAll() => format("market.categoryAll");
  String marketCategoryCoding() => format("market.categoryCoding");
  String marketCategoryDiscover() => format("market.categoryDiscover");
  String marketCategoryEmpty() => format("market.categoryEmpty");
  String marketCategoryEmptyDetail() => format("market.categoryEmptyDetail");
  String marketCategoryEnabled() => format("market.categoryEnabled");
  String marketCategoryFeatured() => format("market.categoryFeatured");
  String marketCategoryInactive() => format("market.categoryInactive");
  String marketCategoryLabelPlugins() => format("market.categoryLabelPlugins");
  String marketCategoryLife() => format("market.categoryLife");
  String marketCategoryMine() => format("market.categoryMine");
  String marketCategoryOffice() => format("market.categoryOffice");
  String marketCategoryPlugins({required Object? category}) =>
      format("market.categoryPlugins", {"category": category});
  String marketCategoryUi() => format("market.categoryUi");
  String marketCategoryUpdates() => format("market.categoryUpdates");
  String marketConnectDesktop() => format("market.connectDesktop");
  String marketCrossDevice() => format("market.crossDevice");
  String marketDelistedInstalled() => format("market.delistedInstalled");
  String marketDesktopConnect() => format("market.desktopConnect");
  String marketDesktopHero() => format("market.desktopHero");
  String marketDesktopPlugins() => format("market.desktopPlugins");
  String marketDesktopUsage() => format("market.desktopUsage");
  String marketDeviceUnsupported() => format("market.deviceUnsupported");
  String marketDisable() => format("market.disable");
  String marketDisabled() => format("market.disabled");
  String marketDiscoverPlugins() => format("market.discoverPlugins");
  String marketEnable() => format("market.enable");
  String marketEnabled() => format("market.enabled");
  String marketExtensions() => format("market.extensions");
  String marketExtensionsReady() => format("market.extensionsReady");
  String marketFeaturedCategory({required Object? category}) =>
      format("market.featuredCategory", {"category": category});
  String marketFullPage() => format("market.fullPage");
  String marketGif() => format("market.gif");
  String marketHero() => format("market.hero");
  String marketHighlights() => format("market.highlights");
  String marketImageUnavailable() => format("market.imageUnavailable");
  String marketInstall() => format("market.install");
  String marketInstallDesktop() => format("market.installDesktop");
  String marketLocalFiles() => format("market.localFiles");
  String marketManageCards() => format("market.manageCards");
  String marketManifestPermissions() => format("market.manifestPermissions");
  String marketMarketNoEditor() => format("market.marketNoEditor");
  String marketMediaUnavailable() => format("market.mediaUnavailable");
  String marketMyPlugins() => format("market.myPlugins");
  String marketNoEditor() => format("market.noEditor");
  String marketNoPermissions() => format("market.noPermissions");
  String marketNotInstalled() => format("market.notInstalled");
  String marketOfflineUnsupported() => format("market.offlineUnsupported");
  String marketOpen() => format("market.open");
  String marketOpenFailed({required Object? title}) =>
      format("market.openFailed", {"title": title});
  String marketPauseVideo() => format("market.pauseVideo");
  String marketPermissions() => format("market.permissions");
  String marketPermissionsCompatibility() =>
      format("market.permissionsCompatibility");
  String marketPlatforms() => format("market.platforms");
  String marketPlayVideo() => format("market.playVideo");
  String marketPluginCount({required Object? count}) =>
      format("market.pluginCount", {"count": count});
  String marketPluginService() => format("market.pluginService");
  String marketPreview() => format("market.preview");
  String marketPreviewTitle({required Object? title}) =>
      format("market.previewTitle", {"title": title});
  String marketProcessing() => format("market.processing");
  String marketProvider() => format("market.provider");
  String marketProviders() => format("market.providers");
  String marketRecommended() => format("market.recommended");
  String marketReference({required Object? source}) =>
      format("market.reference", {"source": source});
  String marketRefresh() => format("market.refresh");
  String marketRegistryAddress() => format("market.registryAddress");
  String marketRemove() => format("market.remove");
  String marketRemoveDescription({required Object? name}) =>
      format("market.removeDescription", {"name": name});
  String marketRemoveTitle() => format("market.removeTitle");
  String marketRestartDesktop() => format("market.restartDesktop");
  String marketSearchInstalled() => format("market.searchInstalled");
  String marketSearchPlugins() => format("market.searchPlugins");
  String marketSelectProvider() => format("market.selectProvider");
  String marketSend() => format("market.send");
  String marketSendFileDescription({
    required Object? name,
    required Object? service,
  }) =>
      format("market.sendFileDescription", {"name": name, "service": service});
  String marketSendFileTitle() => format("market.sendFileTitle");
  String marketTitle() => format("market.title");
  String marketUnavailable() => format("market.unavailable");
  String marketUnsupported() => format("market.unsupported");
  String marketUpdate() => format("market.update");
  String marketUseAddress() => format("market.useAddress");
  String marketVersionAuthor({
    required Object? version,
    required Object? author,
  }) => format("market.versionAuthor", {"version": version, "author": author});
  String marketVideoUnavailable() => format("market.videoUnavailable");
  String marketViewPlugin({required Object? name}) =>
      format("market.viewPlugin", {"name": name});
  String marketWebviewPage() => format("market.webviewPage");
  String marketWorkbenchDisplay() => format("market.workbenchDisplay");
  String marketWorkbenchPreview() => format("market.workbenchPreview");
  String mobileAFileIsAlreadyOpeningPleaseWait2906e99f() =>
      format("mobile.a_file_is_already_opening_please_wait.2906e99f");
  String mobileAboutOpenmuse86b0c36e() =>
      format("mobile.about_openmuse.86b0c36e");
  String mobileAccountA10e8da3() => format("mobile.account.a10e8da3");
  String mobileAccountSettings() => format("mobile.accountSettings");
  String mobileAccountDeletionDidNotComplete2bb8b0dd({
    required Object? value1,
  }) => format("mobile.account_deletion_did_not_complete.2bb8b0dd", {
    "value1": value1,
  });
  String mobileAccountDeletionInformation85b039b1() =>
      format("mobile.account_deletion_information.85b039b1");
  String mobileAccountDevicesB456d767() =>
      format("mobile.account_devices.b456d767");
  String mobileAddAFileFromDesktop90bc2c94() =>
      format("mobile.add_a_file_from_desktop.90bc2c94");
  String mobileAddAFileOrImageBbfddc0f() =>
      format("mobile.add_a_file_or_image.bbfddc0f");
  String mobileAddContent2b54df48() => format("mobile.add_content.2b54df48");
  String mobileAiGeneratedContentB3eeebbf() =>
      format("mobile.ai_generated_content.b3eeebbf");
  String mobileAllow295ba601() => format("mobile.allow.295ba601");
  String mobileAnUnexpectedServerResponsePreventedCo46d0ec61() =>
      format("mobile.an_unexpected_server_response_prevented_co.46d0ec61");
  String mobileAppearance76acb029() => format("mobile.appearance.76acb029");
  String mobileApps() => format("mobile.apps");
  String mobileAppsServices() => format("mobile.apps_services");
  String mobileAttachmentsAreComingInAFutureUpdate7e6cb9d3() =>
      format("mobile.attachments_are_coming_in_a_future_update.7e6cb9d3");
  String mobileAutomaticApproval02a34125() =>
      format("mobile.automatic_approval.02a34125");
  String mobileBrowseDesktopFoldersAndFiles74092ef2() =>
      format("mobile.browse_desktop_folders_and_files.74092ef2");
  String mobileBrowseFoldersAndSearchFiles478df008() =>
      format("mobile.browse_folders_and_search_files.478df008");
  String mobileCancelCda6010b() => format("mobile.cancel.cda6010b");
  String mobileCancelDownload() => format("mobile.cancelDownload");
  String mobileCancelEditing3a1cf24b() =>
      format("mobile.cancel_editing.3a1cf24b");
  String mobileChooseAComputer7f4e6ff3() =>
      format("mobile.choose_a_computer.7f4e6ff3");
  String mobileChooseADesktopWorkspaceFirst11a8134f() =>
      format("mobile.choose_a_desktop_workspace_first.11a8134f");
  String mobileChooseADevice6d041181() =>
      format("mobile.choose_a_device.6d041181");
  String mobileChooseADocxDocument623b72e7() =>
      format("mobile.choose_a_docx_document.623b72e7");
  String mobileChooseAMcpbPackageUseOpenDocumentFo889c6ea4() =>
      format("mobile.choose_a_mcpb_package_use_open_document_fo.889c6ea4");
  String mobileChooseAModelD992696e() =>
      format("mobile.choose_a_model.d992696e");
  String mobileChooseAWorkspace017a91b6() =>
      format("mobile.choose_a_workspace.017a91b6");
  String mobileChooseAWorkspace49d3d398() =>
      format("mobile.choose_a_workspace.49d3d398");
  String mobileChooseAWorkspaceTimeAndMessageF0776a30() =>
      format("mobile.choose_a_workspace_time_and_message.f0776a30");
  String mobileChooseAnOnlineComputer6c104504() =>
      format("mobile.choose_an_online_computer.6c104504");
  String mobileChooseAnOnlineComputerSignedInToThe0a48865d() =>
      format("mobile.choose_an_online_computer_signed_in_to_the.0a48865d");
  String mobileChooseDesktop5600a658() =>
      format("mobile.choose_desktop.5600a658");
  String mobileChooseVisibleApps() => format("mobile.choose_visible_apps");
  String mobileClose94058aa6() => format("mobile.close.94058aa6");
  String mobileCloseForNowFbbecc2d() => format("mobile.close_for_now.fbbecc2d");
  String mobileCloud118c7f15() => format("mobile.cloud.118c7f15");
  String mobileCloudConversationsAreComingSoonEcd03ff2() =>
      format("mobile.cloud_conversations_are_coming_soon.ecd03ff2");
  String mobileCloudWorkspaceIsNotConnected9de62d6c() =>
      format("mobile.cloud_workspace_is_not_connected.9de62d6c");
  String mobileComingSoonDdd8280c() => format("mobile.coming_soon.ddd8280c");
  String mobileCompactHeight557e1f94() =>
      format("mobile.compact_height.557e1f94");
  String mobileCompleted267fbd00() => format("mobile.completed.267fbd00");
  String mobileConfirm7f18937e() => format("mobile.confirm.7f18937e");
  String mobileConnect99f44b5c() => format("mobile.connect.99f44b5c");
  String mobileConnectDesktopA9885c8c() =>
      format("mobile.connect_desktop.a9885c8c");
  String mobileConnectDesktopToBrowseFolders5932deae() =>
      format("mobile.connect_desktop_to_browse_folders.5932deae");
  String mobileConnectDesktopToChooseAWorkspaceA66b600f() =>
      format("mobile.connect_desktop_to_choose_a_workspace.a66b600f");
  String mobileConnectDesktopToScheduleATask45c76ebb() =>
      format("mobile.connect_desktop_to_schedule_a_task.45c76ebb");
  String mobileConnectToAnOnlineDesktopFromTheDeviB83a25f3() =>
      format("mobile.connect_to_an_online_desktop_from_the_devi.b83a25f3");
  String mobileConnectToAnOnlineDesktopOfflineDevic84faed1b() =>
      format("mobile.connect_to_an_online_desktop_offline_devic.84faed1b");
  String mobileConnectToDesktop60b27881() =>
      format("mobile.connect_to_desktop.60b27881");
  String mobileConnectToDesktopFirst31c48b63() =>
      format("mobile.connect_to_desktop_first.31c48b63");
  String mobileConnectToDesktopFirstF10d2933() =>
      format("mobile.connect_to_desktop_first.f10d2933");
  String mobileConnectToLoadWorkspaces732b4807() =>
      format("mobile.connect_to_load_workspaces.732b4807");
  String mobileConnectedAfe95341() => format("mobile.connected.afe95341");
  String mobileConnectedC2fd31af({required Object? value1}) =>
      format("mobile.connected.c2fd31af", {"value1": value1});
  String mobileConnecting3ac1dd38() => format("mobile.connecting.3ac1dd38");
  String mobileConnectingToDesktop19a52f17() =>
      format("mobile.connecting_to_desktop.19a52f17");
  String mobileConnectingToDesktop46ee792f() =>
      format("mobile.connecting_to_desktop.46ee792f");
  String mobileConnectingToDesktop6fa59c15() =>
      format("mobile.connecting_to_desktop.6fa59c15");
  String mobileConnectingToDesktopYourMessageHasBee706b5e0c() =>
      format("mobile.connecting_to_desktop_your_message_has_bee.706b5e0c");
  String mobileConnectingToTheDesktopSessionWaitBef8b792e99() =>
      format("mobile.connecting_to_the_desktop_session_wait_bef.8b792e99");
  String mobileContactSupportE159c0ac() =>
      format("mobile.contact_support.e159c0ac");
  String mobileContinueDcd5e1f3() => format("mobile.continue.dcd5e1f3");
  String mobileConversations03595e53({required Object? value1}) =>
      format("mobile.conversations.03595e53", {"value1": value1});
  String mobileCreateWithOpenmuseC794b360() =>
      format("mobile.create_with_openmuse.c794b360");
  String mobileDark30112ded() => format("mobile.dark.30112ded");
  String mobileDeleteAccount13c11365() =>
      format("mobile.delete_account.13c11365");
  String mobileDeletePermanently0cc7ee72() =>
      format("mobile.delete_permanently.0cc7ee72");
  String mobileDesktopAuthorizationHasExpiredCff089e8() =>
      format("mobile.desktop_authorization_has_expired.cff089e8");
  String mobileDesktopAuthorizationHasExpiredReconnec2671ccbf() =>
      format("mobile.desktop_authorization_has_expired_reconnec.2671ccbf");
  String mobileDesktopAuthorizationHasExpiredReconnecA267882b() =>
      format("mobile.desktop_authorization_has_expired_reconnec.a267882b");
  String mobileDesktopConnectedLiveF96eebdd() =>
      format("mobile.desktop_connected_live.f96eebdd");
  String mobileDesktopDidNotReturnAnAvailableModel5472c170() =>
      format("mobile.desktop_did_not_return_an_available_model.5472c170");
  String mobileDesktopDoesNotYetSupportTheComplete8ad183cf() =>
      format("mobile.desktop_does_not_yet_support_the_complete.8ad183cf");
  String mobileDesktopIsDisconnected0021ecf1() =>
      format("mobile.desktop_is_disconnected.0021ecf1");
  String mobileDesktopIsDisconnected2ddace9d({required Object? value1}) =>
      format("mobile.desktop_is_disconnected.2ddace9d", {"value1": value1});
  String mobileDesktopIsDisconnected5be7fb00() =>
      format("mobile.desktop_is_disconnected.5be7fb00");
  String mobileDesktopIsUnavailableRetryingAutomaticaC2b4ac01() =>
      format("mobile.desktop_is_unavailable_retrying_automatica.c2b4ac01");
  String mobileDesktopSelectedWorkspacesWillLoadWhen38e1c99f() =>
      format("mobile.desktop_selected_workspaces_will_load_when.38e1c99f");
  String mobileDevice37d06f01() => format("mobile.device.37d06f01");
  String mobileDevicesAndWorkspacesF7253890() =>
      format("mobile.devices_and_workspaces.f7253890");
  String mobileDevicesOnYourAccountE2737cc7() =>
      format("mobile.devices_on_your_account.e2737cc7");
  String mobileDisablePlugin() => format("mobile.disable_plugin");
  String mobileDisabledHere() => format("mobile.disabled_here");
  String mobileDiscardChanges() => format("mobile.discard_changes");
  String mobileDiscoverCb2fb443() => format("mobile.discover.cb2fb443");
  String mobileDiscoveryIsUnavailable52d7df72() =>
      format("mobile.discovery_is_unavailable.52d7df72");
  String mobileDone92e3eec4() => format("mobile.done.92e3eec4");
  String mobileEditApps() => format("mobile.edit_apps");
  String mobileEditLayoutBb3d555a() => format("mobile.edit_layout.bb3d555a");
  String mobileEditOverview() => format("mobile.edit_overview");
  String mobileEmailAddress() => format("mobile.emailAddress");
  String mobileEnableAutomaticApproval424e38bb() =>
      format("mobile.enable_automatic_approval.424e38bb");
  String mobileEnableFullAccess91562a40() =>
      format("mobile.enable_full_access.91562a40");
  String mobileEnablePlugin() => format("mobile.enable_plugin");
  String mobileEnabledHere() => format("mobile.enabled_here");
  String mobileEnglishE6ce2fc8() => format("mobile.english.e6ce2fc8");
  String mobileExpand71acc101() => format("mobile.expand.71acc101");
  String mobileExpandedHeight61c3586f() =>
      format("mobile.expanded_height.61c3586f");
  String mobileExplorePlugins1ff2cee8() =>
      format("mobile.explore_plugins.1ff2cee8");
  String mobileFilingNumber({required Object? number}) =>
      format("mobile.filingNumber", {"number": number});
  String mobileFindMoreApps() => format("mobile.find_more_apps");
  String mobileFindingAnInstalledEditor78027f4d() =>
      format("mobile.finding_an_installed_editor.78027f4d");
  String mobileFinishingRecognition0072ccbb() =>
      format("mobile.finishing_recognition.0072ccbb");
  String mobileFinishingSpeechRecognition8a8c1c0b() =>
      format("mobile.finishing_speech_recognition.8a8c1c0b");
  String mobileFollowingDesktopActivity15d954b3() =>
      format("mobile.following_desktop_activity.15d954b3");
  String mobileFullAccess6d8c6618() => format("mobile.full_access.6d8c6618");
  String mobileHideApp() => format("mobile.hide_app");
  String mobileHoldToSpeakF4dcaf7f() => format("mobile.hold_to_speak.f4dcaf7f");
  String mobileInProgress9591c222() => format("mobile.in_progress.9591c222");
  String mobileInstallModel() => format("mobile.installModel");
  String mobileInstallASignedPdfPackage76ca0551() =>
      format("mobile.install_a_signed_pdf_package.76ca0551");
  String mobileInstallASignedSpreadsheetPackage8e3f20c8() =>
      format("mobile.install_a_signed_spreadsheet_package.8e3f20c8");
  String mobileInstallASignedWordPackageCc6154ed() =>
      format("mobile.install_a_signed_word_package.cc6154ed");
  String mobileInstallThePptxViewerToOpenThisFile8e07e3cd() =>
      format("mobile.install_the_pptx_viewer_to_open_this_file.8e07e3cd");
  String mobileInvalidProvider() => format("mobile.invalidProvider");
  String mobileInvalidDirectoryResponseEf6bbe6d() =>
      format("mobile.invalid_directory_response.ef6bbe6d");
  String mobileInvalidParameter5d4a68c6({required Object? value1}) =>
      format("mobile.invalid_parameter.5d4a68c6", {"value1": value1});
  String mobileInvalidResourceReferenceD9a208e5() =>
      format("mobile.invalid_resource_reference.d9a208e5");
  String mobileInvalidScheduledTaskInput2dac2d75() =>
      format("mobile.invalid_scheduled_task_input.2dac2d75");
  String mobileInvalidScheduledTaskResponse03840268() =>
      format("mobile.invalid_scheduled_task_response.03840268");
  String mobileInvalidSearchQueryLength9db7d4a7() =>
      format("mobile.invalid_search_query_length.9db7d4a7");
  String mobileInvalidTaskIdBee5d4a5() =>
      format("mobile.invalid_task_id.bee5d4a5");
  String mobileKeepEditing() => format("mobile.keep_editing");
  String mobileLanguageA8ae4ccd() => format("mobile.language.a8ae4ccd");
  String mobileLanguageSelection() => format("mobile.languageSelection");
  String mobileLayoutSaveFailed() => format("mobile.layout_save_failed");
  String mobileLight3f812d90() => format("mobile.light.3f812d90");
  String mobileListeningReleaseToFinishOrSlideUpTo8cca1710() =>
      format("mobile.listening_release_to_finish_or_slide_up_to.8cca1710");
  String mobileLocalCopySavedF1994442({required Object? value1}) =>
      format("mobile.local_copy_saved.f1994442", {"value1": value1});
  String mobileManageInstalledPluginsAndUpdates836983e9() =>
      format("mobile.manage_installed_plugins_and_updates.836983e9");
  String mobileManagePlugins1429ebfe() =>
      format("mobile.manage_plugins.1429ebfe");
  String mobileMessage97302219() => format("mobile.message.97302219");
  String mobileMicrophoneAccess86ee6902() =>
      format("mobile.microphone_access.86ee6902");
  String mobileMinimize4f64bec2() => format("mobile.minimize.4f64bec2");
  String mobileModelBusy() => format("mobile.modelBusy");
  String mobileModelDescription() => format("mobile.modelDescription");
  String mobileModelDownloadCancelled() =>
      format("mobile.modelDownloadCancelled");
  String mobileModelDownloadFailed() => format("mobile.modelDownloadFailed");
  String mobileModelDownloading() => format("mobile.modelDownloading");
  String mobileModelInstallPrompt() => format("mobile.modelInstallPrompt");
  String mobileModelInstalled() => format("mobile.modelInstalled");
  String mobileModelNotInstalled() => format("mobile.modelNotInstalled");
  String mobileModelRemove() => format("mobile.modelRemove");
  String mobileModelStorage() => format("mobile.modelStorage");
  String mobileModelUnavailable() => format("mobile.modelUnavailable");
  String mobileModelWorking() => format("mobile.modelWorking");
  String mobileModels() => format("mobile.models");
  String mobileMoveDown() => format("mobile.move_down");
  String mobileMoveUp() => format("mobile.move_up");
  String mobileMyPlugins487f2acf() => format("mobile.my_plugins.487f2acf");
  String mobileNewConversation4e67ed63() =>
      format("mobile.new_conversation.4e67ed63");
  String mobileNewConversationBcec2f30() =>
      format("mobile.new_conversation.bcec2f30");
  String mobileNewConversationIn6c1f1a27({required Object? value1}) =>
      format("mobile.new_conversation_in.6c1f1a27", {"value1": value1});
  String mobileNewTaskE8e54ef0() => format("mobile.new_task.e8e54ef0");
  String mobileNoAppsDetail() => format("mobile.no_apps_detail");
  String mobileNoAppsTitle() => format("mobile.no_apps_title");
  String mobileNoCloudWorkspaces0eaeeb2f() =>
      format("mobile.no_cloud_workspaces.0eaeeb2f");
  String mobileNoDesktopIsAvailableOnThisAccountOp8ae08e25() =>
      format("mobile.no_desktop_is_available_on_this_account_op.8ae08e25");
  String mobileNoPendingTasks492f3c23() =>
      format("mobile.no_pending_tasks.492f3c23");
  String mobileNoSpeechWasRecognizedTryAgain8a4b2ce5() =>
      format("mobile.no_speech_was_recognized_try_again.8a4b2ce5");
  String mobileNoWorkbenchPluginsAreAvailable4cba5019() =>
      format("mobile.no_workbench_plugins_are_available.4cba5019");
  String mobileNoWorkspacesAreAvailableOnDesktopAb22cd6e() =>
      format("mobile.no_workspaces_are_available_on_desktop.ab22cd6e");
  String mobileNoWorkspacesAreAvailableOnThisDevice08eae7ea() =>
      format("mobile.no_workspaces_are_available_on_this_device.08eae7ea");
  String mobileNotNow5c258359() => format("mobile.not_now.5c258359");
  String mobileNotSignedIn3c4f1f91() => format("mobile.not_signed_in.3c4f1f91");
  String mobileOffline054e29cd() => format("mobile.offline.054e29cd");
  String mobileOneColumn84e0d35c() => format("mobile.one_column.84e0d35c");
  String mobileOpen9cdb481b({required Object? value1}) =>
      format("mobile.open.9cdb481b", {"value1": value1});
  String mobileOpenADesktopSessionBeforeChangingWorC6752606() =>
      format("mobile.open_a_desktop_session_before_changing_wor.c6752606");
  String mobileOpenADocxDocument2a94c464() =>
      format("mobile.open_a_docx_document.2a94c464");
  String mobileOpenApp() => format("mobile.open_app");
  String mobileOpenAssistant018b4e66() =>
      format("mobile.open_assistant.018b4e66");
  String mobileOpenDocxFilesOfflineF0b63475() =>
      format("mobile.open_docx_files_offline.f0b63475");
  String mobileOpenPdfFilesOffline8ea93832() =>
      format("mobile.open_pdf_files_offline.8ea93832");
  String mobileOpenXlsxAndCsvFilesOfflineBd039729() =>
      format("mobile.open_xlsx_and_csv_files_offline.bd039729");
  String mobileOpenXlsxOrCsv67db3684() =>
      format("mobile.open_xlsx_or_csv.67db3684");
  String mobileOpeningTheLocalEditor59b2af8c() =>
      format("mobile.opening_the_local_editor.59b2af8c");
  String mobileOpenmuseUsesTheMicrophoneToTurnSpeec80d628f6() =>
      format("mobile.openmuse_uses_the_microphone_to_turn_speec.80d628f6");
  String mobileOperatedByDongguanChengxuanElectronic4bdff712() =>
      format("mobile.operated_by_dongguan_chengxuan_electronic.4bdff712");
  String mobileOverview() => format("mobile.overview");
  String mobilePdfDocument54149cdd() => format("mobile.pdf_document.54149cdd");
  String mobilePendingNextA4e2950f({
    required Object? value1,
    required Object? value2,
  }) => format("mobile.pending_next.a4e2950f", {
    "value1": value1,
    "value2": value2,
  });
  String mobilePermanentlyDeleteYourAccount4524c47f() =>
      format("mobile.permanently_delete_your_account.4524c47f");
  String mobilePinFc99471d() => format("mobile.pin.fc99471d");
  String mobilePluginsOnThisDevice() => format("mobile.plugins_on_this_device");
  String mobilePluginsSummary() => format("mobile.plugins_summary");
  String mobilePreferences() => format("mobile.preferences");
  String mobilePreparingTheLocalEditor139ca003() =>
      format("mobile.preparing_the_local_editor.139ca003");
  String mobilePreparingToOpenTheFile6349c791() =>
      format("mobile.preparing_to_open_the_file.6349c791");
  String mobilePreviousConversation196d79ba() =>
      format("mobile.previous_conversation.196d79ba");
  String mobilePrivacyPolicy9233e2fb() =>
      format("mobile.privacy_policy.9233e2fb");
  String mobileReadOnlyCb269872() => format("mobile.read_only.cb269872");
  String mobileReasoningEffort73c10416() =>
      format("mobile.reasoning_effort.73c10416");
  String mobileReceivingTheDesktopFileDb4f8afd() =>
      format("mobile.receiving_the_desktop_file.db4f8afd");
  String mobileRecentTasksCdbbd048() => format("mobile.recent_tasks.cdbbd048");
  String mobileReconnect87c50d4d() => format("mobile.reconnect.87c50d4d");
  String mobileReconnectF80f36fd() => format("mobile.reconnect.f80f36fd");
  String mobileRefresh20ada9a3() => format("mobile.refresh.20ada9a3");
  String mobileReleaseToCancelC48c1de4() =>
      format("mobile.release_to_cancel.c48c1de4");
  String mobileReleaseToCancelDictation75875019() =>
      format("mobile.release_to_cancel_dictation.75875019");
  String mobileReleaseToFinishSlideUpToCancel354d09d6() =>
      format("mobile.release_to_finish_slide_up_to_cancel.354d09d6");
  String mobileReleaseToFinishSlideUpToCancel5755f1f0() =>
      format("mobile.release_to_finish_slide_up_to_cancel.5755f1f0");
  String mobileRemoteDshConnectedF83ee6e6() =>
      format("mobile.remote_dsh_connected.f83ee6e6");
  String mobileResetPluginSize1482e95f() =>
      format("mobile.reset_plugin_size.1482e95f");
  String mobileResizeC93a70d1() => format("mobile.resize.c93a70d1");
  String mobileResumeFollowingDesktopActivity5eeb7004() =>
      format("mobile.resume_following_desktop_activity.5eeb7004");
  String mobileRetry() => format("mobile.retry");
  String mobileRetry4c2e361c() => format("mobile.retry.4c2e361c");
  String mobileSave() => format("mobile.save");
  String mobileSaveALocalXlsxCopy0740ebca() =>
      format("mobile.save_a_local_xlsx_copy.0740ebca");
  String mobileSaveAScreenshotOfThisQrCodeAndSeleEa55dc9f() =>
      format("mobile.save_a_screenshot_of_this_qr_code_and_sele.ea55dc9f");
  String mobileSaveChanges() => format("mobile.save_changes");
  String mobileSaveLayoutChanges() => format("mobile.save_layout_changes");
  String mobileScheduleMessagesForADesktopWorkspace9367bb21() =>
      format("mobile.schedule_messages_for_a_desktop_workspace.9367bb21");
  String mobileScheduledTasksD8f7d270() =>
      format("mobile.scheduled_tasks.d8f7d270");
  String mobileSearchFilenamesAndPreviewFiles64b34549() =>
      format("mobile.search_filenames_and_preview_files.64b34549");
  String mobileSend8457aed9() => format("mobile.send.8457aed9");
  String mobileServices() => format("mobile.services");
  String mobileSetAsTheDefaultEditor310a2582() =>
      format("mobile.set_as_the_default_editor.310a2582");
  String mobileShowApp() => format("mobile.show_app");
  String mobileSignInToLoadWorkspaces7d3d9897() =>
      format("mobile.sign_in_to_load_workspaces.7d3d9897");
  String mobileSignInToSyncDevicesAndPlugins2fba3876() =>
      format("mobile.sign_in_to_sync_devices_and_plugins.2fba3876");
  String mobileSignInToTheSameAccountOnYourComput3e0a75c4() =>
      format("mobile.sign_in_to_the_same_account_on_your_comput.3e0a75c4");
  String mobileSignOutB67e6932() => format("mobile.sign_out.b67e6932");
  String mobileSpeechModelName() => format("mobile.speechModelName");
  String mobileSpeechRecognitionFailedTryAgain965b77bb() =>
      format("mobile.speech_recognition_failed_try_again.965b77bb");
  String mobileSpreadsheet2a8d5279() => format("mobile.spreadsheet.2a8d5279");
  String mobileStandardHeight173970c3() =>
      format("mobile.standard_height.173970c3");
  String mobileStopGenerating21733a02() =>
      format("mobile.stop_generating.21733a02");
  String mobileStorageReadAndWrite34cf9652() =>
      format("mobile.storage_read_and_write.34cf9652");
  String mobileStorageReadOnlyE72ee688() =>
      format("mobile.storage_read_only.e72ee688");
  String mobileStoreProviderDescription() =>
      format("mobile.storeProviderDescription");
  String mobileStoreProviderTitle() => format("mobile.storeProviderTitle");
  String mobileSwitchToDictation740f6546() =>
      format("mobile.switch_to_dictation.740f6546");
  String mobileSwitchToKeyboard2f4cc281() =>
      format("mobile.switch_to_keyboard.2f4cc281");
  String mobileSyncDesktopAgainFd673371() =>
      format("mobile.sync_desktop_again.fd673371");
  String mobileSystem9a0c51db() => format("mobile.system.9a0c51db");
  String mobileSystemDefault9ab83435() =>
      format("mobile.system_default.9ab83435");
  String mobileTaskSettings2fc94999() =>
      format("mobile.task_settings.2fc94999");
  String mobileTasks49bc7474() => format("mobile.tasks.49bc7474");
  String mobileTermsOfServiceEd11dce1() =>
      format("mobile.terms_of_service.ed11dce1");
  String mobileTextSize896c8e39() => format("mobile.text_size.896c8e39");
  String mobileTheAccountServiceUrlIsNotSecureThe656db052() =>
      format("mobile.the_account_service_url_is_not_secure_the.656db052");
  String mobileTheDeletionRequestDidNotCompleteHttp82b79147({
    required Object? value1,
  }) => format("mobile.the_deletion_request_did_not_complete_http.82b79147", {
    "value1": value1,
  });
  String mobileTheDesktopRelayIsUnavailableReconnectEbf1165a() =>
      format("mobile.the_desktop_relay_is_unavailable_reconnect.ebf1165a");
  String mobileTheDesktopRequestFailedHttpEb760a86({required Object? value1}) =>
      format("mobile.the_desktop_request_failed_http.eb760a86", {
        "value1": value1,
      });
  String mobileTheDesktopResponseExceedsThe64MibLiDc5ead0a() =>
      format("mobile.the_desktop_response_exceeds_the_64_mib_li.dc5ead0a");
  String mobileTheDesktopScheduledTaskRequestFailed04189498({
    required Object? value1,
  }) => format("mobile.the_desktop_scheduled_task_request_failed.04189498", {
    "value1": value1,
  });
  String mobileTheDeviceDirectoryIsUnavailableF874e876() =>
      format("mobile.the_device_directory_is_unavailable.f874e876");
  String mobileTheFileExceedsThe64MibLimitAd41f580() =>
      format("mobile.the_file_exceeds_the_64_mib_limit.ad41f580");
  String mobileTheFileTransferTimedOut3f953668() =>
      format("mobile.the_file_transfer_timed_out.3f953668");
  String mobileTheHostIsNotReady222d3501() =>
      format("mobile.the_host_is_not_ready.222d3501");
  String mobileTheNetworkIsUnavailableYourAccountHa29677dbd() =>
      format("mobile.the_network_is_unavailable_your_account_ha.29677dbd");
  String mobileThePairedDesktopPluginIsUnavailableD368a69e() =>
      format("mobile.the_paired_desktop_plugin_is_unavailable.d368a69e");
  String mobileTheRemoteDshUrlIsInvalidAndWasBlocE6098bb8() =>
      format("mobile.the_remote_dsh_url_is_invalid_and_was_bloc.e6098bb8");
  String mobileTheScheduledTaskResponseIsTooLarge8fb95058() =>
      format("mobile.the_scheduled_task_response_is_too_large.8fb95058");
  String mobileTheServerDidNotConfirmAccountDeletio6b42db38() =>
      format("mobile.the_server_did_not_confirm_account_deletio.6b42db38");
  String mobileTheTaskWasReceivedOnFilesModelsAndA82dcdb7({
    required Object? value1,
  }) => format("mobile.the_task_was_received_on_files_models_and.a82dcdb7", {
    "value1": value1,
  });
  String mobileTheWordPackageIsUnavailableB3078b51() =>
      format("mobile.the_word_package_is_unavailable.b3078b51");
  String mobileTheWorkbenchIsUnavailable9160d296() =>
      format("mobile.the_workbench_is_unavailable.9160d296");
  String mobileThisDeletesYourOpenmuseAccountTheWorDaf1a68b() =>
      format("mobile.this_deletes_your_openmuse_account_the_wor.daf1a68b");
  String mobileThisDesktopPluginRequiresItsWebConve49ea4be8() =>
      format("mobile.this_desktop_plugin_requires_its_web_conve.49ea4be8");
  String mobileThisDevice() => format("mobile.this_device");
  String mobileThisExpandsTheToolsAndFileOperations6adfe213() =>
      format("mobile.this_expands_the_tools_and_file_operations.6adfe213");
  String mobileThisFileIsTooLargeForAnInlinePrevi9350b84b() =>
      format("mobile.this_file_is_too_large_for_an_inline_previ.9350b84b");
  String mobileThisFileReferenceHasExpiredRefreshTh19c36658() =>
      format("mobile.this_file_reference_has_expired_refresh_th.19c36658");
  String mobileThisPluginIsDisabled64309558() =>
      format("mobile.this_plugin_is_disabled.64309558");
  String mobileTimedOutCheckYourNetworkAndRetry031c4f34({
    required Object? value1,
  }) => format("mobile.timed_out_check_your_network_and_retry.031c4f34", {
    "value1": value1,
  });
  String mobileTwoColumnsA45919ff() => format("mobile.two_columns.a45919ff");
  String mobileUnableToCancelDictationTryAgainLater0bdbf4b0() =>
      format("mobile.unable_to_cancel_dictation_try_again_later.0bdbf4b0");
  String mobileUnableToChangePermissionsD8c46f1a({required Object? value1}) =>
      format("mobile.unable_to_change_permissions.d8c46f1a", {
        "value1": value1,
      });
  String mobileUnableToCreateADesktopSession370dc1c1({
    required Object? value1,
  }) => format("mobile.unable_to_create_a_desktop_session.370dc1c1", {
    "value1": value1,
  });
  String mobileUnableToFinishDictationTryAgainLaterF198d1db() =>
      format("mobile.unable_to_finish_dictation_try_again_later.f198d1db");
  String mobileUnableToInstallThePdfPackage73981105({
    required Object? value1,
  }) => format("mobile.unable_to_install_the_pdf_package.73981105", {
    "value1": value1,
  });
  String mobileUnableToInstallTheSpreadsheetPackage91122c4c({
    required Object? value1,
  }) => format("mobile.unable_to_install_the_spreadsheet_package.91122c4c", {
    "value1": value1,
  });
  String mobileUnableToInstallTheWordPackage21262110({
    required Object? value1,
  }) => format("mobile.unable_to_install_the_word_package.21262110", {
    "value1": value1,
  });
  String mobileUnableToLoadTheAssistantCheckYourCo12dc91dd() =>
      format("mobile.unable_to_load_the_assistant_check_your_co.12dc91dd");
  String mobileUnableToLoadThePdfPackage9a77d994({required Object? value1}) =>
      format("mobile.unable_to_load_the_pdf_package.9a77d994", {
        "value1": value1,
      });
  String mobileUnableToLoadTheSpreadsheetPackageOr8d1d8c71({
    required Object? value1,
  }) => format("mobile.unable_to_load_the_spreadsheet_package_or.8d1d8c71", {
    "value1": value1,
  });
  String mobileUnableToOpenDocxBa4be5f8({required Object? value1}) =>
      format("mobile.unable_to_open_docx.ba4be5f8", {"value1": value1});
  String mobileUnableToOpenDocxE1967d9f() =>
      format("mobile.unable_to_open_docx.e1967d9f");
  String mobileUnableToOpenThisDocumentA6fd0081() =>
      format("mobile.unable_to_open_this_document.a6fd0081");
  String mobileUnableToOpenThisLinkRightNowC8600c59() =>
      format("mobile.unable_to_open_this_link_right_now.c8600c59");
  String mobileUnableToReadTheDocument4e6f6997() =>
      format("mobile.unable_to_read_the_document.4e6f6997");
  String mobileUnableToReadTheFeaturePackage7b18502c() =>
      format("mobile.unable_to_read_the_feature_package.7b18502c");
  String mobileUnableToReadTheFile8c868ffb() =>
      format("mobile.unable_to_read_the_file.8c868ffb");
  String mobileUnableToSaveALocalCopyB8d12b7a({required Object? value1}) =>
      format("mobile.unable_to_save_a_local_copy.b8d12b7a", {"value1": value1});
  String mobileUnableToSaveTheLayoutTryAgainLater8661017f() =>
      format("mobile.unable_to_save_the_layout_try_again_later.8661017f");
  String mobileUnableToSendF0d76162({required Object? value1}) =>
      format("mobile.unable_to_send.f0d76162", {"value1": value1});
  String mobileUnableToStartDictationTryAgainLater01330b7f() =>
      format("mobile.unable_to_start_dictation_try_again_later.01330b7f");
  String mobileUnableToSwitchModels602eabea({required Object? value1}) =>
      format("mobile.unable_to_switch_models.602eabea", {"value1": value1});
  String mobileUninstallDescription() => format("mobile.uninstall_description");
  String mobileUninstallPlugin() => format("mobile.uninstall_plugin");
  String mobileUnknownFileOperationAe6b4b8a() =>
      format("mobile.unknown_file_operation.ae6b4b8a");
  String mobileUnknownScheduledTaskActionD2a60cc3() =>
      format("mobile.unknown_scheduled_task_action.d2a60cc3");
  String mobileUnpinEf1e845f() => format("mobile.unpin.ef1e845f");
  String mobileVersion() => format("mobile.version");
  String mobileWaitingForDesktopToReadTheFile2407e9d7() =>
      format("mobile.waiting_for_desktop_to_read_the_file.2407e9d7");
  String mobileWasInterruptedReconnectDesktopAndRetr38590292({
    required Object? value1,
  }) => format("mobile.was_interrupted_reconnect_desktop_and_retr.38590292", {
    "value1": value1,
  });
  String mobileWasInterruptedTryAgain217c79d1({required Object? value1}) =>
      format("mobile.was_interrupted_try_again.217c79d1", {"value1": value1});
  String mobileWhichEditorShouldOpenBd34ddac({required Object? value1}) =>
      format("mobile.which_editor_should_open.bd34ddac", {"value1": value1});
  String mobileWordDocument793e039c() =>
      format("mobile.word_document.793e039c");
  String mobileWorkbenchA6f1b51f() => format("mobile.workbench.a6f1b51f");
  String mobileWorkspaceCbc7c9de() => format("mobile.workspace.cbc7c9de");
  String mobileWorkspaceFilesC16b24dd() =>
      format("mobile.workspace_files.c16b24dd");
  String mobileWorkspacePermissionsB43b081a() =>
      format("mobile.workspace_permissions.b43b081a");
  String mobileWorkspacePermissionsEb2b5fb3() =>
      format("mobile.workspace_permissions.eb2b5fb3");
  String mobileWorkspaceWriteAccessFae86ef7() =>
      format("mobile.workspace_write_access.fae86ef7");
  String mobileWorkspaces21170e4d({
    required Object? value1,
    required Object? value2,
  }) => format("mobile.workspaces.21170e4d", {
    "value1": value1,
    "value2": value2,
  });
  String mobileWorkspaces3d22293a({required Object? value1}) =>
      format("mobile.workspaces.3d22293a", {"value1": value1});
  String mobileYourAccountAndDevicesAreConnectedA002bdae() =>
      format("mobile.your_account_and_devices_are_connected.a002bdae");
  String mobileYourAppsArranged() => format("mobile.your_apps_arranged");
  String mobileYourOverview() => format("mobile.your_overview");
  String mobileYourSessionHasExpiredSignInAgainAnd9fd9856b() =>
      format("mobile.your_session_has_expired_sign_in_again_and.9fd9856b");
  String mobileYourToolsAtAGlance9f2e8073() =>
      format("mobile.your_tools_at_a_glance.9f2e8073");
  String officeParagraphE6bbad92({required Object? value1}) =>
      format("office.paragraph.e6bbad92", {"value1": value1});
  String officePdfReadOnlyTextView213a26ed() =>
      format("office.pdf_read_only_text_view.213a26ed");
  String officePptxReadOnlyCompatibilityViewFc7ca3d5() =>
      format("office.pptx_read_only_compatibility_view.fc7ca3d5");
  String officeSave70f4816a() => format("office.save.70f4816a");
  String officeSaveConflictOrServiceUnavailable8869ea79() =>
      format("office.save_conflict_or_service_unavailable.8869ea79");
  String officeSaved7c66f039({required Object? value1}) =>
      format("office.saved.7c66f039", {"value1": value1});
  String officeSavingB528e788() => format("office.saving.b528e788");
  String officeThisDocumentContainsUnsupportedStructuEe583283() =>
      format("office.this_document_contains_unsupported_structu.ee583283");
  String officeUnableToSafelyOpenThisDocumentC0a1ba9d() =>
      format("office.unable_to_safely_open_this_document.c0a1ba9d");
  String officeUnableToSafelyOpenThisDocxFileD2dd2cd6() =>
      format("office.unable_to_safely_open_this_docx_file.d2dd2cd6");
  String officeXlsxReadOnlyCompatibilityView9db253b8() =>
      format("office.xlsx_read_only_compatibility_view.9db253b8");
  String resourcecenterAddToChat() => format("resourcecenter.addToChat");
  String resourcecenterAlreadyAttached() =>
      format("resourcecenter.alreadyAttached");
  String resourcecenterAttachLimit() => format("resourcecenter.attachLimit");
  String resourcecenterBrowseLocal() => format("resourcecenter.browseLocal");
  String resourcecenterCancel() => format("resourcecenter.cancel");
  String resourcecenterClose() => format("resourcecenter.close");
  String resourcecenterConflict() => format("resourcecenter.conflict");
  String resourcecenterConnectPc() => format("resourcecenter.connectPc");
  String resourcecenterContextTooLarge() =>
      format("resourcecenter.contextTooLarge");
  String resourcecenterContextUnavailable() =>
      format("resourcecenter.contextUnavailable");
  String resourcecenterConversation() => format("resourcecenter.conversation");
  String resourcecenterConversationVersion() =>
      format("resourcecenter.conversationVersion");
  String resourcecenterCopied() => format("resourcecenter.copied");
  String resourcecenterCopy() => format("resourcecenter.copy");
  String resourcecenterCopyPath() => format("resourcecenter.copyPath");
  String resourcecenterCreateOpen() => format("resourcecenter.createOpen");
  String resourcecenterCreateResource() =>
      format("resourcecenter.createResource");
  String resourcecenterCreatedOpenFailed() =>
      format("resourcecenter.createdOpenFailed");
  String resourcecenterCreating() => format("resourcecenter.creating");
  String resourcecenterCreatorUnavailable() =>
      format("resourcecenter.creatorUnavailable");
  String resourcecenterDefaultHandler() =>
      format("resourcecenter.defaultHandler");
  String resourcecenterDenied() => format("resourcecenter.denied");
  String resourcecenterDiscard() => format("resourcecenter.discard");
  String resourcecenterDocuments() => format("resourcecenter.documents");
  String resourcecenterDraft() => format("resourcecenter.draft");
  String resourcecenterEarlier() => format("resourcecenter.earlier");
  String resourcecenterEdit() => format("resourcecenter.edit");
  String resourcecenterEmptyFolder() => format("resourcecenter.emptyFolder");
  String resourcecenterEmptyHint() => format("resourcecenter.emptyHint");
  String resourcecenterExpired() => format("resourcecenter.expired");
  String resourcecenterFailed() => format("resourcecenter.failed");
  String resourcecenterHome() => format("resourcecenter.home");
  String resourcecenterImport() => format("resourcecenter.import");
  String resourcecenterImportOrganize() =>
      format("resourcecenter.importOrganize");
  String resourcecenterImported() => format("resourcecenter.imported");
  String resourcecenterIncompatible() => format("resourcecenter.incompatible");
  String resourcecenterIntegrityFailure() =>
      format("resourcecenter.integrityFailure");
  String resourcecenterInvalidName() => format("resourcecenter.invalidName");
  String resourcecenterInvalidReference() =>
      format("resourcecenter.invalidReference");
  String resourcecenterInvalidUrl() => format("resourcecenter.invalidUrl");
  String resourcecenterKeepEditing() => format("resourcecenter.keepEditing");
  String resourcecenterKeepExtension() =>
      format("resourcecenter.keepExtension");
  String resourcecenterLoading() => format("resourcecenter.loading");
  String resourcecenterLocalImportHint() =>
      format("resourcecenter.localImportHint");
  String resourcecenterMarkdown() => format("resourcecenter.markdown");
  String resourcecenterMessageHint() => format("resourcecenter.messageHint");
  String resourcecenterMore() => format("resourcecenter.more");
  String resourcecenterName() => format("resourcecenter.name");
  String resourcecenterNameConflict() => format("resourcecenter.nameConflict");
  String resourcecenterNew() => format("resourcecenter.new");
  String resourcecenterNewFolder() => format("resourcecenter.newFolder");
  String resourcecenterNoCreators() => format("resourcecenter.noCreators");
  String resourcecenterNoHandler() => format("resourcecenter.noHandler");
  String resourcecenterNoRecent() => format("resourcecenter.noRecent");
  String resourcecenterNoRecentHint() => format("resourcecenter.noRecentHint");
  String resourcecenterNoResults() => format("resourcecenter.noResults");
  String resourcecenterNoResultsHint() =>
      format("resourcecenter.noResultsHint");
  String resourcecenterNoSources() => format("resourcecenter.noSources");
  String resourcecenterNotSent() => format("resourcecenter.notSent");
  String resourcecenterNotes() => format("resourcecenter.notes");
  String resourcecenterOffline() => format("resourcecenter.offline");
  String resourcecenterOfflineCopy() => format("resourcecenter.offlineCopy");
  String resourcecenterOnline() => format("resourcecenter.online");
  String resourcecenterOpen() => format("resourcecenter.open");
  String resourcecenterOpenBrowser() => format("resourcecenter.openBrowser");
  String resourcecenterOpenOriginal() => format("resourcecenter.openOriginal");
  String resourcecenterOpenWith() => format("resourcecenter.openWith");
  String resourcecenterOriginOnly() => format("resourcecenter.originOnly");
  String resourcecenterPaste() => format("resourcecenter.paste");
  String resourcecenterPathCopied() => format("resourcecenter.pathCopied");
  String resourcecenterPeerUpgrade() => format("resourcecenter.peerUpgrade");
  String resourcecenterPlainText() => format("resourcecenter.plainText");
  String resourcecenterReadOnly() => format("resourcecenter.readOnly");
  String resourcecenterRecent() => format("resourcecenter.recent");
  String resourcecenterRefresh() => format("resourcecenter.refresh");
  String resourcecenterRemote() => format("resourcecenter.remote");
  String resourcecenterRemotePc() => format("resourcecenter.remotePc");
  String resourcecenterRemoveRecent() => format("resourcecenter.removeRecent");
  String resourcecenterRemoveReference() =>
      format("resourcecenter.removeReference");
  String resourcecenterRemovedRecent() =>
      format("resourcecenter.removedRecent");
  String resourcecenterRename() => format("resourcecenter.rename");
  String resourcecenterRetain() => format("resourcecenter.retain");
  String resourcecenterRetainHint() => format("resourcecenter.retainHint");
  String resourcecenterRetry() => format("resourcecenter.retry");
  String resourcecenterSave() => format("resourcecenter.save");
  String resourcecenterSaveCopy() => format("resourcecenter.saveCopy");
  String resourcecenterSaveIn() => format("resourcecenter.saveIn");
  String resourcecenterSaved() => format("resourcecenter.saved");
  String resourcecenterSaving() => format("resourcecenter.saving");
  String resourcecenterSearch() => format("resourcecenter.search");
  String resourcecenterSearchRecent() => format("resourcecenter.searchRecent");
  String resourcecenterSend() => format("resourcecenter.send");
  String resourcecenterSnapshotNoWrite() =>
      format("resourcecenter.snapshotNoWrite");
  String resourcecenterSnapshots() => format("resourcecenter.snapshots");
  String resourcecenterSource() => format("resourcecenter.source");
  String resourcecenterSourceReadOnly() =>
      format("resourcecenter.sourceReadOnly");
  String resourcecenterSourceUnavailable() =>
      format("resourcecenter.sourceUnavailable");
  String resourcecenterThisDevice() => format("resourcecenter.thisDevice");
  String resourcecenterTitle() => format("resourcecenter.title");
  String resourcecenterToday() => format("resourcecenter.today");
  String resourcecenterTooLarge() => format("resourcecenter.tooLarge");
  String resourcecenterUndo() => format("resourcecenter.undo");
  String resourcecenterUnsaved() => format("resourcecenter.unsaved");
  String resourcecenterUntitledMarkdown() =>
      format("resourcecenter.untitledMarkdown");
  String resourcecenterUntitledText() => format("resourcecenter.untitledText");
  String resourcecenterUrl() => format("resourcecenter.url");
  String resourcecenterVersionUnavailable() =>
      format("resourcecenter.versionUnavailable");
  String resourcecenterView() => format("resourcecenter.view");
  String resourcecenterWebLink() => format("resourcecenter.webLink");
  String shellAccountDevices() => format("shell.accountDevices");
  String shellDesktopDirect() => format("shell.desktopDirect");
  String shellNoWorkspaces() => format("shell.noWorkspaces");
  String shellNotSignedIn() => format("shell.notSignedIn");
  String shellRefreshWorkspaces() => format("shell.refreshWorkspaces");
  String shellSignInCloud() => format("shell.signInCloud");
  String shellSignOut() => format("shell.signOut");
  String shellWorkspacesFailed() => format("shell.workspacesFailed");
  String updatesDismiss() => format("updates.dismiss");
  String updatesDownloadingUpdate63359be5() =>
      format("updates.downloading_update.63359be5");
  String updatesGetUpdate987882f6() => format("updates.get_update.987882f6");
  String updatesLater78fd2cc2() => format("updates.later.78fd2cc2");
  String updatesOpenmuseIsDownloadedAndVerifiedRestar14710fdb({
    required Object? value1,
  }) => format("updates.openmuse_is_downloaded_and_verified_restar.14710fdb", {
    "value1": value1,
  });
  String updatesReadyToUpdateB17ee8fe() =>
      format("updates.ready_to_update.b17ee8fe");
  String updatesRestart8c8f1d01() => format("updates.restart.8c8f1d01");
  String updatesRestartLaterD2da82eb() =>
      format("updates.restart_later.d2da82eb");
  String updatesRestartNow09959419() => format("updates.restart_now.09959419");
  String updatesRetry4c2e361c() => format("updates.retry.4c2e361c");
  String updatesUpdate() => format("updates.update");
  String updatesUpdateAvailable5157b4c7() =>
      format("updates.update_available.5157b4c7");
  String updatesUpdateAvailableEb64bf34({required Object? value1}) =>
      format("updates.update_available.eb64bf34", {"value1": value1});
  String updatesUpdateFailed6aaaf1ba() =>
      format("updates.update_failed.6aaaf1ba");
  String updatesUpdateFailed9d48e538({required Object? value1}) =>
      format("updates.update_failed.9d48e538", {"value1": value1});
  String updatesUpdateReadyEf88289a() =>
      format("updates.update_ready.ef88289a");
  String updatesWhatsNew() => format("updates.whatsNew");
}
