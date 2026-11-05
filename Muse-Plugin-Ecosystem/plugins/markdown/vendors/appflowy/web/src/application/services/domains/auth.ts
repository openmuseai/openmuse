export { signInWithUrl, verifyToken, getServerInfo, getAuthProviders } from '../js-services/http/auth-api';
export { resendSignUpCode } from '../js-services/http/gotrue';
export {
  loginAuth as login,
  signInGoogleWithRedirect as signInGoogle,
  signInAppleWithRedirect as signInApple,
  signInGithubWithRedirect as signInGithub,
  signInDiscordWithRedirect as signInDiscord,
  signInSamlWithRedirect as signInSaml,
  signInWithPasswordWithRedirect as signInWithPassword,
  signUpWithPasswordWithRedirect as signUpWithPassword,
  forgotPassword,
  changePassword,
  signInMagicLinkWithRedirect as signInMagicLink,
  signInOTPWithRedirect as signInOTP,
} from '../js-services/cached-api';
