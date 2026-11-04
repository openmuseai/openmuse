import 'package:openmuse_i18n/openmuse_i18n.dart';
import 'package:web/web.dart' as web;

/// HTML accessibility metadata follows the same catalog as the Flutter tree.
void syncBrowserLocale(OpenMuseLocaleController locale) {
  final option = locale.options.firstWhere((value) => value.tag == locale.tag);
  web.document.documentElement
    ?..setAttribute('lang', locale.tag)
    ..setAttribute('dir', option.direction);
}
