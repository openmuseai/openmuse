// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'openmuse_auth_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Chinese (`zh`).
class OpenMuseAuthLocalizationsZh extends OpenMuseAuthLocalizations {
  OpenMuseAuthLocalizationsZh([String locale = 'zh']) : super(locale);

  @override
  String get welcomeTitle => '欢迎使用 OpenMuse';

  @override
  String cloudLabel(String label) {
    return '云端：$label';
  }

  @override
  String get emailHint => '请输入邮箱';

  @override
  String get continueWithEmail => '使用邮箱验证码继续';

  @override
  String get continueWithPassword => '使用密码继续';

  @override
  String get invalidEmail => '请输入有效的邮箱地址。';

  @override
  String get checkYourEmailTitle => '查收邮件';

  @override
  String signInCodeSent(String email) {
    return '我们已向 $email 发送登录验证码。';
  }

  @override
  String get passcodeHint => '输入验证码';

  @override
  String get verifying => '验证中…';

  @override
  String get continueAction => '继续';

  @override
  String get backToLogin => '返回登录';

  @override
  String get enterPasswordTitle => '输入密码';

  @override
  String get passwordHint => '请输入密码';

  @override
  String get loginAsLabel => '登录账号';

  @override
  String get forgotPassword => '忘记密码？';

  @override
  String get createAccount => '创建账号';

  @override
  String get createAccountTitle => '创建你的账号';

  @override
  String signUpCodeSent(String email) {
    return '请输入发送至 $email 的验证码以确认账号。';
  }

  @override
  String recoveryCodeSent(String email) {
    return '请输入发送至 $email 的密码重置验证码。';
  }

  @override
  String get resendCode => '重新发送验证码';

  @override
  String resendCodeCountdown(int seconds) {
    return '$seconds 秒后可重新发送';
  }

  @override
  String get resetPasswordTitle => '重置密码';

  @override
  String get newPassword => '新密码';

  @override
  String get confirmPassword => '确认密码';

  @override
  String get setNewPassword => '设置新密码';

  @override
  String get passwordTooShort => '密码至少需要 8 个字符。';

  @override
  String get passwordMismatch => '两次输入的密码不一致。';

  @override
  String get agreementPrefix => '继续即表示你同意我们的';

  @override
  String get termsOfService => '服务条款';

  @override
  String get agreementConjunction => '和';

  @override
  String get privacyPolicy => '隐私政策';

  @override
  String get agreementSuffix => '。';

  @override
  String get settings => '设置';

  @override
  String get anonymousMode => '匿名模式';

  @override
  String get accountSectionTitle => '登录账号';

  @override
  String get notSignedIn => '未登录';

  @override
  String get signOut => '退出登录';

  @override
  String get errorInvalidCredentials => '邮箱或密码不正确。';

  @override
  String get errorInvalidResponse => '认证服务返回了无法识别的响应。';

  @override
  String get errorSessionExpired => '登录状态已过期，请重新登录。';

  @override
  String get errorNetwork => '无法连接到服务，请检查网络后重试。';

  @override
  String get errorServer => '服务暂时无法完成请求，请稍后重试。';

  @override
  String get errorBootstrap => '账号工作区初始化失败。';

  @override
  String get errorInvalidConfiguration => '服务地址配置无效。';

  @override
  String get errorInvalidCode => '验证码无效或已过期。';

  @override
  String get errorAccountExists => '该邮箱已有账号，请直接登录。';

  @override
  String get errorWeakPassword => '请设置更强的密码。';

  @override
  String get errorRateLimited => '请求过于频繁，请稍后重试。';
}
