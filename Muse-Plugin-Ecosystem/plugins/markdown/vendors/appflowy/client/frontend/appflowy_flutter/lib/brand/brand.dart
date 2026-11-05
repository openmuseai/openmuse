/// User-visible product brand.
///
/// Display names come from [BrandValues], generated from
/// `frontend/client/brand/config.yaml` by `scripts/apply-brand.py`.
/// Edit the YAML (not this file) when the English or Chinese name changes.
import 'package:appflowy/brand/brand_values.g.dart';
import 'package:flutter/foundation.dart';

class Brand {
  Brand._();

  static const productName = BrandValues.productNameEn;
  static const productNameZh = BrandValues.productNameZh;
  static const artifactPrefix = BrandValues.artifactPrefix;
  static const binaryName = BrandValues.binaryWindows;
  static const companyName = BrandValues.companyName;
  static const dataDirName = BrandValues.dataDir;
  static const dataDirLinux = BrandValues.dataDirLinux;

  static const logoAsset = 'assets/images/dsh_office_logo.png';

  static const siteUrl = 'https://openmuseai.com';
  static const termsUrl = 'https://openmuseai.com/terms';
  static const privacyUrl = 'https://openmuseai.com/privacy';
  static const downloadUrl = 'https://openmuseai.com/download';
  static const githubUrl = 'https://github.com/openmuseai/openmuse';
  static const githubIssuesUrl =
      'https://github.com/openmuseai/openmuse/issues/new/choose';
  static const githubReleasesUrl =
      'https://github.com/openmuseai/openmuse/releases';
  static const githubDiscussionsUrl =
      'https://github.com/openmuseai/openmuse/discussions';

  /// Empty → hide the corresponding UI entry.
  static const discordUrl = '';
  static const docsUrl = '';

  static const primaryScheme = 'openmuse';
  static const legacySchemes = ['appflowy-flutter', 'dsh-office'];

  static bool isConfiguredUrl(String? url) =>
      url != null && url.trim().isNotEmpty;

  static String get docsOrSite => isConfiguredUrl(docsUrl) ? docsUrl : siteUrl;

  /// OS-locale display name. In-app copy should prefer i18n `appName`.
  static String get localizedProductName {
    final language =
        PlatformDispatcher.instance.locale.languageCode.toLowerCase();
    if (language == 'zh') {
      return productNameZh;
    }
    return productName;
  }
}
