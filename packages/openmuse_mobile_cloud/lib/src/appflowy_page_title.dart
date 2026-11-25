part of 'appflowy_cloud_workspace_service.dart';

/// SiYuan document titles count Unicode code points (SIZE_TITLE=512), not
/// filesystem path segments or UTF-16 units. Keep literal slashes and emoji.
String _checkedPageTitle(String name) {
  final title = name.trim();
  if (title.isEmpty ||
      title.runes.length > 512 ||
      RegExp(r'[\x00-\x1f\x7f\u2028\u2029]').hasMatch(title)) {
    throw ArgumentError('Invalid page title');
  }
  return title;
}
