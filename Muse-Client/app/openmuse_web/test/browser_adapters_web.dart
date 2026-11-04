import 'browser_auth_test_web.dart' as auth;
import 'browser_paired_client_test_web.dart' as paired;
import 'web_session_page_test_web.dart' as session;

void runBrowserAdapterTests() {
  auth.main();
  paired.main();
  session.main();
}
