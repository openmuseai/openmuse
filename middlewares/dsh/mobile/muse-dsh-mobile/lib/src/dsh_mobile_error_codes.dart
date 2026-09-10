/// Frozen user-visible DSH mobile shell errors. Keep messages identical to
/// `contracts/dsh-mobile-error-codes.v1.json`.
class DshMobileErrorCode {
  const DshMobileErrorCode._(this.code, this.message);

  final String code;
  final String message;

  static const tlsUntrusted = DshMobileErrorCode._(
    'DSH_TLS_UNTRUSTED',
    'DSH 的 HTTPS 证书未通过手机验证。请检查服务器证书链、手机时间及网络；不会跳过证书验证。',
  );
  static const documentLoadFailed = DshMobileErrorCode._(
    'DSH_DOCUMENT_LOAD_FAILED',
    'DSH 页面加载失败，请检查 HTTPS 连接。',
  );
  static const configInvalid = DshMobileErrorCode._(
    'DSH_CONFIG_INVALID',
    'DSH 页面加载失败，请检查工作区和 HTTPS 地址配置。',
  );
  static const needAuth = DshMobileErrorCode._(
    'NEED_AUTH',
    '登录 Cloud 后才能启动远程 Agent。',
  );
  static const scopeChanged = DshMobileErrorCode._(
    'DSH_SCOPE_CHANGED',
    '账号或工作区已改变，请返回后重新打开 Agent。',
  );
  static const networkOffline = DshMobileErrorCode._(
    'NETWORK_OFFLINE',
    '网络已断开，请恢复网络后重试；文档仍可离线编辑。',
  );
  static const appBackgrounded = DshMobileErrorCode._(
    'APP_BACKGROUNDED',
    '应用已进入后台，控制连接已关闭；DSH 页面保留。',
  );
  static const bridgeDisconnected = DshMobileErrorCode._(
    'BRIDGE_DISCONNECTED',
    '控制连接中断，可继续使用 DSH 页面。',
  );
  static const cloudLoginRequired = DshMobileErrorCode._(
    'CLOUD_LOGIN_REQUIRED',
    '登录 Cloud 后可尝试连接；当前仅加载 DSH 页面。',
  );
  static const hostUpgradeRequired = DshMobileErrorCode._(
    'HOST_UPGRADE_REQUIRED',
    'DSH 服务端尚未支持 Mobile 合同桥；当前仅加载 DSH 页面。',
  );
  static const bridgeUnavailable = DshMobileErrorCode._(
    'BRIDGE_UNAVAILABLE',
    '控制连接暂不可用；当前仅加载 DSH 页面。',
  );
  static const credentialExpiring = DshMobileErrorCode._(
    'CREDENTIAL_EXPIRING',
    '设备凭据即将到期，请重新连接以刷新。',
  );
  static const contextSyncFailed = DshMobileErrorCode._(
    'CONTEXT_SYNC_FAILED',
    '文档上下文同步失败，请重新连接。',
  );
  static const contextRefreshFailed = DshMobileErrorCode._(
    'CONTEXT_REFRESH_FAILED',
    '工作区上下文刷新失败。',
  );
  static const surfaceCloseFailed = DshMobileErrorCode._(
    'SURFACE_CLOSE_FAILED',
    '关闭文档上下文失败。',
  );
  static const nativeDocumentReturned = DshMobileErrorCode._(
    'NATIVE_DOCUMENT_RETURNED',
    '文档操作已结束，请重新连接 Agent 以刷新上下文。',
  );

  static const fatal = [
    tlsUntrusted,
    documentLoadFailed,
    configInvalid,
    needAuth,
    scopeChanged,
  ];

  static const degraded = [
    networkOffline,
    appBackgrounded,
    bridgeDisconnected,
    cloudLoginRequired,
    hostUpgradeRequired,
    bridgeUnavailable,
    credentialExpiring,
    contextSyncFailed,
    contextRefreshFailed,
    surfaceCloseFailed,
    nativeDocumentReturned,
  ];

  static String credentialUnavailable(String nestedCode) =>
      '设备凭据暂不可用（$nestedCode）；DSH 页面不受此限制。';
}
